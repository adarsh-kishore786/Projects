mod error;
mod file;
mod logic;

use error::Error;
use logic::freq;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error::exit(
            "Usage: ./zipf path/to/file",
            Error::NoArgs 
        );
    }

    let file = file::read_file(&args[1]);
    println!("Details for the file: {}", file.file_path);
    println!("{:?}", freq::get_frequency(&file));
}
