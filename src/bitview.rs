/// # BitView
///
/// Holds a reference to a data buffer of `u8`s, as well as a position
/// and change in position (a stride) to that data buffer. Can be
/// iterated over, providing a `Vec<bool>` starting at the position
/// and the stride long.
pub struct BitView<'a> {
    stride: usize,
    index: usize,
    offset: usize,
    data: &'a [u8]
}

impl<'a> Iterator for BitView<'a> {
    type Item = Vec<bool>;
    
    fn next(&mut self) -> Option<Vec<bool>> {
        if self.index >= self.data.len() {
            None
        } else {
            let mut buffer = Vec::with_capacity(self.stride);
            
            for i in 0..self.stride {
                let index = self.index + (self.offset + i) / 8;
                let offset = (self.offset + i) % 8;
                if index >= self.data.len() {
                    buffer.push(false);
                } else {
                    let bitflag = (2 as u8).pow(7 - offset as u32);
                    buffer.push((self.data[index] & bitflag) != 0);
                }
            }
            
            self.index += (self.offset + self.stride) / 8;
            self.offset = (self.offset + self.stride) % 8;
            Some(buffer)
        }
    }
}

/// # n_bits
///
/// Creates a BitView from a `&[u8]`, with a given stride
pub fn n_bits<'a>(buffer: &'a [u8], n: usize) -> BitView<'a> {
    BitView{stride: n, index: 0, offset: 0, data: buffer}
}

/// # PartBytes
///
/// Holds a partially full vector of bytes that may contain
/// a fractional number of bytes
pub struct PartBytes {
    data: Vec<u8>,
    offset: u8
}

/// # empty_part
///
/// Creates an empty PartBytes
pub fn empty_part(capacity: usize) -> PartBytes {
    PartBytes { data: Vec::with_capacity(capacity), offset: 0}
}

/// # to_part_bytes
///
/// Can be used as an accumulator function with .fold() to
/// collect an iterator over a BitView back into a [u8]
pub fn to_part_bytes(mut acc: PartBytes, element: Vec<bool>) -> PartBytes {
    let mut max = acc.data.len() as isize - 1;
    for boolean in element {
        if acc.offset == 0 {
            acc.data.push(0);
            max += 1;
        }
        assert!(max >= 0);
        if boolean {
            let bitflag = (2 as u8).pow(7 - acc.offset as u32);
            acc.data[max as usize] |= bitflag;
        }
        acc.offset = (acc.offset + 1) % 8;
    }
    return acc
}

/// # result_to_part_bytes
///
/// Can be used as an accumulator function with .fold() to
/// collect an iterator of type Result<Vec<bool>, &'static str>
/// into a [u8]
pub fn result_to_part_bytes(accumulator: Result<PartBytes,&'static str>,
    element: Result<Vec<bool>, &'static str> )
    ->  Result<PartBytes, &'static str>
{
    if let Ok(acc) = accumulator {
        if let Ok(elem) = element {
            Ok(to_part_bytes(acc, elem))
        } else {
            Err(element.unwrap_err())
        }
    } else {
        accumulator
    }
}

/// # to_bytes
///
/// Extracts the underlying `[u8]` object from a PartBytes object intelligently
/// without extra zeroes on the end.
pub fn to_bytes(pb: PartBytes) -> Vec<u8> {
    if pb.offset == 0 {
        pb.data
    } else {
        let mut buf = pb.data;
        let end = buf.len()-1;
        buf.remove(end);
        buf
    }
}

/// # auto_pipleine
///
/// Performs the operations this file is intended to facilitate with one function call.
/// However, performance may be improved by performing the operations manually.
pub fn auto_pipeline(input: &[u8], stride: usize, function: &Fn(Vec<bool>) -> Vec<bool>) -> Vec<u8> {
    to_bytes(
        n_bits(&input, stride)
            .map(function)
            .fold(empty_part(input.len()), to_part_bytes)
    )
}

/// # result_auto_pipleine
///
/// Performs the operations this file is intended to facilitate with one function call.
/// However, performance may be improved by performing the operations manually.
/// Useful in cases where errors can occur.
pub fn result_auto_pipeline(input: &[u8], stride: usize,
    function: &Fn(Vec<bool>) -> Result<Vec<bool>, &'static str>)
    -> Result<Vec<u8>, &'static str>
{
        n_bits(&input, stride)
            .map(function)
            .fold(Ok(empty_part(input.len())), result_to_part_bytes)
            .map(to_bytes)
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn identity() {
        let data: [u8; 12] = [1,2,3,4,5,6,7,8,9,10,11,12];
        let processed = auto_pipeline(&data, 4, &(|x| x));
        assert_eq!(&data[..], &processed[..]);
    }

    #[test]
    fn identity_weird_stride() {
        let data: [u8; 12] = [1,3,5,7,9,11,13,15,17,19,21,23];
        let processed = auto_pipeline(&data, 3, &(|x| x));
        assert_eq!(&data[..], &processed[..]);
    }

    fn double(input: Vec<bool>) -> Vec<bool> {
        let mut out = Vec::with_capacity(2 * input.len());
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
        let processed = auto_pipeline(&data, 8, &double);
        assert_eq!(&output[..], &processed[..]);
    }
    
    #[test]
    fn doubled_weird_stride() {
        let data: [u8; 2] = [1, 2];
        let output: [u8; 4] = [0, 3, 0, 12];
        let processed = auto_pipeline(&data, 3, &double);
        assert_eq!(&output[..], &processed[..]);
    }
    
    fn result_double(input: Vec<bool>) -> Result<Vec<bool>, &'static str> {
        let mut out = Vec::with_capacity(2 * input.len());
        for i in input {
            out.push(i);
            out.push(i);
        }
        return Ok(out);
    }
    
    #[test]
    fn result_doubled() {
        let data: [u8; 2] = [1, 2];
        let output: [u8; 4] = [0, 3, 0, 12];
        let processed = result_auto_pipeline(&data, 3, &result_double).unwrap();
        assert_eq!(&output[..], &processed[..]);
    }
    
    fn result_fails(_input: Vec<bool>) -> Result<Vec<bool>, &'static str> {
        Err("This should fail")
    }
    
    #[test]
    fn result_failed() {
        let data: [u8; 2] = [1, 2];
        let processed = result_auto_pipeline(&data, 3, &result_fails);
        assert!(processed.is_err());
    }
}
