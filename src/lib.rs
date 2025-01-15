pub mod repo;

use std::process::Command;

pub struct GitLog {
    path: String,
    skip: u32,
    max: u32,
}

impl GitLog {
    pub fn new(path: String) -> GitLog {
        GitLog {
            path,
            skip: 0,
            max: 10,
        }
    }

    pub fn skip(&mut self, n: u32) -> &mut GitLog {
        self.skip = n;
        self
    }

    pub fn max_count(&mut self, n: u32) -> &mut GitLog {
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
