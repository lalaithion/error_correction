extern crate error_correction;

#[cfg(test)]
mod tests {
    
    use error_correction::bitview::*;

    #[test]
    fn identity() {
        let data: [u8; 12] = [1,2,3,4,5,6,7,8,9,10,11,12];
        let processed = n_bits(&data, 4).fold(empty_part(data.len()), to_part_bytes);
        assert_eq!(&data[..], &to_bytes(processed)[..]);
    }

    #[test]
    fn identity_weird_stride() {
        let data: [u8; 1] = [1];
        let processed = n_bits(&data, 5).fold(empty_part(data.len()), to_part_bytes);
        assert_eq!(&data[..], &to_bytes(processed)[..]);
    }

    fn double(input: Vec<bool>) -> Vec<bool> {
        let mut out = Vec::with_capacity(input.len());
        for i in input {
            out.push(i);
            out.push(i);
        }
        return out;
    }

    #[test]
    fn doubled() {
        let data: [u8; 2] = [1, 2];
        let output: [u8; 4] = [0, 3, 0, 12];
        let processed = n_bits(&data, 8).map(double).fold(empty_part(2 * data.len()), to_part_bytes);
        assert_eq!(&output[..], &to_bytes(processed)[..]);
    }
    
    #[test]
    fn doubled_weird_stride() {
        let data: [u8; 2] = [1, 2];
        let output: [u8; 4] = [0, 3, 0, 12];
        let processed = n_bits(&data, 3).map(double).fold(empty_part(2 * data.len()), to_part_bytes);
        assert_eq!(&output[..], &to_bytes(processed)[..]);
    }
}
