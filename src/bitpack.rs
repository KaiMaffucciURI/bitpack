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
    //let n_bits = n.to_be_bytes().len() as u64 * 8;
    //n >= -(1 << (width - 1)) && n < 1 << (width - 1)
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
    }
    // create a mask to clear the field
    let mask = !(((1 << width) - 1) << lsb);
    // clear the field
    let cleared_word = word & mask;
    // shift the field to the least significant bit
    let shifted_word = cleared_word >> lsb;
    // check if the field is negative
    if shifted_word & (1 << (width - 1)) != 0 {
        // create a mask to extend the sign bit
        let sign_extend = !((1 << width) - 1);
        // extend the sign bit
        let extended_word = shifted_word | sign_extend;
        // convert the field to a signed value
        let signed_word = extended_word as i64;
        Some(signed_word)
    } else {
        Some(shifted_word as i64)
    }
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
///
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    // check if the field is possible
    if lsb + width > 64 {
        return None;
    }
    // create a mask to clear the field
    let mask = !(((1 << width) - 1) << lsb);
    // clear the field
    let cleared_word = word & mask;
    // shift the field to the least significant bit
    let shifted_word = cleared_word >> lsb;
    Some(shifted_word)
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
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
    // check if the value fits in the field of the word starting at lsb and having width bits
    if fitsu(value, width) {
        // create a mask to clear the field
        let mask = !(((1 << width) - 1) << lsb);
        // clear the field
        let cleared_word = word & mask;
        // shift the value to the correct position
        let shifted_value = value << lsb;
        // set the field to the value
        let new_word = cleared_word | shifted_value;
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
        // create a mask to clear the field
        let mask = !(((1 << width) - 1) << lsb);
        // clear the field
        let cleared_word = word & mask;
        // shift the value to the correct position
        let shifted_value = (value as u64) << lsb;
        // set the field to the value
        let new_word = cleared_word | shifted_value;
        Some(new_word)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
