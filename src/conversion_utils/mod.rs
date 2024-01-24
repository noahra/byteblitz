pub mod ascii;
pub mod from_eight_bytes;
pub mod from_four_bytes;
pub mod from_one_byte_to_i8;
pub mod from_three_bytes;
pub mod from_two_bytes;
pub mod hexadecimal;

/// Helper function to make it easier to implement the conversions
fn add_bytes_as_number_impl<T, const N: usize>(
    bytes: &[u8],
    numbers: &mut Vec<T>,
    f: impl Fn([u8; N]) -> T,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = bytes
        // Divides the bytes into chunks of N
        .chunks_exact(N)
        // Converts the &[u8]s into [u8; N]
        // Unwrap is fine since chunks_exact always returns the right size
        .map(|x| x.try_into().unwrap())
        // Converts the [u8; N]s to Ts
        .map(f);
    // Append to numbers
    numbers.extend(result);
    Ok(())
}
