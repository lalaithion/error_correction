use base::Bits;

pub fn encode(input: Vec<u8>) -> Vec<u8> {
    let mut output = Bits::new();

    for byte in input {
        for index in 0..=7 {
            let mask = 1 << (7 - index);
            output.push(byte & mask != 0);
        }
    }

    return output.finalize();
}

pub fn decode(input: Vec<u8>) -> Vec<u8> {
    unimplemented!()
}
