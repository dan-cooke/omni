lalrpop_mod!(parser);

use parser::*;

/// Parse an Omni file
/// This is just a placeholder for the public API for now
pub fn parse_file(file_path: &str) -> File {
    let source = std::fs::read_to_string(file_path).unwrap();
    let lexer = Lexer::new(&source);
    let parser = FileParser::new();
    parser.parse(lexer).unwrap()
}

use lalrpop_util::lalrpop_mod;

use crate::{ast::File, lexer::Lexer};
