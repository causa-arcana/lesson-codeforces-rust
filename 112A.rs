use std::cmp::Ordering;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut stdin  = io::stdin();
    let mut stdout = io::stdout();

    let lines: Vec<String> =
        BufReader::new(stdin)
        .lines()
        .map(|line| line.unwrap().to_lowercase())
        .collect();

    let line1 = lines[0].as_str();
    let line2 = lines[1].as_str();

    let result = match line1.cmp(line2) {
        Ordering::Less    => -1,
        Ordering::Equal   => 0,
        Ordering::Greater => 1,
    };

    stdout.write_all(format!("{}\n", result).as_ref()).unwrap();
}
