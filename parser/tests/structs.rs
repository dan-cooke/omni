use insta::assert_json_snapshot;
use omni_parser::*;

#[test]
pub fn structs_can_be_parsed() {
    let ast = parse_file("tests/test-schemas/types/structs.omni");

    assert_json_snapshot!(ast);
}
