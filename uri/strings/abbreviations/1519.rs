use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct WordStats {
    count: i32,
    first_pos: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        if line == "." {
            break;
        }

        let mut stats: Vec<HashMap<String, WordStats>> = vec![HashMap::new(); 26];
        let mut tokens: Vec<(bool, String)> = Vec::with_capacity(line.len());

        let mut in_word = false;
        let mut word = String::new();
        let mut sep = String::new();
        let mut word_index: i32 = 0;

        let mut flush_word = |tokens: &mut Vec<(bool, String)>, stats: &mut Vec<HashMap<String, WordStats>>, word: &mut String, word_index: &mut i32| {
            if !word.is_empty() {
                let token = word.clone();
                tokens.push((true, token.clone()));
                let initial = token.chars().next().unwrap();
                let idx = (initial as u8 - b'a') as usize;
                let entry = stats[idx].entry(token).or_insert(WordStats {
                    count: 0,
                    first_pos: *word_index,
                });
                entry.count += 1;
                if entry.count == 1 {
                    entry.first_pos = *word_index;
                }
                *word_index += 1;
                word.clear();
            }
        };

        let mut flush_sep = |tokens: &mut Vec<(bool, String)>, sep: &mut String| {
            if !sep.is_empty() {
                tokens.push((false, sep.clone()));
                sep.clear();
            }
        };

        for ch in line.chars() {
            if ch.is_ascii_lowercase() {
                if !in_word {
                    flush_sep(&mut tokens, &mut sep);
                    in_word = true;
                }
                word.push(ch);
            } else {
                if in_word {
                    flush_word(&mut tokens, &mut stats, &mut word, &mut word_index);
                    in_word = false;
                }
                sep.push(ch);
            }
        }
        if in_word {
            flush_word(&mut tokens, &mut stats, &mut word, &mut word_index);
        }
        flush_sep(&mut tokens, &mut sep);

        let mut chosen: Vec<Option<String>> = vec![None; 26];
        let mut best_gain: Vec<i64> = vec![0; 26];
        let mut best_pos: Vec<i32> = vec![i32::MAX; 26];

        for idx in 0..26 {
            for (word, info) in stats[idx].iter() {
                let len = word.len() as i64;
                if len <= 2 {
                    continue;
                }
                let gain = (len - 2) * info.count as i64;
                if gain <= 0 {
                    continue;
                }
                if gain > best_gain[idx] || (gain == best_gain[idx] && info.first_pos < best_pos[idx]) {
                    best_gain[idx] = gain;
                    best_pos[idx] = info.first_pos;
                    chosen[idx] = Some(word.clone());
                }
            }
        }

        let mut output = String::with_capacity(line.len());
        for (is_word, token) in &tokens {
            if *is_word {
                let letter = token.chars().next().unwrap();
                let idx = (letter as u8 - b'a') as usize;
                if let Some(selected) = &chosen[idx] {
                    if selected == token {
                        output.push(letter);
                        output.push('.');
                        continue;
                    }
                }
                output.push_str(token);
            } else {
                output.push_str(token);
            }
        }

        println!("{}", output);
        let mut abbreviations: Vec<(char, String)> = Vec::new();
        for idx in 0..26 {
            if let Some(word) = &chosen[idx] {
                abbreviations.push(((b'a' + idx as u8) as char, word.clone()));
            }
        }
        println!("{}", abbreviations.len());
        for (letter, word) in abbreviations {
            println!("{}. = {}", letter, word);
        }
    }
}
