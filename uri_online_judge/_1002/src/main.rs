use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_f64(string: String) -> f64 {
    string.trim().parse().unwrap_or(0.0)
}

pub fn get_circumference(a: f64) -> f64 {
    3.14159 * a * a
}

fn main() {
    let a: f64 = parse_f64(get_input());

    println!("A = {:.4}", get_circumference(a));
}
