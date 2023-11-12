use std::process::Command;

#[test]
pub fn generates_a_node_express_service_with_typescript() {
    let status = Command::new("make")
        .current_dir("../examples/investing")
        .arg("generate")
        .status()
        .unwrap();

    assert!(status.success());
}
