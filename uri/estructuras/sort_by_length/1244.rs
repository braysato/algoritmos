use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let linea = lines.next().unwrap().unwrap();
        let mut palabras: Vec<(usize, &str)> = linea
            .split_whitespace()
            .enumerate()
            .collect();
        
        palabras.sort_by(|a, b| {
            let cmp = b.1.len().cmp(&a.1.len());
            if cmp == std::cmp::Ordering::Equal {
                a.0.cmp(&b.0)
            } else {
                cmp
            }
        });
        
        let resultado: Vec<&str> = palabras.iter().map(|(_, s)| *s).collect();
        writeln!(out, "{}", resultado.join(" ")).unwrap();
    }
}
