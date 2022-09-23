use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{Read, Result};
use std::path::Path;

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn sha256_digest_hex<R: Read>(reader: R) -> String {
    HEXLOWER.encode(sha256_digest(reader).unwrap().as_ref())
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
