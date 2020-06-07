use crate::mode_parsing::Filter;
// use crate::mode_parsing::{Filter, Token};
// use log::info;
// use logos::{Lexer, Logos};
use lsp_types::*;
use std::fs;
use std::path::PathBuf;

pub fn hover_keyword(params: HoverParams, filter: &Filter) -> Vec<MarkedString> {
    let mut hovers = vec![];
    if let Some((string, _path)) = get_string_and_path(&params) {
        // info!("successfully got path and string");
        let byte = byte_pos_in_string(
            params.text_document_position_params.position.line as usize,
            params.text_document_position_params.position.character as usize,
            string,
        );
        if let Some(token) = filter.search_bytes(byte) {
            if let Some(keyword) = token.keyword_type() {
                let hover_keyword_text = format!("Type: {:?}\n\n{}", keyword, token.description());
                hovers.push(MarkedString::String(hover_keyword_text))
            }
        }
    }
    hovers
}

fn get_string_and_path(params: &HoverParams) -> Option<(String, PathBuf)> {
    if let Ok(path) = params
        .text_document_position_params
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

pub fn byte_pos_in_string(line: usize, char: usize, string: String) -> usize {
    let mut byte_pos: usize = 0;
    for (i, s) in string.lines().enumerate() {
        if i == line {
            byte_pos += char;
            return byte_pos;
        } else {
            byte_pos += s.len();
            byte_pos += 2;
        }
    }
    // info!("{}", byte_pos);

    byte_pos
}

// OLD
// pub fn hover_keyword(params: HoverParams) -> Vec<MarkedString> {
// 	let mut hovers = vec![];
//     if let Some((string, path)) = get_string_and_path(&params) {
//         // info!("successfully got path and string");
//         let line = string
//             .lines()
//             .nth(params.text_document_position_params.position.line as usize);
//         let character = params
//             .text_document_position_params
//             .position
//             .character
//             .clone() as usize;
//         // info!("Line: {:?} Column: {:?}", line, character);
//         let mut flag = false;
//         if let Some(line_string) = line {
//             // info!("Line String: {:?}", line_string);
// 			let mut line_lex = Token::lexer(line_string).spanned();
// 			while let Some((token, span)) = line_lex.next() {
//                 // info!("Token Found: {:?} Span: {:?}", token, span);
// 				if span.start <= character && span.end >= character {
//                     info!("Token Match {:?}", token);
//                     hovers.push(MarkedString::String(format!("Type: {:?}", token)));
//                     break
//                 }
// 			}

// 		}
//     }
//     hovers
// }
