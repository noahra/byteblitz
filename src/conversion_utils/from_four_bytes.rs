use crate::enums::endian::Endian;

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
    endian: &Endian,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = bytes.len() - (bytes.len() % 4);

    for i in (0..max_index).step_by(4) {
        let chunk = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        let number = match endian {
            Endian::Big => T::from_be_bytes(chunk),
            Endian::Little => T::from_le_bytes(chunk),
        };
        numbers.push(number);
    }

    Ok(())
}

