use std::io;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let values: Vec<&str> = input.trim().split_whitespace().collect();
                if values.len() != 2 {
                    break;
                }
                
                let x: u64 = match values[0].parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                let m: u64 = match values[1].parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                
                if x == 0 && m == 0 {
                    break;
                }
                
                let resultado = x * m;
                println!("{}", resultado);
            }
            Err(_) => break,
        }
    }
}