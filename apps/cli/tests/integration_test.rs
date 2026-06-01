use assert_cmd::Command;

#[test]
fn test_help_succeeds() {
    let mut cmd = Command::cargo_bin("{{ project-name }}").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_version_succeeds() {
    let mut cmd = Command::cargo_bin("{{ project-name }}").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}
