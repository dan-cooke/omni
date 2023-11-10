use omni_parser::*;

#[test]
pub fn prelude_can_be_parsed() {
    let ast = parse_file("tests/test-schemas/types/prelude.omni");

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

    assert_eq!(ast, expected_ast);
}
