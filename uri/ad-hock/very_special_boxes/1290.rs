use std::io::{self, BufRead, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    
    loop {
        let line = lines.next().unwrap().unwrap();
        let nm: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nm[0];
        let m = nm[1];
        
        if n == 0 {
            break;
        }
        
        let line = lines.next().unwrap().unwrap();
        let mut item: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        item.sort();
        let vol_item = item[0] * item[1] * item[2];
        
        let mut conteo: HashMap<(i32, i32, i32), i32> = HashMap::new();
        
        for _ in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut caja: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            caja.sort();
            
            if caja[0] >= item[0] && caja[1] >= item[1] && caja[2] >= item[2] {
                *conteo.entry((caja[0], caja[1], caja[2])).or_insert(0) += 1;
            }
        }
        
        let mut mejor: Option<i32> = None;
        for ((a, b, c), count) in &conteo {
            if *count >= n {
                let vol_caja = a * b * c;
                let espacio = vol_caja - vol_item;
                mejor = Some(mejor.map_or(espacio, |m| m.min(espacio)));
            }
        }
        
        match mejor {
            None => writeln!(out, "impossible").unwrap(),
            Some(v) => writeln!(out, "{}", v).unwrap(),
        }
    }
}
