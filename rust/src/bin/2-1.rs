fn solution(input: &str) -> u32 {
    input.lines().map(parse).sum()
}

fn parse(input: &str) -> u32 {
    let dimensions: Vec<u32> = input.split("x").map(|n| str::parse(n).unwrap()).collect();
    if let [l, w, h] = dimensions[..3] {
        let planes = [l * w, w * h, h * l].into_iter();
        2 * planes.clone().sum::<u32>() + planes.min().unwrap()
    } else {
        panic!("No dimensions");
    }
}

use rust::run;
fn main() {
    run(solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_1() {
        assert_eq!(parse("2x3x4"), 58);
    }
}
