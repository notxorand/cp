use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let line = line.unwrap();
        if line.parse::<u32>().is_ok() || (line.is_empty() && line.len() > 100) {
            continue;
        } else if line.len() <= 10 {
            println!("{line}");
        } else {
            let mut chars = line.chars();
            let first = chars.next().unwrap();
            let last = chars.next_back().unwrap();
            println!("{first}{}{last}", (line.len() - 2));
        }
    }
}
