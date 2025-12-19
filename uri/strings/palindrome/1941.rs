use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    special: i32,
    length: i32,
}

fn better(a: &Node, b: &Node) -> bool {
    if a.special != b.special {
        a.special > b.special
    } else if a.length != b.length {
        a.length > b.length
    } else {
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let s_line = match lines.next() {
        Some(line) => line.trim().to_string(),
        None => return,
    };
    let n = s_line.len();
    if n == 0 {
        println!("0");
        return;
    }

    let info_line = lines.next().unwrap_or("").trim().to_string();
    let numbers: Vec<usize> = if info_line.is_empty() {
        Vec::new()
    } else {
        info_line
            .split_whitespace()
            .map(|token| token.parse::<usize>().unwrap())
            .collect()
    };

    let mut special = vec![false; n];
    if !numbers.is_empty() {
        let count = numbers[0];
        for pos in numbers.iter().skip(1).take(count) {
            if *pos >= 1 && *pos <= n {
                special[pos - 1] = true;
            }
        }
    }

    let bytes = s_line.as_bytes();
    let mut dp = vec![vec![Node { special: 0, length: 0 }; n]; n];

    for i in 0..n {
        dp[i][i] = Node {
            special: if special[i] { 1 } else { 0 },
            length: 1,
        };
    }

    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let mut best = dp[i + 1][j];
            if better(&dp[i][j - 1], &best) {
                best = dp[i][j - 1];
            }
            if bytes[i] == bytes[j] {
                let extra = (if special[i] { 1 } else { 0 }) + (if special[j] { 1 } else { 0 });
                let candidate = if len == 2 {
                    Node {
                        special: extra,
                        length: 2,
                    }
                } else {
                    Node {
                        special: dp[i + 1][j - 1].special + extra,
                        length: dp[i + 1][j - 1].length + 2,
                    }
                };
                if better(&candidate, &best) {
                    best = candidate;
                }
            }
            dp[i][j] = best;
        }
    }

    println!("{}", dp[0][n - 1].length);
}
