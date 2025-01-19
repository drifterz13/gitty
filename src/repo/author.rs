use super::{commit::Commit, stats::Stats};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct AuthorStats {
    pub loc: Stats,
    pub loc_diff: i32,
    pub total_commits: usize,
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
    commits: Vec<Weak<Commit>>,
}

impl Author {
    pub fn new(name: String, commits: Vec<Rc<Commit>>) -> Self {
        let commits = commits.iter().map(|commit| Rc::downgrade(commit)).collect();
        Self { name, commits }
    }

    pub fn get_stats(&self) -> AuthorStats {
        let (insertions, deletions) = self.commits.iter().fold((0, 0), |mut acc, commit| {
            let commit = &commit.upgrade().unwrap();
            match &commit.stats {
                Some(stats) => {
                    acc.0 += stats.insertions;
                    acc.1 += stats.deletions;
                    acc
                }
                None => acc,
            }
        });

        AuthorStats {
            loc: Stats {
                insertions,
                deletions,
            },
            loc_diff: (insertions - deletions).try_into().unwrap(),
            total_commits: self.commits.len(),
        }
    }
}
