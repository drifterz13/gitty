use std::{collections::HashMap, path::Path, process::Command};

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pr_hashes: Vec<String>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Author {
            name,
            pr_hashes: vec![],
        }
    }

    pub fn add_pr_hashes(mut self, prs: Vec<String>) -> Self {
        self.pr_hashes.extend(prs);
        self
    }

    pub fn get_author_prs(
        &self,
        repo_path: &Path,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(&[
                "log",
                "--merges",
                "--pretty=format:%an|%H",
                "--first-parent",
                "main",
            ])
            .output()?;

        let output_str = String::from_utf8(output.stdout)?;
        let prs_by_author: HashMap<String, Vec<String>> = output_str
            .lines()
            .filter_map(|line| {
                line.split_once("|")
                    .map(|(author_name, hash)| (author_name.to_string(), hash.to_string()))
            })
            .fold(HashMap::new(), |mut acc, (author, hash)| {
                acc.entry(author).or_default().push(hash);
                acc
            });

        prs_by_author
            .get(&self.name)
            .cloned()
            .ok_or_else(|| format!("No PRs found for author: {}", self.name).into())
    }
}
