use strum_macros::EnumIter; // To derive the iterator functionality.
#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]

pub enum Format {
    Hex,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int24,
    Uint24,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Ascii,
    F32,
    F64,
}
