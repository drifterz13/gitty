#[derive(Debug)]
pub struct Commit {
    pub owner: String,
    pub rel_time: String,
    pub message: String,
    pub hash: String,
}

impl From<String> for Commit {
    fn from(value: String) -> Self {
        let mut arr = value.split("|");
        let hash = arr.next().expect("Commit hash is missing").to_string();
        let owner = arr.next().expect("Owner is missing").to_string();
        let rel_time = arr.next().expect("Relative time is miisng").to_string();
        let message = arr.next().expect("Commit message is missing").to_string();

        Commit {
            hash,
            owner,
            rel_time,
            message,
        }
    }
}
