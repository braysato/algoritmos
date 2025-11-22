use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    
    for i in 0..100 {
        let num: f64 = lines[i].trim().parse().unwrap();
        
        if num <= 10.0 {
            println!("A[{}] = {:.1}", i, num);
        }
    }
}
