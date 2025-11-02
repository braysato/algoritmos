use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let y: i32 = input.trim().parse().unwrap();
    
    let min_val = x.min(y);
    let max_val = x.max(y);
    
    let mut sum = 0;
    
    for i in min_val..=max_val {
        if i % 13 != 0 {
            sum += i;
        }
    }
    
    println!("{}", sum);
}
