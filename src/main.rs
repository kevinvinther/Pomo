extern crate core;

mod application;
mod domain;
mod infrastructure;

use crate::application::pomodoro;
use crate::domain::config::Config;
use crate::domain::session_kind::SessionKind;
use crate::infrastructure::config::file_repository;
use crate::infrastructure::journaling::{Journal, MarkdownJournal};
use crate::infrastructure::notifications::{DesktopNotifier, Notifier};
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
    let config = file_repository::read_config()?;
    config.validate()?;

    let notifier = DesktopNotifier;
    let journal = MarkdownJournal;

    match args.cmd {
        Some(Command::Start) => timer_loop(config, &notifier, &journal),
        _ => Ok(()),
    }
    .with_context(|| "Matching commands")?;

    Ok(())
}

fn timer_loop(
    config: Config,
    notifier: &dyn Notifier,
    journal: &dyn Journal,
) -> anyhow::Result<()> {
    let total_cycles = config.long_break_interval;

    for session_num in 1..=total_cycles {
        pomodoro::start(SessionKind::Work, &config, notifier, journal)
            .with_context(|| format!("Starting work session #{session_num}"))?;

        if session_num < total_cycles {
            pomodoro::start(SessionKind::ShortBreak, &config, notifier, journal)
                .with_context(|| format!("Starting short break after session #{session_num}"))?;
        } else {
            pomodoro::start(SessionKind::LongBreak, &config, notifier, journal)
                .with_context(|| format!("Starting long break after session #{session_num}"))?;
        }
    }

    Ok(())
}
