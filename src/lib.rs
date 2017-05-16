extern crate rand;

mod bitview;

pub mod duplicate;

pub fn add_errors(input: &[u8]) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let mut output = Vec::with_capacity(input.len());
    for &i in input {
        let rnd = rng.gen::<u32>();
        let flag = rng.gen::<u32>();
        if rnd % 3 != 0 {
            // flip a random bit 2/3 of the time.
            output.push(i ^ ((2 as u8).pow(flag % 8)))
        }
        else {
            output.push(i);
        }
    }
    return output;
}
