use filter_lib::*;
use log::info;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::*;
use lsp_types::request::*;
use lsp_types::*;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    flexi_logger::Logger::with_str("info").start().unwrap();
    // info!("starting POE Filter LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_settings = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Full),
                will_save: None,
                will_save_wait_until: None,
                save: Some(SaveOptions::default()),
            },
        )),
        hover_provider: Some(true),
        completion_provider: Some(CompletionOptions {
            resolve_provider: None,
            trigger_characters: Some(vec!["\"".to_string()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        // signature_help_provider: Some(SignatureHelpOptions {
        //     trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
        //     retrigger_characters: None,
        //     work_done_progress_options: WorkDoneProgressOptions {
        //         work_done_progress: None,
        //     },
        // }),
        // declaration_provider: None,
        // definition_provider: Some(true),
        // type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        // implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        // references_provider: Some(true),
        // document_highlight_provider: Some(true),
        // document_symbol_provider: Some(true),
        // workspace_symbol_provider: Some(true),
        // code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
        // Advertise support for all built-in CodeActionKinds
        // code_action_kinds: Some(vec![
        // lsp_types::code_action_kind::EMPTY.to_string(),
        // lsp_types::code_action_kind::QUICKFIX.to_string(),
        // lsp_types::code_action_kind::REFACTOR.to_string(),
        // lsp_types::code_action_kind::REFACTOR_EXTRACT.to_string(),
        // lsp_types::code_action_kind::REFACTOR_INLINE.to_string(),
        // lsp_types::code_action_kind::REFACTOR_REWRITE.to_string(),
        // lsp_types::code_action_kind::SOURCE.to_string(),
        // lsp_types::code_action_kind::SOURCE_ORGANIZE_IMPORTS.to_string(),
        //     ]),
        //     work_done_progress_options: Default::default(),
        // })),
        // code_lens_provider: Some(CodeLensOptions {
        //     resolve_provider: Some(true),
        // }),
        // document_formatting_provider: Some(true),
        // document_range_formatting_provider: None,
        // document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
        //     first_trigger_character: "=".to_string(),
        //     more_trigger_character: Some(vec![".".to_string(), ">".to_string()]),
        // }),
        // selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        // semantic_highlighting: None,
        // folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        // rename_provider: Some(RenameProviderCapability::Options(RenameOptions {
        //     prepare_provider: Some(true),
        //     work_done_progress_options: WorkDoneProgressOptions {
        //         work_done_progress: None,
        //     },
        // })),
        // document_link_provider: None,
        // color_provider: None,
        // execute_command_provider: None,
        // workspace: None,
        // call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        // semantic_tokens_provider: Some(
        //     SemanticTokensOptions {
        //         legend: SemanticTokensLegend {
        //             token_types: semantic_tokens::SUPPORTED_TYPES.to_vec(),
        //             token_modifiers: semantic_tokens::SUPPORTED_MODIFIERS.to_vec(),
        //         },

        //         document_provider: Some(SemanticTokensDocumentProvider::Bool(true)),
        //         range_provider: Some(true),
        //         work_done_progress_options: Default::default(),
        //     }
        //     .into(),
        // ),
        experimental: Default::default(),
        ..ServerCapabilities::default()
    };
    let server_capabilities = serde_json::to_value(&server_settings).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(&connection, initialization_params)?;
    io_threads.join()?;

    // info!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: &Connection,
    _params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    // info!("starting main loop");

    let mut parsed_filter = mode_parsing::Filter::default();
    let poe_data = data_parsing::PoeData::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let mut request = ReqMessage { req };
                if let Some(resp) = handle_hover(&mut request, &parsed_filter) {
                    // info!("request: {:?}", resp);
                    handle_request(connection, resp);
                }
                if let Some(resp) =
                    handle_completion(&mut request, &parsed_filter, poe_data.clone())
                {
                    handle_request(connection, resp);
                }
            }
            Message::Response(_resp) => {}
            Message::Notification(not) => {
                let mut notification = NotMessage { not };
                if let Some(parsed) = handle_save(&mut notification) {
                    parsed_filter = parsed;
                }
                if let Some(parsed) = handle_change(&mut notification) {
                    parsed_filter = parsed;
                }
                if let Some(parsed) = handle_open(&mut notification) {
                    parsed_filter = parsed;
                }
            }
        }
    }
    Ok(())
}

fn handle_request(connection: &Connection, response: Response) {
    let _resp = connection.sender.send(Message::Response(response));
}

fn handle_hover(
    request: &mut ReqMessage,
    parsed_filter: &mode_parsing::Filter,
) -> Option<Response> {
    if let Ok((id, params)) = request.cast::<HoverRequest>() {
        let result = Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Array(hover::hover_keyword(params, parsed_filter)),
            range: None,
        });
        let result = serde_json::to_value(&result).unwrap();
        let resp = Response {
            id,
            result: Some(result),
            error: None,
        };
        return Some(resp);
    }
    None
}

// TODO swap vec![] with completion function
fn handle_completion(
    request: &mut ReqMessage,
    parsed_filter: &mode_parsing::Filter,
    poe_data: data_parsing::PoeData,
) -> Option<Response> {
    if let Ok((id, params)) = request.cast::<Completion>() {
        if let Ok(json) = serde_json::to_value(&CompletionResponse::Array(
            completion::completion_parse(params, parsed_filter, poe_data),
        )) {
            return Some(Response {
                id,
                result: Some(json),
                error: None,
            });
        }
    }
    None
}

fn handle_change(notification: &mut NotMessage) -> Option<mode_parsing::Filter> {
    if let Ok(params) = notification.cast::<DidChangeTextDocument>() {
        let filter_file = params.content_changes;
        let text = &filter_file[0].text;
        return Some(mode_parsing::parse(text));
    }
    None
}
fn handle_save(notification: &mut NotMessage) -> Option<mode_parsing::Filter> {
    if let Ok(params) = notification.cast::<DidSaveTextDocument>() {
        if let Ok(path) = params.text_document.uri.to_file_path() {
            if let Ok(text) = fs::read_to_string(&path) {
                return Some(mode_parsing::parse(&text));
            }
        }
    }
    None
}
fn handle_open(notification: &mut NotMessage) -> Option<mode_parsing::Filter> {
    if let Ok(params) = notification.cast::<DidOpenTextDocument>() {
        if let Ok(path) = params.text_document.uri.to_file_path() {
            if let Ok(text) = fs::read_to_string(&path) {
                return Some(mode_parsing::parse(&text));
            }
        }
    }
    None
}

#[derive(Clone)]
struct ReqMessage {
    req: Request,
}

impl ReqMessage {
    fn cast<R>(&mut self) -> Result<(RequestId, R::Params), Request>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        self.clone().req.extract(R::METHOD)
    }
}

#[derive(Clone)]
struct NotMessage {
    not: Notification,
}
impl NotMessage {
    fn cast<N>(&mut self) -> Result<N::Params, Notification>
    where
        N: lsp_types::notification::Notification,
        N::Params: serde::de::DeserializeOwned,
    {
        self.clone().not.extract(N::METHOD)
    }
}
