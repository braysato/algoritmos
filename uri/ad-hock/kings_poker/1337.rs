use std::io::{self, BufRead};
use std::cmp::{min, max};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    while let Some(Ok(line)) = lines.next() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        
        if a == 0 && b == 0 && c == 0 {
            break;
        }
        
        let mut cards = vec![a, b, c];
        cards.sort();
        
        if cards[0] == cards[1] && cards[1] == cards[2] {
            let rank = cards[0];
            if rank == 13 {
                println!("*");
            } else {
                println!("{} {} {}", rank + 1, rank + 1, rank + 1);
            }
        } else if cards[0] == cards[1] || cards[1] == cards[2] {
            let (pair_rank, single) = if cards[0] == cards[1] {
                (cards[0], cards[2])
            } else {
                (cards[1], cards[0])
            };
            
            let mut next_card = single + 1;
            if next_card == pair_rank {
                next_card += 1;
            }
            
            if next_card <= 13 {
                println!("{} {} {}", min(next_card, pair_rank), pair_rank, max(next_card, pair_rank));
            } else if pair_rank < 13 {
                println!("1 {} {}", pair_rank + 1, pair_rank + 1);
            } else {
                println!("1 1 1");
            }
        } else {
            println!("1 1 2");
        }
    }
}
