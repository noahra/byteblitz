use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading the contents of {}", filename);
}
