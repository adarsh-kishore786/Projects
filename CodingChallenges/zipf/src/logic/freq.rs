use crate::file::File;
use std::collections::HashMap;

pub fn get_frequency(file: &File) -> HashMap<char, u32> {
    let mut frequency = HashMap::new();

    for char in file.contents.chars() {
        *frequency.entry(char).or_insert(0) += 1;
    }
    return frequency;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_frequency() {
        let test_file = File {
            file_path: "test.txt".to_string(),
            contents: "aaabbc".to_string()
        };
        let freq_map = get_frequency(&test_file);
        assert_eq!(freq_map.get(&'a'), Some(&3));
        assert_eq!(freq_map.get(&'b'), Some(&2));
        assert_eq!(freq_map.get(&'c'), Some(&1));
    }

    #[test]
    fn test_get_frequency_empty() {
        let test_file = File {
            file_path: "empty.txt".to_string(),
            contents: "".to_string()
        };
        let freq_map = get_frequency(&test_file);
        assert!(freq_map.is_empty());
    }

    #[test]
    fn test_get_frequency_real_file() {
        let test_file = crate::file::read_file("test/sample.txt");
        let freq_map = get_frequency(&test_file);
        assert_eq!(freq_map.get(&'c'), Some(&32));
        assert_eq!(freq_map.get(&'d'), Some(&42));
        assert_eq!(freq_map.get(&'e'), Some(&120));
        assert_eq!(freq_map.get(&'k'), Some(&7));
        assert_eq!(freq_map.get(&'l'), Some(&42));
        assert_eq!(freq_map.get(&'m'), Some(&24));
        assert_eq!(freq_map.get(&'u'), Some(&37));
        assert_eq!(freq_map.get(&'z'), Some(&2));
    }
}
