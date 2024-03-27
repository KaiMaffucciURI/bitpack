// tests functions involving just unsigned values
//use bitpack::bitpack::*;
use bitpack::bitpack::{newu, fitsu};

// fitsu tests


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
    assert_eq!(newu(word, 0, 0, 4), None);
    assert_eq!(newu(word, 0, 0, 5), None);
    assert_eq!(newu(word, 0, 0, 6), None);
    assert_eq!(newu(word, 0, 0, 7), None);
    // trying to fit various values in 2 bits
    assert_eq!(newu(word, 2, 0, 0), Some(0));
    assert_eq!(newu(word, 2, 0, 1), Some(1));
    assert_eq!(newu(word, 2, 0, 2), Some(2));
    assert_eq!(newu(word, 2, 0, 3), Some(3));
    assert_eq!(newu(word, 2, 0, 4), None);
    assert_eq!(newu(word, 2, 0, 5), None);
    // what happens if we have a width that is too large for its respective lsb?


}