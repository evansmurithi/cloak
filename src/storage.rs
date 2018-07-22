use std::error::Error;
use std::fs;
use toml;

const FILE_PATH: &str = "/home/evans/.2fa";

#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts {
    accounts: Option<Vec<Account>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub key: String,
    pub hash_function: String,
    pub counter: Option<u64>,
}

pub fn get(name: &str) -> Result<Option<Account>, Box<Error>> {
    let accounts_toml = fs::read_to_string(FILE_PATH).expect("Unable to read file");
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
    let accounts_toml = fs::read_to_string(FILE_PATH).expect("Unable to read file");
    let accounts: Accounts = toml::from_str(&accounts_toml).unwrap();
    Ok(accounts.accounts.unwrap_or(vec![]))
}
