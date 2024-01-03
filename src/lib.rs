use std::error::Error;
use std::fmt::Display;
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
    for i in (0..1600).step_by(4) {
        let bytes = [
            contents[i],
            contents[i + 1],
            contents[i + 2],
            contents[i + 3],
        ];
        if let Some(u32_integer) = convert_to_u32(bytes) {
            u32_numbers.push(u32_integer);

            print_converted_elements_with_padding(&u32_numbers, u32_integer);
        } else {
            return Err(From::from("Failed to convert bytes to u32."));
        }
    }

    Ok(())
}

fn print_converted_elements_with_padding<T: Display>(elements: &Vec<T>, element: T) {
    print!("{:10} ", element);
    if (elements.len() % 8) == 0 {
        println!();
    }
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
