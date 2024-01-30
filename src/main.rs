use byteblitz::config::Config;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = byteblitz::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    Ok(())
}
