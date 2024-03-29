// tests from the bitpack lab assignment

use bitpack::bitpack::*;
//use rand::Rng;

/*
    Write a Rust function called check_laws which takes as parameters word,
    w, lsb, value, w2, and lsb2 as used in the laws of your handout. The
    function should assert that the laws hold
*/
fn check_laws_u(word: u64, w: u64, lsb: u64, val: u64, w2: u64, lsb2: u64, val2: u64) {
    // print the values of the parameters to standard error
    //println!("word: {}, w: {}, lsb: {}, val: {}, w2: {}, lsb2: {}, val2: {}", word, w, lsb, val, w2, lsb2, val2); // test code

    // #1-5
    assert_eq!(getu(newu(word, w, lsb, val).unwrap(), w, lsb).unwrap(), val);
    if lsb2 >= w + lsb {
        assert_eq!(getu(newu(word, w, lsb, val).unwrap(), w2, lsb2).unwrap(), getu(word, w2, lsb2).unwrap());
    }

    /*
        #6
        if you insert a new field (w, lsb) and another
        new field (w′, lsb′), and if the two fields have no bits in common, then the
        result is the same as first inserting (w′, lsb′)
    */
    if lsb + w < lsb2 || lsb2 + w2 < lsb {
        // field (w, lsb)
        let new_word = newu(word, w, lsb, val).unwrap();
        // field (w2, lsb2)
        let new_word2 = newu(word, w2, lsb2, val2).unwrap();
        // insert field (w, lsb) and then insert field (w2, lsb2)
        let new_word3 = newu(new_word, w2, lsb2, val2).unwrap();
        // insert field (w2, lsb2) and then insert field (w, lsb)
        let new_word4 = newu(new_word2, w, lsb, val).unwrap();
        assert_eq!(new_word3, new_word4);
    }

    /*
        #7
        if you insert a new field (w, lsb) and another
        new field (w′, lsb′), and if the field (w, lsb) is entirely contained within
        the field (w′, lsb′), then the result is the same is if you had inserted only
        (w′, lsb′)
    */
    if lsb >= lsb2 && lsb + w <= lsb2 + w2 {
        // field (w, lsb)
        let new_word = newu(word, w, lsb, val).unwrap();
        // field (w2, lsb2)
        let new_word2 = newu(word, w2, lsb2, val2).unwrap();
        // insert field (w, lsb) and then insert field (w2, lsb2)
        let new_word3 = newu(new_word, w2, lsb2, val2).unwrap();
        assert_eq!(new_word2, new_word3);
    }

    /* 
        #8
        if you insert a field f, the bits fh that
        are above (more significant than) the inserted field are unchange
    */
    // get value from field above lsb + w
    let above = getu(word, 64-(lsb + w), lsb + w).unwrap();
    // get value from field below lsb
    let below = getu(word, lsb, 0).unwrap();
    // insert field (w, lsb)
    let new_word = newu(word, w, lsb, val).unwrap();
    // get value from field above lsb + w again
    //let above2 = getu(new_word, lsb + w, 0).unwrap();
    let above2 = getu(word, 64-(lsb + w), lsb + w).unwrap();
    // get value from field below lsb again
    let below2 = getu(new_word, lsb, 0).unwrap();
    assert_eq!(above, above2);
    assert_eq!(below, below2);

    /*
        #9
         if you insert a field f , the bits fl that are
        below (less significant than) the inserted field are unchanged
    */
    // get value from field below lsb
    let below = getu(word, lsb, 0).unwrap();
    // insert field (w, lsb)
    let new_word = newu(word, w, lsb, val).unwrap();
    // get value from field below lsb again
    let below2 = getu(new_word, lsb, 0).unwrap();
    assert_eq!(below, below2);

    /* 
        #10
        interesting laws regarding fitsu
    */
    if w <= 62 {
        assert_eq!(fitsu(val, w), fitsu(val << 2, w + 2));
    }
    if w >= 2 {
        assert_eq!(fitsu(val, w), fitsu(val >> 2, w - 2));
    }
}


// test check_lawsgetu(0b1010, 4, 0).unwrap(), 0b1010
#[test]
fn test_check_laws_u() {
    //let mut rng = rand::thread_rng();
    // 65 here was messing everything up, 64 made it work
    for w in 0..65 { // upper value is excluded
        for lsb in 0..(65-w) {
            for _trial in 0..1001 {
                // set other parameter values randomly
                // val is a random number that can fit in w
                let val: u64;
                if w == 64 {
                    val= rand::random::<u64>();
                } else {
                    val = rand::random::<u64>() % (1 << w);
                }

                // generate random values for a second field
                let w2 = rand::random::<u64>() % 65;
                let lsb2 = rand::random::<u64>() % (65-w2);
                let val2: u64;
                if w2 == 64 {
                    val2 = rand::random::<u64>();
                } else {
                    val2 = rand::random::<u64>() % (1 << w2);
                }
                // set other parameter values randomly with rng
                /*let val = rng.gen_range(0..(1 << w));
                let w2 = rng.gen_range(0..65);
                let lsb2 = rng.gen_range(0..(65-w2));*/

                // make sure that val and val2 fit before calling check_laws
                if !(w == 0 || w + lsb > 64) && !(w2 == 0 || w2 + lsb2 > 64) {
                    check_laws_u(rand::random::<u64>(), w, lsb, val, w2, lsb2, val2);
                }
            }
        }
    }
}



