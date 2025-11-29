use std::cmp::min;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();
    let mut output = String::new();

    loop {
        let mut n_line_opt = lines.next();
        while let Some(line) = n_line_opt {
            if line.trim().is_empty() {
                n_line_opt = lines.next();
            } else {
                break;
            }
        }
        let n_line = match n_line_opt {
            Some(line) => line.trim(),
            None => break,
        };
        if n_line.is_empty() {
            continue;
        }
        let n: usize = n_line.parse().unwrap();
        if n == 0 {
            break;
        }
        let mut text = String::new();
        for _ in 0..n {
            if let Some(line) = lines.next() {
                text.push_str(line);
            }
        }
        let bytes = text.as_bytes();
        if bytes.len() < 2 {
            output.push('\n');
            continue;
        }
        let mut counts: HashMap<[u8; 2], usize> = HashMap::new();
        for window in bytes.windows(2) {
            let key = [window[0], window[1]];
            *counts.entry(key).or_insert(0) += 1;
        }
        let total = bytes.len() - 1;
        let mut items: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(k, v)| (String::from_utf8(vec![k[0], k[1]]).unwrap(), v))
            .collect();
        items.sort_by(|a, b| {
            if b.1 != a.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        let limit = min(5, items.len());
        for i in 0..limit {
            let relative = items[i].1 as f64 / total as f64;
            output.push_str(&format!("{} {} {:.6}\n", items[i].0, items[i].1, relative));
        }
        output.push('\n');
    }

    print!("{}", output);
}
