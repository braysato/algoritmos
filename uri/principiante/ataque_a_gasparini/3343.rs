use std::io;
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0];
    let x = parts[1];
    
    let mut titans = String::new();
    io::stdin().read_line(&mut titans).unwrap();
    let titans = titans.trim();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sizes_vec: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let p = sizes_vec[0];
    let m = sizes_vec[1];
    let g = sizes_vec[2];
    
    let mut sizes = HashMap::new();
    sizes.insert('P', p);
    sizes.insert('p', p);
    sizes.insert('M', m);
    sizes.insert('m', m);
    sizes.insert('G', g);
    sizes.insert('g', g);
    
    let mut walls: Vec<i32> = Vec::new();
    
    for titan in titans.chars() {
        let titan_size = sizes[&titan];
        let mut stopped = false;
        
        for i in 0..walls.len() {
            if walls[i] == 0 {
                continue;
            }
            if walls[i] >= titan_size {
                walls[i] -= titan_size;
                stopped = true;
                break;
            }
        }
        
        if !stopped {
            walls.push(x - titan_size);
        }
    }
    
    println!("{}", walls.len());
}
