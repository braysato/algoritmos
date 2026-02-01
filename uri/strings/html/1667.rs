use std::io::{self, Read, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    
    let mut pos = 0;
    
    for token in input.split_whitespace() {
        if token == "<br>" {
            if pos > 0 {
                writeln!(out).unwrap();
                pos = 0;
            }
        } else if token == "<hr>" {
            if pos > 0 {
                writeln!(out).unwrap();
                pos = 0;
            }
            writeln!(out, "{}", "-".repeat(80)).unwrap();
        } else {
            let len = token.len();
            if pos == 0 {
                write!(out, "{}", token).unwrap();
                pos = len;
            } else if pos + 1 + len <= 80 {
                write!(out, " {}", token).unwrap();
                pos += 1 + len;
            } else {
                writeln!(out).unwrap();
                write!(out, "{}", token).unwrap();
                pos = len;
            }
        }
    }
    
    if pos > 0 {
        writeln!(out).unwrap();
    }
}
