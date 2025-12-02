use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "assets/day_2_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);

    let mut invalid_ids: Vec<u64> = vec![];

    for list_item in reader.split(b',').map(|line| String::from_utf8_lossy(&line.unwrap()).to_string()) {
        println!("------------------------------------");
        let mut items: Vec<&str> = list_item.split("-").collect();
        if items.len() != 2 {
            println!("ERROR: This item doesn't contain a first and last: {:?}", list_item);
            continue;
        }
        let (last_str,mut first_str ) = (items.pop().unwrap().to_owned(), items.pop().unwrap().to_owned());
        let Ok(mut first) = first_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", first_str);
            continue;
        };
        let Ok(last) = last_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", last_str);
            continue;
        };

        if first_str.len() % 2 == 1 {
            first = 10u64.pow(first_str.len() as u32);
            first_str = first.to_string();
        }
        
        if first > last {
            println!("first ({:?}) must be less than last ({:?}), skipping...", first, last);
            continue;
        }

        let (first_left_str, first_right_str) = split_string_bytes(first_str.as_str());
        let (last_left_str, last_right_str) = match last_str.len() % 2 {
            0 => split_string_bytes(last_str.as_str()),
            1 => last_str.split_at(first_left_str.len() + 1) ,
            _ => unreachable!(),
        };
        
        let Ok(first_left) = first_left_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", first_left_str);
            continue;
        };
        let Ok(first_right) = first_right_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", first_right_str);
            continue;
        };
        let Ok(last_left) = last_left_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", last_left_str);
            continue;
        };
        let Ok(last_right) = last_right_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", last_right_str);
            continue;
        };

        for i in first_left..=last_left {
            let Ok(new_combined) = (i.to_string() + &i.to_string()).parse::<u64>() else {
                println!("ERROR: Not a valid number: {:?}", first_left_str);
                continue;
            };
            println!("new_combined: {:?}, first: {:?}, last: {:?}", new_combined, first, last);
            if new_combined <= last && new_combined >= first {
                invalid_ids.push(new_combined);
            }
        }

        println!("first_left: {:?} last_left: {:?}", first_left, last_left);
        // println!("first_left: {:?} first_right: {:?}", first_left, first_right);
        // println!("last_left: {:?} last_right: {:?}", last_left, last_right);
        // println!("first: {:?} last: {:?}", first, last);
        // println!("range: {:?}", range);
    }
    println!("======================================");
    println!("invalid_ids: {:?}", invalid_ids);
    println!("sum: {:?}", invalid_ids.iter().sum::<u64>());
}

// first: 3737332285 last: 3737422568
// range: 90283

// 37373
// 32285

// 37374
// 22568

fn split_string_bytes(s: &str) -> (&str, &str) {
    if s.len() % 2 == 1 {
        println!("Can't split string: \"{:?}\" with odd length: {:?}", s, s.len());
        panic!("Error splitting string");
    }
    let mid_index = s.len() / 2;
    s.split_at(mid_index) 
}