//! This module contains the base types and functions that are used throughout the
//! rest of this library. It deals with common issues, such as how to stream data
//! through a Rust binary when the output may not be a even multiple of bytes for each
//! input byte, or vice versa.

use std::convert::{From, Into};

/// This struct is a packed representation of a Vec<Bool>, semantically. It allows
/// storing and retrieving n booleans in ~n/8 bytes, even when n is not a multiple
/// of 8.
#[derive(PartialEq, Eq, Debug)]
pub struct Bits {
    /// this field holds the data in a big-endian format; that is, if we have 9
    /// bits set, the data would be stored as 11111111 10000000. The unused bits
    /// at the end should always be 0.
    data: Vec<u8>,
    /// stores the number of bits in the last byte that are part of the data. In the
    /// above example, this value would be 1. This value should never be more than 8.
    /// A value of 0 is disallowed. A Bits struct which has an unused byte at the end
    /// should remove that byte, and set last_len to 8. An empty Bits struct should
    /// have an empty data Vec, and last_len should be 8.
    ///
    /// Examples:
    ///
    ///  1xxxxxxx  last_len = 1
    ///  xxxxxxxx  DISALLOWED
    ///  11101xxx  last_len = 5
    ///  10101010  last_len = 8
    ///
    last_len: u8,
}

/// The Bits struct is meant to be a buffer that is slowly built up and rarely
/// destructured. Therefore, I've only implemented methods that allow it to be slowly
/// built up in memory, and then turned into Vec<u8> to be written out. Some methods I
/// have not implemented include:
///
///  - concatenating together two Bits'
///  - indexing into the Bits
///  - removing elements from the Bits
///
/// These methods may prove useful, but they should not be part of the standard model
/// for interacting with a Bits value, so take note befor implementing them.
/// Concatenating Bits values might prove very useful when doing parallelization, but
/// it requires doing a lot more work than concatenating two
/// byte-aligned structures, as it cannot be implemented by a memcopy.
impl Bits {
    pub fn new() -> Bits {
        Bits {
            data: Vec::new(),
            last_len: 8,
        }
    }

    pub fn push(&mut self, value: bool) {
        if self.last_len == 8 {
            self.data.push(if value { 0b10000000 } else { 0b00000000 });
            self.last_len = 1;
        } else {
            let mask = if value { 1 << (7 - self.last_len) } else { 0 };
            let last_index = self.data.len() - 1;
            self.data[last_index] |= mask;
            self.last_len += 1;
        }
    }

    pub fn push_block(&mut self, value: u8) {
        if self.last_len == 8 {
            self.data.push(value);
        } else {
            let first_half = value >> self.last_len;
            let second_half = value << (8 - self.last_len);
            let last_index = self.data.len() - 1;
            self.data[last_index] |= first_half;
            self.data.push(second_half)
        }
    }

    /// split consumes the Bits value, returning the slice
    /// of bytes that ends at the last byte boundary as a
    /// Vector of u8s, and any remaining data in a new Bits
    /// value.
    pub fn split(mut self) -> (Vec<u8>, Bits) {
        if self.last_len == 8 {
            (self.data, Bits::new())
        } else {
            let last = self.data.pop();
            debug_assert!(
                last.is_some(),
                "self.data must hold at least one value when last_len is not 8"
            );
            (
                self.data,
                Bits {
                    data: vec![last.unwrap()],
                    last_len: self.last_len,
                },
            )
        }
    }

    /// Finalize consumes the Bits value and returns the
    /// data in it, padded with zeros until the next byte
    /// boundary.
    pub fn finalize(self) -> Vec<u8> {
        self.data
    }
}

/// Convert a vector of booleans to a packed representation.
/// This and the Into impl block are probably mostly going to
/// be used for debugging/testing.
impl From<Vec<bool>> for Bits {
    fn from(vec: Vec<bool>) -> Bits {
        let mut x = Bits::new();
        for b in vec {
            x.push(b);
        }
        return x;
    }
}

// Convert the packed representation to a vector of booleans.
impl Into<Vec<bool>> for Bits {
    fn into(self) -> Vec<bool> {
        let mut out = Vec::with_capacity(self.data.len() * 8);
        for (block_index, block) in self.data.iter().enumerate() {
            let final_index = if block_index == self.data.len() - 1 {
                self.last_len - 1
            } else {
                7
            };

            for index in 0..=final_index {
                // if the block has the indexth bit set, push true, otherwise, false
                let is_set = block & (1 << (7 - index)) != 0;
                out.push(is_set);
            }
        }

        return out;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn to_from(input: Vec<bool>) {
        let tmp = input.clone();
        let output: Vec<bool> = Bits::into(Bits::from(tmp));
        assert_eq!(output, input);
    }

    #[test]
    fn to_from_ex() {
        let input = vec![true];
        let tmp = input.clone();
        let output: Vec<bool> = Bits::into(Bits::from(tmp));
        assert_eq!(output, input);
    }

    #[test]
    fn from() {
        let val = vec![false, false, true, false, true, true, true, true];
        assert_eq!(Bits::from(val).finalize(), vec![47]);
    }

    #[test]
    fn into() {
        let mut x = Bits::new();
        x.push_block(47);
        let actual: Vec<bool> = Bits::into(x);
        let expected = vec![false, false, true, false, true, true, true, true];
        assert_eq!(actual, expected);
    }

    #[test]
    fn pushing() {
        let mut x = Bits::new();
        // bit 0
        x.push(false);
        // bit 1
        x.push(true);
        // bit 2
        x.push(false);
        // bits 3-7 of byte 1, bits 0-2 of byte 2
        x.push_block(0b00001111);
        // bit 3
        x.push(false);
        // bit 4
        x.push(false);
        // bit 5
        x.push(false);
        // bit 6
        x.push(true);
        // bit 7
        x.push(true);
        // bits 0-7 of byte 3
        x.push_block(0b11111111);

        assert_eq!(x.finalize(), vec![0b01000001, 0b11100011, 0b11111111])
    }
}
