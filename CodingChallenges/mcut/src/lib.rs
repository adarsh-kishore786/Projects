mod file;
mod error;

use error::Error;

pub fn process(args: &Vec<String>) {
    if args.len() < 2 {
        println!();
        error::exit("Usage: ./mcut <file_name>", Error::NoArgs);
    }

    let input_file = file::read_file(&args[1]);
    println!("{}", input_file.file_path);
    println!("{}", input_file.contents);
}
