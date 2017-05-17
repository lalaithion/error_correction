extern crate error_correction;

use error_correction::repeat::*;
use error_correction::add_errors;

#[test]
fn duplicate_round_trip() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = panic_decode(&encoded, 3);
    assert_eq!(input, output);
}

#[test]
fn large_duplicate_round_trip() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 9);
    println!("{:?}", encoded);
    let output = panic_decode(&encoded, 9);
    assert_eq!(input, output);
}

#[test]
fn duplicate_errors() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 5);
    println!("{:?}", encoded);
    let output = panic_decode(&add_errors(&encoded), 5);
    assert_eq!(input, output);
}

#[test]
#[should_panic]
fn unrecoverable() {
    let encoded = vec![0,0,0,0,0,0,0,15];
    panic_decode(&encoded, 8);
}

#[test]
fn results() {
    let input = vec![1,2,34,56];
    let encoded = encode(&input, 3);
    println!("{:?}", encoded);
    let output = decode(&encoded, 3);
    assert!(output.is_ok());
}


#[test]
fn unrecoverable_result() {
    let encoded = vec![0,0,0,0,0,0,0,15];
    assert!(decode(&encoded, 8).is_err());
}

#[test]
fn wrong_result() {
    let encoded = vec![0,0,0,0,0,0,0,15];
    assert_eq!(wrong_decode(&encoded, 8), vec![0]);
}
