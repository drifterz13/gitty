#[derive(Debug)]
pub struct Stats {
    pub insertions: u32,
    pub deletions: u32,
}

impl Stats {
    pub fn set_insertions(mut self, insertions: u32) -> Self {
        self.insertions = insertions;
        self
    }

    pub fn set_deletions(mut self, deletions: u32) -> Self {
        self.deletions = deletions;
        self
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            insertions: 0,
            deletions: 0,
        }
    }
}
