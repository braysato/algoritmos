use std::io;

fn main() {
    let mut input = String::new();
    
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        
        if n == 0 {
            break;
        }
        
        let mut strs = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            strs.push(input.trim().to_string());
        }
        
        strs.sort_by_key(|s| s.len());
        
        let mut dp = vec![1; n];
        let mut result = 1;
        
        for i in 1..n {
            for j in (0..i).rev() {
                if strs[j].len() + 1000 < strs[i].len() {
                    break;
                }
                if strs[i].contains(&strs[j]) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            result = result.max(dp[i]);
        }
        
        println!("{}", result);
    }
}
