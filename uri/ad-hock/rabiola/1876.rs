use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        let vine = line.unwrap();
        let mut segments = Vec::new();
        let mut count = 0;
        
        for c in vine.chars() {
            if c == 'o' {
                count += 1;
            } else {
                if count > 0 {
                    segments.push(count);
                    count = 0;
                }
            }
        }
        
        if count > 0 {
            segments.push(count);
        }
        
        let mut max_size = 0;
        
        for (i, &segment) in segments.iter().enumerate() {
            let size = if i == 0 || i == segments.len() - 1 {
                segment
            } else {
                segment / 2
            };
            max_size = max_size.max(size);
        }
        
        println!("{}", max_size);
    }
}

