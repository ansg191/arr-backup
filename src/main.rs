use std::{fs::File, path::Path, time::Duration};

use anyhow::{Context, Result};
use slog::{crit, debug, info, o, Drain, Logger};
use zip::result::ZipError;

use crate::{
    client::{Backup, Client},
    config::Config,
};

mod client;
mod config;

fn main() -> Result<()> {
    let config = Config::from_env()?;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(drain);
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());

    inner_main(config, log)
}

fn inner_main(config: Config, logger: Logger) -> Result<()> {
    pre_checks(&config)?;

    let client = Client::new(logger.clone(), &config.base_url, &config.api_key)?;

    let backups = client.get_backup(Duration::from_secs(3600))?;
    info!(logger, "Found backup"; "backup" => &backups.name, "id" => backups.id);

    copy_backup(&config, logger.clone(), &backups)?;

    if config.delete_backup {
        client.delete_backup(backups.id)?;
    } else {
        debug!(logger, "Skipping backup deletion"; "id" => backups.id);
    }

    info!(logger, "Backup complete");

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
fn copy_backup(config: &Config, logger: Logger, backup: &Backup) -> Result<()> {
    let backup_file = config.config_dir.join("Backups/manual").join(&backup.name);
    info!(logger, "Copying backup"; "src" => backup_file.to_str(), "dst" => config.dest_dir.to_str());

    let file = File::open(&backup_file).context("Failed to open backup file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read backup zip file")?;
    // archive
    //     .extract(&config.dest_dir)
    //     .context("Failed to extract backup")?;
    extract_archive(&logger, &mut archive, &config.dest_dir)?;
    Ok(())
}

/// Extracts a zip archive to a directory.
///
/// Does not handle symlinks or permissions.
fn extract_archive(
    logger: &Logger,
    archive: &mut zip::ZipArchive<File>,
    directory: &Path,
) -> Result<()> {
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
            crit!(
                logger,
                "symlink encountered";
                "path" => outpath.to_str().unwrap(),
            );
            anyhow::bail!("symlink encountered");
        }

        let mut outfile = File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
    }

    Ok(())
}
