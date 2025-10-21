use std::io;

fn divide_by_two(num: &str) -> (String, i32) {
    let mut result = String::new();
    let mut carry = 0;
    
    for ch in num.chars() {
        let digit = ch.to_digit(10).unwrap() as i32;
        let current = carry * 10 + digit;
        result.push(char::from_digit((current / 2) as u32, 10).unwrap());
        carry = current % 2;
    }
    
    let remainder = carry;
    
    let trimmed = result.trim_start_matches('0');
    let final_result = if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    };
    
    (final_result, remainder)
}

fn count_ones(num: &str) -> i32 {
    let mut count = 0;
    let mut current = num.to_string();
    
    while current != "0" {
        let (new_num, remainder) = divide_by_two(&current);
        if remainder == 1 {
            count += 1;
        }
        current = new_num;
    }
    
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num = input.trim();
        println!("{}", count_ones(num));
    }
}
