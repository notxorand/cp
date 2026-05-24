use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let mut _input = line.unwrap().parse::<u32>().unwrap();
        let mut steps = 0;

        for number in [5, 4, 3, 2, 1] {
            while _input >= number {
                _input -= number;
                steps += 1;
            }
        }
        println!("{steps}");
    }
}
