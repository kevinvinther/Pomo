use crate::entities::Seconds;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub work: Seconds,
    pub short_break: Seconds,
    pub long_break: Seconds,
    pub long_break_interval: u8,
    pub auto_start_next: bool,
    pub journal_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            work: Seconds::from_minutes(25),
            short_break: Seconds::from_minutes(5),
            long_break: Seconds::from_minutes(20),
            long_break_interval: 4,
            auto_start_next: false,
            journal_path: "$HOME/.config/pomo".to_string(),
        }
    }
}

impl Config {
    pub fn from_toml_str(s: &str) -> anyhow::Result<Self> {
        let mut cfg: Config = toml::from_str(s)?;
        cfg.journal_path = shellexpand::full(&cfg.journal_path)?.into_owned();
        Ok(cfg)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.long_break_interval > 0,
            "Long break interval must be greater than 0!"
        );
        Ok(())
    }
}
