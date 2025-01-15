use std::{collections::HashMap, process::Command};

use commit::Commit;

pub mod commit;

pub struct Repo {
    pub commits: Vec<Commit>,
}

impl Repo {
    pub fn new() -> Repo {
        Repo { commits: vec![] }
    }

    pub fn get_total_commits(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let cmd = format!("cd {path} && git rev-list --count main");
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;

        match output.status.success() {
            true => {
                let stdout = String::from_utf8(output.stdout)?;
                let count: u32 = stdout.trim().parse().unwrap();
                Ok(count)
            }
            false => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(stderr.into())
            }
        }
    }

    // TODO: Remove bot and prevent duplicate commit message (?)
    pub fn count_by_owner(commits: Vec<Commit>) -> HashMap<String, u32> {
        commits.iter().fold(HashMap::new(), |mut acc, commit| {
            match acc.get(&commit.owner) {
                Some(count) => {
                    acc.insert(commit.owner.clone(), count + 1);
                    acc
                }
                None => {
                    acc.insert(commit.owner.clone(), 1);
                    acc
                }
            }
        })
    }
}
