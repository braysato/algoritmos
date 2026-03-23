use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut tokens = input.split_whitespace();
    
    let nc: usize = tokens.next().unwrap().parse().unwrap();
    
    for _ in 0..nc {
        let n: usize = tokens.next().unwrap().parse().unwrap();
        
        let mut cnt = [0u32; 231];
        
        for _ in 0..n {
            let h: usize = tokens.next().unwrap().parse().unwrap();
            cnt[h] += 1;
        }
        
        let mut first = true;
        for h in 20..=230 {
            for _ in 0..cnt[h] {
                if !first {
                    write!(out, " ").unwrap();
                }
                write!(out, "{}", h).unwrap();
                first = false;
            }
        }
        writeln!(out).unwrap();
    }
}
