use dirs;
use open;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use toml;

const APP_DIR: &str = ".2fa/";
const RECOVERY_CODES_DIR: &str = "recovery_codes/";
const ACCOUNTS_FILE: &str = "accounts";

#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts {
    accounts: Option<Vec<Account>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub key: String,
    pub totp: bool,
    pub hash_function: String,
    pub counter: Option<u64>,
}

pub fn get(name: &str) -> Result<Option<Account>, Box<Error>> {
    let file_path = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(ACCOUNTS_FILE);
    let accounts_toml = fs::read_to_string(file_path).expect("Unable to read file");
    let accounts: Accounts = toml::from_str(&accounts_toml).unwrap();
    let mut found_account: Option<Account> = None;
    for account in accounts.accounts.unwrap() {
        if account.name == name {
            found_account = Some(account);
        }
    }
    Ok(found_account)
}

pub fn list() -> Result<Vec<Account>, Box<Error>> {
    let file_path = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(ACCOUNTS_FILE);
    let accounts_toml = fs::read_to_string(file_path).expect("Unable to read file");
    let accounts: Accounts = toml::from_str(&accounts_toml).unwrap();
    Ok(accounts.accounts.unwrap_or(vec![]))
}

pub fn open_recovery_codes(account_name: &str) -> io::Result<()> {
    let recovery_codes_dir = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(RECOVERY_CODES_DIR);
    let file_path = recovery_codes_dir.join(account_name);
    fs::create_dir_all(&recovery_codes_dir)?;
    if !file_path.is_file() {
        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path);
    }
    match open::that(file_path) {
        Ok(_) => {}
        Err(err) => println!("Error {}", err),
    };
    Ok(())
}
