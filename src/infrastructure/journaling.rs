use anyhow::Context;
use chrono::Local;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::env;

pub trait Journal {
    fn create_journal(&self, journal_dir: PathBuf) -> anyhow::Result<PathBuf>;
}

pub struct MarkdownJournal;

impl MarkdownJournal {
    fn move_journal(&self, dir: &PathBuf, src: &PathBuf, now: chrono::DateTime<Local>) -> anyhow::Result<PathBuf> {
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

        
        self.move_journal(&path, &temp_path, now).with_context(|| "Moving journal location")?;

        // We disregard the error, as it will always error if the file has been moved.
        // However, if it for some reason has not been moved, and an error was not raised, we want
        // to close the file.
        temp.close().ok();

        Ok(path.clone())
    }
}

impl Journal for MarkdownJournal {
    fn create_journal(&self, journal_dir: PathBuf) -> anyhow::Result<PathBuf> {
        self.start_journal(&journal_dir)?;
        
        Ok(journal_dir)
    }
    
}