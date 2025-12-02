use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut outputs = Vec::new();

    loop {
        let s = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => break,
            None => break,
        };
        let part = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => break,
            None => {
                outputs.push(String::from("null value"));
                break;
            }
        };
        if part.is_empty() {
            outputs.push(String::from("null value"));
            continue;
        }
        let mut result = String::with_capacity(s.len());
        for ch in s.chars() {
            result.push(ch);
            if result.len() >= part.len() {
                let start = result.len() - part.len();
                if &result[start..] == part {
                    result.truncate(start);
                }
            }
        }
        if result.is_empty() {
            outputs.push(String::from("null value"));
        } else {
            outputs.push(result);
        }
    }

    for line in outputs {
        println!("{}", line);
    }
}
