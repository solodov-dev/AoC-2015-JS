use fancy_regex::Regex;
use std::{
    fmt::Display,
    io::{self, BufRead},
};

pub fn run<T: Display>(f: fn(&str) -> T) {
    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    println!("{}", f(input.trim()));
}

pub fn check(regex: &str, str: &str) -> bool {
    Regex::new(regex).unwrap().is_match(str).unwrap()
}
