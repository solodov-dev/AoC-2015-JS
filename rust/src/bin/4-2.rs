use md5;

fn parse(input: &str) -> i32 {
    let mut c = -1;

    loop {
        c += 1;
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, c)));
        if hash.starts_with("000000") {
            break;
        }
    }

    c
}

use rust::run;
fn main() {
    run(parse);
}
