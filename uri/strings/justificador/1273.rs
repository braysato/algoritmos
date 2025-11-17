use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut first = true;
    
    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        
        if n == 0 {
            break;
        }
        
        if !first {
            println!();
        }
        first = false;
        
        let mut words = Vec::new();
        let mut max_length = 0;
        
        for _ in 0..n {
            if let Some(Ok(word)) = lines.next() {
                let word = word.trim().to_string();
                max_length = max_length.max(word.len());
                words.push(word);
            }
        }
        
        for word in &words {
            let spaces = max_length - word.len();
            for _ in 0..spaces {
                print!(" ");
            }
            println!("{}", word);
        }
    }
}
