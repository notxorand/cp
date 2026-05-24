use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let value: f32 = line.unwrap().parse().unwrap();
        if (1.0..=100.0).contains(&value) && (value - 2.0) % 2.0 == 0.0 && value != 2.0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
