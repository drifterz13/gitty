use std::path::Path;

use gitty::{
    repo::{commit::Commit, Repo},
    GitLog,
};

fn main() {
    println!("Running gitty...");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing arguments.")
    }

    let repo_path = Path::new(&args[1])
        .to_str()
        .expect("Repo path is missing.")
        .to_string();

    let total_commits = Repo::get_total_commits(&repo_path).unwrap();
    println!("total commits = {total_commits}");

    let batch_size = 100;
    let total_batch = total_commits.div_ceil(batch_size);

    println!("total batch = {total_batch}");
    let mut commits: Vec<Commit> = vec![];

    for n in 0..total_batch {
        let commit_logs = GitLog::new(repo_path.clone())
            .skip(n * batch_size)
            .max_count(batch_size)
            .run()
            .unwrap();

        for commit_log in commit_logs {
            commits.push(Commit::from(commit_log));
        }
    }

    let commits_by_owner = Repo::count_by_owner(commits);
    println!("commits by owner = {:#?}", commits_by_owner);
}
