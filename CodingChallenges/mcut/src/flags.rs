use crate::error;
use crate::error::Error;

#[derive(Debug)]
pub enum Flag {
    Empty,
    Field(u32),
    Delimeter(char),
}

pub fn process(args: &Vec<String>) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();

    for arg in args {
        if arg.chars().nth(0) != Some('-') {
            continue;
        }

        let temp_arg = arg[1..].trim();

        let res: Flag = match temp_arg.chars().nth(0) {
            Some('f') => process_field(&temp_arg[1..]),
            Some('d') => process_delimeter(&temp_arg[1..]),
            Some(_c) => continue,
            None => continue
        };

        flags.push(res);
    }
    
    return flags;
}

fn process_field(arg: &str) -> Flag {
    let temp_val = arg.trim();
    let res: Flag = match temp_val.parse() {
        Ok(val) => Flag::Field(val),
        Err(_) => {
            error::exit(&format!("mcut: cannot convert {} to usize!", temp_val), Error::ConversionError);
            return Flag::Empty;
        }
    };
    return res;
}

fn process_delimeter(arg: &str) -> Flag {
    let temp_val = arg.trim();
    let res: Flag = match temp_val.chars().nth(0) {
        Some(ch) => Flag::Delimeter(ch),
        None => {
            error::exit(&format!("Cannot convert {} to char!", temp_val), Error::ConversionError);
            return Flag::Empty;
        }
    };
    return res;
}
