use dirs_rs;
use std::env;
use std::path::{Path, PathBuf};

pub struct CloakAppDirs {
    accounts_dir: PathBuf,
}

impl CloakAppDirs {
    fn new() -> Option<CloakAppDirs> {
        let accounts_dir = env::var("CLOAK_ACCOUNTS_DIR")
            .ok()
            .map(PathBuf::from)
            .filter(|acc_dir| acc_dir.is_absolute())
            .or_else(|| dirs_rs::home_dir().map(|d| d.join(".cloak/")))?;

        Some(CloakAppDirs { accounts_dir })
    }

    pub fn accounts_dir(&self) -> &Path {
        &self.accounts_dir
    }
}

lazy_static! {
    pub static ref CLOAK_DIRS: CloakAppDirs =
        CloakAppDirs::new().expect("Could not get cloak's accounts directory");
}
