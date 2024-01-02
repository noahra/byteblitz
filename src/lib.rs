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
    let u32_integer = convert_to_u32(bytes);
    u32_numbers.push(u32_integer);

    match u32_integer {
        Some(n) => println!("{}", n),
        None => return Err(From::from("Failed to convert bytes to u32.")),
    }

    Ok(())
}

pub fn convert_to_u32(bytes: [u8; 4]) -> Option<u32> {
    if bytes.len() >= 4 {
        let b = [bytes[0], bytes[1], bytes[2], bytes[3]];
        Some(u32::from_be_bytes(b))
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion() {
        let result = convert_to_u32([0x89, 0x50, 0x4E, 0x47]);
        assert_eq!(result, Some(2303741511));
    }
}
