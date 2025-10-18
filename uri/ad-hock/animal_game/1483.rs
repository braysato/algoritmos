use std::io;

fn get_group(last_two_digits: i32) -> i32 {
    if last_two_digits == 0 {
        return 24;
    }
    (last_two_digits - 1) / 4
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 3 {
            break;
        }
        
        let v: f64 = parts[0].parse().unwrap();
        let n: i32 = parts[1].parse().unwrap();
        let m: i32 = parts[2].parse().unwrap();
        
        if v == 0.0 && n == 0 && m == 0 {
            break;
        }
        
        let mut prize = 0.0;
        
        let last4_n = n % 10000;
        let last4_m = m % 10000;
        let last3_n = n % 1000;
        let last3_m = m % 1000;
        let last2_n = n % 100;
        let last2_m = m % 100;
        
        if last4_n == last4_m {
            prize = v * 3000.0;
        } else if last3_n == last3_m {
            prize = v * 500.0;
        } else if last2_n == last2_m {
            prize = v * 50.0;
        } else if get_group(last2_n) == get_group(last2_m) {
            prize = v * 16.0;
        }
        
        println!("{:.2}", prize);
    }
}
