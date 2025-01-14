pub mod core {
    use std::{collections::HashMap, process::Command};

    #[derive(Debug)]
    pub struct Commit {
        pub owner: String,
        pub rel_time: String,
        pub message: String,
        pub hash: String,
    }

    impl Commit {
        pub fn get_total_commits(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
            let go_to = format!("cd {path}");
            let cmd = format!("{go_to} && git rev-list --count main");
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

    impl From<String> for Commit {
        fn from(value: String) -> Self {
            let mut arr = value.split(" - ");
            let hash = arr.next().expect("Commit hash is missing").to_string();
            let owner = arr.next().expect("Owner is missing").to_string();
            let rel_time = arr.next().expect("Relative time is miisng").to_string();
            let message = arr.next().expect("Commit message is missing").to_string();

            Commit {
                hash,
                owner,
                rel_time,
                message,
            }
        }
    }

    pub struct GitLogger {
        path: String,
        skip: u32,
        max: u32,
    }

    impl GitLogger {
        pub fn new(path: String) -> GitLogger {
            GitLogger {
                path,
                skip: 0,
                max: 10,
            }
        }

        pub fn skip(&mut self, n: u32) -> &mut GitLogger {
            self.skip = n;
            self
        }

        pub fn max_count(&mut self, n: u32) -> &mut GitLogger {
            self.max = n;
            self
        }

        pub fn run(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
            let skip = self.skip;
            let max_count = self.max;
            let repo_path = &self.path;
            let log_cmd = format!("git log --oneline --skip={skip} --max-count={max_count} --pretty=format:\"%h - %an - %ar - %s\"");
            let cmd = format!("cd {repo_path} && {log_cmd}");
            let output = Command::new("sh").arg("-c").arg(&cmd).output()?;

            match output.status.success() {
                true => {
                    let val = String::from_utf8(output.stdout)?;
                    let mut values: Vec<String> = vec![];

                    for line in val.lines() {
                        values.push(line.to_string());
                    }
                    Ok(values)
                }
                false => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(stderr.into())
                }
            }
        }
    }
}
