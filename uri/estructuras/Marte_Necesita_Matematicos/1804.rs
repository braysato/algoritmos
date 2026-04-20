use std::io::{self, Read, Write, BufWriter};

struct BIT {
    tree: Vec<i32>,
    n: usize,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT { tree: vec![0; n + 1], n }
    }

    fn update(&mut self, mut i: usize, val: i32) {
        while i <= self.n {
            self.tree[i] += val;
            i += i & i.wrapping_neg();
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut p = vec![0i32; n + 1];
    let mut bit = BIT::new(n);

    for i in 1..=n {
        p[i] = iter.next().unwrap().parse().unwrap();
        bit.update(i, p[i]);
    }

    while let Some(op) = iter.next() {
        let idx: usize = iter.next().unwrap().parse().unwrap();
        if op == "a" {
            bit.update(idx, -p[idx]);
            p[idx] = 0;
        } else {
            writeln!(out, "{}", bit.query(idx - 1)).unwrap();
        }
    }
}
