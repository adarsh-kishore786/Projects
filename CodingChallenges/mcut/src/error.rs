pub enum Error {
    NoArgs,
    ReadError,
    FieldIndexZeroError,
    ConversionError
}

pub fn exit(message: &str, error_code: Error) {
    println!("{}", message);
    std::process::exit(error_code as i32);
}
