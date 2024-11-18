use std::env;

pub struct Config {
    pub file_path: String,
    pub little_endianess: bool,
    pub show_help: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            return Ok(Config {
                file_path: String::new(),
                little_endianess: false,
                show_help: true,
            });
        }

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let little_endianess = env::var("LITTLE_ENDIAN").is_ok();

        let file_path = args[1].clone();
        Ok(Config {
            file_path,
            little_endianess,
            show_help: false,
        })
    }
}
