use std::{fs::File, path::Path, time::Duration};

use anyhow::{Context, Result};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use zip::result::ZipError;

use crate::{
    client::{Backup, Client},
    config::Config,
};

mod client;
mod config;

fn main() -> Result<()> {
    let config = Config::from_env()?;

    if config.production {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();
    }

    inner_main(config)
}

fn inner_main(config: Config) -> Result<()> {
    pre_checks(&config)?;

    let client = Client::new(&config.base_url, &config.api_key)?;

    let backup = client.get_backup(Duration::from_secs(3600))?;
    info!(backup.name, backup.id, "Found backup");

    copy_backup(&config, &backup)?;

    if config.delete_backup {
        client.delete_backup(backup.id)?;
    } else {
        debug!(backup.id, "Skipping backup deletion");
    }

    info!("Backup complete");

    Ok(())
}

fn pre_checks(config: &Config) -> Result<()> {
    // Check destination directory exists
    if !config.dest_dir.exists() {
        anyhow::bail!("Destination directory does not exist");
    }
    // Check config directory exists
    if !config.config_dir.exists() {
        anyhow::bail!("Config directory does not exist");
    }
    // Check destination directory is empty
    if config.dest_dir.read_dir()?.next().is_some() {
        anyhow::bail!("Destination directory is not empty");
    }

    Ok(())
}

/// Gets backup zip file from config dir and unzips into dest dir.
fn copy_backup(config: &Config, backup: &Backup) -> Result<()> {
    let backup_file = config.config_dir.join("Backups/manual").join(&backup.name);
    info!(
        src = backup_file.to_str(),
        dst = config.dest_dir.to_str(),
        "Copying backup"
    );

    let file = File::open(&backup_file).context("Failed to open backup file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read backup zip file")?;
    // archive
    //     .extract(&config.dest_dir)
    //     .context("Failed to extract backup")?;
    extract_archive(&mut archive, &config.dest_dir)?;
    Ok(())
}

/// Extracts a zip archive to a directory.
///
/// Does not handle symlinks or permissions.
fn extract_archive(archive: &mut zip::ZipArchive<File>, directory: &Path) -> Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let filepath = file
            .enclosed_name()
            .ok_or(ZipError::InvalidArchive("Invalid file path"))?;

        let outpath = directory.join(filepath);
        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
            continue;
        }
        // Should be no symlinks
        if outpath.is_symlink() {
            error!(path = outpath.to_str().unwrap(), "symlink encountered");
            anyhow::bail!("symlink encountered");
        }

        let mut outfile = File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
    }

    Ok(())
}
