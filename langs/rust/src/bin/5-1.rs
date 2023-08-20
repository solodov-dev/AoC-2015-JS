use rust::{check, run};

fn solution(input: &str) -> usize {
    input.lines().map(is_good).filter(|b| *b).count()
}

fn is_good(s: &str) -> bool {
    let has_bads = r"(ab|cd|pq|xy)";
    let has_double = r"(.)\1";
    let three_vowels = r"^(.*[aeuio].*){3,}$";

    !check(has_bads, s) && check(has_double, s) && check(three_vowels, s)
}

fn main() {
    run(solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_1() {
        assert_eq!(is_good("ugknbfddgicrmopn"), true);
        assert_eq!(is_good("aaa"), true);
        assert_eq!(is_good("jchzalrnumimnmhp"), false);
        assert_eq!(is_good("haegwjzuvuyypxyu"), false);
        assert_eq!(is_good("dvszwmarrgswjxmb"), false);
    }
}
