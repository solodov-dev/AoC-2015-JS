pub fn part_01(input: &str) -> i32 {
    input.split("").fold(0, |acc, s| match s {
        ")" => acc - 1,
        "(" => acc + 1,
        _ => acc,
    })
}

pub fn part_02(input: &str) -> usize {
    let mut sum = 0;
    let mut i = 0;

    while sum >= 0 {
        if input.chars().nth(i) == Some(')') {
            sum -= 1;
        } else {
            sum += 1;
        };
        i += 1;
    }

    i
}

#[allow(dead_code)]
pub fn part_02_iterators(input: &str) -> usize {
    let mut acc = 0;
    let pos = input
        .chars()
        .position(|s| {
            if s == '(' {
                acc += 1;
            } else {
                acc -= 1;
            };
            acc < 0
        })
        .unwrap();

    pos + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(part_01("(())"), part_01("()()"));
        assert_eq!(part_01("((("), part_01("(()(()("));
        assert_eq!(part_01("))((((("), 3);
        assert_eq!(part_01("())"), part_01("))("));
        assert_eq!(part_01(")))"), part_01(")())())"));
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part_02(")"), 1);
        assert_eq!(part_02("()())"), 5);
    }

    #[test]
    fn test_day1_part2_iterators() {
        assert_eq!(part_02_iterators(")"), 1);
        assert_eq!(part_02_iterators("()())"), 5);
    }
}
