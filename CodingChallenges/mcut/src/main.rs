use std::env;

fn main() {
    let args = env::args().collect();
    mcut::process(&args);
}
