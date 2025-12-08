use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut first = true;
    for line in input.lines() {
        if !first {
            // Preserve blank lines between outputs when input has blank lines
            println!();
        }
        first = false;
        if line.is_empty() {
            print!("");
            continue;
        }
        let word = line.trim();
        let bytes = word.as_bytes();
        let n = bytes.len();
        let mut corrected = word;
        for len in 1..=n / 2 {
            if &bytes[n - 2 * len..n - len] == &bytes[n - len..n] {
                corrected = &word[..n - len];
                break;
            }
        }
        print!("{}", corrected);
    }
    if !first {
        println!();
    }
}
