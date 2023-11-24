use insta::assert_json_snapshot;
use omni_parser::*;

#[test]
pub fn services_can_be_parsed() {
    let ast = parse_file("tests/test-schemas/types/service_and_operations.omni");

    assert_json_snapshot!(ast);
}
