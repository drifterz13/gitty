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
            let result = String::from_utf8(output.stdout);

            match result {
                Ok(output_str) => Ok(output_str),
                Err(e) => Err(format!("Git command error: {:#?}", e).into()),
            }
        }
    }
}
