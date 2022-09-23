use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap, fs, path::Path};

use crate::{constant::OBJECTS_DIRECTORY, file::Mode, index, utils::sha256_digest_hex};
use std::io::Write;

#[derive(Debug)]
pub struct TreeEntry {
    typ: Mode,
    path: String,
    sha: Option<String>,
}

#[derive(Debug)]
pub struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { entries: vec![] }
    }
}

pub struct BuildTree {
    pub trees: Rc<RefCell<HashMap<String, Tree>>>,
    writer: RefCell<Vec<u8>>,
}

impl BuildTree {
    pub fn new() -> Self {
        let trees = Rc::new(RefCell::new(HashMap::new()));
        trees.borrow_mut().insert("".into(), Tree::new());
        BuildTree {
            trees,
            writer: RefCell::new(vec![]),
        }
    }

    pub fn build(&mut self, entries: Vec<index::Entry>) -> String {
        for entry in entries {
            self.commit_index_entry(entry)
        }
        self.persist_recursive("", self.trees.clone().borrow_mut().get_mut("").unwrap())
    }

    fn commit_index_entry(&mut self, entry: index::Entry) {
        let mut fullpath = String::new();

        for path in entry.path.split("/") {
            let path = Path::new(&fullpath).join(Path::new(path));
            fullpath = path.display().to_string();
            self.do_build(&entry, &path);
        }
    }

    fn do_build(&mut self, entry: &index::Entry, path: &Path) {
        if self
            .trees
            .borrow()
            .contains_key(&path.display().to_string())
        {
            return;
        }

        let mut typ = Mode::Dir;
        let mut sha = String::new();
        if entry.path.eq(&path.file_name().unwrap().to_str().unwrap()) {
            typ = Mode::Blob;
            sha = String::from(&entry.sha);
        } else {
            self.trees
                .borrow_mut()
                .insert(path.display().to_string(), Tree::new());
        }

        if let Some(tree) = self
            .trees
            .borrow_mut()
            .get_mut(path.parent().unwrap().to_str().unwrap())
        {
            let es = &mut tree.entries;
            es.push(TreeEntry {
                typ,
                path: String::from(path.file_name().unwrap().to_str().unwrap()),
                sha: Some(sha),
            });
        }
    }

    fn persist_recursive(&mut self, parent: &str, tree: &mut Tree) -> String {
        for entry in &mut tree.entries {
            if entry.typ != Mode::Dir && entry.sha.is_some() {
                continue;
            }
            let path = Path::new(parent)
                .join(Path::new(&entry.path))
                .display()
                .to_string();

            println!("{}", Rc::strong_count(&self.trees));
            let trees = self.trees.clone();

            let sha = self.persist_recursive(&path, trees.borrow_mut().get_mut(&path).unwrap());
            entry.sha = Some(String::from(&sha));
        }

        for entry in &tree.entries {
            // #[cfg(debug)]
            // println!("{:?}", entry);
            writeln!(
                self.writer.borrow_mut(),
                "{} {} {}",
                entry.typ,
                entry.sha.as_ref().unwrap(),
                entry.path
            )
            .unwrap();
        }

        let sha = sha256_digest_hex(self.writer.borrow().as_slice());

        let obj_dir = format!("{}/{}", OBJECTS_DIRECTORY, &sha[0..2]);
        fs::create_dir_all(&obj_dir).unwrap_or(());
        let obj_path = format!("{}/{}", obj_dir, &sha[3..]);

        let mut obj_file = fs::File::create(&obj_path).unwrap();
        writeln!(
            obj_file,
            "{}",
            String::from_utf8(self.writer.borrow().to_vec()).unwrap()
        )
        .unwrap_or(());

        sha
    }
}
