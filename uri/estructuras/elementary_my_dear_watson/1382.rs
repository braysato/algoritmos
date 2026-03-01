use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let arr: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut visitado = vec![false; n + 1];
        let mut ciclos = 0;
        
        for i in 1..=n {
            if !visitado[i] {
                ciclos += 1;
                let mut j = i;
                while !visitado[j] {
                    visitado[j] = true;
                    j = arr[j - 1];
                }
            }
        }
        
        writeln!(out, "{}", n - ciclos).unwrap();
    }
}
