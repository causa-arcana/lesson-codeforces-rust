use std::io::{self, BufRead};

fn main() {
    let matrix: Vec<Option<i32>> =
        io::BufReader::new(io::stdin())
        .lines()
        .map(|item| {
            item.unwrap().split_whitespace().position(|item| item == "1")
                .and_then(|item| Some(item as i32))
        })
        .collect();

    let vertical_ops = (2 - (matrix.iter().position(|item| *item != None).unwrap() as i32)).abs();
    let horizontal_ops = (2 - matrix.iter().find(|item| **item != None).unwrap().unwrap()).abs();

    println!("{}", vertical_ops + horizontal_ops);
}
