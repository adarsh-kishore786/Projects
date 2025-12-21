use std::env;

fn main() {
    let args = env::args().collect();
    println!("{:?}", &args);
    mcut::process(&args);
}
