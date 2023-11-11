use std::process::Command;

#[test]
pub fn typescript_generates_without_error() {
    let status = Command::new("make")
        .current_dir("../examples/investing")
        .arg("generate")
        .status()
        .unwrap();

    assert!(status.success());
}
