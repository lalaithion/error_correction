//! # Repeat
//!
//! All modules in this project implement two public functions;
//! one that encodes data, and onetjat reverses that encoding.
//!
//! This module error corrects data by providing some number of
//! redundant bits for each bit in the plaintext.
//!
//! https://en.wikipedia.org/wiki/Repetition_code


use bitview::*;

fn encode_vec(input: Vec<bool>, number: usize) -> Vec<bool> {
    assert!(input.len() == 1);
    let mut output = Vec::with_capacity(number);
    for _ in 0..number {
        output.push(input[0]);
    }
    println!("{:?}", output);
    return output
}


/// # encode
///
/// encode takes a buffer and a number, and repeats each bit
/// in the buffer number of times.
pub fn encode(buffer: &[u8], number: usize) -> Vec<u8> {
    auto_pipeline(buffer, 1, &(|x| encode_vec(x, number)))
}

fn decode_vec(input: Vec<bool>) -> Vec<bool> {
    println!("{:?}", input);
    let number = input.len();
    let mut hamming_w = 0;
    for b in input {
        if b { hamming_w += 1; }
    }
    if hamming_w * 2 > number {
        vec![true]
    } else if hamming_w * 2 < number {
        vec![false]
    } else { // hamming_w == number
        panic!("Unrecoverable corruption has occured in this data.")
    }
}

/// # decode
///
/// decode reverses encode, and takes a majority vote among number
/// bits to determine what the original bit value was.
pub fn decode(buffer: &[u8], number: usize) -> Vec<u8> {
    auto_pipeline(buffer, number, &decode_vec)
}
