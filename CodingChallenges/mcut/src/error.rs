pub enum Error {
    NoFileError,
    ReadError,
    FieldIndexZeroError,
    ConversionError,
    NoFieldError,
}

pub fn exit(message: &str, error_code: Error) {
    println!("{}", message);
    std::process::exit(error_code as i32);
}
