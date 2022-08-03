mod common;

use crate::common::cloak;
use assert_fs::fixture::TempDir;
use predicates::prelude::*;

#[test]
fn no_subcommands() {
    let temp_dir = TempDir::new().unwrap();
    cloak(&temp_dir)
        .assert()
        .success()
        .stderr("No subcommand chosen. Add --help | -h to view the subcommands.\n");
    temp_dir.close().unwrap();
}

#[test]
fn add_account() {
    let temp_dir = TempDir::new().unwrap();
    cloak(&temp_dir)
        .arg("add")
        .arg("test_app")
        .arg("MFZWIYLTMRQXGZDRO5YWK4LXMVYXOZLRO4FA")
        .assert()
        .success()
        .stdout("Account successfully created\n");
    assert!(predicates::path::is_file().eval(&temp_dir.path().join("accounts")));
    temp_dir.close().unwrap();
}
