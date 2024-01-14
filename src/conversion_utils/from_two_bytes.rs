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
    endian: &Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = bytes.len() - (bytes.len() % 2);

    for i in (0..max_index).step_by(2) {
        let chunk = [bytes[i], bytes[i + 1]];
        let number = match endian {
            Endian::Big => T::from_be_bytes(chunk),
            Endian::Little => T::from_le_bytes(chunk),
        };
        numbers.push(number);
    }

    Ok(())
}
