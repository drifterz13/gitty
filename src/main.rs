use gitty::repo::Repo;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing arguments.")
    }

    let repo_path = Path::new(&args[1]);
    let repo = Repo::build(repo_path.to_path_buf());

    let total_commits = Repo::get_total_commits(repo_path.to_path_buf()).unwrap();
    println!("total commits = {total_commits}");

    let commits_by_owner = repo.commits_by_owner();
    println!("commits by owner = {:#?}", commits_by_owner);

    for author in repo.authors.borrow().iter() {
        let author_stats = author.get_stats();
        println!("author name {} stats = {:#?}", author.name, author_stats);
    }
}
