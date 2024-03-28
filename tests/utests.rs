// tests functions involving just unsigned values
//use bitpack::bitpack::*;
use bitpack::bitpack::{newu, fitsu};

// fitsu tests
#[test]
fn test_fitsu() {
    // test whether unsigned value 'n' fits into 'width' unsigned bits
    // 0 fits in 0 bits
    assert_eq!(fitsu(0, 0), true);
    // 0 fits in 1 bit
    assert_eq!(fitsu(0, 1), true);
    // 0 fits in 2 bits
    assert_eq!(fitsu(0, 2), true);
    // 0 fits in 3 bits
    assert_eq!(fitsu(0, 3), true);
    // 0 fits in 4 bits
    assert_eq!(fitsu(0, 4), true);
    // 0 fits in 5 bits
    assert_eq!(fitsu(0, 5), true);
    // 0 fits in 6 bits
    assert_eq!(fitsu(0, 6), true);
    // 0 fits in 7 bits
    assert_eq!(fitsu(0, 7), true);
    // 0 fits in 8 bits
    assert_eq!(fitsu(0, 8), true);
    // 0 fits in 9 bits
    assert_eq!(fitsu(0, 9), true);
    // 0 fits in 10 bits
    assert_eq!(fitsu(0, 10), true);
    // 5 fits in 2 bits
    assert_eq!(fitsu(5, 2), false);
    // 5 fits in 3 bits
    assert_eq!(fitsu(5, 3), true);
    // 17 fits in 4 bits
    assert_eq!(fitsu(17, 4), false);
    // 17 fits in 5 bits
    assert_eq!(fitsu(17, 5), true);
    // 1 bit into 65 bits - throws error
    assert_eq!(fitsu(1, 64), true);
}

// newu tests (requires fitsu to be implemented first and tested)
#[test]
fn test_newu() {
    // testing with an empty word
    let word = 0;
    // 0 fits in 1 bit
    assert_eq!(newu(word, 0, 0, 0), Some(0));
    // trying to fit other various values into 0 bits
    assert_eq!(newu(word, 0, 0, 1), None);
    assert_eq!(newu(word, 0, 0, 2), None);
    assert_eq!(newu(word, 0, 0, 3), None);
    // trying to fit various values in 2 bits
    assert_eq!(newu(word, 2, 0, 0), Some(0));
    assert_eq!(newu(word, 2, 0, 1), Some(1));
    assert_eq!(newu(word, 2, 0, 2), Some(2));
    assert_eq!(newu(word, 2, 0, 3), Some(3));
    assert_eq!(newu(word, 2, 0, 4), None);
    assert_eq!(newu(word, 2, 0, 5), None);

    assert_eq!(newu(11574355939197657136, 1, 0, 1), Some(11574355939197657137));
    // what happens if we have a width that is too large for its respective lsb?
    //assert_eq!(newu(word, 65, 0, 0), None);
    //assert_eq!(newu(word, 65, 0, 1), None);
    //assert_eq!(newu(word, 65, 0, 18446744073709551615), None);
}