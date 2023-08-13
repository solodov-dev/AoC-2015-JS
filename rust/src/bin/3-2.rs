struct Point {
    x: i32,
    y: i32,
    map: HashSet<String>,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            map: HashSet::new(),
        }
    }

    fn step(&mut self, dir: char) {
        match dir {
            '>' => self.y += 1,
            'v' => self.x -= 1,
            '<' => self.y -= 1,
            '^' => self.x += 1,
            _ => panic!("Unknown direction: {}", dir),
        };
    }

    fn present(&mut self) {
        let key = format!("{}_{}", self.x, self.y);
        self.map.insert(key);
    }
}

fn parse(input: &str) -> usize {
    let mut santa = Point::new();
    let mut robot = Point::new();
    santa.present();
    robot.present();
    for pair in input.chars().collect::<Vec<char>>().chunks(2) {
        santa.step(pair[0]);
        robot.step(pair[1]);
        santa.present();
        robot.present();
    }
    santa.map.union(&robot.map).count()
}

use core::panic;
use std::collections::HashSet;

use rust::run;
fn main() {
    run(parse);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_1() {
        assert_eq!(parse("^v"), 3);
        assert_eq!(parse("^>v<"), 3);
        assert_eq!(parse("^v^v^v^v^v"), 11);
    }
}
