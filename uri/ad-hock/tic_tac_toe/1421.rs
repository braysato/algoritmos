use std::io::{self, Read};

#[derive(Copy, Clone)]
struct Direction {
    dx: i32,
    dy: i32,
    dz: i32,
}

fn check_win(board: &Vec<Vec<Vec<char>>>, n: usize, x: usize, y: usize, z: usize, player: char) -> bool {
    let dirs = [
        Direction { dx: 1, dy: 0, dz: 0 },
        Direction { dx: 0, dy: 1, dz: 0 },
        Direction { dx: 0, dy: 0, dz: 1 },
        Direction { dx: 1, dy: 1, dz: 0 },
        Direction { dx: 1, dy: -1, dz: 0 },
        Direction { dx: 1, dy: 0, dz: 1 },
        Direction { dx: 1, dy: 0, dz: -1 },
        Direction { dx: 0, dy: 1, dz: 1 },
        Direction { dx: 0, dy: 1, dz: -1 },
        Direction { dx: 1, dy: 1, dz: 1 },
        Direction { dx: 1, dy: 1, dz: -1 },
        Direction { dx: 1, dy: -1, dz: 1 },
        Direction { dx: 1, dy: -1, dz: -1 },
    ];
    let n_i32 = n as i32;
    for dir in dirs.iter() {
        let mut count = 1;
        let mut nx = x as i32 + dir.dx;
        let mut ny = y as i32 + dir.dy;
        let mut nz = z as i32 + dir.dz;
        while nx >= 0 && nx < n_i32 && ny >= 0 && ny < n_i32 && nz >= 0 && nz < n_i32 {
            if board[nx as usize][ny as usize][nz as usize] != player {
                break;
            }
            count += 1;
            nx += dir.dx;
            ny += dir.dy;
            nz += dir.dz;
        }
        nx = x as i32 - dir.dx;
        ny = y as i32 - dir.dy;
        nz = z as i32 - dir.dz;
        while nx >= 0 && nx < n_i32 && ny >= 0 && ny < n_i32 && nz >= 0 && nz < n_i32 {
            if board[nx as usize][ny as usize][nz as usize] != player {
                break;
            }
            count += 1;
            nx -= dir.dx;
            ny -= dir.dy;
            nz -= dir.dz;
        }
        if count >= n_i32 {
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut instance = 1;
    while let Some(n_token) = iter.next() {
        let n: usize = n_token.parse().unwrap();
        if n == 0 {
            break;
        }
        let mut board = vec![vec![vec!['.'; n]; n]; n];
        let mut height = vec![vec![0usize; n]; n];
        let mut winner = '.';
        let total_moves = n * n * n;
        for move_idx in 0..total_moves {
            let i: usize = iter.next().unwrap().parse().unwrap();
            let j: usize = iter.next().unwrap().parse().unwrap();
            let x = i - 1;
            let y = j - 1;
            let z = height[x][y];
            height[x][y] += 1;
            let player = if move_idx % 2 == 0 { 'B' } else { 'A' };
            board[x][y][z] = player;
            if winner == '.' && check_win(&board, n, x, y, z, player) {
                winner = player;
            }
        }
        println!("Instancia {}", instance);
        match winner {
            'B' => println!("Branco ganhou"),
            'A' => println!("Azul ganhou"),
            _ => println!("Empate"),
        }
        println!();
        instance += 1;
    }
}
