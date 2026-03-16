use std::io::{self, Read, Write, BufWriter};

struct UnionFind {
    parent: Vec<usize>,
    points: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize, initial_points: Vec<i64>) -> Self {
        let parent = (0..=n).collect();
        let mut points = vec![0; n + 1];
        for i in 1..=n {
            points[i] = initial_points[i - 1];
        }
        UnionFind { parent, points }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn unite(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.parent[pb] = pa;
            self.points[pa] += self.points[pb];
        }
    }
    
    fn get_points(&mut self, x: usize) -> i64 {
        let p = self.find(x);
        self.points[p]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut tokens = input.split_whitespace();
    
    loop {
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let m: usize = tokens.next().unwrap().parse().unwrap();
        
        if n == 0 && m == 0 {
            break;
        }
        
        let mut initial_points = Vec::with_capacity(n);
        for _ in 0..n {
            let p: i64 = tokens.next().unwrap().parse().unwrap();
            initial_points.push(p);
        }
        
        let mut uf = UnionFind::new(n, initial_points);
        let mut wins = 0;
        
        for _ in 0..m {
            let q: i32 = tokens.next().unwrap().parse().unwrap();
            let a: usize = tokens.next().unwrap().parse().unwrap();
            let b: usize = tokens.next().unwrap().parse().unwrap();
            
            if q == 1 {
                uf.unite(a, b);
            } else {
                let pa = uf.find(a);
                let pb = uf.find(b);
                let rafael = uf.find(1);
                
                if pa == pb {
                    continue; 
                }
                
                let points_a = uf.points[pa];
                let points_b = uf.points[pb];
                
                if rafael == pa {
                    if points_a > points_b {
                        wins += 1;
                    }
                } else if rafael == pb {
                    if points_b > points_a {
                        wins += 1;
                    }
                }
            }
        }
        
        writeln!(out, "{}", wins).unwrap();
    }
}
