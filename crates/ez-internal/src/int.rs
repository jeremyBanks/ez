/// Implicint.
pub struct Int(i128);

/// Coerce or parse a value to an int.
pub fn int(value: i128) -> Int {
    Int(value)
}
