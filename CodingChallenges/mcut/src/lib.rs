mod arguments;
mod error;
mod file;
mod flag;

use error::Error;
use file::File;
use flag::Flag;
use std::io::Write;

pub fn process(args: &str) {
    let flag_args = arguments::parse(args);
    let mut input_file: Option<File> = None;
    let mut field_index: Option<usize> = None;
    let mut delimeter: Option<String> = None;

    // println!("{:?}", flag_args);

    for flag in flag_args {
        match flag {
            Flag::File(val) => {
                input_file = Some(file::read_file(&val))
            },
            Flag::Field(val) => {
                if val == 0 {
                    error::exit("mcut: fields are numbered from 1", Error::FieldIndexZeroError);
                }
                field_index = Some(val as usize - 1);
            },
            Flag::Delimeter(delim) => {
                delimeter = Some(delim);
            }
            _ => {}
        }
    }

    if let Some(file) = input_file {
        if let Some(idx) = field_index {
            cut(&file, idx, &delimeter.unwrap_or(String::from("\t")));
        } else {
            error::exit("mcut: no field index provided", Error::NoFieldError);
        }
    } else {
        error::exit("mcut: no file provided", Error::NoFileError);
    }
}

pub fn cut(input_file: &File, index: usize, delimeter: &str) {
    let lines = input_file.contents.split("\n").collect::<Vec<&str>>();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for line in lines {
        let item = line.split(delimeter).nth(index).unwrap_or("");

        if let Err(_) = writeln!(handle, "{}", item) {
            break;
        }
    }
}
