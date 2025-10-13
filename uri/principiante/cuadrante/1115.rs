use std::io;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let values: Vec<&str> = input.trim().split_whitespace().collect();
                if values.len() != 2 {
                    break;
                }
                
                let x: i32 = match values[0].parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                let y: i32 = match values[1].parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                
                if x == 0 || y == 0 {
                    break;
                }
                
                if x > 0 && y > 0 {
                    println!("primeiro");
                } else if x < 0 && y > 0 {
                    println!("segundo");
                } else if x < 0 && y < 0 {
                    println!("terceiro");
                } else if x > 0 && y < 0 {
                    println!("quarto");
                }
            }
            Err(_) => break,
        }
    }
}