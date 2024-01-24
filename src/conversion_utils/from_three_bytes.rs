use core::fmt;

use crate::enums::endian::Endian;
use super::add_bytes_as_number_impl;


pub trait From3Bytes: Sized {
    fn from_be_bytes(bytes: [u8; 3]) -> Self;
    fn from_le_bytes(bytes: [u8; 3]) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u32);

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

impl fmt::Display for U24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I24(i32);

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

impl fmt::Display for I24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


pub fn add_three_bytes_as_number<T: From3Bytes>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
    // FIXME: It may be better to get Endian instead of &Endian since it is
    // just a simple enum with no value inside
    endian: &Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    add_bytes_as_number_impl(
        bytes, numbers,
        match endian {
            Endian::Big    => T::from_be_bytes,
            Endian::Little => T::from_le_bytes,
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_bytes() {
        let mut v: Vec<U24> = Vec::new();
        add_three_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Big).unwrap();
        add_three_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Little).unwrap();
        assert_eq!(v.as_slice(), &[U24(66049), U24(262146), U24(131172), U24(66049), U24(131076), U24(6553602)]);
    }
}

