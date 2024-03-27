// tests from the bitpack lab assignment
// TODO: something is wrong if check_laws is failing

use bitpack::bitpack::*;
//use rand::Rng;

/*
Write a Rust function called check_laws which takes as parameters word,
w, lsb, value, w2, and lsb2 as used in the laws of your handout. The
function should assert that the laws hold
*/
fn check_laws(word: u64, w: u64, lsb: u64, val: u64, w2: u64, lsb2: u64) {
    // print the values of the parameters to standard error
    println!("word: {}, w: {}, lsb: {}, val: {}, w2: {}, lsb2: {}", word, w, lsb, val, w2, lsb2);
    assert_eq!(getu(newu(word, w, lsb, val).unwrap(), w, lsb).unwrap(), val);
    assert_eq!(getu(newu(word, w, lsb, val).unwrap(), w2, lsb2).unwrap(), getu(word, w2, lsb2).unwrap());
}

// test check_laws
#[test]
fn test_check_laws() {
    //let mut rng = rand::thread_rng();
    for w in 0..65 { // upper value is excluded
        for lsb in 0..(65-w) {
            for _trial in 0..1001 {
                // set other parameter values randomly
                // val is a random number that can fit in w
                let val = rand::random::<u64>() % (1 << w);
                let w2 = rand::random::<u64>() % 65;
                let lsb2 = rand::random::<u64>() % (65-w2);
                // set other parameter values randomly with rng
                /*let val = rng.gen_range(0..(1 << w));
                let w2 = rng.gen_range(0..65);
                let lsb2 = rng.gen_range(0..(65-w2));*/
                check_laws(rand::random::<u64>(), w, lsb, val, w2, lsb2);
            }
        }
    }
}