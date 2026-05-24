use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut num_q = 0;
    let mut num_k = 0;
    for line in lines {
        let value = line.unwrap();

        if num_q == 0 {
            num_q = value.parse::<u32>().unwrap();
            continue;
        }

        let value = value.split_whitespace().take(3);

        let mut num_w = 0;
        for v in value {
            if v.parse::<u32>().unwrap() == 1 {
                num_w += 1;
            }
        }
        if num_w >= 2 {
            num_k += 1;
        }

        num_q -= 1;
        if num_q == 0 {
            println!("{num_k}");
            num_k = 0;
        }
    }
}
