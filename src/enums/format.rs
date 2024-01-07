use strum_macros::EnumIter; // To derive the iterator functionality.
#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]

pub enum Format {
    Uint32,
    Int32,
    Int8,
    Uint8,
    Ascii,
}
