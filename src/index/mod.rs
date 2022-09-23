#![allow(dead_code)]
use crate::{constant::*, file::Mode, utils::read_lines};

#[derive(Debug)]
pub struct Entry {
    pub typ: Mode,
    pub path: String,
    pub sha: String,
}

impl Entry {
    fn new(line: &str) -> Self {
        let mut contents = line.split(" ");
        let sha = contents.next().unwrap();
        let typ = contents.next().unwrap();
        let path = contents.next().unwrap();
        let typ = match typ {
            "blob" => Mode::Blob,
            "dir" => Mode::Dir,
            _ => unreachable!(),
        };
        Entry {
            typ,
            path: String::from(path),
            sha: String::from(sha),
        }
    }
}

pub fn indexes() -> Vec<Entry> {
    let mut indexes = vec![];
    if let Ok(lines) = read_lines(INDEX_PATH) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                indexes.push(Entry::new(&ip));
            }
        }
    }
    return indexes;
}
