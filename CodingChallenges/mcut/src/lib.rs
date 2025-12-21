mod error;
mod file;
mod flags;
mod logic;

use error::Error;
use logic::field;

pub fn process(args: &Vec<String>) {
    if args.len() < 2 {
        println!();
        error::exit("Usage: ./mcut <file_name>", Error::NoArgs);
    }

    let input_file = file::read_file(&args[1]);
    let option_flags = flags::process(args);

    println!("{}", input_file.file_path);

    let mut field_index: std::option::Option<usize> = None;

    for flag in option_flags {
        if let flags::Flag::Field(val) = flag {
            field_index = Some(val as usize - 1);
        } else if let flags::Flag::Delimeter(chr) = flag {
            println!("Delimeter: {chr}");
        }
    }

    field::process(&input_file, field_index);
}
