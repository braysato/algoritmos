use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let n = nums[0];
    let b = nums[1];
    
    let mut layer = 0;
    let mut total = 0;
    
    loop {
        let side = n - 2 * layer;
        if side <= 0 {
            break;
        }
        
        let perimeter = if side == 1 {
            1
        } else {
            4 * side - 4
        };
        
        if total + perimeter >= b {
            break;
        }
        
        total += perimeter;
        layer += 1;
    }
    
    let offset = b - total - 1;
    let side = n - 2 * layer;
    let mut row = 1 + layer;
    let mut col = 1 + layer;
    
    if side == 1 {
        println!("{} {}", row, col);
        return;
    }
    
    if offset < side - 1 {
        row = 1 + layer;
        col = 1 + layer + offset;
    } else if offset < 2 * (side - 1) {
        row = 1 + layer + (offset - (side - 1));
        col = n - layer;
    } else if offset < 3 * (side - 1) {
        row = n - layer;
        col = n - layer - (offset - 2 * (side - 1));
    } else {
        row = n - layer - (offset - 3 * (side - 1));
        col = 1 + layer;
    }
    
    println!("{} {}", row, col);
}

