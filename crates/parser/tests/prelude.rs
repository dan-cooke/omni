use insta::assert_json_snapshot;
use parser::*;

#[test]
pub fn prelude_can_be_parsed() {
    let ast = parse_file("tests/test-schemas/types/prelude.omni");

    assert_json_snapshot!(ast);
}