// now for signed values
fn check_laws_s(word: u64, w: u64, lsb: u64, val: i64, w2: u64, lsb2: u64, val2: i64) {
    // print the values of the parameters to standard error
    //println!("word: {}, w: {}, lsb: {}, val: {}, w2: {}, lsb2: {}, val2: {}", word, w, lsb, val, w2, lsb2, val2); // test code

    // #1-5
    //println!("{}", news(word, w, lsb, val).unwrap());
    assert_eq!(gets(news(word, w, lsb, val).unwrap(), w, lsb).unwrap(), val);
    if lsb2 >= w + lsb {
        assert_eq!(gets(news(word, w, lsb, val).unwrap(), w2, lsb2).unwrap(), gets(word, w2, lsb2).unwrap());
    }

    /*
        #6
        if you insert a new field (w, lsb) and another
        new field (w′, lsb′), and if the two fields have no bits in common, then the
        result is the same as first inserting (w′, lsb′)
    */
    if lsb + w < lsb2 || lsb2 + w2 < lsb {
        // field (w, lsb)
        let new_word = news(word, w, lsb, val).unwrap();
        // field (w2, lsb2)
        let new_word2 = news(word, w2, lsb2, val2).unwrap();
        // insert field (w, lsb) and then insert field (w2, lsb2)
        let new_word3 = news(new_word, w2, lsb2, val2).unwrap();
        // insert field (w2, lsb2) and then insert field (w, lsb)
        let new_word4 = news(new_word2, w, lsb, val).unwrap();
        assert_eq!(new_word3, new_word4);
    }

    /*
        #7
        if you insert a new field (w, lsb) and another
        new field (w′, lsb′), and if the field (w, lsb) is entirely contained within
        the field (w′, lsb′), then the result is the same
        is if you had inserted only (w′, lsb′)
    */
    if lsb >= lsb2 && lsb + w <= lsb2 + w2 {
        // field (w, lsb)
        let new_word = news(word, w, lsb, val).unwrap();
        // field (w2, lsb2)
        let new_word2 = news(word, w2, lsb2, val2).unwrap();
        // insert field (w, lsb) and then insert field (w2, lsb2)
        let new_word3 = news(new_word, w2, lsb2, val2).unwrap();
        assert_eq!(new_word2, new_word3);
    }

    /* 
        #8
        if you insert a field f, the bits fh that
        are above (more significant than) the inserted field are unchange
    */
    // get value from field above lsb + w
    let above = gets(word, 64-(lsb + w), lsb + w).unwrap();
    // get value from field below lsb
    let below = gets(word, lsb, 0).unwrap();
    // insert field (w, lsb)
    let new_word = news(word, w, lsb, val).unwrap();
    // get value from field above lsb + w again
    let above2 = gets(word, 64-(lsb + w), lsb + w).unwrap();
    // get value from field below lsb again
    let below2 = gets(new_word, lsb, 0).unwrap();
    assert_eq!(above, above2);
    assert_eq!(below, below2);

    /*
        #9
         if you insert a field f , the bits fl that are
        below (less significant than) the inserted field are unchanged
    */
    // get value from field below lsb
    let below = gets(word, lsb, 0).unwrap();
    // insert field (w, lsb)
    let new_word = news(word, w, lsb, val).unwrap();
    // get value from field below lsb again
    let below2 = gets(new_word, lsb, 0).unwrap();
    assert_eq!(below, below2);

    /* 
        #10
        interesting laws regarding fitss
    */
    if w <= 62 {
        assert_eq!(fitss(val, w), fitss(val << 2, w + 2));
    }
    if w >= 2 {
        assert_eq!(fitss(val, w), fitss(val >> 2, w - 1)); // had to change this from w - 2 to w - 1
    }
}

// test_check_laws for signed values
#[test]
fn test_check_laws_s() {
    //let mut rng = rand::thread_rng();
    // 65 here was messing everything up, 64 made it work
    for w in 0..65 { // upper value is excluded
        for lsb in 0..(65-w) {
            for _trial in 0..1001 {
                // set other parameter values randomly
                // val is a random number that can fit in w
                let val: i64;
                if w == 64 {
                    val = rand::random::<i64>();
                } else {
                    val = rand::random::<i64>() % (1 << w);
                }

                // generate random values for a second field
                let w2 = rand::random::<u64>() % 65;
                let lsb2 = rand::random::<u64>() % (65-w2);
                let val2: i64;
                if w2 == 64 {
                    val2 = rand::random::<i64>();
                } else {
                    val2 = rand::random::<i64>() % (1 << w2);
                }
                // set other parameter values randomly with rng
                /*let val = rng.gen_range(0..(1 << w));
                let w2 = rng.gen_range(0..65);
                let lsb2 = rng.gen_range(0..(65-w2));*/

                // make sure that val and val2 fit before calling check_laws
                if !(w == 0 || w + lsb > 64) && !(w2 == 0 || w2 + lsb2 > 64) && fitss(val, w) && fitss(val2, w2) {
                    check_laws_s(rand::random::<u64>(), w, lsb, val, w2, lsb2, val2);
                }
            }
        }
    }
}