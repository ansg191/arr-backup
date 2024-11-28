use std::{sync::Arc, time::Duration};

use anyhow::Result;
use http::{HeaderValue, Request, Response};
use time::OffsetDateTime;
use tracing::{debug, error, info};
use ureq::{
    config::AutoHeaderValue,
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
                .user_agent(AutoHeaderValue::Provided(Arc::new(
                    "arr-backup/0.1.0".to_owned(),
                )))
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

    #[tracing::instrument(skip_all, fields(http.method = %req.method(), http.uri = %req.uri()))]
    fn send_request(
        &self,
        mut req: http::Request<impl AsSendBody>,
    ) -> Result<Response<Body>, ureq::Error> {
        // Add X-Api-Key header
        req.headers_mut().insert("X-Api-Key", self.api_key.clone());

        // Send request
        match self.client.run(req) {
            Ok(res) => Ok(res),
            Err(ureq::Error::StatusCode(code)) => {
                let status = http::StatusCode::from_u16(code).unwrap_or_default();
                error!(status = ?status, "Bad Status Code Response");
                Err(ureq::Error::StatusCode(code))
            }
            Err(err) => {
                error!(%err, "Request Error");
                Err(err)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn get_backups(&self) -> Result<Vec<Backup>> {
        debug!(url = self.base_url, "Getting backups");

        let request = Request::get(format!("{}/api/v3/system/backup", self.base_url)).body(())?;
        let response = self.send_request(request)?;

        Ok(response.into_body().read_json()?)
    }

    #[tracing::instrument(skip(self))]
    pub fn get_latest_backup(&self) -> Result<Option<Backup>> {
        let backups = self.get_backups()?;
        debug!(backups = backups.len(), "Found backups");
        Ok(backups
            .into_iter()
            .filter(|b| b.r#type == BackupType::Manual)
            .max_by_key(|backup| backup.time))
    }

    #[tracing::instrument(skip(self))]
    pub fn trigger_backup(&self) -> Result<()> {
        debug!(url = self.base_url, "Triggering backup");

        let request = Request::post(format!("{}/api/v3/command", self.base_url))
            .header("Content-Type", "application/json")
            .body(r#"{"name": "Backup"}"#)?;
        self.send_request(request)?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn delete_backup(&self, id: u64) -> Result<()> {
        debug!(url = self.base_url, id, "Deleting backup");

        let request =
            Request::delete(format!("{}/api/v3/system/backup/{}", self.base_url, id)).body(())?;
        self.send_request(request)?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn get_backup(&self, max_age: Duration) -> Result<Backup> {
        let backup = self.get_latest_backup()?;

        if let Some(backup) = backup {
            let age = backup.age();
            info!(
                backup.name,
                backup.id,
                backup.age = %age,
                "Latest Backup Found"
            );
            if backup.is_recent(max_age) {
                return Ok(backup);
            } else {
                info!(
                    backup.name,
                    backup.id,
                    backup.age = %age,
                    max_age = ?max_age,
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
            info!(
                immediate = true,
                url = self.base_url,
                "Waiting for backup to complete"
            );
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
