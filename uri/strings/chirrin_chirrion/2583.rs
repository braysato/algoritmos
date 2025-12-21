use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut iter = input.split_whitespace();
    let cases: usize = match iter.next() {
        Some(value) => value.parse().unwrap_or(0),
        None => return,
    };
    let mut outputs = Vec::new();
    for _ in 0..cases {
        let n: usize = match iter.next() {
            Some(value) => value.parse().unwrap_or(0),
            None => 0,
        };
        let mut owned: HashSet<String> = HashSet::with_capacity(n);
        for _ in 0..n {
            let item = match iter.next() {
                Some(value) => value.to_string(),
                None => String::new(),
            };
            let cmd = match iter.next() {
                Some(value) => value,
                None => continue,
            };
            if cmd == "chirrin" {
                owned.insert(item);
            } else if cmd == "chirrion" {
                owned.remove(&item);
            }
        }
        let mut items: Vec<String> = owned.into_iter().collect();
        items.sort();
        outputs.push("TOTAL".to_string());
        outputs.extend(items);
    }
    println!("{}", outputs.join("\n"));
}
