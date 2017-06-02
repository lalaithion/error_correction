//! # Hamming
//!
//! All modules in this project implement two public functions;
//! one that encodes data, and onetjat reverses that encoding.
//!
//! This module error corrects data by providing overlapping
//! parity bits to check for correctness
//!
//! https://en.wikipedia.org/wiki/Hamming_code

use bitview::*;
use ::binary_str;

fn parity_to_data(parity: usize) -> usize {
    (2u64.pow(parity as u32) - parity as u64 - 1) as usize
}

fn parity_to_length(parity: usize) -> usize {
    (2u64.pow(parity as u32) - 1) as usize
}

fn is_power_of_2(number: usize) -> bool {
    // bitwise hack for this thing, very fast. fails for number == 0.
    number & (number - 1) == 0
}

fn set_parity(mut encoded: Vec<bool>, parity: usize) -> Vec<bool> {
    for p in 0..parity {
        let index = ((2u64).pow(p as u32) - 1) as usize;
        let mut parity = false;
        for e in (index + 1)..encoded.len() {
            if (index + 1) & (e + 1) != 0 {
                parity ^= encoded[e];
            }
        }
        encoded[index] = parity;
    }
    return encoded;
}

fn encode_vec(buffer: Vec<bool>, parity: usize) -> Vec<bool> {
    assert!(buffer.len() == parity_to_data(parity));
    let mut encoded = Vec::with_capacity(parity_to_length(parity));
    
    println!("{}", binary_str(&buffer));
    
    let mut buffer_index: usize = 0;
    for index in 0..parity_to_length(parity) {
        // this algorithm is weird because it uses 1 based indexing! AAAH
        if is_power_of_2(index + 1) {
            encoded.push(false)
        } else {
            encoded.push(buffer[buffer_index]);
            buffer_index += 1;
        }
    }
    
    encoded = set_parity(encoded, parity);
    
    println!("{}", binary_str(&encoded));
    
    return encoded;
}

fn check_parity(encoded: &Vec<bool>, parity: usize) -> Vec<usize> {
    let mut errors = Vec::new();
    for p in 0..parity {
        let index = ((2u64).pow(p as u32) - 1) as usize;
        let mut parity = false;
        for e in (index + 1)..encoded.len() {
            if (index + 1) & (e + 1) != 0 {
                parity ^= encoded[e];
            }
        }
        if encoded[index] != parity {
            errors.push(index);
        }
    }
    return errors;
}

fn decode_vec(mut encoded: Vec<bool>, parity: usize) -> Vec<bool> {
    assert!(encoded.len() == parity_to_length(parity));
    let mut decoded = Vec::with_capacity(parity_to_data(parity));

    println!("{}", binary_str(&encoded));

    let errors = check_parity(&encoded, parity);

    if errors.len() > 1 {
        let error_index = errors.iter().fold(0,|a, &b| a + b + 1) - 1;
        
        encoded[error_index] = !encoded[error_index];
    }
    for index in 0..parity_to_length(parity) {
        if is_power_of_2(index + 1) {
            ;
        } else {
            decoded.push(encoded[index]);
        }
    }

    println!("{}", binary_str(&decoded));
    
    return decoded;
}

/// # encode
///
/// encode takes a buffer and a number, and adds parity bits for
/// error correction and detection
pub fn encode(buffer: &[u8], parity: usize) -> Vec<u8> {
    auto_pipeline(buffer, parity_to_data(parity), &(|x| encode_vec(x, parity)), DISCARD_NONE)
}

/// # decode
///
/// decode reverses encode, and takes a majority vote among number
/// bits to determine what the original bit value was. Decoding can never fail,
/// so there is only one decode function
pub fn decode(buffer: &[u8], parity: usize) -> Vec<u8> {
    auto_pipeline(buffer, parity_to_length(parity), &(|x| decode_vec(x, parity)), DISCARD_ALL)
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_parity_to_data() {
        assert_eq!(parity_to_data(2), 1);
        assert_eq!(parity_to_data(3), 4);
        assert_eq!(parity_to_data(4), 11);
        assert_eq!(parity_to_data(5), 26);
        assert_eq!(parity_to_data(6), 57);
        assert_eq!(parity_to_data(7), 120);
    }
    
    #[test]
    fn test_parity_to_length() {
        assert_eq!(parity_to_length(2), 3);
        assert_eq!(parity_to_length(3), 7);
        assert_eq!(parity_to_length(4), 15);
        assert_eq!(parity_to_length(5), 31);
        assert_eq!(parity_to_length(6), 63);
        assert_eq!(parity_to_length(7), 127);
    }
    
    #[test]
    fn test_is_power_of_2() {
        assert!(is_power_of_2(1));
        assert!(is_power_of_2(2));
        assert!(is_power_of_2(4));
        assert!(is_power_of_2(8));
        assert!(is_power_of_2(16));
        assert!(is_power_of_2(32));
        assert!(is_power_of_2(64));
        
        assert!(!is_power_of_2(11));
        assert!(!is_power_of_2(3));
        assert!(!is_power_of_2(5));
        assert!(!is_power_of_2(17));
        assert!(!is_power_of_2(9));
        assert!(!is_power_of_2(22));
        assert!(!is_power_of_2(24));
    }
}
