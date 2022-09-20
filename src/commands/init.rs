use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

use crate::constant::*;
use crate::error::Error;

pub fn init() -> Result<(), Error> {
    if Path::new(RGIT_DIRECTORY).is_dir() {
        return Err(Error::DirExists);
    }

    build_rgit_directory();
    build_objects_directory();
    build_refs_directory();
    initialize_head();
    print_success();
    Ok(())
}

fn print_success() {
    let pwd = current_dir().unwrap();
    print!(
        "Initialized empty Git repository in {}/{}\n",
        pwd.as_path().display(),
        RGIT_DIRECTORY
    )
}

fn build_rgit_directory() {
    _ = create_dir(Path::new(RGIT_DIRECTORY));
}

fn build_objects_directory() {
    _ = create_dir(Path::new(OBJECTS_DIRECTORY));
    let mut info = String::new();
    info.push_str(OBJECTS_DIRECTORY);
    info.push_str("/info");
    _ = create_dir(Path::new(&info));
    let mut pack = String::new();
    pack.push_str(OBJECTS_DIRECTORY);
    pack.push_str("/pack");
    _ = create_dir(Path::new(&pack));
}

fn build_refs_directory() {
    _ = create_dir(Path::new(REFS_DIRECTORY));
    let mut info = String::new();
    info.push_str(REFS_DIRECTORY);
    info.push_str("/heads");
    _ = create_dir(Path::new(&info));
    let mut pack = String::new();
    pack.push_str(REFS_DIRECTORY);
    pack.push_str("/tags");
    _ = create_dir(Path::new(&pack));
}

fn initialize_head() {
    let mut head = String::new();
    head.push_str(RGIT_DIRECTORY);
    head.push_str("/HEAD");
    let mut f = File::create(&head).unwrap();
    _ = writeln!(f, "ref: refs/heads/main"); // TODO custom first branch
}
