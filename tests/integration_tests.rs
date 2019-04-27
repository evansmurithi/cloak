extern crate assert_cmd;
extern crate assert_fs;
extern crate escargot;
#[macro_use]
extern crate lazy_static;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::fixture::TempDir;
use assert_fs::prelude::*;
use escargot::CargoRun;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

lazy_static! {
    static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
        .bin("cloak")
        .current_release()
        .run()
        .unwrap();
}

fn cloak(temp_dir: &TempDir) -> Command {
    let mut cmd = CARGO_RUN.command();
    cmd.env("CLOAK_ACCOUNTS_DIR", temp_dir.path().to_str().unwrap());
    cmd
}

fn load_accounts_file(temp_dir: &TempDir) {
    temp_dir
        .child("accounts")
        .write_str(
            "
[test_app]
key = \"MFZWIYLTMRQXGZCBBI\"
totp = true
hash_function = \"SHA1\"
",
        )
        .unwrap();
}

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
    assert_eq!(
        true,
        predicates::path::is_file().eval(&temp_dir.path().join("accounts"))
    );
    temp_dir.close().unwrap();
}

#[test]
fn view_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    // normal view subcommand
    cloak(&temp_dir)
        .arg("view")
        .arg("test_app")
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"^\d{6}\n$").unwrap());
    // view subcommand with length argument
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
fn list_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"^Account: test_app\nTOTP: \d{6}\n\n\n$").unwrap());
    temp_dir.close().unwrap();
}

#[test]
fn delete_account() {
    let temp_dir = TempDir::new().unwrap();
    load_accounts_file(&temp_dir);
    cloak(&temp_dir)
        .arg("delete")
        .arg("test_app")
        .with_stdin()
        .buffer("y\n")
        .assert()
        .success()
        .stdout("Are you sure you want to delete test_app [N/y]? Account successfully deleted\n");
    assert_eq!(
        true,
        fs::read_to_string(&temp_dir.path().join("accounts"))
            .unwrap()
            .is_empty()
    );
    temp_dir.close().unwrap();
}
