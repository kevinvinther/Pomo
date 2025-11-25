use crate::domain::config::Config;
use crate::domain::session_kind::SessionKind;
use crate::domain::time::Seconds;
use crate::infrastructure::journaling::Journal;
use crate::infrastructure::notifications::Notifier;
use anyhow::Context;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{io, thread};

pub fn start(
    kind: SessionKind,
    config: &Config,
    notifier: &dyn Notifier,
    journal: &dyn Journal,
) -> anyhow::Result<()> {
    match kind {
        SessionKind::Work => {
            work_session(
                &config.work,
                "Work time!",
                Path::new(&config.journal_path),
                notifier,
                journal,
            )?;
            handle_pause(config)?;
            Ok(())
        }
        SessionKind::LongBreak => {
            break_session(&config.long_break, "Long break!", notifier)?;
            handle_pause(config)?;
            Ok(())
        }
        SessionKind::ShortBreak => {
            break_session(&config.short_break, "Short break!", notifier)?;
            handle_pause(config)?;
            Ok(())
        }
    }
}

fn handle_pause(config: &Config) -> anyhow::Result<()> {
    if !config.auto_start_next {
        println!("Press the enter key to continue...");
        io::stdin().read_line(&mut String::new())?;
    }
    Ok(())
}

fn work_session(
    minutes: &Seconds,
    message: &str,
    path: &Path,
    notifier: &dyn Notifier,
    journal: &dyn Journal,
) -> anyhow::Result<()> {
    run_timer(minutes, message);
    notify_work_done(notifier).context("Sending work done notification")?;
    journal
        .create_journal(path.to_path_buf())
        .context("Creating journal")?;

    Ok(())
}

fn break_session(minutes: &Seconds, message: &str, notifier: &dyn Notifier) -> anyhow::Result<()> {
    run_timer(minutes, message);
    notify_break_done(notifier)?;
    Ok(())
}

fn run_timer(seconds: &Seconds, message: &str) {
    let total_secs = seconds.0;
    let pb = setup_progressbar(message, total_secs);

    let start = Instant::now();

    for elapsed in 0..=total_secs {
        let remaining = total_secs.saturating_sub(elapsed);

        pb.set_message(format!("{mmss} {message}", mmss = format_mmss(remaining)));
        pb.set_position(elapsed);

        if elapsed == total_secs {
            break;
        }

        let next_tick = start + Duration::from_secs(elapsed + 1);
        thread::sleep(next_tick.saturating_duration_since(Instant::now()));
    }

    pb.finish_with_message(format!("00:00 {message} - Done!"));
}

fn format_mmss(total_secs: u64) -> String {
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{mins:02}:{secs:02}")
}

fn setup_progressbar(message: &str, seconds_value: u64) -> ProgressBar {
    let pb = ProgressBar::new(seconds_value + 1);

    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_style(ProgressStyle::with_template("[{bar:40.cyan/blue}] {msg}").unwrap());
    pb.set_message(format!(
        "{:#02}:{:#02} {message}",
        seconds_value / 60,
        seconds_value % 60
    ));
    pb.set_length(seconds_value);
    pb.set_position(0);

    pb
}

fn notify_work_done(notifier: &dyn Notifier) -> anyhow::Result<()> {
    notifier.send_notification("Work done!", "Your timer has completed, time to journal!")?;
    Ok(())
}

fn notify_break_done(notifier: &dyn Notifier) -> anyhow::Result<()> {
    notifier.send_notification("Break done!", "It's time to get back to working!")?;
    Ok(())
}
