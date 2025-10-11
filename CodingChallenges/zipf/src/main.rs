use std::env;
use std::fs;
use std::process;

enum Error {
    NoArgs,
    ReadError
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    file_path: String,
    contents: String
}

fn read_file(file_path: &str) -> File {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./zipf path/to/file");
        process::exit(Error::NoArgs as i32);
    }

    let file = read_file(&args[1]);
    println!("{:?}", file);
}
