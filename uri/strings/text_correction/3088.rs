use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut output = String::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut result = String::with_capacity(line.len());
        for ch in line.chars() {
            if (ch == ',' || ch == '.') && result.ends_with(' ') {
                result.pop();
            }
            result.push(ch);
        }
        output.push_str(&result);
        output.push('\n');
    }
    print!("{}", output);
}
