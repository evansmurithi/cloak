extern crate assert_cmd;
extern crate assert_fs;
extern crate escargot;
extern crate lazy_static;
extern crate predicates;

use assert_cmd::Command;
use assert_fs::fixture::TempDir;
use assert_fs::prelude::*;
use escargot::CargoRun;
use lazy_static::lazy_static;

lazy_static! {
    static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
        .bin("cloak")
        .current_release()
        .run()
        .unwrap();
}

#[allow(dead_code)]
pub fn cloak(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::from(CARGO_RUN.command());
    cmd.env("CLOAK_ACCOUNTS_DIR", temp_dir.path().to_str().unwrap());
    cmd
}

#[allow(dead_code)]
pub fn load_accounts_file(temp_dir: &TempDir) {
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
