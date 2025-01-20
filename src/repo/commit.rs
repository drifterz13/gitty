use regex::Regex;
use std::rc::{Rc, Weak};

use super::{stats::Stats, Repo};
use crate::utils::future::AsyncCommand;

#[derive(Debug)]
pub struct Commit {
    pub repo: Weak<Repo>,
    pub owner: String,
    pub rel_time: String,
    pub message: String,
    pub hash: String,
    pub stats: Option<Stats>,
}

impl Commit {
    pub fn build(repo: &Rc<Repo>, commit_log: String) -> Commit {
        let mut arr = commit_log.split("|");
        let hash = arr.next().expect("Commit hash is missing").to_string();
        let owner = arr.next().expect("Owner is missing").to_string();
        let rel_time = arr.next().expect("Relative time is miisng").to_string();
        let message = arr.next().expect("Commit message is missing").to_string();
        let repo = Rc::downgrade(repo);

        Commit {
            repo,
            hash,
            owner,
            rel_time,
            message,
            stats: None,
        }
    }

    pub async fn get_stats(&self) -> Result<Stats, Box<dyn std::error::Error>> {
        let repo_path = match self.repo.upgrade() {
            Some(repo) => repo.path.clone(),
            None => return Err("Repo has been dropped.".into()),
        };

        let args: Vec<String> = vec!["show".to_string(), "--stat".to_string(), self.hash.clone()];
        let mut cmd = AsyncCommand::new(String::from("git"))
            .with_dir(repo_path)
            .with_args(args)
            .build();
        cmd.stdout(std::process::Stdio::piped());

        let output = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn git show --stat {}: {}", self.hash, e))?
            .wait_with_output()
            .await?;

        if !output.status.success() {
            return Err(format!("Command failed with status: {}", output.status).into());
        }

        let output_str = String::from_utf8(output.stdout)?;
        let stats_line = output_str.lines().last().unwrap().trim();
        let stats = Stats::default();

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
