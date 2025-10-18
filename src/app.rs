use crate::entities::{Config, Seconds, SessionKind};
use crate::helpers::send_notification;
use anyhow::Context;
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use std::{env, io, thread};

pub fn start(kind: SessionKind, config: &Config) -> anyhow::Result<()> {
    match kind {
        SessionKind::Work => {
            work_session(&config.work, "Work time!", Path::new(&config.path))?;
            handle_pause(config)?;
            Ok(())
        },
        SessionKind::LongBreak => {
            break_session(&config.long_break, "Long break!")?;
            handle_pause(config)?;
            Ok(())
        },
        SessionKind::ShortBreak => {
            break_session(&config.short_break, "Short break!")?;
            handle_pause(config)?;
            Ok(())
        },
    }
}

fn handle_pause(config: &Config) -> anyhow::Result<()> {
    if !config.auto_start_next {
        println!("Press the enter key to continue...");
        io::stdin()
            .read_line(&mut String::new())?;
    }
    Ok(())
}

fn work_session(minutes: &Seconds, message: &str, path: &Path) -> anyhow::Result<()> {
    run_timer(minutes, message);
    send_notification("Work done!", "Your timer has completed, time to journal!")?;
    start_journal(path).with_context(|| "Couldn't start journal")?;

    Ok(())
}

fn break_session(minutes: &Seconds, message: &str) -> anyhow::Result<()> {
    run_timer(minutes, message);
    send_notification("Break done!", "It's time to get back to working!")?;
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

fn start_journal(path: &Path) -> anyhow::Result<()> {
    let editor = env::var_os("VISUAL")
        .or_else(|| env::var_os("EDITOR"))
        .with_context(|| "Neither $VISUAL nor $EDITOR was set.")?;

    let now = Local::now();
    let header = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut temp = tempfile::Builder::new()
        .prefix("pomo-session")
        .suffix(".md")
        .tempfile()
        .context("Failed to create temporary journal file.")?;

    writeln!(temp, "# {}", header).context("Writing header to temp file")?;

    temp.flush().context("Flushing temp file")?;

    let temp_path = temp.path().to_path_buf();

    Command::new(&editor)
        .arg(&temp_path)
        .status()
        .with_context(|| "Trying to edit file")?;

    move_journal(path, &temp_path, now).with_context(|| "Moving journal location")?;

    temp.close().ok();

    Ok(())
}

fn move_journal(dir: &Path, src: &Path, now: chrono::DateTime<Local>) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Creating destination directory: {}", dir.display()))?;

    let filename = format!("{}.md", now.format("%Y-%m-%d_%H-%M-%S"));
    let dest = dir.join(filename);

    std::fs::copy(src, &dest).with_context(|| {
        format!(
            "Moving file from {} to desired path: {}",
            src.display(),
            &dest.display()
        )
    })?;

    std::fs::remove_file(src).with_context(|| format!("Removing file {}", src.display()))?;

    Ok(dest)
}
