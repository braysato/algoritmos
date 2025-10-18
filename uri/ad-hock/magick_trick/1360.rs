use std::io;

fn get_value_index(value: char) -> usize {
    match value {
        '1'..='9' => (value as usize) - ('1' as usize),
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        _ => 0,
    }
}

fn get_value_char(index: usize) -> char {
    match index {
        0..=8 => (('1' as u8) + (index as u8)) as char,
        9 => 'T',
        10 => 'J',
        11 => 'Q',
        12 => 'K',
        _ => '1',
    }
}

fn get_suit_index(suit: char) -> usize {
    match suit {
        'H' => 0,
        'C' => 1,
        'D' => 2,
        'S' => 3,
        _ => 0,
    }
}

fn compare_cards(a: &str, b: &str) -> std::cmp::Ordering {
    let value_a = a.chars().nth(0).unwrap();
    let suit_a = a.chars().nth(1).unwrap();
    let value_b = b.chars().nth(0).unwrap();
    let suit_b = b.chars().nth(1).unwrap();
    
    let suit_index_a = get_suit_index(suit_a);
    let suit_index_b = get_suit_index(suit_b);
    
    if suit_index_a != suit_index_b {
        return suit_index_a.cmp(&suit_index_b);
    }
    
    get_value_index(value_a).cmp(&get_value_index(value_b))
}

fn get_offset(ordered: &Vec<String>, original: &Vec<String>) -> usize {
    if original[0] == ordered[0] && original[1] == ordered[1] && original[2] == ordered[2] {
        return 1;
    }
    if original[0] == ordered[0] && original[1] == ordered[2] && original[2] == ordered[1] {
        return 2;
    }
    if original[0] == ordered[1] && original[1] == ordered[0] && original[2] == ordered[2] {
        return 3;
    }
    if original[0] == ordered[1] && original[1] == ordered[2] && original[2] == ordered[0] {
        return 4;
    }
    if original[0] == ordered[2] && original[1] == ordered[0] && original[2] == ordered[1] {
        return 5;
    }
    if original[0] == ordered[2] && original[1] == ordered[1] && original[2] == ordered[0] {
        return 6;
    }
    1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let cards: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();
        
        let card1 = &cards[0];
        let suit = card1.chars().nth(1).unwrap();
        let value_index = get_value_index(card1.chars().nth(0).unwrap());
        
        let last_three = vec![cards[1].clone(), cards[2].clone(), cards[3].clone()];
        let mut ordered = last_three.clone();
        ordered.sort_by(|a, b| compare_cards(a, b));
        
        let offset = get_offset(&ordered, &last_three);
        
        let hidden_value_index = (value_index + offset) % 13;
        let hidden_value = get_value_char(hidden_value_index);
        
        println!("{}{}", hidden_value, suit);
    }
}
