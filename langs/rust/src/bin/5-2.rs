use rust::{check, run};

fn solution(input: &str) -> usize {
    input.lines().map(is_good).filter(|b| *b).count()
}

fn is_good(s: &str) -> bool {
    let has_pair = r".*(\w\w).*\1";
    let has_middle = r".*(\w).\1";
    check(has_pair, s) && check(has_middle, s)
}

fn main() {
    run(solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_1() {
        assert_eq!(is_good("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_good("xxyxx"), true);
        assert_eq!(is_good("uurcxstgmygtbstg"), false);
        assert_eq!(is_good("ieodomkazucvgmuy"), false);
    }
}
