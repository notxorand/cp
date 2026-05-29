use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let value = line.unwrap();
        let mut hello = "";

        for char in value.chars() {
            if char.to_lowercase().to_string() == "h" && hello.is_empty() {
                hello = "h";
            } else if char.to_lowercase().to_string() == "e" && hello == "h" {
                hello = "he";
            } else if char.to_lowercase().to_string() == "l" && hello == "he" {
                hello = "hel";
            } else if char.to_lowercase().to_string() == "l" && hello == "hel" {
                hello = "hell";
            } else if char.to_lowercase().to_string() == "o" && hello == "hell" {
                hello = "hello";
            }
        }

        if hello == "hello" {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
