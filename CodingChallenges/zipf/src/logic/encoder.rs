use std::collections::HashMap;

pub fn get_compressed_contents(file_contents: &str, huffman_codes: &HashMap<char, String>) -> Vec<u8> {
    let mut header = String::new();
    let mut body = String::new();

    // header
    for (ch, code) in huffman_codes {
        header.push_str(&format!("{}{}", ch, code));
    }

    // body
    for ch in file_contents.chars() {
        if let Some(code) = huffman_codes.get(&ch) {
            body.push_str(code);
        }
    }

    let body_padding = body.len() % 8;

    let mut contents = Vec::new();
    contents.push(header.len() as u8);
    contents.push(body_padding as u8);
    contents.extend(header.as_bytes());
    contents.extend(encode_binary_string_to_bytes(&mut body));

    return contents;
}

fn encode_binary_string_to_bytes(binary_string: &mut String) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut i = 0;

    if (binary_string.len() % 8) != 0 {
        let padding = 8 - (binary_string.len() % 8);
        for _ in 0..padding {
            binary_string.push('0');
        }
    }

    while i < binary_string.len() {
        let byte_str = &binary_string[i..i+8];
        let byte = u8::from_str_radix(byte_str, 2).unwrap();
        bytes.push(byte);
        i += 8;
    }
    return bytes;
}
