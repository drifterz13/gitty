use gitty::repo::Repo;
use std::{collections::HashSet, path::Path};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing arguments.")
    }

    let repo_path = Path::new(&args[1]);
    let repo = Repo::build(repo_path.to_path_buf());

    let total_commits = Repo::get_total_commits(repo_path.to_path_buf()).unwrap();
    println!("total commits = {total_commits}");

    let commits_by_owner = repo.merge_commits_by_owner();
    println!("commits by owner = {:#?}", commits_by_owner);

    let authors: HashSet<String> = repo
        .commits
        .borrow()
        .iter()
        .map(|commit| commit.owner.to_string())
        .collect();
    println!("repo authors = {:#?}", authors);
}
