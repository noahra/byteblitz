use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();
    let bytes = [contents[0], contents[1], contents[2], contents[3]];
    let num = u32::from_be_bytes(bytes);
    u32_numbers.push(num);
    for num in u32_numbers {
        println!("{}", num);
    }

    Ok(())
}
