extern crate rand;

mod bitview;

pub mod repeat;
pub mod hamming;

pub fn add_errors(input: &[u8]) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut index = 0;
    
    let mut output = Vec::with_capacity(input.len());
    for &i in input {
        index += 1;
        let rnd = rng.gen::<u32>();
        let flag = rng.gen::<u32>();
        if rnd % 3 != 0 && index % 2 == 0{
            // flip a random bit 2/3 of the time, on alternating
            // bytes.
            output.push(i ^ ((2 as u8).pow(flag % 8)))
        }
        else {
            output.push(i);
        }
    }
    return output;
}

fn binary_str(input: &[bool]) -> String {
    let mut binary = String::from("");
    let mut counter = 0;
    for i in input {
        if *i {
            binary.push('1');
        } else {
            binary.push('0');
        }
        counter += 1;
        if counter == 8 {
            counter = 0;
            binary.push(' ');
        }
    }
    return binary;
}
