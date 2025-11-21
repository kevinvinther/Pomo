use crate::domain::config::Config;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::domain::time::Seconds;

pub fn default_config() -> Config {
    // TODO: Journal path should depend on OS.
    Config {
        work: Seconds::from_minutes(25),
        short_break: Seconds::from_minutes(5),
        long_break: Seconds::from_minutes(20),
        long_break_interval: 4,
        auto_start_next: false,
        journal_path: get_config_directory_path_linux().join("journals/"),
    }
}

pub fn read_config() -> anyhow::Result<Config> {
    if !get_config_path_linux().exists() {
        create_config()?;
    }

    let file_config = load_from_path(&get_config_path_linux())?;

    let config = Config::try_from(&file_config)?;

    Ok(config)
}

fn load_from_path(path: &Path) -> anyhow::Result<FileConfig> {
    let file_contents = fs::read_to_string(path)
        .with_context(|| format!("Reading config from {}", path.display()))?;
    let cfg = toml::from_str(&file_contents)
        .with_context(|| format!("Parsing TOML in {}", path.display()))?;
    Ok(cfg)
}

fn create_config() -> anyhow::Result<()> {
    let cfg_default = default_config();

    let file_config = FileConfig::try_from(&cfg_default)?;

    let config_path = get_config_path_linux();

    // Create directory and all parents if it doesn't exist.
    fs::create_dir_all(
        config_path
            .parent()
            .expect("Could not get config directory"),
    )
        .with_context(|| format!("Creating directory {}", &config_path.to_string_lossy()))?;

    let mut file = File::create(config_path)?;

    let toml_contents = toml::to_string_pretty(&file_config)?;
    file.write_all(toml_contents.as_bytes())?;
    Ok(())
}

fn get_config_path_linux() -> PathBuf {
    get_config_directory_path_linux().join("config.toml")
}

fn get_config_directory_path_linux() -> PathBuf {
    let home = home_dir().expect(
        "\
        Pomo only works on unix systems, sorry! \
        Error: home directory could not be found.",
    );
    home.join(".config/pomo")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileConfig {
    pub work_minutes: u64,
    pub short_break_minutes: u64,
    pub long_break_minutes: u64,
    pub long_break_interval: u8,
    pub auto_start_next: bool,
    pub journal_path: String,
}

fn expand_path(path: String) -> anyhow::Result<PathBuf> {
    let path = shellexpand::full(&path)?.into_owned();

    Ok(PathBuf::from(path))
}

impl TryFrom<&FileConfig> for Config {
    type Error = anyhow::Error;

    fn try_from(file_config: &FileConfig) -> Result<Self, Self::Error> {
        let expanded_path = expand_path(file_config.journal_path.clone())
            .context("Failed to expand journal_path")?;

        let config = Config {
            work: Seconds::from_minutes(file_config.work_minutes),
            short_break: Seconds::from_minutes(file_config.short_break_minutes),
            long_break: Seconds::from_minutes(file_config.long_break_minutes),
            long_break_interval: file_config.long_break_interval,
            auto_start_next: file_config.auto_start_next,
            journal_path: expanded_path,
        };

        config.validate()?;

        Ok(config)
    }
}

impl TryFrom<&Config> for FileConfig {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        Ok(FileConfig {
            work_minutes: Seconds::from(config.work).to_minutes(),
            short_break_minutes: Seconds::from(config.short_break).to_minutes(),
            long_break_minutes: Seconds::from(config.long_break).to_minutes(),
            long_break_interval: config.long_break_interval,
            auto_start_next: config.auto_start_next,
            journal_path: config.journal_path.to_string_lossy().to_string(),
        })
    }
}
