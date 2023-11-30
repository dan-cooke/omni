lalrpop_mod!(omni);
use std::path::Path;

use crate::{ast::File, lexer::Lexer};
use lalrpop_util::lalrpop_mod;
use omni::*;

/// Parse an Omni file
/// This is just a placeholder for the public API for now
pub fn parse_file<P: AsRef<Path>>(file_path: P) -> File {
    let source = std::fs::read_to_string(&file_path).expect(
        format!(
            "Could not find file {}",
            file_path.as_ref().to_string_lossy()
        )
        .as_str(),
    );
    let lexer = Lexer::new(&source);

    let omni_parser = FileParser::new();
    omni_parser.parse(&file_path, lexer).unwrap()
}
