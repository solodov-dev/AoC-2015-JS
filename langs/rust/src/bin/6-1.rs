use rust::run;

fn solution(input: &str) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    let commands = input.lines().map(split_commands);
    for command in commands {
        match &command[..] {
            [cmd, x1, y1, x2, y2] => {
                let from_x = x1.parse::<usize>().unwrap();
                let to_x = x2.parse::<usize>().unwrap();
                let from_y = y1.parse::<usize>().unwrap();
                let to_y = y2.parse::<usize>().unwrap();
                for i in from_x..to_x + 1 {
                    for j in from_y..to_y + 1 {
                        match cmd.as_str() {
                            "on" => grid[i][j] = true,
                            "off" => grid[i][j] = false,
                            "toggle" => grid[i][j] = !grid[i][j],
                            _ => panic!("unknown command"),
                        }
                    }
                }
            }
            _ => panic!("unknown command"),
        }
    }
    grid.iter()
        .flatten()
        .filter(|v| **v == true)
        .collect::<Vec<&bool>>()
        .len()
}

fn split_commands(s: &str) -> Vec<String> {
    s.replace("turn ", "")
        .replace(",", " ")
        .replace("through ", "")
        .split_whitespace()
        .take(5)
        .map(str::to_string)
        .collect()
}

fn main() {
    run(solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6_1() {
        assert_eq!(solution("turn on 0,0 through 999,999"), 1000 * 1000);
        assert_eq!(solution("toggle 0,0 through 999,0"), 1000);
        assert_eq!(
            solution("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
            1000 * 1000 - 4
        );
    }
}