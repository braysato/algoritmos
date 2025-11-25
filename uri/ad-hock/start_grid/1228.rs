use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        
        let start_line = lines.next().unwrap().unwrap();
        let start: Vec<i32> = start_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let finish_line = lines.next().unwrap().unwrap();
        let finish: Vec<i32> = finish_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let mut start_position = HashMap::new();
        for (i, &competitor) in start.iter().enumerate() {
            start_position.insert(competitor, i);
        }
        
        let mut overtakes = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if start_position[&finish[i]] > start_position[&finish[j]] {
                    overtakes += 1;
                }
            }
        }
        
        println!("{}", overtakes);
    }
}
