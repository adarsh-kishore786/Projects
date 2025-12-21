mod arguments;
mod error;
mod file;
mod flag;
mod logic;

use error::Error;
use file::File;
use flag::Flag;
use logic::field;

pub fn process(args: &str) {
    let flag_args = arguments::parse(args);
    let mut input_file: Option<File> = None;
    let mut field_index: Option<usize> = None;

    println!("{:?}", flag_args);

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
            _ => {}
        }
    }

    if let Some(file) = input_file {
        field::process(&file, field_index);
    } else {
        error::exit("mcut: no file provided", Error::NoFileError);
    }
}
