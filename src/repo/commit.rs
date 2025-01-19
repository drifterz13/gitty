use regex::Regex;
use stats::CommitStats;
use std::rc::{Rc, Weak};

use super::Repo;
use crate::utils::git::GitCommand;

mod stats;

#[derive(Debug)]
pub struct Commit {
    pub repo: Weak<Repo>,
    pub owner: String,
    pub rel_time: String,
    pub message: String,
    pub hash: String,
    pub stats: Option<CommitStats>,
}

impl Commit {
    pub fn build(repo: &Rc<Repo>, commit_log: String) -> Commit {
        let mut arr = commit_log.split("|");
        let hash = arr.next().expect("Commit hash is missing").to_string();
        let owner = arr.next().expect("Owner is missing").to_string();
        let rel_time = arr.next().expect("Relative time is miisng").to_string();
        let message = arr.next().expect("Commit message is missing").to_string();

        let repo = Rc::downgrade(repo);
        let mut commit = Commit {
            repo,
            hash,
            owner,
            rel_time,
            message,
            stats: None,
        };

        let stats = commit
            .get_stats()
            .map_err(|e| format!("Failed to get commit stats: {:#?}", e))
            .unwrap();

        commit.stats = Some(stats);
        commit
    }

    pub fn get_stats(&self) -> Result<CommitStats, Box<dyn std::error::Error>> {
        let repo_path = match self.repo.upgrade() {
            Some(repo) => repo.path.clone(),
            None => panic!("Repo has been dropped."),
        };

        let git_cmd = GitCommand::new(repo_path);
        let output_str = git_cmd
            .run(&["show", "--stat", &format!("{}", self.hash)])
            .unwrap();
        let stats_line = output_str.lines().last().unwrap().trim();
        let stats = CommitStats::default();

        // 2 files changed, 8 insertions(+), 4 deletions(-)
        let re = Regex::new(r"(?P<files>\d+) files? changed(?:, (?P<insertions>\d+) insertions?\(\+\))?(?:, (?P<deletions>\d+) deletions?\(-\))?").unwrap();
        let caps = re.captures(stats_line).unwrap();
        let insertions = caps
            .name("insertions")
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
        let deletions = caps
            .name("deletions")
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
        let stats = stats.set_insertions(insertions).set_deletions(deletions);
        Ok(stats)
    }
}
