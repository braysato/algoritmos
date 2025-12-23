use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: i32 = match iter.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    for _ in 0..t {
        let m1: i32 = iter.next().unwrap().parse().unwrap();
        iter.next();
        let v1: i32 = iter.next().unwrap().parse().unwrap();
        let m2: i32 = iter.next().unwrap().parse().unwrap();
        iter.next();
        let v2: i32 = iter.next().unwrap().parse().unwrap();
        let mut points1 = 0;
        let mut points2 = 0;
        if m1 > v1 {
            points1 += 3;
        } else if m1 < v1 {
            points2 += 3;
        } else {
            points1 += 1;
            points2 += 1;
        }
        if m2 > v2 {
            points2 += 3;
        } else if m2 < v2 {
            points1 += 3;
        } else {
            points1 += 1;
            points2 += 1;
        }
        let goals1 = m1 + v2;
        let goals2 = v1 + m2;
        let diff = goals1 - goals2;
        let away1 = v2;
        let away2 = v1;
        if points1 > points2 {
            println!("Time 1");
        } else if points2 > points1 {
            println!("Time 2");
        } else if diff > 0 {
            println!("Time 1");
        } else if diff < 0 {
            println!("Time 2");
        } else if away1 > away2 {
            println!("Time 1");
        } else if away2 > away1 {
            println!("Time 2");
        } else {
            println!("Penaltis");
        }
    }
}
