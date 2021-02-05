use dirs_next;
use errors::{Error, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CLOAK_DIR_NAME: &str = ".cloak/";
const CLOAK_ACCOUNTS_FILE_NAME: &str = "accounts";

fn accounts_file_path() -> Result<PathBuf> {
    let cloak_dir = env::var("CLOAK_ACCOUNTS_DIR")
        .ok()
        .map(PathBuf::from)
        .filter(|acc_dir| acc_dir.is_absolute())
        .or_else(|| dirs_next::home_dir().map(|d| d.join(CLOAK_DIR_NAME)))
        .ok_or(Error::CloakDirNotFound)?;
    fs::create_dir_all(&cloak_dir)?;
    let file_path = Path::new(&cloak_dir).join(CLOAK_ACCOUNTS_FILE_NAME);
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

lazy_static! {
    pub static ref CLOAK_ACCOUNTS_FILE_PATH: PathBuf =
        accounts_file_path().expect("Could not get cloak's accounts file path");
}
