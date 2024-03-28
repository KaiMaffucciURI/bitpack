//use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let max = 1 << (width - 1);
    let min = -(1 << (width - 1));
    n >= min && n < max
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    n < 1 << width
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bytes.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
/// or `None` if the field is impossible
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    // check if the field is possible
    if lsb + width > 64 {
        return None;
    } else if width == 0 || width + lsb == 64 {
        return Some(0);
    }
    Some(((((1 << width) - 1) << lsb) & word) as i64 >> lsb)
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bytes.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
///
/// or None
/// if `lsb + width > 64`
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    // check if the field is possible
    if lsb + width > 64 {
        return None;
    } else if width == 0 || width + lsb == 64 {
        return Some(0);
    }
    let new_word: u64 = word << (64 - (lsb + width));
    Some(new_word >> (64 - width))
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
// /// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the unsigned value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if lsb + width >= 64 {
        return None;
    }
    // check if the value fits in the field of the word starting at lsb and having width bits
    if fitsu(value, width) {

        let mask: u64 = ((1 << width) - 1) << lsb;
        let masked_word = word & !mask;
        let masked_value = ((value) << lsb) & mask;
        let new_word = masked_word | masked_value;
        Some(new_word)
    } else {
        None
    }
}

/// Given an unsigned 64-bit `word`, and a signed `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the signed value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
///
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    // check if the value fits in the field of the word starting at lsb and having width bits
    if fitss(value, width) {
        let mask: u64 = ((1 << width) - 1) << lsb;
        let masked_word = word & !mask;
        let masked_value = ((value as u64) << lsb) & mask;
        let new_word = masked_word | masked_value;
        Some(new_word)
        
    } else {
        None
    }
}
