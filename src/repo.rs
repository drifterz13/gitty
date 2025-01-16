use std::{collections::HashMap, path::Path, process::Command};

use commit::Commit;

pub mod author;
pub mod commit;

pub struct Repo {
    pub commits: Vec<Commit>,
}

impl Repo {
    pub fn new() -> Repo {
        Repo { commits: vec![] }
    }

    pub fn get_total_commits(path: &Path) -> Result<u32, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .current_dir(path)
            .args(["rev-list", "--count", "main"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let count: u32 = stdout
                .trim()
                .parse()
                .map_err(|e| format!("Failed to parse commit count: {}", e))?;

            Ok(count)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Git command failed: {}", stderr).into())
        }
    }

    // TODO: Remove bot and prevent duplicate commit message (?)
    pub fn count_by_owner(commits: &[Commit]) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        commits.iter().for_each(|commit| {
            counts
                .entry(commit.owner.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        counts
    }
}
