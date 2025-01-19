use author::Author;
use commit::Commit;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::PathBuf,
    rc::Rc,
};

use crate::utils::git::GitCommand;

pub mod author;
pub mod commit;
pub mod stats;

#[derive(Debug)]
pub struct Repo {
    pub path: PathBuf,
    pub commits: RefCell<Vec<Rc<Commit>>>,
    pub authors: RefCell<Vec<Author>>,
}

impl Default for Repo {
    fn default() -> Self {
        Repo {
            path: PathBuf::new(),
            commits: RefCell::new(vec![]),
            authors: RefCell::new(vec![]),
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
        let commits: Vec<Rc<Commit>> = commits.into_iter().map(|commit| Rc::new(commit)).collect();
        {
            let mut repo_commits_mut = repo.commits.borrow_mut();
            repo_commits_mut.extend(commits);
        };

        let authors = Repo::create_authors(&repo);
        {
            let mut repo_authors_mut = repo.authors.borrow_mut();
            repo_authors_mut.extend(authors);
        };

        repo
    }

    fn create_commits(repo: &Rc<Repo>) -> Vec<Commit> {
        let git_cmd = GitCommand::new(repo.path.clone());
        let output_str = git_cmd
            .run(&[
                "log",
                "--oneline",
                &format!("--pretty=format:%h|%an|%ar|%s"),
                "main",
            ])
            .unwrap();

        let commits: Vec<Commit> = output_str
            .lines()
            .into_iter()
            .map(|commit_log| {
                let commit = Commit::build(&repo, commit_log.to_string());
                commit
            })
            .collect();

        commits
    }

    fn create_authors(repo: &Rc<Repo>) -> Vec<Author> {
        let repo_authors = repo.get_repo_authors();
        let commits = repo.commits.borrow();
        let mut authors = Vec::new();

        for author in repo_authors.into_iter() {
            let author_commits: Vec<Rc<Commit>> = commits
                .iter()
                .filter(|commit| commit.owner == author)
                .map(|commit| Rc::clone(commit))
                .collect();

            let author = Author::new(author, author_commits);
            authors.push(author);
        }

        authors
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

    pub fn get_repo_authors(&self) -> HashSet<String> {
        let authors: HashSet<String> = self
            .commits
            .borrow()
            .iter()
            .map(|commit| commit.owner.to_string())
            .collect();

        authors
    }

    pub fn commits_by_owner(&self) -> HashMap<String, u32> {
        let git_cmd = GitCommand::new(self.path.clone());
        let output_str = git_cmd.run(&["shortlog", "-sn", "main"]).unwrap();

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
