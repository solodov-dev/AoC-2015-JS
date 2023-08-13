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

    fn count(&self) -> usize {
        self.map.len()
    }
}

fn parse(input: &str) -> usize {
    let mut point = Point::new();
    point.present();
    for dir in input.chars() {
        point.step(dir);
        point.present();
    }
    point.count()
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
        assert_eq!(parse(">"), 2);
        assert_eq!(parse("^>v<"), 4);
        assert_eq!(parse("^v^v^v^v^v"), 2);
    }
}
