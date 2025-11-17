use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let l: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let mut hash = 0;
        
        for i in 0..l {
            if let Some(Ok(line)) = lines.next() {
                for (j, ch) in line.chars().enumerate() {
                    let value = (ch as i32 - 'A' as i32) + i as i32 + j as i32;
                    hash += value;
                }
            }
        }
        
        println!("{}", hash);
    }
}
