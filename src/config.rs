use std::env;

use crate::enums::format::Format;

pub struct Config {
    pub file_path: String,
    pub format: Format,
    pub little_endianess: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let little_endianess = env::var("LITTLE_ENDIAN").is_ok();
        let mut format = Format::Hex;
        let file_path: String;

        if args.len() == 3 {
            file_path = args[1].clone();
        } else if args.len() > 3 {
            file_path = args[2].clone();
            format = match &args[1] {
                "-hex" => Format::Hex,
                "-int8" => Format::Int8,
                "-uint8" => Format::Uint8,
                // ... handle other formats
                _ => return Err("invalid format"),
            };
        }

        Ok(Config {
            file_path,
            format,
            little_endianess,
        })
    }
}
