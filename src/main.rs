use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read(file_path).expect("Should have been able to read the file");
    let mut u32_numbers = Vec::new();
    let bytes = [contents[0], contents[1], contents[2], contents[3]];
    let num = u32::from_be_bytes(bytes);
    u32_numbers.push(num);
    for num in u32_numbers {
        println!("{}", num);
    }
}
