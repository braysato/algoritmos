use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let x: i32 = values[0].parse().expect("Invalid integer");
    let y: f32 = values[1].parse().expect("Invalid float");
    
    let consumo = x as f32 / y;
    
    println!("{:.3} km/l", consumo);
}