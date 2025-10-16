use std::io::{self, BufRead};

fn get_month_number(month: &str) -> usize {
    let months = ["", "JAN", "FEV", "MAR", "ABR", "MAI", "JUN", "JUL", "AGO", "SET", "OUT", "NOV", "DEZ"];
    for i in 1..=12 {
        if months[i] == month {
            return i;
        }
    }
    1
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_days_in_month(month: usize, year: i32) -> i32 {
    let days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month == 2 && is_leap_year(year) {
        return 29;
    }
    days[month]
}

fn get_day_of_week(d: i32, m: usize, y: i32) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = y;
    if m < 3 {
        year -= 1;
    }
    ((year + year/4 - year/100 + year/400 + t[m-1] + d) % 7) as usize
}

fn add_seconds(day: &mut i32, month: &mut usize, year: &mut i32, hour: &mut i32, minute: &mut i32, second: &mut i32, add_sec: i32) {
    *second += add_sec;
    
    *minute += *second / 60;
    *second %= 60;
    
    *hour += *minute / 60;
    *minute %= 60;
    
    *day += *hour / 24;
    *hour %= 24;
    
    while *day > get_days_in_month(*month, *year) {
        *day -= get_days_in_month(*month, *year);
        *month += 1;
        if *month > 12 {
            *month = 1;
            *year += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut case_number = 1;
    
    let months = ["", "JAN", "FEV", "MAR", "ABR", "MAI", "JUN", "JUL", "AGO", "SET", "OUT", "NOV", "DEZ"];
    let days = ["DOM", "SEG", "TER", "QUA", "QUI", "SEX", "SAB"];
    
    while let Some(Ok(line)) = lines.next() {
        if line == "FIM" {
            break;
        }
        
        let day: i32 = line[4..6].parse().unwrap();
        let month_str = &line[6..9];
        let year: i32 = line[9..13].parse().unwrap();
        let hour: i32 = line[14..16].parse().unwrap();
        let minute: i32 = line[17..19].parse().unwrap();
        let second: i32 = line[20..22].parse().unwrap();
        
        let month_num = get_month_number(month_str);
        
        println!("Previsao #{}", case_number);
        
        for _ in 0..5 {
            if let Some(Ok(sec_line)) = lines.next() {
                let seconds: i32 = sec_line.trim().parse().unwrap();
                
                let mut new_day = day;
                let mut new_month = month_num;
                let mut new_year = year;
                let mut new_hour = hour;
                let mut new_minute = minute;
                let mut new_second = second;
                
                add_seconds(&mut new_day, &mut new_month, &mut new_year, &mut new_hour, &mut new_minute, &mut new_second, seconds);
                
                let day_of_week_num = get_day_of_week(new_day, new_month, new_year);
                
                println!("{},{:02}{}{:04}:{:02}:{:02}:{:02}", 
                    days[day_of_week_num], 
                    new_day, 
                    months[new_month], 
                    new_year, 
                    new_hour, 
                    new_minute, 
                    new_second
                );
            }
        }
        
        case_number += 1;
    }
}
