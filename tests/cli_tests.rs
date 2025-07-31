use assert_cmd::Command;
use predicates::str::is_match;

#[test]
fn generates_password_with_default_length() {
    let mut cmd = Command::cargo_bin("PassGen").unwrap();
    cmd.assert()
        .success()
        .stdout(is_match(r"^[a-z]{16}\r?\n?$").unwrap()); // 小文字のみ・16文字
}

#[test]
fn generates_password_with_uppercase_and_digits() {
    let mut cmd = Command::cargo_bin("PassGen").unwrap();
    cmd.args(["-l", "32", "-u", "-n"])
        .assert()
        .success()
        .stdout(is_match(r"^[a-zA-Z0-9]{32}\r?\n?$").unwrap());
}
