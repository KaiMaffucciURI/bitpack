// tests functions involving just signed values

use bitpack::bitpack::*;


#[test]
fn test_signed() {
    assert_eq!(news(0, 2, 0, 1).unwrap(), 1);
    assert_eq!(news(0, 2, 0, 0).unwrap(), 0);
    assert_eq!(news(100, 3, 2, -1).unwrap(), 124);
    assert_eq!(news(100, 3, 2, -2).unwrap(), 120);
    assert_eq!(news(101, 3, 2, -1).unwrap(), 125);
    assert_eq!(news(101, 3, 2, -2).unwrap(), 121);
    assert_eq!(news(100, 3, 2, 4), None);
    assert_eq!(news(100, 3, 2, -4).unwrap(), 112);
    assert_eq!(news(101, 3, 2, -4).unwrap(), 113);

    assert_eq!(gets(news(100, 3, 2, 2).unwrap(), 3, 2), Some(2));
    assert_eq!(gets(news(101, 3, 2, 3).unwrap(), 3, 2), Some(2));
    assert_eq!(gets(news(100, 3, 2, -4).unwrap(), 3, 2), Some(-4));
    assert_eq!(gets(news(101, 3, 2, -3).unwrap(), 3, 2), Some(-3));
}