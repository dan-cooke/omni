lalrpop_mod!(pub omni);

use lalrpop_util::lalrpop_mod;

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use omni_ast::{Def, Mod};

    use super::*;

    #[test]
    pub fn simple_parsing() {
        let source = std::fs::read_to_string("test-schemas/simple.omni").unwrap();
        let lexer = Lexer::new(&source);
        let parser = omni::ModParser::new();

        let expected_ast = Mod::Module {
            body: vec![
                Def::ServiceDef {
                    id: "service".to_string(),
                    body: vec![],
                },
                Def::ResourceDef {
                    id: "resource".to_string(),
                    body: vec![],
                },
                Def::OperationDef {
                    id: "operation".to_string(),
                    body: vec![],
                },
            ],
        };

        let ast = parser.parse(lexer).unwrap();
    }
}
