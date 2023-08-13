use std::{
    fmt::Display,
    io::{self, BufRead},
};

pub fn run<T: Display>(f: fn(&str) -> T) {
    let mut input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
    // Remove last new line
    input.pop();
    println!("{}", f(&input));
}
