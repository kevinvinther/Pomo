pub mod config;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
pub struct Seconds(pub u64);

impl Seconds {
    pub fn from_minutes(minutes: u64) -> Self {
        Self(minutes * 60)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SessionKind {
    Work,
    ShortBreak,
    LongBreak,
}
