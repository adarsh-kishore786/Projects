#[derive(Debug)]
pub enum Flag {
    Empty,
    File(String),
    Field(u32),
    #[allow(dead_code)]
    Delimeter(char),
}
