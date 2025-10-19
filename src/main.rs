extern crate core;

mod app;
mod config_repository;
mod entities;
mod helpers;

use crate::entities::SessionKind;
use crate::entities::config::Config;
use anyhow::Context;
use clap::*;

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
    let config = config_repository::get_config()?;
    config.validate()?;

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
