use std::io::{self, BufRead};

pub fn run(f: fn(&str) -> String) {
    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
    println!("{}", f(&input));
}
