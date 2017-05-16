pub struct BitView<'a> {
    stride: usize,
    index: usize,
    offset: usize,
    data: &'a [u8]
}

pub fn n_bits<'a>(buffer: &'a [u8], n: usize) -> BitView<'a> {
    BitView{stride: n, index: 0, offset: 0, data: buffer}
}

impl<'a> Iterator for BitView<'a> {
    type Item = Vec<bool>;
    
    fn next(&mut self) -> Option<Vec<bool>> {
        if self.index >= self.data.len() {
            None
        } else {
            let mut buffer = Vec::with_capacity(self.stride);
            for i in 0..self.stride {
                let index = self.index + i / 8;
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

pub struct PartBytes {
    data: Vec<u8>,
    offset: u8
}

pub fn empty_part(capacity: usize) -> PartBytes {
    PartBytes { data: Vec::with_capacity(capacity), offset: 0}
}

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
