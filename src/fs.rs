use dirs;
use errors::{Error, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

const APP_DIR: &str = ".cloak/";
const RECOVERY_CODES_DIR: &str = "recovery_codes/";
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
    let app_dir = Path::new(&dirs::home_dir().ok_or(Error::HomeDirNotFound)?).join(APP_DIR);
    let file_path = get_file_path(&app_dir, ACCOUNTS_FILE)?;
    let accounts_str = fs::read_to_string(file_path)?;
    let accounts: BTreeMap<String, Account> = toml::from_str(&accounts_str)?;
    Ok(accounts)
}

// Write a map of an `Account` struct to the `accounts` file
pub fn write(accounts: &BTreeMap<String, Account>) -> Result<()> {
    let app_dir = Path::new(&dirs::home_dir().ok_or(Error::HomeDirNotFound)?).join(APP_DIR);
    let file_path = get_file_path(&app_dir, ACCOUNTS_FILE)?;
    let accounts_str = toml::to_string(accounts)?;
    fs::write(file_path, accounts_str)?;
    Ok(())
}

// Return the file path containing the recovery codes for the account
pub fn recovery_codes(account_name: &str) -> Result<(PathBuf)> {
    let recovery_codes_dir = Path::new(&dirs::home_dir().ok_or(Error::HomeDirNotFound)?)
        .join(APP_DIR)
        .join(RECOVERY_CODES_DIR);
    get_file_path(&recovery_codes_dir, account_name)
}

// Given directory and file name, return the entire file path. If file does
// not exist, create it
fn get_file_path(dir: &PathBuf, file_name: &str) -> Result<(PathBuf)> {
    fs::create_dir_all(&dir)?;
    let file_path = dir.join(file_name);
    if !file_path.is_file() {
        create_file(&file_path)?;
    }
    Ok(file_path)
}

#[cfg(unix)]
fn create_file(file_path: &PathBuf) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut options = fs::OpenOptions::new();
    options.mode(0o600);
    let _ = options.write(true).create_new(true).open(file_path);
    Ok(())
}

#[cfg(not(unix))]
fn create_file(file_path: &PathBuf) -> Result<()> {
    let _ = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path);
    Ok(())
}
