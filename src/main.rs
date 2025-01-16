use std::{collections::HashSet, path::Path};

use gitty::{
    repo::{author::Author, commit::Commit, Repo},
    GitLog,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing arguments.")
    }

    let repo_path = Path::new(&args[1]);

    let total_commits = Repo::get_total_commits(&repo_path).unwrap();
    println!("total commits = {total_commits}");

    let batch_size = 100;
    let total_batch = total_commits.div_ceil(batch_size);

    println!("total batch = {total_batch}");
    let mut commits = Vec::new();

    for n in 0..total_batch {
        let commit_logs = GitLog::new(repo_path.to_path_buf())
            .skip(n * batch_size)
            .max_count(batch_size)
            .run()
            .unwrap();

        commits.extend(commit_logs.into_iter().map(Commit::from));
    }

    let commits_by_owner = Repo::count_by_owner(&commits);
    println!("commits by owner = {:#?}", commits_by_owner);

    let contributors: HashSet<String> = commits
        .iter()
        .map(|commit| commit.owner.to_string())
        .collect();

    for contributor in contributors {
        let author = Author::new(contributor);

        if let Ok(prs) = author.get_author_prs(&repo_path) {
            println!(
                "author {} contribute to total PRs = {}",
                author.name,
                prs.len()
            );
            author.add_pr_hashes(prs);
        }
    }
}
