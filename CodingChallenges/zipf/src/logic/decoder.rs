use crate::file::File;
use std::collections::BTreeMap;

pub fn decode_contents(file: &File) -> String {
    let header_length = file.contents[0] as usize;
    let body_padding = file.contents[1] as usize;
    let header_bytes = &file.contents[2..2 + header_length];
    let body_bytes = &file.contents[2 + header_length..];

    let header_str = String::from_utf8(header_bytes.to_vec()).unwrap();
    let huffman_codes: BTreeMap<char, u32> = parse_header(&header_str);
    let binary_string = decode_bytes_to_binary_string(body_bytes, body_padding);
    let decoded_string = reconstruct_original_string(&binary_string, &huffman_codes);
    println!("{decoded_string}");

    return decoded_string;
}

fn parse_header(header: &str) -> BTreeMap<char, u32> {
    let mut huffman_codes: BTreeMap<char, u32> = BTreeMap::new();
    let mut chars = header.chars().peekable();

    while let Some(ch) = chars.next() {
        let mut code = String::new();
        while let Some(&next_ch) = chars.peek() {
            if next_ch == '0' || next_ch == '1' {
                code.push(next_ch);
                chars.next();
            } else {
                break;
            }
        }
        huffman_codes.insert(ch, u32::from_str_radix(&code, 2).unwrap());
    }

    return huffman_codes;
}

fn decode_bytes_to_binary_string(body_bytes: &[u8], body_padding: usize) -> String {
    let mut binary_string = String::new();

    for &byte in body_bytes {
        let byte_str = format!("{:08b}", byte);
        binary_string.push_str(&byte_str);
    }

    if body_padding > 0 {
        let len = binary_string.len();
        binary_string.truncate(len - body_padding);
    }

    return binary_string;
}

fn reconstruct_original_string(binary_string: &str, huffman_codes: &BTreeMap<char, u32>) -> String {
    let mut inverted_codes: BTreeMap<u32, char> = BTreeMap::new();
    for (ch, code) in huffman_codes {
        inverted_codes.insert(*code, *ch);
    }

    let mut original_string = String::new();
    let mut current_code: u32 = 0;

    for bit in binary_string.chars() {
        current_code = (current_code << 1) | if bit == '1' { 1 } else { 0 };

        if let Some(&ch) = inverted_codes.get(&current_code) {
            original_string.push(ch);
            current_code = 0;
        }
    }

    return original_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::File;

    #[test]
    fn test_decode_contents() {
        let file = File {
            contents: vec![8, 3, b'a', b'0', b'b', b'1', b'0', b'c', b'1', b'1', 0b10101100, 0b00000000],
            file_path: String::from("test.zipf"),
        };
        let decoded = decode_contents(&file);
        assert_eq!(decoded, "bbcaaaaaaa");
    }

    #[test]
    fn test_parse_header() {
        let header = "a0b10c11";
        let huffman_codes = parse_header(header);
        assert_eq!(huffman_codes.get(&'a'), Some(&0));
        assert_eq!(huffman_codes.get(&'b'), Some(&2));
        assert_eq!(huffman_codes.get(&'c'), Some(&3));
    }

    #[test]
    fn test_decode_bytes_to_binary_string() {
        let body_bytes = vec![0b10101100, 0b00000000];
        let binary_string = decode_bytes_to_binary_string(&body_bytes, 3);
        assert_eq!(binary_string, "1010110000000");
    }

    #[test]
    fn test_reconstruct_original_string() {
        let binary_string = "1010110000000";
        let mut huffman_codes: BTreeMap<char, u32> = BTreeMap::new();
        huffman_codes.insert('a', 0);
        huffman_codes.insert('b', 2);
        huffman_codes.insert('c', 3);
        let original_string = reconstruct_original_string(binary_string, &huffman_codes);
        assert_eq!(original_string, "bbcaaaaaaa");
    }

    #[test]
    fn test_empty_file() {
        let file = File {
            contents: vec![0, 0],
            file_path: String::from("empty.zipf"),
        };
        let decoded = decode_contents(&file);
        assert_eq!(decoded, "");
    }
}
