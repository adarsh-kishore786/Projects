mod error;
mod file;
mod logic;

use std::collections::HashMap;

use error::Error;
use file::File;
use logic::freq;
use logic::tree::HuffmanNode;

pub fn compress(args: &Vec<String>) {
    if args.len() < 2 {
        error::exit(
            "Usage: ./zipf path/to/file",
            Error::NoArgs 
        );
    }

    let file = file::read_file(&args[1]);
    println!("Compressing {}...", file.file_path);

    let freq_map = freq::get_frequency(&file);
    let tree = construct_huffman_tree(&freq_map);

    let codes = tree.get_huffman_codes();
    let output_file_path = format!("{}.zipf", &file.file_path);
    write_to_file(&file, &output_file_path, &codes);
    println!("File compressed and saved to {}", output_file_path);
}

fn construct_huffman_tree(freq_map: &HashMap<char, u32>) -> HuffmanNode {
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

fn write_to_file(input_file: &File, file_path: &str, huffman_codes: &HashMap<char, String>) {
    let mut contents = String::new();
    let file_contents = &input_file.contents;

    // header
    for (ch, code) in huffman_codes {
        contents.push_str(&format!("{}{}", ch, code));
    }
    contents.push_str("==");

    // body
    for ch in file_contents.chars() {
        if let Some(code) = huffman_codes.get(&ch) {
            contents.push_str(code);
        }
    }

    file::write_file(file_path, &contents);
}
