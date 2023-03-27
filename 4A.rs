use std::io::{self, Read, Write};

fn main() {
    let mut stdin  = io::stdin();
    let mut stdout = io::stdout();

    let mut weight_string = String::new();
    stdin.read_to_string(&mut weight_string).unwrap();

    let weight = weight_string.trim_end().parse::<i32>().unwrap();

    if weight % 2 == 0 && weight != 2 {
        stdout.write_all(b"YES\n").unwrap();
    }
    else {
        stdout.write_all(b"NO\n").unwrap();
    }
}
