use std::io::{self, Read};

fn next_permutation(data: &mut [i32]) -> bool {
    if data.len() < 2 {
        return false;
    }
    let mut i = data.len() - 1;
    while i > 0 && data[i - 1] >= data[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = data.len() - 1;
    while data[j] <= data[i - 1] {
        j -= 1;
    }
    data.swap(i - 1, j);
    data[i..].reverse();
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    loop {
        let a = match iter.next() {
            Some(v) => v.parse::<i32>().unwrap(),
            None => break,
        };
        let b = iter.next().unwrap().parse::<i32>().unwrap();
        let c = iter.next().unwrap().parse::<i32>().unwrap();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        if a == 0 && b == 0 && c == 0 && x == 0 && y == 0 {
            break;
        }
        let mut princess = vec![a, b, c];
        princess.sort();
        let prince = vec![x, y];
        let mut used = [false; 53];
        for v in princess.iter() {
            used[*v as usize] = true;
        }
        for v in prince.iter() {
            used[*v as usize] = true;
        }
        let mut answer = -1;
        for candidate in 1..=52 {
            if used[candidate as usize] {
                continue;
            }
            let mut order = vec![prince[0], prince[1], candidate as i32];
            order.sort();
            let mut min_wins = 3;
            loop {
                let mut available = princess.clone();
                let mut prince_wins = 0;
                for &card in order.iter() {
                    let mut idx = None;
                    for (pos, &pv) in available.iter().enumerate() {
                        if pv > card {
                            idx = Some(pos);
                            break;
                        }
                    }
                    if let Some(pos) = idx {
                        available.remove(pos);
                    } else {
                        available.remove(0);
                        prince_wins += 1;
                    }
                }
                if prince_wins < min_wins {
                    min_wins = prince_wins;
                }
                if !next_permutation(&mut order) {
                    break;
                }
            }
            if min_wins >= 2 {
                answer = candidate as i32;
                break;
            }
        }
        println!("{}", answer);
    }
}
