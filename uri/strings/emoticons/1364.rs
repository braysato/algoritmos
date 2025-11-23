use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    loop {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (n, m) = (parts[0], parts[1]);
        
        if n == 0 && m == 0 {
            break;
        }
        
        let mut emoticons = Vec::new();
        for _ in 0..n {
            emoticons.push(lines.next().unwrap().unwrap());
        }
        
        let mut total_changes = 0;
        
        for _ in 0..m {
            let text = lines.next().unwrap().unwrap();
            
            let mut occurrences = Vec::new();
            
            for emoticon in &emoticons {
                let mut pos = 0;
                while let Some(idx) = text[pos..].find(emoticon) {
                    let start = pos + idx;
                    let end = start + emoticon.len() - 1;
                    occurrences.push((start, end));
                    pos = start + 1;
                }
            }
            
            if occurrences.is_empty() {
                continue;
            }
            
            occurrences.sort_by_key(|&(_, end)| end);
            
            let mut last_hit = -1i32;
            for (start, end) in occurrences {
                if last_hit < start as i32 {
                    last_hit = end as i32;
                    total_changes += 1;
                }
            }
        }
        
        println!("{}", total_changes);
    }
}
