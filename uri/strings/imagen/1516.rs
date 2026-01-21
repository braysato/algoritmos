use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let dims: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = dims[0];
        let m = dims[1];
        
        if n == 0 && m == 0 {
            break;
        }
        
        let mut imagen: Vec<String> = Vec::new();
        for _ in 0..n {
            let mut linea = String::new();
            io::stdin().read_line(&mut linea).unwrap();
            imagen.push(linea.trim_end().to_string());
        }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nuevas_dims: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let a = nuevas_dims[0];
        let b = nuevas_dims[1];
        
        let factor_v = a / n;
        let factor_h = b / m;
        
        for i in 0..n {
            let mut linea_expandida = String::new();
            for ch in imagen[i].chars() {
                for _ in 0..factor_h {
                    linea_expandida.push(ch);
                }
            }
            
            for _ in 0..factor_v {
                println!("{}", linea_expandida);
            }
        }
        
        println!();
    }
}
