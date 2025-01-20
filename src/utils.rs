pub mod git {
    use std::{path::PathBuf, process::Command};

    pub struct GitCommand {
        path: PathBuf,
    }

    impl GitCommand {
        pub fn new(path: PathBuf) -> GitCommand {
            GitCommand { path }
        }

        pub fn run(&self, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
            let output = Command::new("git")
                .current_dir(&self.path)
                .args(args)
                .output()?;

            if !output.status.success() {
                return Err("Failed to execute git command".into());
            }

            match String::from_utf8(output.stdout) {
                Ok(output_str) => Ok(output_str),
                Err(e) => Err(format!("Failed to convert stdout to string: {}", e).into()),
            }
        }
    }
}

pub mod future {
    use std::path::PathBuf;
    use tokio::process::Command;

    pub enum Error {
        CommandError(String),
        ParseError(String),
    }

    pub struct AsyncCommand {
        pub cmd: String,
        pub path: PathBuf,
        pub args: Vec<String>,
    }

    impl Default for AsyncCommand {
        fn default() -> Self {
            Self {
                cmd: String::from("git"),
                path: PathBuf::default(),
                args: vec![],
            }
        }
    }

    impl AsyncCommand {
        pub fn new(cmd: String) -> Self {
            Self {
                cmd,
                ..Default::default()
            }
        }

        pub fn with_dir(mut self, path: PathBuf) -> Self {
            self.path = path;
            self
        }

        pub fn with_args(mut self, args: Vec<String>) -> Self {
            self.args.extend(args);
            self
        }

        pub fn build(&self) -> Command {
            let mut cmd = Command::new(&self.cmd);
            cmd.current_dir(&self.path);
            cmd.args(&self.args);
            cmd
        }
    }
}
