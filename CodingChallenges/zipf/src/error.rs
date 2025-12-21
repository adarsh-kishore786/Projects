pub enum Error {
    NoArgs,
    ReadError,
    WriteError,
}

pub fn exit(message: &str, error_code: Error) {
    eprintln!("{}", message);
    std::process::exit(error_code as i32);
}
