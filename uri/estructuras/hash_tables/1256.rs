use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut primero = true;
    
    for _ in 0..n {
        let line1 = lines.next().unwrap().unwrap();
        let mut iter = line1.split_whitespace();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let _c: usize = iter.next().unwrap().parse().unwrap();
        
        let line2 = lines.next().unwrap().unwrap();
        let claves: Vec<usize> = line2.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut tabla: Vec<Vec<usize>> = vec![vec![]; m];
        
        for clave in claves {
            let indice = clave % m;
            tabla[indice].push(clave);
        }
        
        if !primero {
            writeln!(out).unwrap();
        }
        primero = false;
        
        for i in 0..m {
            write!(out, "{} -> ", i).unwrap();
            for val in &tabla[i] {
                write!(out, "{} -> ", val).unwrap();
            }
            writeln!(out, "\\").unwrap();
        }
    }
}
