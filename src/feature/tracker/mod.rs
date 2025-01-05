use chrono::{DateTime, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};
pub mod flatfile;
pub mod reporter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);

impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);
impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
    pub start: StartTime,
    pub end: EndTime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StartupStatus {
    Running,
    Started,
}

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct TrackerError;
pub trait Tracker {
    fn start(&mut self) -> Result<StartupStatus, TrackerError>;

    fn is_running(&self) -> bool;

    fn stop(&mut self) -> Result<(), TrackerError>;

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError>;
}
