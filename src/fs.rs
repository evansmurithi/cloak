use dirs::CLOAK_DIRS;
use errors::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

const ACCOUNTS_FILE: &str = "accounts";

// Structure representing an Account
#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub key: String,
    pub totp: bool,
    pub hash_function: String,
    pub counter: Option<u64>,
}

// Read the contents of the `accounts` file to a map of an `Account` struct
pub fn read() -> Result<BTreeMap<String, Account>> {
    let file_path = get_file_path(CLOAK_DIRS.accounts_dir(), ACCOUNTS_FILE)?;
    let accounts_str = fs::read_to_string(file_path)?;
    let accounts: BTreeMap<String, Account> = toml::from_str(&accounts_str)?;
    Ok(accounts)
}

// Write a map of an `Account` struct to the `accounts` file
pub fn write(accounts: &BTreeMap<String, Account>) -> Result<()> {
    let file_path = get_file_path(CLOAK_DIRS.accounts_dir(), ACCOUNTS_FILE)?;
    let accounts_str = toml::to_string(accounts)?;
    fs::write(file_path, accounts_str)?;
    Ok(())
}

// Given directory and file name, return the entire file path. If file does
// not exist, create it
fn get_file_path(dir: &Path, file_name: &str) -> Result<PathBuf> {
    fs::create_dir_all(&dir)?;
    let file_path = dir.join(file_name);
    if !file_path.is_file() {
        create_file(&file_path)?;
    }
    Ok(file_path)
}

#[cfg(unix)]
fn create_file(file_path: &Path) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut options = fs::OpenOptions::new();
    options.mode(0o600);
    let _ = options.write(true).create_new(true).open(file_path);
    Ok(())
}

#[cfg(not(unix))]
fn create_file(file_path: &Path) -> Result<()> {
    let _ = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path);
    Ok(())
}
