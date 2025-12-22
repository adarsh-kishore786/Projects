#[derive(Debug)]
pub enum Flag {
    Empty,
    File(String),
    Field(u32),
    Delimeter(String),
}
