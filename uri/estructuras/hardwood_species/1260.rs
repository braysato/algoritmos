use std::io::{self, BufRead, Write, BufWriter};
use std::collections::BTreeMap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    lines.next(); // línea en blanco
    
    let mut primero = true;
    
    for _ in 0..n {
        let mut especies: BTreeMap<String, i32> = BTreeMap::new();
        let mut total = 0;
        
        loop {
            match lines.next() {
                Some(Ok(linea)) if !linea.is_empty() => {
                    *especies.entry(linea).or_insert(0) += 1;
                    total += 1;
                }
                _ => break,
            }
        }
        
        if !primero {
            writeln!(out).unwrap();
        }
        primero = false;
        
        for (especie, count) in &especies {
            let porcentaje = (*count as f64 * 100.0) / total as f64;
            writeln!(out, "{} {:.4}", especie, porcentaje).unwrap();
        }
    }
}
