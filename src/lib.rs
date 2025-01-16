pub mod repo;

use std::{path::PathBuf, process::Command};

pub struct GitLog {
    path: PathBuf,
    skip: u32,
    max: u32,
}

impl Default for GitLog {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            skip: 0,
            max: 10,
        }
    }
}

impl GitLog {
    pub fn new(path: PathBuf) -> Self {
        GitLog {
            path,
            ..Default::default()
        }
    }

    pub fn skip(mut self, n: u32) -> Self {
        self.skip = n;
        self
    }

    pub fn max_count(mut self, n: u32) -> Self {
        self.max = n;
        self
    }

    pub fn run(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .current_dir(&self.path)
            .args(&[
                "log",
                "--oneline",
                &format!("--skip={}", self.skip),
                &format!("--max-count={}", self.max),
                &format!("--pretty=format:\"%h|%an|%ar|%s\""),
            ])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            Ok(stdout.lines().map(String::from).collect())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(stderr.into())
        }
    }
}
