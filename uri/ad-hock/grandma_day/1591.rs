use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(l);
        for _ in 0..l {
            let row: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
            grid.push(row);
        }
        
        let p: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        for _ in 0..p {
            let word: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
            let len = word.len();
            let mut count = 0;
            
            if len == 1 {
                for i in 0..l {
                    for j in 0..c {
                        if grid[i][j] == word[0] {
                            count += 1;
                        }
                    }
                }
            } else {
                for i in 0..l {
                    for j in 0..=c.saturating_sub(len) {
                        if j + len <= c {
                            let mut matched = true;
                            for k in 0..len {
                                if grid[i][j + k] != word[k] {
                                    matched = false;
                                    break;
                                }
                            }
                            if matched { count += 1; }
                        }
                    }
                }
                
                for j in 0..c {
                    for i in 0..=l.saturating_sub(len) {
                        if i + len <= l {
                            let mut matched = true;
                            for k in 0..len {
                                if grid[i + k][j] != word[k] {
                                    matched = false;
                                    break;
                                }
                            }
                            if matched { count += 1; }
                        }
                    }
                }
            }
            
            writeln!(out, "{}", count).unwrap();
        }
    }
}
