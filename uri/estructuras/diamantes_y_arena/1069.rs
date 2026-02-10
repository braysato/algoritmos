use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let linea = lines.next().unwrap().unwrap();
        
        let mut pila = 0;
        let mut diamantes = 0;
        
        for c in linea.chars() {
            if c == '<' {
                pila += 1;
            } else if c == '>' {
                if pila > 0 {
                    pila -= 1;
                    diamantes += 1;
                }
            }
        }
        
        writeln!(out, "{}", diamantes).unwrap();
    }
}
