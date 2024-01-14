use crate::enums::endian::Endian;

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
    let max_index = bytes.len() - (bytes.len() % 8);

    for i in (0..max_index).step_by(8) {
        let chunk = [
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
            bytes[i + 4],
            bytes[i + 5],
            bytes[i + 6],
            bytes[i + 7],
        ];
        let number = match endian {
            Endian::Big => T::from_be_bytes(chunk),
            Endian::Little => T::from_le_bytes(chunk),
        };

        numbers.push(number);
    }

    Ok(())
}
