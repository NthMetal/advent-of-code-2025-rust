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
        println!("{:?}", list_item);
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

        let (first_left_str, _) = split_string_bytes(first_str.as_str());
        let (last_left_str, last_right_str) = split_string_bytes(last_str.as_str());

        let Ok(last_left) = last_left_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", last_left_str);
            continue;
        };
        let Ok(last_right) = last_right_str.parse::<u64>() else {
            println!("ERROR: Not a valid number: {:?}", last_right_str);
            continue;
        };

        // println!("first_left: {:?} last_left: {:?}", first_left_str, last_left_str);

        let mut current_char_len = 1;
        // let mut current_value = first_str.chars().nth(0).and_then(|c| c.to_digit(10)).expect("first_str must have chars");
        let mut current_value = 1;
        while (current_value as u64) <= last_left {
            let current_val_string = current_value.to_string();
            // 49046150754
            let max = ((last_str.len() as f32) / (current_val_string.len() as f32)).ceil() as usize;
            for i in 2..=max {
                let Ok(new_value) = current_val_string.repeat(i).parse::<u64>() else {
                    // println!("Repeating {:?}, {:?} times is overflow", current_val_string, i);
                    continue;
                };
                if new_value <= last && new_value >= first {
                    // println!("Adding: {:?}", new_value);
                    invalid_ids.push(new_value);
                }
            }
            current_value += 1;
            // if current_value == 9 {
            //     current_char_len += 1;
            //     current_value = get_first_n_chars(first_str.as_str(), current_char_len).parse().expect("first_str should be parsable");
            // } else {
            //     current_value += 1;
            // }
        }
    }
    println!("======================================");
    println!("invalid_ids: {:?}", invalid_ids);
    invalid_ids.sort_unstable();
    invalid_ids.dedup();
    println!("sum: {:?}", invalid_ids.iter().sum::<u64>());
}

fn split_string_bytes(s: &str) -> (&str, &str) {
    let mid_index = (s.len() + 1) / 2; // round up
    s.split_at(mid_index)
}
