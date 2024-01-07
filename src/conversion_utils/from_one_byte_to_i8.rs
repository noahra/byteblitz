pub fn add_byte_as_i8(
    bytes: &[u8],
    numbers: &mut Vec<i8>,
) -> Result<(), Box<dyn std::error::Error>> {
    for &byte in bytes {
        numbers.push(byte as i8);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_byte_as_i8() {
        let bytes: Vec<u8> = vec![0, 127, 128, 255];

        let mut numbers: Vec<i8> = Vec::new();

        assert!(add_byte_as_i8(&bytes, &mut numbers).is_ok());

        let expected: Vec<i8> = vec![0, 127, -128, -1];

        assert_eq!(numbers, expected, "The conversion from u8 to i8 did not produce the expected results.");
    }
}