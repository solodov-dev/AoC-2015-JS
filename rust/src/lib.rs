use std::{
    fmt::Display,
    io::{self, BufRead},
};

pub fn run<T: Display>(f: fn(&str) -> T) {
    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
    println!("{}", f(&input));
}
