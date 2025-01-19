#[derive(Debug)]
pub struct CommitStats {
    insertions: u32,
    deletions: u32,
}

impl CommitStats {
    pub fn set_insertions(mut self, insertions: u32) -> Self {
        self.insertions = insertions;
        self
    }

    pub fn set_deletions(mut self, deletions: u32) -> Self {
        self.deletions = deletions;
        self
    }
}

impl Default for CommitStats {
    fn default() -> Self {
        CommitStats {
            insertions: 0,
            deletions: 0,
        }
    }
}
