use md5;

fn parse(input: &str) -> i32 {
    let mut c = -1;

    loop {
        c += 1;
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, c)));
        if hash.starts_with("00000") {
            break;
        }
    }

    c
}

use rust::run;
fn main() {
    run(parse);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4_1() {
        assert_eq!(parse("abcdef"), 609043);
        assert_eq!(parse("pqrstuv"), 1048970);
    }
}
