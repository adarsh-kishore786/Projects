mod error;
mod file;
mod logic;

use file::File;
use std::collections::HashMap;

use error::Error;
use logic::encoder;
use logic::decoder;
use logic::freq;
use logic::tree::HuffmanNode;

pub fn process(args: &Vec<String>) {
    if args.len() < 2 {
        error::exit(
            "Usage: ./zipf path/to/file",
            Error::NoArgs 
        );
    }

    let input_file = file::read_file(&args[1]);
    let output_file_path = format!("{}.zipf", &input_file.file_path);
    let contents = String::from_utf8(input_file.contents).expect("Not supported for non-UTF8 files!").trim().to_string();

    println!("Compressing {}...", input_file.file_path);

    let freq_map = freq::get_frequency(&contents);
    let tree = construct_huffman_tree(&freq_map);
    let codes = tree.get_huffman_codes();

    compress(&contents, &output_file_path, &codes);
    println!("File compressed and saved to {}", output_file_path);

    decompress(file::read_file(&output_file_path));
    println!("Done!");
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

fn compress(contents: &str, file_path: &str, huffman_codes: &HashMap<char, String>) {
    let contents = encoder::get_compressed_contents(&contents, huffman_codes);
    file::write_file(file_path, &contents);
}

fn decompress(input_file: File) {
    println!("Decompressing {}...", input_file.file_path);
    let decoded_string = decoder::decode_contents(&input_file);
    let output_file_path = input_file.file_path.replace(".zipf", ".unzipf");
    file::write_file(&output_file_path, decoded_string.as_bytes());
    println!("File decompressed and saved to {}", output_file_path);
}
