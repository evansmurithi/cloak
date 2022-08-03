mod common;

use crate::common::{cloak, load_accounts_file};
use assert_fs::fixture::TempDir;
use std::fs;

#[test]
fn delete_existent_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("delete")
        .arg("test_app")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout("Are you sure you want to delete test_app [N/y]? Account successfully deleted\n");
    assert!(fs::read_to_string(&temp_dir.path().join("accounts"))
        .unwrap()
        .is_empty());
    temp_dir.close().unwrap();
}

#[test]
fn delete_non_existent_account() {
    let temp_dir = TempDir::new().unwrap();
    cloak(&temp_dir)
        .arg("delete")
        .arg("404app")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout("Are you sure you want to delete 404app [N/y]? Account does not exist\n");
}
