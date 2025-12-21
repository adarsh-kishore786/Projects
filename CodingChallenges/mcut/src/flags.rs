#[derive(Debug)]
pub enum Flag {
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
    let val: u32 = temp_val.parse().expect(&format!("Cannot convert {} to integer!", temp_val));
    return Flag::Field(val);
}

fn process_delimeter(arg: &str) -> Flag {
    let temp_val = arg.trim();
    let ch: char = temp_val.chars().nth(0).expect(&format!("Cannot convert {} to char!", temp_val));
    return Flag::Delimeter(ch);
}
