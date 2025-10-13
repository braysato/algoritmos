use std::io;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let m: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                
                if m < 90 || m == 360 {
                    println!("Bom Dia!!");
                } else if m >= 90 && m < 180 {
                    println!("Boa Tarde!!");
                } else if m >= 180 && m < 270 {
                    println!("Boa Noite!!");
                } else if m >= 270 && m < 360 {
                    println!("De Madrugada!!");
                }
            }
            Err(_) => break,
        }
    }
}