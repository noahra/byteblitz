use strum_macros::EnumIter; // To derive the iterator functionality.
#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]

pub enum Format {
    Uint32,
    Int32,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int24,
    Uint24,
    Uint64,
    Int64,
    Ascii,
}
