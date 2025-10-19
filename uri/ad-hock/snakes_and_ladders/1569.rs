use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        
        let mut board: HashMap<i32, i32> = HashMap::new();
        
        for _ in 0..b {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
            board.insert(parts[0], parts[1]);
        }
        
        let mut players = vec![1; a + 1];
        
        let mut game_over = false;
        let mut current_player = 1;
        
        for _ in 0..c {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let roll: i32 = input.trim().parse().unwrap();
            
            if !game_over {
                players[current_player] += roll;
                
                if players[current_player] > 100 {
                    players[current_player] = 100;
                }
                
                if let Some(&destination) = board.get(&players[current_player]) {
                    players[current_player] = destination;
                }
                
                if players[current_player] == 100 {
                    game_over = true;
                }
                
                current_player += 1;
                if current_player > a {
                    current_player = 1;
                }
            }
        }
        
        for i in 1..=a {
            println!("Position of player {} is {}.", i, players[i]);
        }
    }
}
