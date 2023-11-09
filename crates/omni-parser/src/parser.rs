lalrpop_mod!(pub omni);

use lalrpop_util::lalrpop_mod;

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use omni_ast::{Def, File, Ident, Prop, ResourceOperation, Value};

    use super::*;

    #[test]
    pub fn _00_simple_parsing_works() {
        let source = std::fs::read_to_string("test-schemas/00_prelude/00_prelude.omni").unwrap();
        let lexer = Lexer::new(&source);
        let parser = omni::OmniFileParser::new();

        let expected_ast = File {
            imports: None,

            body: vec![
                Def::Service {
                    id: Ident {
                        id: String::from("JokeService"),
                    },
                    body: vec![Prop {
                        key: String::from("version"),
                        value: Value::String(String::from("1.0.0")),
                        decorators: None,
                    }],
                },
                Def::Read(ResourceOperation {
                    id: Ident {
                        id: String::from("GetJoke"),
                    },
                    resource: Ident {
                        id: String::from("JokeService"),
                    },
                    body: vec![Prop {
                        key: String::from("output"),
                        value: Value::Map(vec![Prop {
                            key: String::from("joke"),
                            value: Type::String,
                            decorators: None,
                        }]),
                        decorators: None,
                    }],
                }),
            ],
        };

        let ast = parser.parse(lexer).unwrap();

        assert_eq!(ast, expected_ast);
    }
    #[test]
    pub fn _01_simple_parsing_works() {
        let source = std::fs::read_to_string("test-schemas/01/01.omni").unwrap();
        let lexer = Lexer::new(&source);
        let parser = omni::OmniFileParser::new();

        let expected_ast = File {
            imports: None,

            body: vec![
                Def::Service {
                    id: Ident {
                        id: String::from("JokeService"),
                    },
                    body: vec![Prop {
                        key: String::from("version"),
                        value: Value::String(String::from("1.0.0")),
                        decorators: None,
                    }],
                },
                Def::Read(ResourceOperation {
                    id: Ident {
                        id: String::from("GetJoke"),
                    },
                    resource: Ident {
                        id: String::from("JokeService"),
                    },
                    body: vec![Prop {
                        key: String::from("output"),
                        value: Value::Map(vec![Prop {
                            key: String::from("joke"),
                            value: Type::String,
                            decorators: None,
                        }]),
                        decorators: None,
                    }],
                }),
            ],
        };

        let ast = parser.parse(lexer).unwrap();

        assert_eq!(ast, expected_ast);
    }
}
