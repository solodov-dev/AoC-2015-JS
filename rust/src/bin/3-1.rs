struct Point {
    x: i32,
    y: i32,
    map: HashMap<String, bool>,
}

impl Point {
    fn new() -> Point {
        let mut point = Point {
            x: 0,
            y: 0,
            map: HashMap::new(),
        };
        point.present();
        point
    }

    fn step(&mut self, dir: char) {
        match dir {
            '>' => self.y += 1,
            'v' => self.x -= 1,
            '<' => self.y -= 1,
            '^' => self.x += 1,
            '\n' => (),
            _ => panic!("Unknown direction: {}", dir),
        };
        self.present();
    }

    fn present(&mut self) {
        let key = format!("{}_{}", self.x, self.y);
        self.map.insert(key, true);
    }

    fn count(&self) -> u32 {
        self.map
            .values()
            .fold(0, |acc, el| if *el { acc + 1 } else { acc })
    }
}

fn parse(input: &str) -> u32 {
    let mut point = Point::new();
    for dir in input.chars() {
        point.step(dir);
    }
    point.count()
}

use core::panic;
use std::collections::HashMap;

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
