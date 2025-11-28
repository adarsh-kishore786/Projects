use crate::file::File;
use std::collections::HashMap;

pub fn decode_contents(file: &File) -> String {
    let header_length = file.contents[0] as usize;
    let body_padding = file.contents[1] as usize;
    let header_bytes = &file.contents[2..2 + header_length];
    let body_bytes = &file.contents[2 + header_length..];

    let header_str = String::from_utf8(header_bytes.to_vec()).unwrap();
    let huffman_codes: HashMap<char, u32> = parse_header(&header_str);
    let binary_string = decode_bytes_to_binary_string(body_bytes, body_padding);
    let decoded_string = reconstruct_original_string(&binary_string, &huffman_codes);

    return decoded_string;
}

fn parse_header(header: &str) -> HashMap<char, u32> {
    let mut huffman_codes: HashMap<char, u32> = HashMap::new();
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

fn reconstruct_original_string(binary_string: &str, huffman_codes: &HashMap<char, u32>) -> String {
    let mut inverted_codes: HashMap<u32, char> = HashMap::new();
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
