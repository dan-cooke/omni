use havocompare::compare_folders;
use std::{fs, path::PathBuf, str::FromStr};

use assert_cmd::Command;

#[test]
pub fn generates_a_node_express_service_with_typescript() {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("generate")
        .current_dir("examples/simple")
        .assert();

    assert.success();

    let mut project = fs::read_dir("examples/node_simple").unwrap();

    // Has the correct files
    project.any(|f| f.unwrap().file_name() == "main.omni");
    project.any(|f| f.unwrap().file_name() == "Omni.toml");

    // Has generated the correct files
    let result = compare_folders(
        "tests/examples/node_simple/expected/",
        "examples/node_simple/generated/",
        "tests/examples/node_simple/compare.yml",
        "tests/examples/node_simple/report/",
    )
    .unwrap();

    assert!(result);
}
