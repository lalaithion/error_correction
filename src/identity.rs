pub fn encode(input: Vec<u8>) -> Vec<u8> {
    input
}

pub fn decode(input: Vec<u8>) -> Vec<u8> {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn roundtrip(input: Vec<u8>) {
        let tmp = input.clone();
        let output = decode(encode(tmp));
        assert_eq!(output, input);
    }
}
