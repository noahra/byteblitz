use core::fmt;

use super::FromBytes;
use crate::enums::endian::Endian;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u32);


impl fmt::Display for U24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromBytes<3> for U24 {
    fn from_bytes(bytes: [u8; 3], endian: Endian) -> Self {
        match endian {
            Endian::Big => {
                let num = (bytes[0] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[2] as u32);
                U24(num)
            }
            Endian::Little => {
                let num = (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[0] as u32);
                U24(num)
            }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I24(i32);


impl fmt::Display for I24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromBytes<3> for I24 {
    fn from_bytes(bytes: [u8; 3], endian: Endian) -> Self {
        match endian {
            Endian::Big => {
                let num = ((bytes[0] as i32) << 16) | ((bytes[1] as i32) << 8) | (bytes[2] as i32);
                // Sign extension for 24-bit numbers
                I24((num << 8) >> 8)
            }
            Endian::Little => {
                let num = ((bytes[2] as i32) << 16) | ((bytes[1] as i32) << 8) | (bytes[0] as i32);
                // Sign extension for 24-bit numbers
                I24((num << 8) >> 8)
            }

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_bytes() {
        let mut v: Vec<U24> = Vec::new();
        U24::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Big, &mut v).unwrap();
        U24::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Little, &mut v).unwrap();
        assert_eq!(
            v.as_slice(),
            &[
                U24(66049),
                U24(262146),
                U24(131172),
                U24(66049),
                U24(131076),
                U24(6553602)
            ]
        );
    }
}

