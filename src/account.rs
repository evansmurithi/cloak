use crate::dirs::CLOAK_ACCOUNTS_FILE_PATH;
use crate::errors::Result;
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub key: String,
    pub totp: bool,
    pub hash_function: String,
    pub counter: Option<u64>,
}

pub struct AccountStore {
    accounts: BTreeMap<String, Account>,
}

impl AccountStore {
    pub fn new() -> Result<AccountStore> {
        let accounts_str = fs::read_to_string(CLOAK_ACCOUNTS_FILE_PATH.as_path())?;
        let accounts: BTreeMap<String, Account> = toml::from_str(&accounts_str)?;
        Ok(AccountStore { accounts })
    }

    pub fn get(&self, account_name: &str) -> Option<&Account> {
        self.accounts.get(account_name)
    }

    pub fn list(&self) -> &BTreeMap<String, Account> {
        &self.accounts
    }

    pub fn add(&mut self, account_name: String, account: Account) {
        self.accounts.insert(account_name, account);
    }

    pub fn delete(&mut self, account_name: &str) -> Option<Account> {
        self.accounts.remove(account_name)
    }

    pub fn save(&self) -> Result<()> {
        let accounts_str = toml::to_string(&self.accounts)?;
        fs::write(CLOAK_ACCOUNTS_FILE_PATH.as_path(), accounts_str)?;
        Ok(())
    }
}
