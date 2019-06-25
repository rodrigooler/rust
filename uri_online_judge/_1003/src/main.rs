use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_i32(string: String) -> i32 {
    string.trim().parse().unwrap_or(0)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a: i32 = parse_i32(get_input());
    let b: i32 = parse_i32(get_input());

    println!("X = {}", add(a, b));
}
