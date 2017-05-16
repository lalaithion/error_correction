extern crate error_correction;

use error_correction::repeat::*;
use error_correction::add_errors;

#[test]
fn duplicate_round_trip() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = decode(&encoded, 3);
    assert_eq!(input, output);
}

#[test]
fn large_duplicate_round_trip() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 9);
    println!("{:?}", encoded);
    let output = decode(&encoded, 9);
    assert_eq!(input, output);
}

#[test]
fn duplicate_errors() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 5);
    println!("{:?}", encoded);
    let output = decode(&add_errors(&encoded), 5);
    assert_eq!(input, output);
}

#[test]
#[should_panic]
fn unrecoverable() {
    let encoded = vec![0,0,0,0,0,0,0,15];
    decode(&encoded, 8);
}
