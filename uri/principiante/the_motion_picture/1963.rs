use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut values = input.split_whitespace();
    if let (Some(a_str), Some(b_str)) = (values.next(), values.next()) {
        let a: f64 = a_str.parse().unwrap();
        let b: f64 = b_str.parse().unwrap();
        let pct = (b - a) / a * 100.0;
        println!("{:.2}%", pct);
    }
}
