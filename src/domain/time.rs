use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
pub struct Seconds(pub u64);

impl Seconds {
    pub fn from_seconds(secs: u64) -> Self {
        Self(secs)
    }

    pub fn from_minutes(minutes: u64) -> Self {
        Self(minutes * 60)
    }

    pub fn as_seconds(&self) -> u64 {
        self.0
    }

    pub fn as_minutes(self) -> u64 {
        self.0 / 60
    }
}
