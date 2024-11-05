use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct Config {
    pub base_url: String,
    pub api_key: String,

    pub config_dir: PathBuf,
    pub dest_dir: PathBuf,

    pub delete_backup: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("ARR_URL").context("ARR_URL missing")?;
        let api_key = std::env::var("ARR_API_KEY").context("ARR_API_KEY missing")?;
        let config_dir = std::env::var_os("ARR_CONFIG_DIR")
            .map(PathBuf::from)
            .context("ARR_CONFIG_DIR missing")?;
        let dest_dir = std::env::var_os("ARR_DEST_DIR")
            .map(PathBuf::from)
            .context("ARR_DEST_DIR missing")?;
        let delete_backup = std::env::var("ARR_DELETE_BACKUP")
            .map(|s| s.parse())
            .unwrap_or(Ok(true))?;
        Ok(Self {
            base_url,
            api_key,
            config_dir,
            dest_dir,
            delete_backup,
        })
    }
}
