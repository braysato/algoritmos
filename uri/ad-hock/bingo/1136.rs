use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    loop {
        let line = lines.next().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = parts[0];
        let b = parts[1];
        
        if n == 0 && b == 0 {
            break;
        }
        
        let balls: Vec<i32> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let mut possible = vec![false; (n + 1) as usize];
        
        for &ball_i in &balls {
            for &ball_j in &balls {
                let diff = (ball_i - ball_j).abs();
                if diff <= n {
                    possible[diff as usize] = true;
                }
            }
        }
        
        let can_generate_all = (0..=n).all(|i| possible[i as usize]);
        
        println!("{}", if can_generate_all { "Y" } else { "N" });
    }
}