use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut numbers: Vec<String> = buf.trim_end().split('+').map(|item| item.to_string()).collect();
    numbers.sort();

    println!("{}", numbers.join("+"));
}
