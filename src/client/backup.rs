use std::time::Duration;

use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize)]
pub struct Backup {
    pub id: u64,
    pub name: String,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub r#type: BackupType,
}

impl Backup {
    pub fn age(&self) -> time::Duration {
        let now = OffsetDateTime::now_utc();
        now - self.time
    }

    pub fn is_recent(&self, max_age: Duration) -> bool {
        self.age() < max_age
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackupType {
    Manual,
    Scheduled,
}
