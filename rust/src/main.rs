use std::{env, fs};

mod day_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let filepath = String::from("../input/day_") + day;
    let input = fs::read_to_string(&filepath).expect(&format!("Cannot open file {}", filepath));

    match day.as_str() {
        "1" => match part.as_str() {
            "1" => println!("{}", day_01::part_01(&input)),
            "2" => println!("{}", day_01::part_02(&input)),
            _ => panic!("No solutions for part {}", part),
        },
        _ => panic!("No solutions for day {}", day),
    };
}
