use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let mut letters = HashSet::new();
            
            for c in line.chars() {
                if c >= 'a' && c <= 'z' {
                    letters.insert(c);
                }
            }
            
            let count = letters.len();
            
            if count == 26 {
                println!("frase completa");
            } else if count >= 13 {
                println!("frase quase completa");
            } else {
                println!("frase mal elaborada");
            }
        }
    }
}
