mod common;

use crate::common::{cloak, load_accounts_file};
use assert_fs::fixture::TempDir;

#[test]
fn view_existent_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("view")
        .arg("test_app")
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"^\d{6}\n$").unwrap());
    temp_dir.close().unwrap();
}

#[test]
fn view_account_with_length_arg() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("view")
        .arg("test_app")
        .arg("--length=8")
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"^\d{8}\n$").unwrap());
    temp_dir.close().unwrap();
}

#[test]
fn view_non_existent_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("view")
        .arg("404app")
        .assert()
        .success()
        .stdout("Account with the name '404app' does not exist. Consider adding it.\n");
    temp_dir.close().unwrap();
}
