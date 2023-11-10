lalrpop_mod!(pub omni);

use lalrpop_util::lalrpop_mod;

#[cfg(test)]
mod tests {

    use crate::ast::*;
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    pub fn _00_prelude_works() {
        let source = std::fs::read_to_string("test-schemas/00_prelude/00_prelude.omni").unwrap();
        let lexer = Lexer::new(&source);
        let parser = omni::FileParser::new();

        let expected_ast = File {
            body: vec![
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("String"),
                    },
                    _type: Type::String,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Timestamp"),
                    },
                    _type: Type::Timestamp,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Boolean"),
                    },
                    _type: Type::Boolean,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Byte"),
                    },
                    _type: Type::Byte,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Short"),
                    },
                    _type: Type::Short,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Integer"),
                    },
                    _type: Type::Integer,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Long"),
                    },
                    _type: Type::Long,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Float"),
                    },
                    _type: Type::Float,
                },
                Statement::SimpleTypeDef {
                    id: Identifier {
                        name: String::from("Double"),
                    },
                    _type: Type::Double,
                },
            ],
        };

        let ast = parser.parse(lexer).unwrap();

        assert_eq!(ast, expected_ast);
    }
}
