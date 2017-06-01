extern crate error_correction;

use error_correction::hamming::*;
use error_correction::add_errors;

#[test]
fn hamming_single() {
    println!("");
    let input = vec![1];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = decode(&encoded, 3);
    assert_eq!(input, output);
}

#[test]
fn hamming_round_trip() {
    println!("");
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = decode(&encoded, 3);
    assert_eq!(input, output);
}

#[test]
fn large_hamming_round_trip() {
    println!("");
    let input = vec![1, 2, 34, 54];
    let encoded = encode(&input, 4);
    println!("{:?}", encoded);
    let output = decode(&encoded, 4);
    assert_eq!(input, output);
}

#[test]
fn hamming_errors() {
    println!("");
    let input = vec![127, 0, 80, 12];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = decode(&add_errors(&encoded), 3);
    assert_eq!(input, output);
}
