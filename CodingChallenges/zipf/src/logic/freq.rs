use crate::file::File;
use std::collections::HashMap;

pub fn get_frequency(file: &File) -> HashMap<char, i32> {
    let mut frequency = HashMap::new();

    for char in file.contents.chars() {
        *frequency.entry(char).or_insert(0) += 1;
    }
    return frequency;
}
