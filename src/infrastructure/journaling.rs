use crate::application::ports::journaling::Journal;
use anyhow::Context;
use chrono::Local;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct MarkdownJournal;

impl MarkdownJournal {
    fn move_journal(
        &self,
        from: &PathBuf,
        to: &PathBuf,
        now: chrono::DateTime<Local>,
    ) -> anyhow::Result<PathBuf> {
        std::fs::create_dir_all(to)
            .with_context(|| format!("Creating destination directory: {}", to.display()))?;

        let filename = format!("{}.md", now.format("%Y-%m-%d_%H-%M-%S"));
        let dest = to.join(filename);

        std::fs::copy(from, &dest).with_context(|| {
            format!(
                "Moving file from {} to desired path: {}",
                from.display(),
                &dest.display()
            )
        })?;

        std::fs::remove_file(from).with_context(|| format!("Removing file {}", from.display()))?;

        Ok(dest)
    }

    fn start_journal(&self, path: &PathBuf) -> anyhow::Result<PathBuf> {
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

        let journal_path = self
            .move_journal(&temp_path, &path, now)
            .with_context(|| "Moving journal location")?;

        // We disregard the error, as it will always error if the file has been moved.
        // However, if it for some reason has not been moved, and an error was not raised, we want
        // to close the file.
        temp.close().ok();

        Ok(journal_path)
    }
}

impl Journal for MarkdownJournal {
    fn create_journal(&self, journal_dir: &PathBuf) -> anyhow::Result<PathBuf> {
        Ok(self.start_journal(&journal_dir)?)
    }
}
