use crate::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
#[allow(dead_code)]
pub struct File {
    file_path: String,
    contents: String
}

pub fn read_file(file_path: &str) -> File {
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(s) => return File {
            file_path: file_path.to_string(),
            contents: s
        },
        Err(e) => {
            println!("There was an error reading the file: {}", e);
            process::exit(Error::ReadError as i32);
        }
    }
}
