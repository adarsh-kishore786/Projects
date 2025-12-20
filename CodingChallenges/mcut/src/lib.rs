mod error;
mod file;
mod flags;

use error::Error;

pub fn process(args: &Vec<String>) {
    if args.len() < 2 {
        println!();
        error::exit("Usage: ./mcut <file_name>", Error::NoArgs);
    }

    let input_file = file::read_file(&args[1]);
    let option_flags = flags::process(args);

    println!("{}", input_file.file_path);
    println!("{}", input_file.contents);
    println!("{:?}", option_flags);
}
