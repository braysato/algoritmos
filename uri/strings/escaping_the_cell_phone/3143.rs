use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();
    
    let n: usize = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    
    let mut total_lines = 0;
    
    for line in lines_iter {
        let message: String = line.unwrap().trim_end().to_string();
        
        if message.is_empty() {
            continue;
        }
        
        let chars: Vec<char> = message.chars().collect();
        let mut lines = 1;
        let mut char_count = 0;
        let mut i = 0;
        let len = chars.len();
        
        while i < len {
            char_count += 1;
            i += 1;
            
            if char_count == n && i < len {
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                if i < len {
                    lines += 1;
                    char_count = 0;
                }
            }
        }
        
        total_lines += lines;
    }
    
    println!("{}", total_lines);
}
