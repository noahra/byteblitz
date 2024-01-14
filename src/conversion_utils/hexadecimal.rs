pub fn convert_to_hex(byte: u8) -> String {
    format!("{:02X}", byte)
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
        let bytes = vec![72, 101, 108, 108, 111];
        let expected_hex: Vec<String> = ["48", "65", "6C", "6C", "6F"]
            .iter()
            .map(|&s| s.into())
            .collect();
        let mut hex_strings = Vec::new();
        let result = convert_bytes_to_hex(&bytes, &mut hex_strings);
        assert!(result.is_ok());
        assert_eq!(hex_strings, expected_hex);
    }
}
