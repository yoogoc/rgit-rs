use std::fs::{self, create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

use crate::error::Error;
use crate::index::indexes;
use crate::tree::BuildTree;
use crate::utils::sha256_digest_hex;
use crate::{constant::*, index};

const COMMIT_TEMPLATE: &str = r#"

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
"#;

// TODO: 检查是否有change
pub fn commit(message: Option<String>) -> Result<(), Error> {
    let indexes = indexes();
    let tree_sha = build_tree(indexes);
    let commit_sha = build_commit(&tree_sha, message);
    update_refs(&commit_sha);
    clear_index();

    Ok(())
}

fn build_tree(entries: Vec<index::Entry>) -> String {
    let bt = BuildTree::new();
    bt.build(entries)
}

fn build_commit(tree_sha: &str, message: Option<String>) -> String {
    try_create_commit_message();
    let commit_message_path = format!("{}/COMMIT_EDITMSG", RGIT_DIRECTORY);
    let commit_msg = if let Some(msg) = message {
        msg
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("vim {}", commit_message_path))
            .output()
            .unwrap();
        read_to_string(&commit_message_path).unwrap()
    };
    let now = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap();
    let committer = "user";
    let sha_name = format!("{}{}", &now, committer);
    let sha = sha256_digest_hex(sha_name.as_bytes());
    let obj_dir = format!("{}/{}", OBJECTS_DIRECTORY, &sha[0..2]);
    fs::create_dir_all(&obj_dir).unwrap();
    let obj_path = format!("{}/{}", obj_dir, &sha[2..]);

    let mut obj_file = fs::File::create(&obj_path).unwrap();
    writeln!(obj_file, "tree {}", tree_sha).unwrap();
    writeln!(obj_file, "author {}", committer).unwrap();
    writeln!(obj_file, "commit_at {}", &now).unwrap();
    writeln!(obj_file, "").unwrap();
    writeln!(obj_file, "{}", commit_msg).unwrap();
    sha
}

fn update_refs(commit_sha: &str) {
    let head_path = format!("{}/HEAD", RGIT_DIRECTORY);
    let head = read_to_string(head_path).unwrap();
    let current_branch = head.split(" ").last().unwrap();
    try_create_branch(current_branch, commit_sha);
}

fn clear_index() {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(INDEX_PATH)
        .unwrap();
}

fn try_create_commit_message() {
    let p = format!("{}/COMMIT_EDITMSG", RGIT_DIRECTORY);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&p)
        .unwrap();
    file.write(COMMIT_TEMPLATE.as_bytes()).unwrap();
}

fn try_create_branch(path: &str, sha: &str) {
    println!("{}", path);
    let file_path = Path::new(path);

    let path = Path::new(RGIT_DIRECTORY).join(&file_path.parent().unwrap());

    create_dir_all(&path).unwrap_or(());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path.join(Path::new(file_path.file_name().unwrap())))
        .unwrap();
    file.write(sha.as_bytes()).unwrap();
}
