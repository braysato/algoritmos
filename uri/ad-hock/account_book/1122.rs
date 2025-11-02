use std::io;
use std::collections::HashSet;

fn possible_sums(trans: &Vec<i32>, start: usize, end: usize) -> HashSet<i32> {
    let mut sums = HashSet::new();
    sums.insert(0);
    
    for i in start..=end {
        let mut new_sums = HashSet::new();
        for &sum in &sums {
            new_sums.insert(sum + trans[i]);
            new_sums.insert(sum - trans[i]);
        }
        sums = new_sums;
    }
    
    sums
}

fn main() {
    let mut input = String::new();
    
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = parts[0] as usize;
        let f = parts[1];
        
        if n == 0 && f == 0 {
            break;
        }
        
        let mut transactions = Vec::new();
        
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let t: i32 = input.trim().parse().unwrap();
            transactions.push(t);
        }
        
        let all_sums = possible_sums(&transactions, 0, n - 1);
        
        if !all_sums.contains(&f) {
            println!("*");
            continue;
        }
        
        let mut result = String::new();
        
        for i in 0..n {
            let before = if i > 0 {
                possible_sums(&transactions, 0, i - 1)
            } else {
                let mut set = HashSet::new();
                set.insert(0);
                set
            };
            
            let after = if i < n - 1 {
                possible_sums(&transactions, i + 1, n - 1)
            } else {
                let mut set = HashSet::new();
                set.insert(0);
                set
            };
            
            let mut can_be_positive = false;
            let mut can_be_negative = false;
            
            for &b in &before {
                for &a in &after {
                    if b + transactions[i] + a == f {
                        can_be_positive = true;
                    }
                    if b - transactions[i] + a == f {
                        can_be_negative = true;
                    }
                }
            }
            
            if can_be_positive && !can_be_negative {
                result.push('+');
            } else if !can_be_positive && can_be_negative {
                result.push('-');
            } else {
                result.push('?');
            }
        }
        
        println!("{}", result);
    }
}