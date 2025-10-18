extern crate core;

mod app;
mod entities;
mod helpers;

use crate::entities::{Config, SessionKind};
use anyhow::Context;
use clap::*;
use tokio::fs;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Start,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config_path = "$HOME/.config/pomo";

    let config = Config::default();

    // Create directory and all parents if it doesn't exist.
    fs::create_dir_all(&config_path)
        .await
        .with_context(|| format!("Creating directory {}", config_path))?;

    match args.cmd {
        Some(Command::Start) => timer_loop(config),
        _ => Ok(()),
    }
    .with_context(|| "Matching commands")?;

    Ok(())
}

fn timer_loop(config: Config) -> anyhow::Result<()> {
    let total_cycles = config.long_break_interval;

    for session_num in 1..=total_cycles {
        app::start(SessionKind::Work, &config)
            .with_context(|| format!("Starting work session #{session_num}"))?;

        if session_num < total_cycles {
            app::start(SessionKind::ShortBreak, &config)
                .with_context(|| format!("Starting short break after session #{session_num}"))?;
        } else {
            app::start(SessionKind::LongBreak, &config)
                .with_context(|| format!("Starting long break after session #{session_num}"))?;
        }
    }

    Ok(())
}
