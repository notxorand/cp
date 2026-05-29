use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let line = line.unwrap();
        if line.split_whitespace().count() != 4 {
            continue;
        } else {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let mut first = numbers[0];
            let mut second = numbers[1];
            let mut third = numbers[2];
            let mut fourth = numbers[3];
            if first > 12 || second > 12 || third > 12 || fourth > 12 {
                continue;
            }
            if first == third || second == fourth || first == fourth || second == third {
                println!("NO");
            }
            if first > second {
                std::mem::swap(&mut first, &mut second);
            }
            if third > fourth {
                std::mem::swap(&mut third, &mut fourth);
            }
            if (first < third && third < second && second < fourth)
                || (third < first && first < fourth && fourth < second)
            {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
