use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut outputs = Vec::new();
    loop {
        let c_value = match iter.next() {
            Some(token) => token,
            None => break,
        };
        let c: usize = c_value.parse().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let mut slots = vec![0i32; c];
        let mut parked: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut revenue = 0;
        for _ in 0..n {
            let event = iter.next().unwrap();
            if event == "C" {
                let plate: i32 = iter.next().unwrap().parse().unwrap();
                let length: usize = iter.next().unwrap().parse().unwrap();
                let mut start = None;
                for pos in 0..=c.saturating_sub(length) {
                    let mut free = true;
                    for k in 0..length {
                        if slots[pos + k] != 0 {
                            free = false;
                            break;
                        }
                    }
                    if free {
                        start = Some(pos);
                        break;
                    }
                }
                if let Some(pos) = start {
                    for k in 0..length {
                        slots[pos + k] = plate;
                    }
                    parked.insert(plate, (pos, length));
                    revenue += 10;
                }
            } else {
                let plate: i32 = iter.next().unwrap().parse().unwrap();
                if let Some((pos, length)) = parked.remove(&plate) {
                    for k in 0..length {
                        slots[pos + k] = 0;
                    }
                }
            }
        }
        outputs.push(revenue.to_string());
    }
    println!("{}", outputs.join("\n"));
}
