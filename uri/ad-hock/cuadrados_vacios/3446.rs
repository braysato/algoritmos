use std::io::{self, Read};

fn shift_bits(src: &[u64], shift: usize, words: usize, right_len: usize) -> Vec<u64> {
    let mut dest = vec![0u64; words];
    if shift == 0 {
        dest.copy_from_slice(src);
    } else {
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in (0..words).rev() {
            if i < word_shift {
                continue;
            }
            let mut val = src[i - word_shift] << bit_shift;
            if bit_shift != 0 && i >= word_shift + 1 {
                val |= src[i - word_shift - 1] >> (64 - bit_shift);
            }
            dest[i] = val;
        }
    }
    let total_bits = right_len + 1;
    let extra_bits = words * 64 - total_bits;
    if extra_bits > 0 {
        let mask = if extra_bits >= 64 {
            0u64
        } else {
            (!0u64) >> extra_bits
        };
        dest[words - 1] &= mask;
    }
    dest
}

fn has_bit(bits: &[u64], idx: usize) -> bool {
    let word = idx / 64;
    let offset = idx % 64;
    (bits[word] >> offset) & 1 == 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    if input.trim().is_empty() {
        return;
    }
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let e: usize = iter.next().unwrap().parse().unwrap();
    let left_len = e;
    let right_len = n - k - e;
    let words = (right_len / 64) + 1;
    let mut dp = vec![vec![0u64; words]; left_len + 1];
    dp[0][0] |= 1;
    let mut pieces = Vec::with_capacity(n - 1);
    for len in 1..=n {
        if len != k {
            pieces.push(len);
        }
    }
    for w in pieces {
        let mut next = dp.clone();
        if w <= left_len {
            for l in w..=left_len {
                for idx in 0..words {
                    next[l][idx] |= dp[l - w][idx];
                }
            }
        }
        if w <= right_len {
            for l in 0..=left_len {
                let shifted = shift_bits(&dp[l], w, words, right_len);
                for idx in 0..words {
                    next[l][idx] |= shifted[idx];
                }
            }
        }
        dp = next;
    }
    let mut best = 0usize;
    for l in 0..=left_len {
        for r in (0..=right_len).rev() {
            if has_bit(&dp[l], r) {
                let total = l + r;
                if total > best {
                    best = total;
                }
                break;
            }
        }
    }
    let result = (n - k) - best;
    println!("{}", result);
}
