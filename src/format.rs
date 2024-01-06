use strum_macros::EnumIter; // To derive the iterator functionality.
#[derive(EnumIter, Debug, Clone, Copy)]

pub enum Format {
    Uint32,
    UtfEight,
}
