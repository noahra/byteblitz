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

        let file_path = if args.len() >= 3 {
            args[2].clone()
        } else {
            args[1].clone()
        };

        if args.len() > 3 {
            format = match &args[1] {
                arg if arg == "-hex" => Format::Hex,
                arg if arg == "-int8" => Format::Int8,
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
