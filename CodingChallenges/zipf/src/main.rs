use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    zipf::compress(&args);

    let tree = zipf::construct_huffman_tree();
    tree.print_inorder();
}
