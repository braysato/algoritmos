use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let l: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let vagones: Vec<i32> = if l > 0 {
            lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        } else {
            vec![]
        };
        
        let mut swaps = 0;
        for i in 0..l {
            for j in (i + 1)..l {
                if vagones[i] > vagones[j] {
                    swaps += 1;
                }
            }
        }
        
        writeln!(out, "Optimal train swapping takes {} swaps.", swaps).unwrap();
    }
}
