mod error;
mod file;

use error::Error;
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
    println!("{:?}", file);
}
