use std::io;

fn main() {
    let mut instance = 1;
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let m: i32 = input.trim().parse().unwrap();
        
        if m == 0 {
            break;
        }
        
        let mut alphabet: Vec<char> = ('A'..='Z').collect();
        let mut result = String::new();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let positions: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        
        for pos in positions {
            let idx = pos - 1;
            let letter = alphabet[idx];
            result.push(letter);
            
            alphabet.remove(idx);
            alphabet.insert(0, letter);
        }
        
        println!("Instancia {}", instance);
        println!("{}", result);
        println!();
        
        instance += 1;
    }
}

