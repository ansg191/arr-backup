use std::time::Duration;

use anyhow::Result;
use http::{HeaderValue, Request, Response};
use time::OffsetDateTime;
use tracing::{debug, info};
use ureq::{
    tls::{RootCerts, TlsConfig},
    Agent, AsSendBody, Body,
};

mod backup;
pub use backup::{Backup, BackupType};

pub struct Client {
    client: Agent,
    base_url: String,
    api_key: HeaderValue,
}

impl Client {
    pub fn new(base_url: impl Into<String>, api_key: impl AsRef<str>) -> Result<Self> {
        let base_url = base_url.into();
        let mut api_key = HeaderValue::from_str(api_key.as_ref())?;
        api_key.set_sensitive(true);
        Ok(Self {
            client: Agent::config_builder()
                .user_agent(Some("arr-backup/0.1.0".to_owned()))
                .tls_config(
                    TlsConfig::builder()
                        .root_certs(RootCerts::PlatformVerifier)
                        .build(),
                )
                .build()
                .new_agent(),
            base_url,
            api_key,
        })
    }

    fn send_request(
        &self,
        mut req: http::Request<impl AsSendBody>,
    ) -> Result<Response<Body>, ureq::Error> {
        // Add X-Api-Key header
        req.headers_mut().insert("X-Api-Key", self.api_key.clone());

        // Send request
        self.client.run(req)
    }

    pub fn get_backups(&self) -> Result<Vec<Backup>> {
        debug!(url = self.base_url, "Getting backups");

        let request = Request::get(format!("{}/api/v3/system/backup", self.base_url)).body(())?;
        let response = self.send_request(request)?;

        Ok(response.into_body().read_json()?)
    }

    pub fn get_latest_backup(&self) -> Result<Option<Backup>> {
        let backups = self.get_backups()?;
        Ok(backups
            .into_iter()
            .filter(|b| b.r#type == BackupType::Manual)
            .max_by_key(|backup| backup.time))
    }

    pub fn trigger_backup(&self) -> Result<()> {
        debug!(url = self.base_url, "Triggering backup");

        let request = Request::post(format!("{}/api/v3/command", self.base_url))
            .header("Content-Type", "application/json")
            .body(r#"{"name": "Backup"}"#)?;
        self.send_request(request)?;

        Ok(())
    }

    pub fn delete_backup(&self, id: u64) -> Result<()> {
        debug!(url = self.base_url, id, "Deleting backup");

        let request =
            Request::delete(format!("{}/api/v3/system/backup/{}", self.base_url, id)).body(())?;
        self.send_request(request)?;

        Ok(())
    }

    pub fn get_backup(&self, max_age: Duration) -> Result<Backup> {
        let backup = self.get_latest_backup()?;

        if let Some(backup) = backup {
            let age = backup.age();
            info!(
                url = self.base_url,
                backup.name,
                backup.id,
                backup.age = age.as_seconds_f32(),
                "Latest Backup Found"
            );
            if backup.is_recent(max_age) {
                return Ok(backup);
            } else {
                info!(
                    url = self.base_url,
                    backup.name,
                    backup.id,
                    backup.age = age.as_seconds_f32(),
                    max_age = max_age.as_secs_f32(),
                    "Backup is too old"
                );
            }
        }

        // Trigger backup if no backup found or backup is too old
        self.trigger_backup()?;

        // Wait for backup to complete
        const TIMEOUT: Duration = Duration::from_secs(60);
        let start = OffsetDateTime::now_utc();
        loop {
            if start + TIMEOUT < OffsetDateTime::now_utc() {
                return Err(anyhow::anyhow!("Backup creation timed out"));
            }

            // Check if backup is complete
            if let Some(backup) = self.get_latest_backup()? {
                // Check if backup is recent now (because we just created it)
                if backup.is_recent(max_age) {
                    return Ok(backup);
                }
            }

            // Backup is not complete yet, wait a bit and try again
            info!(url = self.base_url, "Waiting for backup to complete");
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
