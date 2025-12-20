#[derive(Debug)]
pub enum Flag {
    Field(u32)
}

pub fn process(args: &Vec<String>) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();

    for arg in args {
        if arg.chars().nth(0) != Some('-') {
            continue;
        }

        let res: Flag = match arg.chars().nth(1) {
            Some('f') => process_field(&arg),
            Some(_c) => continue,
            None => continue
        };

        flags.push(res);
    }
    
    return flags;
}

fn process_field(arg: &str) -> Flag {
    let val: u32 = arg[2..].parse().expect(&format!("Cannot convert {} to integer!", arg));
    return Flag::Field(val);
}
