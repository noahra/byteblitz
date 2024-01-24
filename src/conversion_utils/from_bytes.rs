use thiserror::Error;

use crate::enums::endian::Endian;

#[derive(Error, Debug)]
pub enum FromBytesError {
    #[error("{0:?} trailing bytes were given")]
    Trailing(usize),
}

// https://github.com/rust-lang/rust/issues/76560
// Once (if) this gets completed it may be better to use it
pub trait FromBytes<const N: usize> {
    /// Transforms the array into a single [Self] with the specified
    /// [endianness](Endian).
    fn from_bytes(bytes: [u8; N], endian: Endian) -> Self;

    /// Transforms the array into an iterator of [Self]s.
    ///
    /// # Output
    ///
    /// If no errors occur, an iterator of [Self]. If the array given to it is
    /// not a multiple of [N], returns the [FromBytesError::Trailing]
    /// error.
    fn from_multiple_bytes(bytes: &[u8], endian: Endian)
    -> Result<impl Iterator<Item=Self>, FromBytesError>
        where
    Self: Sized,
    {
        match bytes.len() % N {
            0 => {},
            x => return Err(FromBytesError::Trailing(x)),
        }
        Ok(bytes
            // Divides the bytes into chunks of N
            .chunks_exact(N)
            // Converts the &[u8]s into [u8; N]
            // Unwrap is fine since chunks_exact always returns the right size
            .map(|x| x.try_into().unwrap())
            // Converts the [u8; N]s to Ts
            .map(move |x| Self::from_bytes(x, endian)))
    }

    /// Appends [Self]s taken from the array to the [Vec] `numbers`. If
    /// trailing bytes are there, an error is returned, but `numbers` gets how
    /// many bytes are possible
    ///
    /// # Output
    ///
    /// If no errors occur, nothing is returned. Errors from
    /// [Self::from_multiple_bytes] are propagated.
    fn add_bytes(bytes: &[u8], endian: Endian, numbers: &mut Vec<Self>)
    -> Result<(), FromBytesError>
        where
    Self: Sized,
    {
        match Self::from_multiple_bytes(bytes, endian) {
            Ok(i) => {
                numbers.extend(i);
                Ok(())
            }
            Err(e@FromBytesError::Trailing(x)) => {
                numbers.extend(Self::from_multiple_bytes(
                    &bytes[0..bytes.len() - x], endian
                ).unwrap());
                Err(e)
            }
        }
    }
}


/// This macro makes the implementation of [FromBytes] for types that have a
/// function named `from_be_bytes` and one named `from_le_bytes` easier. This
/// is the case for most number types.
macro_rules! implement_from_bytes {
    ($name:ident, $n:literal) => {
        impl FromBytes<$n> for $name {
            fn from_bytes(bytes: [u8; $n], endian: Endian) -> Self {
                match endian {
                    Endian::Big => Self::from_be_bytes(bytes),
                    Endian::Little => Self::from_le_bytes(bytes),
                }
            }
        }
    }
}


implement_from_bytes!(u16, 2);
implement_from_bytes!(i16, 2);

implement_from_bytes!(u32, 4);
implement_from_bytes!(i32, 4);
implement_from_bytes!(f32, 4);

implement_from_bytes!(u64, 8);
implement_from_bytes!(i64, 8);
implement_from_bytes!(f64, 8);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_bytes() {
        let mut v: Vec<u16> = Vec::new();
        u16::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Big, &mut v).unwrap();
        u16::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Little, &mut v).unwrap();
        assert_eq!(v.as_slice(), &[258, 260, 2, 512, 513, 1025, 512, 2]);
    }


    #[test]
    fn test_add_four_bytes() {
        let mut v: Vec<u32> = Vec::new();
        u32::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Big, &mut v).unwrap();
        u32::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Little, &mut v).unwrap();
        assert_eq!(v.as_slice(), &[16908548, 131584, 67174913, 131584]);
    }


    #[test]
    fn test_add_eight_bytes() {
        let mut v: Vec<u64> = Vec::new();
        u64::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Big, &mut v).unwrap();
        u64::add_bytes(&[1, 2, 1, 4, 0, 2, 2, 0, 100], Endian::Little, &mut v).unwrap();
        println!("{v:?}");
        assert_eq!(v.as_slice(), &[72621660682977792, 565149043851777]);
    }
}

