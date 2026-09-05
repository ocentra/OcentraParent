use super::{DirectoryEntry, FileStat};

impl DirectoryEntry {
    pub(crate) fn new(name: String, stat: FileStat) -> Self {
        Self { name, stat }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stat(&self) -> FileStat {
        self.stat
    }
}
