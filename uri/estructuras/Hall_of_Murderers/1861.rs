use std::collections::{BTreeMap, HashSet};
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut kills: BTreeMap<String, i32> = BTreeMap::new();
    let mut dead: HashSet<String> = HashSet::new();

    let mut it = input.split_whitespace();
    while let Some(killer) = it.next() {
        let victim = it.next().unwrap();
        *kills.entry(killer.to_string()).or_insert(0) += 1;
        dead.insert(victim.to_string());
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "HALL OF MURDERERS").unwrap();
    for (name, count) in &kills {
        if !dead.contains(name) {
            writeln!(out, "{} {}", name, count).unwrap();
        }
    }
}
