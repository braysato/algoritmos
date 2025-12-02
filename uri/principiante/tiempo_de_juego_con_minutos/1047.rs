use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut idx = 0;
    while idx + 3 < nums.len() {
        let start_hour = nums[idx];
        let start_minute = nums[idx + 1];
        let end_hour = nums[idx + 2];
        let end_minute = nums[idx + 3];
        let start_total = start_hour * 60 + start_minute;
        let mut end_total = end_hour * 60 + end_minute;
        if end_total <= start_total {
            end_total += 24 * 60;
        }
        let duration = end_total - start_total;
        let hours = duration / 60;
        let minutes = duration % 60;
        println!(
            "O JOGO DUROU {} HORA(S) E {} MINUTO(S)",
            hours, minutes
        );
        idx += 4;
    }
}
