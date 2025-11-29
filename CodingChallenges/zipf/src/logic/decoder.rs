use crate::file::File;
use std::collections::BTreeMap;

pub fn decode_contents(file: &File) -> String {
    let header_length = ((file.contents[0] as usize) << 8) | (file.contents[1] as usize);
    let body_padding = file.contents[2] as usize;
    let header_bytes = &file.contents[3..3 + header_length];
    let body_bytes = &file.contents[3 + header_length..];

    let header_str = String::from_utf8(header_bytes.to_vec()).unwrap();
    let huffman_codes: BTreeMap<char, u128> = parse_header(&header_str);
    let binary_string = decode_bytes_to_binary_string(body_bytes, body_padding);
    let decoded_string = reconstruct_original_string(&binary_string, &huffman_codes);

    return decoded_string;
}

fn parse_header(header: &str) -> BTreeMap<char, u128> {
    let mut huffman_codes: BTreeMap<char, u128> = BTreeMap::new();
    
    for segment in header.split("=|") {
        if segment.is_empty() {
            continue;
        }
        
        let mut chars = segment.chars();
        
        // First character is the symbol
        if let Some(ch) = chars.next() {
            // Remaining characters are the binary code
            let code: String = chars.collect();
            
            if !code.is_empty() {
                huffman_codes.insert(ch, u128::from_str_radix(&code, 2).unwrap()); // issue, this
                // loses leading zeros
            }
        }
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

fn reconstruct_original_string(binary_string: &str, huffman_codes: &BTreeMap<char, u128>) -> String {
    let mut inverted_codes: BTreeMap<String, char> = BTreeMap::new();
    for (ch, code) in huffman_codes {
        inverted_codes.insert(format!("{:b}", code), *ch);
    }

    let mut original_string = String::new();
    let mut current_code = String::new();

    for bit in binary_string.chars() {
        current_code.push(bit);

        if let Some(&ch) = inverted_codes.get(&current_code) {
            original_string.push(ch);
            current_code.clear();
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
            contents: vec![
                0, 14, 3,
                b'=', b'|', b'a', b'0', 
                b'=', b'|', b'b', b'1', b'0', 
                b'=', b'|', b'=', b'1', b'1',
                0b10101100, 0b00000000
            ],
            file_path: String::from("test.zipf"),
        };
        let decoded = decode_contents(&file);
        assert_eq!(decoded, "bb=aaaaaaa");
    }

    #[test]
    fn test_parse_header() {
        let header = "=|a0=|b10=|111";
        let huffman_codes = parse_header(header);
        assert_eq!(huffman_codes.get(&'a'), Some(&0));
        assert_eq!(huffman_codes.get(&'b'), Some(&2));
        assert_eq!(huffman_codes.get(&'1'), Some(&3));
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
        let mut huffman_codes: BTreeMap<char, u128> = BTreeMap::new();
        huffman_codes.insert('a', 0);
        huffman_codes.insert('b', 2);
        huffman_codes.insert('c', 3);
        let original_string = reconstruct_original_string(binary_string, &huffman_codes);
        assert_eq!(original_string, "bbcaaaaaaa");
    }

    #[test]
    fn test_empty_file() {
        let file = File {
            contents: vec![0, 0, 0],
            file_path: String::from("empty.zipf"),
        };
        let decoded = decode_contents(&file);
        assert_eq!(decoded, "");
    }
}
