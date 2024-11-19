use byteblitz::config::Config;
use std::env;
use std::process;

fn print_help_manual() {
    let help_text = format!(
        "ByteBlitz - Command-line tool for viewing the binary content of files in different formats
USAGE:
    byteblitz [FILE] [OPTIONS]
ARGUMENTS:
    <FILE>    Path to the binary file to be analyzed
OPTIONS:
    -h, --help      Show this help manual
    -l, --little    Force little-endian interpretation (overrides system default)
SUPPORTED FORMATS:
    - Hex         : Hexadecimal representation
    - Int8/Uint8  : 8-bit integers
    - Int16/Uint16: 16-bit integers
    - Int32/Uint32: 32-bit integers
    - Int64/Uint64: 64-bit integers
    - Float32/64  : 32/64-bit floating point numbers
    - ASCII       : Printable ASCII characters
INTERACTIVE CONTROLS:
    j/k           : Navigate up/down in the list
    h/l           : Switch between formats
    e             : Enter line number navigation mode
    q             : Quit the application
ENVIRONMENT VARIABLES:
    LITTLE_ENDIAN  : Set default endianness to little-endian
EXAMPLES:
    # Basic usage
    byteblitz binary_file.bin
    # View specific format
    byteblitz binary_file.bin
    # Navigate to a specific line
    1. Open byteblitz
    2. Press 'e'
    3. Enter line number"
    );

    println!("{}", help_text);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        if err == "user needs help" {
            print_help_manual();
            process::exit(0);
        }
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = byteblitz::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    Ok(())
}
