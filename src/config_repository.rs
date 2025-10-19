use crate::entities::config::Config;
use anyhow::Context;
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn get_config() -> anyhow::Result<Config> {
    let config_path = get_config_path();

    if !config_path.exists() {
        create_config(&config_path)?;
    }

    load_from_path(&config_path)
}

fn load_from_path(path: &Path) -> anyhow::Result<Config> {
    let file_contents = fs::read_to_string(path)
        .with_context(|| format!("Reading config from {}", path.display()))?;
    let cfg = Config::from_toml_str(&file_contents)
        .with_context(|| format!("Parsing TOML in {}", path.display()))?;
    Ok(cfg)
}

fn create_config(config_path: &PathBuf) -> anyhow::Result<()> {
    let cfg_default = Config::default();

    // Create directory and all parents if it doesn't exist.
    fs::create_dir_all(
        config_path
            .parent()
            .expect("Could not get config directory"),
    )
    .with_context(|| format!("Creating directory {}", &config_path.to_string_lossy()))?;

    let mut file = File::create(config_path)?;
    file.write_all(
        toml::to_string_pretty(&cfg_default)
            .expect("Could not serialize config")
            .as_bytes(),
    )?;
    Ok(())
}

fn get_config_path() -> PathBuf {
    let home = home_dir().expect(
        "\
        Pomo only works on unix systems, sorry! \
        Error: home directory could not be found.",
    );
    home.join(".config/pomo/config.toml")
}
