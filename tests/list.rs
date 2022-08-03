mod common;

use crate::common::{cloak, load_accounts_file};
use assert_fs::fixture::TempDir;

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
