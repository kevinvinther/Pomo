use crate::entities::Seconds;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub work: Seconds,
    pub short_break: Seconds,
    pub long_break: Seconds,
    pub long_break_interval: u8,
    pub auto_start_next: bool,
    pub journal_path: PathBuf,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.long_break_interval > 0,
            "Long break interval must be greater than 0!"
        );
        anyhow::ensure!(self.work.0 > 0, "Work time must be greater than 0!");
        anyhow::ensure!(
            self.long_break.0 > 0,
            "Long break time must be greater than 0!"
        );
        anyhow::ensure!(
            self.short_break.0 > 0,
            "Short break time must be greater than 0!"
        );
        Ok(())
    }
}
