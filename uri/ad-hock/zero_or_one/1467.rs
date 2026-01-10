use std::io;

fn main() {
    let mut input = String::new();
    
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let (a, b, c) = (values[0], values[1], values[2]);
        
        if a != b && a != c {
            println!("A");
        } else if b != a && b != c {
            println!("B");
        } else if c != a && c != b {
            println!("C");
        } else {
            println!("*");
        }
        
        input.clear();
    }
}
