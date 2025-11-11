use std::io::{self, BufRead};

fn is_valid(grid: &[[i32; 9]; 9]) -> bool {
    for i in 0..9 {
        let mut used = [false; 10];
        for j in 0..9 {
            let num = grid[i][j] as usize;
            if num < 1 || num > 9 || used[num] {
                return false;
            }
            used[num] = true;
        }
    }
    
    for j in 0..9 {
        let mut used = [false; 10];
        for i in 0..9 {
            let num = grid[i][j] as usize;
            if num < 1 || num > 9 || used[num] {
                return false;
            }
            used[num] = true;
        }
    }
    
    for block in 0..9 {
        let mut used = [false; 10];
        let start_row = (block / 3) * 3;
        let start_col = (block % 3) * 3;
        
        for i in 0..3 {
            for j in 0..3 {
                let num = grid[start_row + i][start_col + j] as usize;
                if num < 1 || num > 9 || used[num] {
                    return false;
                }
                used[num] = true;
            }
        }
    }
    
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for instance in 1..=n {
        let mut grid = [[0; 9]; 9];
        
        for i in 0..9 {
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            
            for j in 0..9 {
                grid[i][j] = nums[j];
            }
        }
        
        println!("Instancia {}", instance);
        if is_valid(&grid) {
            println!("SIM");
        } else {
            println!("NAO");
        }
        println!();
    }
}
