use core::fmt;

use crate::enums::endian::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I24(i32);

pub trait From3Bytes: Sized {
    fn from_be_bytes(bytes: [u8; 3]) -> Self;
    fn from_le_bytes(bytes: [u8; 3]) -> Self;
}

impl From3Bytes for U24 {
    fn from_be_bytes(bytes: [u8; 3]) -> Self {
        let num = (bytes[0] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[2] as u32);
        U24(num)
    }

    fn from_le_bytes(bytes: [u8; 3]) -> Self {
        let num = (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[0] as u32);
        U24(num)
    }
}

impl From3Bytes for I24 {
    fn from_be_bytes(bytes: [u8; 3]) -> Self {
        let num = ((bytes[0] as i32) << 16) | ((bytes[1] as i32) << 8) | (bytes[2] as i32);
        // Sign extension for 24-bit numbers
        I24((num << 8) >> 8)
    }

    fn from_le_bytes(bytes: [u8; 3]) -> Self {
        let num = ((bytes[2] as i32) << 16) | ((bytes[1] as i32) << 8) | (bytes[0] as i32);
        // Sign extension for 24-bit numbers
        I24((num << 8) >> 8)
    }
}

pub fn add_three_bytes_as_number<T: From3Bytes>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
    endian: &Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = bytes.len() - (bytes.len() % 3);

    for i in (0..max_index).step_by(3) {
        let chunk = [bytes[i], bytes[i + 1], bytes[i + 2]];
        let number = match endian {
            Endian::Big => T::from_be_bytes(chunk),
            Endian::Little => T::from_le_bytes(chunk),
        };
        numbers.push(number);
    }

    Ok(())
}

impl fmt::Display for U24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for I24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}