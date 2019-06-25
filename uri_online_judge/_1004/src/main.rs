use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_f64(string: String) -> f64 {
    string.trim().parse().unwrap_or(0)
}

pub fn get_simple_product(a: f64, b: f64) -> f64 {
    a * b
}

fn main() {
    let a: f64 = parse_f64(get_input());
    let b: f64 = parse_f64(get_input());

    println!("MEDIA = {:.5}", get_simple_product(a, b));
}
