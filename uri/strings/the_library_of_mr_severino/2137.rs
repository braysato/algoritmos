use std::io;

fn main() {
    let mut input = String::new();
    
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() || input.trim().is_empty() {
            break;
        }
        
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        
        let mut codes = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            codes.push(input.trim().to_string());
        }
        
        codes.sort();
        
        for code in codes {
            println!("{}", code);
        }
    }
}
