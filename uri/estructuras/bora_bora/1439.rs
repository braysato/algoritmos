use std::io::{self, BufRead, Write, BufWriter};

#[derive(Clone, Copy)]
struct Carta {
    rango: i32,
    palo: char,
}

impl Carta {
    fn valor_palo(&self) -> i32 {
        match self.palo {
            'C' => 0,
            'D' => 1,
            'H' => 2,
            _ => 3,
        }
    }
    
    fn valor(&self) -> i32 {
        self.rango * 10 + self.valor_palo()
    }
    
    fn coincide(&self, otra: &Carta) -> bool {
        self.rango == otra.rango || self.palo == otra.palo
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    loop {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let p = nums[0] as usize;
        let m = nums[1] as usize;
        let n = nums[2] as usize;
        
        if p == 0 && m == 0 && n == 0 {
            break;
        }
        
        let mut deck: Vec<Carta> = Vec::with_capacity(n);
        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let rango: i32 = parts[0].parse().unwrap();
            let palo = parts[1].chars().next().unwrap();
            deck.push(Carta { rango, palo });
        }
        
        let mut manos: Vec<Vec<Carta>> = vec![vec![]; p];
        let mut idx = 0;
        for i in 0..p {
            for _ in 0..m {
                manos[i].push(deck[idx]);
                idx += 1;
            }
        }
        
        let mut top = deck[idx];
        idx += 1;
        let mut stock: Vec<Carta> = deck[idx..].iter().rev().cloned().collect();
        
        let mut dir: i32 = 1;
        let mut jugador: i32 = 0;
        
        if top.rango == 12 {
            dir = -dir;
        }
        
        let mut penalidad = 0;
        let mut skip_turno = false;
        if top.rango == 7 {
            penalidad = 2;
            skip_turno = true;
        } else if top.rango == 1 {
            penalidad = 1;
            skip_turno = true;
        } else if top.rango == 11 {
            skip_turno = true;
        }
        
        loop {
            let j = jugador as usize;
            
            for _ in 0..penalidad {
                if let Some(c) = stock.pop() {
                    manos[j].push(c);
                }
            }
            penalidad = 0;
            
            if skip_turno {
                skip_turno = false;
                jugador = (jugador + dir + p as i32) % p as i32;
                continue;
            }
            
            let mut mejor: Option<usize> = None;
            for i in 0..manos[j].len() {
                if manos[j][i].coincide(&top) {
                    if mejor.is_none() || manos[j][mejor.unwrap()].valor() < manos[j][i].valor() {
                        mejor = Some(i);
                    }
                }
            }
            
            if mejor.is_none() {
                if let Some(robada) = stock.pop() {
                    manos[j].push(robada);
                    if robada.coincide(&top) {
                        mejor = Some(manos[j].len() - 1);
                    }
                }
            }
            
            if let Some(idx) = mejor {
                top = manos[j][idx];
                manos[j].remove(idx);
                
                if manos[j].is_empty() {
                    writeln!(out, "{}", jugador + 1).unwrap();
                    break;
                }
                
                if top.rango == 12 {
                    dir = -dir;
                } else if top.rango == 7 {
                    penalidad = 2;
                    skip_turno = true;
                } else if top.rango == 1 {
                    penalidad = 1;
                    skip_turno = true;
                } else if top.rango == 11 {
                    skip_turno = true;
                }
            }
            
            jugador = (jugador + dir + p as i32) % p as i32;
        }
    }
}
