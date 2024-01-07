use std::convert::From;

pub fn convert_to_ascii(byte: u8) -> Option<char> {
    if byte <= 127 {
        Some(byte as char)
    } else {
        None
    }
}


pub trait From4Bytes: Sized {
    fn from_be_bytes(bytes: [u8; 4]) -> Self;
    fn from_le_bytes(bytes: [u8; 4]) -> Self;
}

impl From4Bytes for u32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        u32::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes)
    }
}

impl From4Bytes for i32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        i32::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        i32::from_le_bytes(bytes)
    }
}

pub fn add_bytes_as_number<T: From4Bytes>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
    endian: Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = bytes.len() - (bytes.len() % 4);

    for i in (0..max_index).step_by(4) {
        let chunk = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        let number = match endian {
            Endian::Big => T::from_be_bytes(chunk),
            Endian::Little => T::from_le_bytes(chunk),
        };
        numbers.push(number);
    }

    Ok(())
}

// Enum to specify endianness
pub enum Endian {
    Big,
    Little,
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
