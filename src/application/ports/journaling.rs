use std::path::PathBuf;

pub trait Journal {
    fn create_journal(&self, journal_dir: &PathBuf) -> anyhow::Result<PathBuf>;
}
