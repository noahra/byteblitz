use std::convert::From;

pub fn convert_to_u32(bytes: [u8; 4]) -> Option<u32> {
    if bytes.len() >= 4 {
        let b = [bytes[0], bytes[1], bytes[2], bytes[3]];
        Some(u32::from_be_bytes(b))
    } else {
        None
    }
}

pub fn convert_to_utf8(byte: u8) -> Option<char> {
    if byte <= 127 {
        Some(byte as char)
    } else {
        None
    }
}

pub fn add_bytes_as_u32(
    bytes: &[u8],
    u32_numbers: &mut Vec<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = bytes.len() - (bytes.len() % 4);

    for i in (0..max_index).step_by(4) {
        let chunk = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        if let Some(u32_integer) = convert_to_u32(chunk) {
            u32_numbers.push(u32_integer);
        } else {
            return Err(From::from("Failed to convert bytes to u32."));
        }
    }

    Ok(())
}
pub fn convert_bytes_to_utf8(
    bytes: &[u8],
    utf8_chars: &mut Vec<char>,
) -> Result<(), Box<dyn std::error::Error>> {
    for &byte in bytes {
        if !byte.is_ascii_alphabetic() {
            // Skip to the next iteration if it's not a letter
            continue;
        }

        match convert_to_utf8(byte) {
            Some(utf8_char) => utf8_chars.push(utf8_char),
            None => return Err(From::from("Failed to convert bytes to UTF-8.")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion() {
        let result = convert_to_u32([0x89, 0x50, 0x4E, 0x47]);
        assert_eq!(result, Some(2303741511));
    }

    #[test]
    fn test_convert_bytes_to_utf8() {
        let bytes = vec![72, 101, 108, 108, 111]; // ASCII for "Hello"
        let expected_chars: Vec<char> = "Hello".chars().collect();
        let mut utf8_chars = Vec::new();
        let result = convert_bytes_to_utf8(&bytes, &mut utf8_chars);
        assert!(result.is_ok()); // Ensure no error occurred
        assert_eq!(utf8_chars, expected_chars); // Check the converted characters
    }
}
