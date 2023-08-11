fn part_01(input: &str) -> String {
    input
        .split("")
        .fold(0, |acc, s| match s {
            ")" => acc - 1,
            "(" => acc + 1,
            _ => acc,
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(part_01("(())"), part_01("()()"));
        assert_eq!(part_01("((("), part_01("(()(()("));
        assert_eq!(part_01("))((((("), "3");
        assert_eq!(part_01("())"), part_01("))("));
        assert_eq!(part_01(")))"), part_01(")())())"));
    }
}

use rust::run;
fn main() {
    run(part_01);
}
