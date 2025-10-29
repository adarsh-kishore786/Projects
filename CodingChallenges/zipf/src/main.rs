use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    zipf::compress(&args);
}
