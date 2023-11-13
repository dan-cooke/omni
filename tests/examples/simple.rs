use assert_cmd::Command;

#[test]
pub fn generates_a_node_express_service_with_typescript() {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir("examples/simple")
        .assert();

    assert.success();
}
