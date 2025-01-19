use std::{collections::HashMap, path::Path};

use crate::utils::git::GitCommand;

#[derive(Debug)]
pub struct Author {
    name: String,
}

// TODO: Do we need it?
impl Author {
    pub fn get_author_prs(
        &self,
        repo_path: &Path,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let git_cmd = GitCommand::new(repo_path.to_path_buf());
        let output_str = git_cmd
            .run(&[
                "log",
                "--merges",
                "--pretty=format:%an|%H",
                "--first-parent",
                "main",
            ])
            .unwrap();

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
