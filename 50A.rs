use std::io::{self, Read, Write};

fn main() {
    let mut stdin  = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut inputs = input.split_whitespace();
    let input1 = inputs.next().unwrap();
    let input2 = inputs.next().unwrap();

    let width = input1.parse::<i32>().unwrap();
    let height: i32 = input2.parse().unwrap();

    let result =
        if width % 2 == 0 || height % 2 == 0 {
            width * height / 2
        }
        else {
            (width * height - 1) / 2
        };

    stdout.write_all(format!("{}\n", result).as_ref()).unwrap();
}
