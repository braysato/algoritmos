use std::io::{self, Read, Write, BufWriter};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut tokens = input.split_whitespace();
    
    while let Some(q_str) = tokens.next() {
        let q: usize = q_str.parse().unwrap();
        
        let mut grid = vec![vec![0i64; 1001]; 1001];
        let mut px: Vec<i32> = Vec::with_capacity(q);
        let mut py: Vec<i32> = Vec::with_capacity(q);
        let mut gcd_global: i64 = 0;
        let mut gcd_dirty = true;
        
        for _ in 0..q {
            let tipo = tokens.next().unwrap();
            let x: i32 = tokens.next().unwrap().parse().unwrap();
            let y: i32 = tokens.next().unwrap().parse().unwrap();
            let d: i64 = tokens.next().unwrap().parse().unwrap();
            
            let gx = (x + 500) as usize;
            let gy = (y + 500) as usize;
            
            if tipo.starts_with('S') {
                if grid[gx][gy] == 0 {
                    px.push(x);
                    py.push(y);
                }
                grid[gx][gy] = d;
                gcd_dirty = true;
            } else {
                let mut result: i64 = 0;
                let n = px.len();
                
                if d >= 2000 {
                    if gcd_dirty {
                        gcd_global = 0;
                        for i in 0..n {
                            if gcd_global == 1 { break; }
                            gcd_global = gcd(gcd_global, grid[(px[i] + 500) as usize][(py[i] + 500) as usize]);
                        }
                        gcd_dirty = false;
                    }
                    result = gcd_global;
                } else {
                    for i in 0..n {
                        if result == 1 { break; }
                        let dist = (px[i] - x).abs() + (py[i] - y).abs();
                        if dist as i64 <= d {
                            result = gcd(result, grid[(px[i] + 500) as usize][(py[i] + 500) as usize]);
                        }
                    }
                }
                writeln!(out, "{}", result).unwrap();
            }
        }
    }
}
