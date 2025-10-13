use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut frecuencia = [0; 2001];
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        frecuencia[x] += 1;
    }
    
    for i in 1..=2000 {
        if frecuencia[i] > 0 {
            println!("{} aparece {} vez(es)", i, frecuencia[i]);
        }
    }
}
