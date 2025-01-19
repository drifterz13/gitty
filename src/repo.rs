use crate::utils::git::GitCommand;
use author::Author;
use commit::Commit;
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

pub mod author;
pub mod commit;

#[derive(Debug)]
pub struct Repo {
    pub path: PathBuf,
    pub commits: RefCell<Vec<Commit>>,
    pub authors: Vec<Author>,
}

impl Default for Repo {
    fn default() -> Self {
        Repo {
            path: PathBuf::new(),
            commits: RefCell::new(vec![]),
            authors: vec![],
        }
    }
}

impl Repo {
    pub fn build(path: PathBuf) -> Rc<Self> {
        let repo = Rc::new(Self {
            path,
            ..Default::default()
        });
        let commits = Repo::create_commits(&repo);

        {
            let mut repo_commits_mut = repo.commits.borrow_mut();
            repo_commits_mut.extend(commits);
        };

        repo
    }

    fn create_commits(repo: &Rc<Repo>) -> Vec<Commit> {
        let git_cmd = GitCommand::new(repo.path.clone());
        let output_str = git_cmd
            .run(&[
                "log",
                "--oneline",
                "--merges",
                &format!("--pretty=format:%h|%an|%ar|%s"),
            ])
            .unwrap();

        let commits: Vec<Commit> = output_str
            .lines()
            .into_iter()
            .map(|commit_log| {
                let commit = Commit::build(repo, commit_log.to_string());
                commit
            })
            .collect();

        commits
    }

    pub fn get_total_commits(path: PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
        let git_cmd = GitCommand::new(path.clone());
        let output_str = git_cmd.run(&["rev-list", "--count", "main"]).unwrap();
        let count: u32 = output_str
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parse commit count: {}", e))?;

        Ok(count)
    }

    pub fn merge_commits_by_owner(&self) -> HashMap<String, u32> {
        let git_cmd = GitCommand::new(self.path.clone());
        let output_str = git_cmd
            .run(&["shortlog", "-sn", "--merges", "main"])
            .unwrap();

        let map: HashMap<String, u32> = output_str
            .lines()
            .into_iter()
            .filter_map(|line| {
                let split_str = line.trim().split_once("\t");
                if let Some((count, author)) = split_str {
                    Some((author.to_string(), count.parse::<u32>().unwrap_or(0)))
                } else {
                    None
                }
            })
            .fold(HashMap::new(), |mut acc, (author, count)| {
                acc.entry(author).or_insert_with(|| count);
                acc
            });

        map
    }
}
