use data_encoding::HEXLOWER;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::read_to_string;
use std::io::Write;
use std::{
    fs::{self, read_dir, OpenOptions},
    path::Path,
};

use crate::constant::*;
use crate::{constant::RGIT_DIRECTORY, error::Error, model::File, utils::sha256_digest};

pub fn add(paths: Vec<String>) -> Result<(), Error> {
    if !Path::new(RGIT_DIRECTORY).is_dir() {
        return Err(Error::NotRgit);
    }
    if paths.is_empty() {
        return Err(Error::NoPath);
    }
    // wtire file after all path success
    let files = get_files(paths)?;
    try_create_index();
    for file in files {
        write_file(&file)?;
    }
    Ok(())
}

fn get_files(paths: Vec<String>) -> Result<Vec<File>, Error> {
    let mut files = vec![];
    for path in paths {
        let p = Path::new(&path);
        if p.is_dir() {
            let mut children_files = vec![];
            for dir in read_dir(path).unwrap() {
                children_files.push(dir.unwrap().path().display().to_string());
            }
            let mut cfs = get_files(children_files)?;
            files.append(&mut cfs);
        } else if p.is_file() {
            files.push(get_file(&path));
        } else {
            return Err(Error::NotMatchPath(path));
        }
    }
    Ok(files)
}

fn get_file(path: &str) -> File {
    let content = read_to_string(path).unwrap();

    let content_bytes = content.as_bytes();
    let sha = sha256_digest(content_bytes).unwrap();

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(content.as_bytes()).unwrap();
    let blob_bytes = e.finish().unwrap();

    // TODO blob and sha should include head, format such as:
    // <type> <size>0<content>
    File {
        file_contents: content,
        sha: HEXLOWER.encode(sha.as_ref()),
        blob: blob_bytes,
        path: String::from(path),
    }
}

fn write_file(file: &File) -> Result<(), Error> {
    let obj_dir = format!("{}/{}", OBJECTS_DIRECTORY, &file.sha[0..2]);
    fs::create_dir_all(&obj_dir).unwrap();
    let blob_path = format!("{}/{}", &obj_dir, &file.sha[3..]);

    let mut blob_file = fs::File::create(&blob_path).unwrap();
    blob_file.write_all(&file.blob).unwrap();

    let mut index_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(INDEX_PATH)
        .unwrap();
    if let Err(e) = writeln!(index_file, "{} {}", file.sha, file.path) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn try_create_index() {
    let path = Path::new(INDEX_PATH);
    if !path.exists() {
        fs::File::create(&path).unwrap();
    }
}
