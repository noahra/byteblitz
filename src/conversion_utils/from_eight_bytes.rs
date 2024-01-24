use crate::enums::endian::Endian;
use super::add_bytes_as_number_impl;

pub trait From8Bytes: Sized {
    fn from_be_bytes(bytes: [u8; 8]) -> Self;
    fn from_le_bytes(bytes: [u8; 8]) -> Self;
}

impl From8Bytes for u64 {
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        u64::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        u64::from_le_bytes(bytes)
    }
}

impl From8Bytes for i64 {
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        i64::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        i64::from_le_bytes(bytes)
    }
}

impl From8Bytes for f64 {
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        f64::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        f64::from_le_bytes(bytes)
    }
}

pub fn add_eight_bytes_as_number<T: From8Bytes>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
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
        let mut v: Vec<u64> = Vec::new();
        add_eight_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Big).unwrap();
        add_eight_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Little).unwrap();
        println!("{v:?}");
        assert_eq!(v.as_slice(), &[72621660682977792, 565149043851777]);
    }
}

