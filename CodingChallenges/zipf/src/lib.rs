mod error;
mod file;
mod logic;

use std::collections::HashMap;

use error::Error;
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
    tree.print_inorder();
}

fn construct_huffman_tree(_freq_map: &HashMap<char, i32>) -> HuffmanNode<i32> {
    let node = HuffmanNode::new(0);

    let node = node.add_left(HuffmanNode::new(1));
    let node = node.add_right(HuffmanNode::new(2));

    let node1 = HuffmanNode::new(4);
    let node1 = node1.add_left(node);

    return node1;
}
