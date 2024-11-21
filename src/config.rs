use std::env;

pub struct Config {
    pub file_path: String,
    pub little_endianess: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            return Err("user needs help");
        }

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let little_endianess = env::var("LITTLE_ENDIAN").is_ok();

        let file_path = args[1].clone();
        Ok(Config {
            file_path,
            little_endianess,
        })
    }
}
