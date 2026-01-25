use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let primera = lines.next().unwrap().unwrap().trim().to_string();
        let mut misma_lengua = true;
        
        for _ in 1..k {
            let actual = lines.next().unwrap().unwrap().trim().to_string();
            if actual != primera {
                misma_lengua = false;
            }
        }
        
        if misma_lengua {
            writeln!(out, "{}", primera).unwrap();
        } else {
            writeln!(out, "ingles").unwrap();
        }
    }
}
