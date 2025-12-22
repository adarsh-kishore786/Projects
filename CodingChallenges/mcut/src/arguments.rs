use crate::error;
use crate::error::Error;
use crate::flag::Flag;

#[derive(PartialEq)]
enum State {
    Start,
    Normal,
    Flag(char)
}

pub fn parse(args: &str) -> Vec<Flag> {
    let mut index: usize = 0;
    let mut state = State::Start;
    let mut flags: Vec<Flag> = Vec::new();

    while index < args.len() {
        while get(args, index) == ' ' {
            index += 1;
        }

        if state == State::Start {
            // Exclude the program name itself
            while index < args.len() && !get(args, index).is_whitespace() {
                index += 1;
            }

            state = State::Normal;
            continue;
        } else if state == State::Normal {
            if get(args, index) == '-' {
                state = State::Flag(get(args, index+1));
                index += 2;
                continue;
            }

            let start = index;
            while index < args.len() && !get(args, index).is_whitespace() {
                index += 1;
            }

            let file_path = &args[start..index];
            flags.push(Flag::File(file_path.to_string()));

        } else {
            let start = index;
            while index < args.len() && !get(args, index).is_whitespace() {
                index += 1;
            }

            let temp = &args[start..index];

            flags.push(match state {
                State::Flag('f') => process_field(temp),
                State::Flag('d') => process_delimeter(temp),
                _ => Flag::Empty
            });

            state = State::Normal;
            continue;
        }

        index += 1;
    }

    return flags;
}

fn get(args: &str, pos: usize) -> char {
    return match args.chars().nth(pos) {
        Some(c) => c,
        None => '\0'
    };
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
    return Flag::Delimeter(temp_val.to_string());
}
