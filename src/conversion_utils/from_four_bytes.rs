use crate::enums::endian::Endian;
use super::add_bytes_as_number_impl;

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
impl From4Bytes for f32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        f32::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        f32::from_le_bytes(bytes)
    }
}

pub fn add_bytes_as_number<T: From4Bytes>(
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
        let mut v: Vec<u32> = Vec::new();
        add_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Big).unwrap();
        add_bytes_as_number(&[1, 2, 1, 4, 0, 2, 2, 0, 100], &mut v, &Endian::Little).unwrap();
        assert_eq!(v.as_slice(), &[16908548, 131584, 67174913, 131584]);
    }
}
