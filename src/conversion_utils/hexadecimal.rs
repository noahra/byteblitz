pub fn convert_to_hex(byte: u8) -> String {
    format!("{:02X}", byte) // Converts byte to a string in hexadecimal format
}

pub fn convert_bytes_to_hex(
    bytes: &[u8],
    hex_strings: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for &byte in bytes {
        let hex_string = convert_to_hex(byte);
        hex_strings.push(hex_string);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_bytes_to_hex() {
        let bytes = vec![72, 101, 108, 108, 111]; // ASCII for "Hello"
        let expected_hex: Vec<String> = ["48", "65", "6c", "6c", "6f"] // Hex for "Hello"
            .iter().map(|&s| s.into()).collect();
        let mut hex_strings = Vec::new();
        let result = convert_bytes_to_hex(&bytes, &mut hex_strings);
        assert!(result.is_ok()); // Ensure no error occurred
        assert_eq!(hex_strings, expected_hex); // Check the converted strings
    }
}
