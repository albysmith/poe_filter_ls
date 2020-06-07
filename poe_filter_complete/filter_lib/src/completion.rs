use crate::data_parsing;
use crate::hover;
use crate::mode_parsing;
// use log::info;
use lsp_types::*;
use std::fs;
use std::path::PathBuf;

pub fn completion_parse(
    params: CompletionParams,
    filter: &mode_parsing::Filter,
    poe_data: data_parsing::PoeData,
) -> Vec<CompletionItem> {
    if let Some((string, _path)) = get_string_and_path(&params) {
        let byte = hover::byte_pos_in_string(
            params.text_document_position.position.line as usize,
            params.text_document_position.position.character as usize,
            string,
        );

        if let Some(context) = params.context {
            if let Some(character) = context.trigger_character {
                // info!("{:?}", character);
                if character == "\"".to_string() {
                    if let Some(block) = filter.search_block(byte) {
                        if let Some(keyword) = block.search_keyword(byte) {
                            if let Some(list) = keyword.valid_values(poe_data) {
                                let completion = list
                                    .iter()
                                    .filter_map(|r| r.name.clone())
                                    .map(|n| CompletionItem {
                                        label: n,
                                        kind: Some(CompletionItemKind::Value),
                                        ..CompletionItem::default()
                                    })
                                    .collect::<Vec<_>>();
                                return completion;
                            }
                        }
                    }
                }
            }
        }

        if let Some(block) = filter.search_block(byte) {
            if let Some(keyword) = block.search_keyword(byte) {
                if let Some(ktype) = keyword.keyword_type() {
                    let list: Vec<mode_parsing::Token> = match ktype {
                        mode_parsing::KeywordType::Conditions => {
                            let cond = ktype.token_list();
                            let act = mode_parsing::KeywordType::Actions.token_list();
                            let out = cond.into_iter().chain(act.into_iter()).collect::<Vec<_>>();
                            out
                        }
                        mode_parsing::KeywordType::Actions => ktype.token_list(),
                        mode_parsing::KeywordType::Block => {
                            mode_parsing::KeywordType::Conditions.token_list()
                        }
                        mode_parsing::KeywordType::Operations => vec![],
                        mode_parsing::KeywordType::Values(_) => vec![],
                    };
                    let out = list
                        .into_iter()
                        .chain(mode_parsing::KeywordType::Block.token_list().into_iter())
                        .map(|t| t.create_completion_item())
                        .collect::<Vec<_>>();
                    return out;
                }
            } else {
                let block = mode_parsing::KeywordType::Block.token_list();
                let cond = mode_parsing::KeywordType::Conditions.token_list();
                let out = block
                    .into_iter()
                    .chain(cond.into_iter())
                    .map(|t| t.create_completion_item())
                    .collect::<Vec<_>>();
                return out;
            }
        }
    }

    let block = mode_parsing::KeywordType::Block.token_list();
    let cond = mode_parsing::KeywordType::Conditions.token_list();
    let act = mode_parsing::KeywordType::Actions.token_list();

    let out = block
        .into_iter()
        .chain(cond.into_iter())
        .chain(act.into_iter())
        .map(|t| t.create_completion_item())
        .collect::<Vec<_>>();
    out
}

pub fn get_string_and_path(params: &CompletionParams) -> Option<(String, PathBuf)> {
    if let Ok(path) = params
        .text_document_position
        .text_document
        .uri
        .to_file_path()
    {
        if let Ok(string) = fs::read_to_string(&path) {
            Some((string, path))
        } else {
            None
        }
    } else {
        None
    }
}
