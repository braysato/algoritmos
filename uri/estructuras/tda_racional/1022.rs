use std::io::{self, BufRead, Write, BufWriter};

fn mcd(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if (b < 0) { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let n1: i64 = parts[0].parse().unwrap();
        let d1: i64 = parts[2].parse().unwrap();
        let op = parts[3].chars().next().unwrap();
        let n2: i64 = parts[4].parse().unwrap();
        let d2: i64 = parts[6].parse().unwrap();
        
        let (num, den) = match op {
            '+' => (n1 * d2 + n2 * d1, d1 * d2),
            '-' => (n1 * d2 - n2 * d1, d1 * d2),
            '*' => (n1 * n2, d1 * d2),
            '/' => (n1 * d2, n2 * d1),
            _ => (0, 1),
        };
        
        let g = mcd(num, den);
        let mut num_simp = num / g;
        let mut den_simp = den / g;
        
        if den_simp < 0 {
            num_simp = -num_simp;
            den_simp = -den_simp;
        }
        
        writeln!(out, "{}/{} = {}/{}", num, den, num_simp, den_simp).unwrap();
    }
}
