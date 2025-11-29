use crate::file::File;
use crate::logic::freq;
use crate::logic::tree::HuffmanNode;
use std::collections::BTreeMap;

pub fn encode_contents(input_file: File) -> Vec<u8> {
    let contents = String::from_utf8(input_file.contents).expect("Not supported for non-UTF8 files!").to_string();

    let freq_map: BTreeMap<char, u32> = freq::get_frequency(&contents);
    println!("Frequency Map: {:?}", freq_map);
    let tree: HuffmanNode = construct_huffman_tree(&freq_map);
    print!("Huffman Tree: ");
    tree.print_preorder();
    let codes: BTreeMap<char, String> = tree.get_huffman_codes();
    println!("Huffman Codes: {:?}", codes);

    return get_compressed_contents(&contents, &codes);
}

fn construct_huffman_tree(freq_map: &BTreeMap<char, u32>) -> HuffmanNode {
    let mut nodes: Vec<HuffmanNode> = Vec::with_capacity(freq_map.len());

    for (&ch, &freq) in freq_map {
        let initial_tree = HuffmanNode::new(freq, ch);
        nodes.push(initial_tree);
    };

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| { a.freq.cmp(&b.freq) });
        let u = nodes.remove(0);
        let v = nodes.remove(0);
        let combined_node = HuffmanNode::combine(u, v);

        nodes.push(combined_node);
    }
    return nodes.remove(0);
}

fn get_compressed_contents(file_contents: &str, huffman_codes: &BTreeMap<char, String>) -> Vec<u8> {
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

    let mut contents: Vec<u8> = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::File;

    #[test]
    fn test_encode_contents() {
        let input_file = File {
            file_path: String::from("test.txt"),
            contents: b"aabaccddddaa".to_vec(),
        };
        let encoded_contents = encode_contents(input_file);
        assert_eq!(encoded_contents, vec![
            13, 6, 
            b'a', b'0',
            b'b', b'1', b'0', b'0', 
            b'c', b'1', b'0', b'1',
            b'd', b'1', b'1',
            0b00100010, 
            0b11011111,
            0b11110000
        ]);
    }

    #[test]
    fn test_encode_binary_string_to_bytes() {
        let mut binary_string = String::from("11010010111100");
        let bytes = encode_binary_string_to_bytes(&mut binary_string);
        assert_eq!(bytes, vec![0b11010010, 0b11110000]);
    }

    #[test]
    fn test_construct_huffman_tree() {
        let mut freq_map = BTreeMap::new();
        freq_map.insert('a', 5);
        freq_map.insert('b', 9);
        freq_map.insert('c', 12);
        freq_map.insert('d', 13);
        freq_map.insert('e', 16);
        freq_map.insert('f', 45);
        let tree = construct_huffman_tree(&freq_map);
        assert_eq!(tree.freq, 100);
    }

    #[test]
    fn test_get_compressed_contents() {
        let file_contents = "aaabbc";
        let mut huffman_codes = BTreeMap::new();
        huffman_codes.insert('a', String::from("0"));
        huffman_codes.insert('b', String::from("10"));
        huffman_codes.insert('c', String::from("11"));
        let compressed_contents = get_compressed_contents(file_contents, &huffman_codes);
        assert_eq!(compressed_contents, vec![
            8, 1,
            b'a', b'0', b'b', b'1', b'0', b'c', b'1', b'1',
            0b00010101,
            0b10000000
        ]);
    }
}
