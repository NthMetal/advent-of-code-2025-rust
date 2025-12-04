use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Sub;
use std::usize;

const MAX_LEN: usize = 12;

fn main() {
    let input_path = "assets/day_4_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);

    let mut previous_line_opt = None;
    let mut current_line_opt = None;
    let mut accessible_count = 0;
    for (next_index, line_result) in reader.lines().chain([Ok("".to_owned())]).enumerate() {
        let Ok(next_line) = line_result else {
            println!("Error reading line: {:?}", next_index);
            return;
        };
        let current_line = match current_line_opt {
            Some(line) => line,
            None => {
                current_line_opt = Some(next_line);
                continue;
            },
        };
        let line_length = current_line.len();
        let previous_line = match previous_line_opt {
            Some(pline) => pline,
            None => "".to_owned(),
        };
        println!("------------------------previous_line>>>{:?}", previous_line);
        println!("------------------------CurrentLine>>>{:?}", current_line);
        println!("------------------------next_line>>>{:?}", next_line);
        for i in 0..current_line.len() {
            let current_char = char_at(&current_line, i).expect("current char to exist");
            if current_char != char::from(b'@') { continue; }
            let left_index = i.checked_sub(1).unwrap_or(usize::MAX);
            let right_index = i.checked_add(1).unwrap_or(usize::MAX);
            let surrounding = vec![
                char_at(&previous_line, left_index), char_at(&previous_line, i), char_at(&previous_line, right_index),
                char_at(&current_line, left_index), Some(current_char), char_at(&current_line, right_index),
                char_at(&next_line, left_index), char_at(&next_line, i), char_at(&next_line, right_index),
            ];
            let total_surrounding = surrounding.iter().fold(0, |acc, curr| {
                if let Some(char) = curr {
                    acc + (if *char == char::from(b'@') { 1 } else { 0 })
                } else {
                    acc
                }
            });
            let mut to_add = 0;
            if total_surrounding <= 4 {
                to_add += 1
            }
            accessible_count += to_add;
            println!("[{:?}] surrounding ({:?}) {:?}", to_add, total_surrounding, surrounding);
        }

        previous_line_opt = Some(current_line);
        current_line_opt = Some(next_line);
    }

    println!("accessible_count: {:?}", accessible_count);
}


pub fn char_at(str: &String, index: usize) -> Option<char> {
    str.chars().nth(index)
}