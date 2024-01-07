use std::convert::From;

pub fn convert_to_ascii(byte: u8) -> Option<char> {
    if byte <= 127 {
        Some(byte as char)
    } else {
        None
    }
}

pub fn convert_bytes_to_ascii(
    bytes: &[u8],
    ascii_chars: &mut Vec<char>,
) -> Result<(), Box<dyn std::error::Error>> {
    for &byte in bytes {
        if !byte.is_ascii_alphabetic() {
            // Skip to the next iteration if it's not a letter
            continue;
        }
        match convert_to_ascii(byte) {
            Some(ascii_char) => ascii_chars.push(ascii_char),
            None => return Err(From::from("Failed to convert bytes to Ascii")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_bytes_to_ascii() {
        let bytes = vec![72, 101, 108, 108, 111]; // ASCII for "Hello"
        let expected_chars: Vec<char> = "Hello".chars().collect();
        let mut ascii_chars = Vec::new();
        let result = convert_bytes_to_ascii(&bytes, &mut ascii_chars);
        assert!(result.is_ok()); // Ensure no error occurred
        assert_eq!(ascii_chars, expected_chars); // Check the converted characters
    }
}
