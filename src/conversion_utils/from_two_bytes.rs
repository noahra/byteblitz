use super::add_bytes_as_number_impl;
use crate::enums::endian::Endian;

pub trait From2Bytes: Sized {
    fn from_be_bytes(bytes: [u8; 2]) -> Self;
    fn from_le_bytes(bytes: [u8; 2]) -> Self;
}

impl From2Bytes for u16 {
    fn from_be_bytes(bytes: [u8; 2]) -> Self {
        u16::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 2]) -> Self {
        u16::from_le_bytes(bytes)
    }
}

impl From2Bytes for i16 {
    fn from_be_bytes(bytes: [u8; 2]) -> Self {
        i16::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 2]) -> Self {
        i16::from_le_bytes(bytes)
    }
}

pub fn add_two_bytes_as_number<T: From2Bytes>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
    // FIXME: It may be better to get Endian instead of &Endian since it is
    // just a simple enum with no value inside
    endian: &Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    add_bytes_as_number_impl(
        bytes,
        numbers,
        match endian {
            Endian::Big => T::from_be_bytes,
            Endian::Little => T::from_le_bytes,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_bytes() {
        let mut v: Vec<u16> = Vec::new();
        add_two_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Big).unwrap();
        add_two_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Little).unwrap();
        assert_eq!(v.as_slice(), &[258, 260, 2, 512, 513, 1025, 512, 2]);
    }
}
