use std::io::{self, BufRead, Write, BufWriter};

fn signo(x: i32) -> i32 {
    if x > 0 { 1 }
    else if x < 0 { -1 }
    else { 0 }
}

fn build(tree: &mut Vec<i32>, arr: &Vec<i32>, node: usize, start: usize, end: usize) {
    if start == end {
        tree[node] = arr[start];
    } else {
        let mid = (start + end) / 2;
        build(tree, arr, 2 * node, start, mid);
        build(tree, arr, 2 * node + 1, mid + 1, end);
        tree[node] = tree[2 * node] * tree[2 * node + 1];
    }
}

fn update(tree: &mut Vec<i32>, arr: &mut Vec<i32>, node: usize, start: usize, end: usize, idx: usize, val: i32) {
    if start == end {
        arr[idx] = val;
        tree[node] = val;
    } else {
        let mid = (start + end) / 2;
        if idx <= mid {
            update(tree, arr, 2 * node, start, mid, idx, val);
        } else {
            update(tree, arr, 2 * node + 1, mid + 1, end, idx, val);
        }
        tree[node] = tree[2 * node] * tree[2 * node + 1];
    }
}

fn query(tree: &Vec<i32>, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
    if r < start || end < l {
        return 1;
    }
    if l <= start && end <= r {
        return tree[node];
    }
    let mid = (start + end) / 2;
    let p1 = query(tree, 2 * node, start, mid, l, r);
    let p2 = query(tree, 2 * node + 1, mid + 1, end, l, r);
    p1 * p2
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut input = String::new();
    stdin.lock().read_line(&mut input).ok();
    
    loop {
        input.clear();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break;
        }
        
        let nums: Vec<i32> = input.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        
        if nums.len() < 2 {
            break;
        }
        
        let n = nums[0] as usize;
        let k = nums[1] as usize;
        
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let valores: Vec<i32> = input.split_whitespace()
            .map(|x| signo(x.parse().unwrap()))
            .collect();
        
        let mut arr = vec![0; n + 1];
        for i in 0..n {
            arr[i + 1] = valores[i];
        }
        
        let mut tree = vec![0; 4 * n];
        build(&mut tree, &arr, 1, 1, n);
        
        let mut resultado = String::new();
        
        for _ in 0..k {
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            let cmd = parts[0];
            
            if cmd == "C" {
                let idx: usize = parts[1].parse().unwrap();
                let val: i32 = parts[2].parse().unwrap();
                update(&mut tree, &mut arr, 1, 1, n, idx, signo(val));
            } else {
                let l: usize = parts[1].parse().unwrap();
                let r: usize = parts[2].parse().unwrap();
                let prod = query(&tree, 1, 1, n, l, r);
                if prod > 0 { resultado.push('+'); }
                else if prod < 0 { resultado.push('-'); }
                else { resultado.push('0'); }
            }
        }
        
        writeln!(out, "{}", resultado).unwrap();
    }
}
