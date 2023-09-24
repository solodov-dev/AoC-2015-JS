use inquire::{self, Select};
use std::{
    collections::HashMap,
    env::set_current_dir,
    fs::{self, metadata},
    process::{Command, Stdio},
    str::from_utf8,
    time::Instant,
};

fn main() {
    let langs = HashMap::from([
        ("haskell", "cabal"),
        ("rust", "cargo"),
        ("javascript", "node"),
        ("awk", "awk"),
    ]);

    println!("Hello and welcome to Advent of Code 2015");

    let root;
    if metadata("./input").is_ok() {
        root = ".";
    } else if metadata("../input").is_ok() {
        root = ".."
    } else {
        panic!("Unknown environment. Cannot find input files");
    }

    let days = fs::read_dir(format!("{}/input", root))
        .expect("Error reading dir 'input'")
        .count();

    let day = Select::new("What day would you like to solve?", (1..days + 1).collect())
        .prompt()
        .unwrap();

    let part = Select::new("Which part?", vec![1, 2]).prompt().unwrap();

    let lang = Select::new("With what language?", langs.keys().collect())
        .prompt()
        .unwrap_or_else(|_| panic!("Error selecting a language. Abort."));

    let cat = Command::new("cat")
        .arg(format!("{}/input/day_{}", root, day))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    set_current_dir(format!("{}/langs/{}", root, lang))
        .unwrap_or_else(|_| panic!("Cannot find directory for lang {}", lang));

    print!("{} solution for day {} part {} is ", lang, day, part,);

    let mut command = Command::new(langs.get(lang).unwrap());

    let filename = format!("{}-{}", day, part);

    match *lang {
        "haskell" => command.arg("run").arg("--verbose=0").arg(filename),
        "rust" => command.arg("run").arg("--quiet").arg("--bin").arg(filename),
        "awk" => command.arg("-i").arg("utils.awk").arg("-f").arg(filename),
        "javascript" => command.arg(filename),
        _ => panic!("Unknown lang"),
    };

    let now = Instant::now();

    let last = command
        .stdin(Stdio::from(cat.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = last.wait_with_output().unwrap();
    let result = from_utf8(&output.stdout).unwrap().trim();
    println!("{} ({:?})", result, now.elapsed());
}
