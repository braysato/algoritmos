use std::io::{self, Read, Write, BufWriter};

const MOD: u64 = 1_000_000_007;

type Mat2 = [[u64; 2]; 2];
type Mat5 = [[u64; 5]; 5];

fn mul2(a: &Mat2, b: &Mat2) -> Mat2 {
    let mut c = [[0u64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn pow2(mut m: Mat2, mut n: u64) -> Mat2 {
    let mut r: Mat2 = [[1, 0], [0, 1]];
    while n > 0 {
        if n & 1 == 1 { r = mul2(&r, &m); }
        m = mul2(&m, &m);
        n >>= 1;
    }
    r
}

fn fib(n: u64) -> (u64, u64) {
    if n == 0 { return (0, 1); }
    let base: Mat2 = [[1, 1], [1, 0]];
    let r = pow2(base, n);
    (r[1][0], r[0][0]) // (F(n), F(n+1))
}

fn mul5(a: &Mat5, b: &Mat5) -> Mat5 {
    let mut c = [[0u64; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn pow5(mut m: Mat5, mut n: u64) -> Mat5 {
    let mut r = [[0u64; 5]; 5];
    for i in 0..5 { r[i][i] = 1; }
    while n > 0 {
        if n & 1 == 1 { r = mul5(&r, &m); }
        m = mul5(&m, &m);
        n >>= 1;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut tokens = input.split_whitespace();

    loop {
        let a: u64 = tokens.next().unwrap().parse().unwrap();
        let b: u64 = tokens.next().unwrap().parse().unwrap();
        let n: u64 = tokens.next().unwrap().parse().unwrap();

        if a == 0 && b == 0 && n == 0 { break; }

        let (fa, fa1) = fib(a);
        let (fb, fb1) = fib(b);

        let p = fa * fb % MOD;
        let q = fa1 * fb % MOD;
        let r = fa * fb1 % MOD;
        let s = fa1 * fb1 % MOD;
        let sum = p;

        if n == 1 {
            writeln!(out, "{}", sum).unwrap();
            continue;
        }

        // Transition: p'=s, q'=r+s, r'=q+s, s'=p+q+r+s, S'=S+s
        let mut t = [[0u64; 5]; 5];
        t[0][3] = 1;
        t[1][2] = 1; t[1][3] = 1;
        t[2][1] = 1; t[2][3] = 1;
        t[3][0] = 1; t[3][1] = 1; t[3][2] = 1; t[3][3] = 1;
        t[4][3] = 1; t[4][4] = 1;

        let tn = pow5(t, n - 1);

        let vec = [p, q, r, s, sum];
        let mut ans: u64 = 0;
        for j in 0..5 {
            ans = (ans + tn[4][j] * vec[j]) % MOD;
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
