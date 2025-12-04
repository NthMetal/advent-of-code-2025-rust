use std::fs::File;
use std::io::{BufReader, Cursor, prelude::*};
use std::ops::Sub;
use std::usize;

const MAX_LEN: usize = 12;

fn main() {
    let input_path = "assets/day_4_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let mut reader = BufReader::new(file);
    let mut initial_string = "".to_owned();
    reader.read_to_string(&mut initial_string).expect("expected to be able to read file");

    let mut count = 0;
    let mut total_removed = 0;
    let mut previous_remove_result = 1;
    while previous_remove_result != 0 {
        println!("[{:?}] Current String: {:?}", previous_remove_result, initial_string);
        let rolls = initial_string.clone();
        previous_remove_result = remove_rolls(rolls, &mut initial_string);
        total_removed += previous_remove_result;

        count += 1;
        if count == 3 {
            // break;
        }
    }
    println!("total removed: {:?}", total_removed);

}


pub fn char_at(str: &String, index: usize) -> Option<char> {
    str.chars().nth(index)
}

pub fn remove_rolls(rolls: String, update: &mut String) -> u32 {
    let cursor = Cursor::new(rolls);
    let reader = BufReader::new(cursor);

    let mut previous_line_opt = None;
    let mut current_line_opt = None;
    let mut accessible_count = 0;
    for (next_index, line_result) in reader.lines().chain([Ok("".to_owned())]).enumerate() {
        let Ok(next_line) = line_result else {
            println!("Error reading line: {:?}", next_index);
            continue;
        };
        let current_line = match current_line_opt {
            Some(line) => line,
            None => {
                current_line_opt = Some(next_line);
                continue;
            },
        };
        let previous_line = match previous_line_opt {
            Some(pline) => pline,
            None => "".to_owned(),
        };
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
            if total_surrounding <= 4 {
                accessible_count += 1;
                let replace_index = ((next_index - 1) * (current_line.len() + 2)) + i;
                if let Some((start, old_char)) = update.char_indices().nth(replace_index) {
                    let end = start + old_char.len_utf8();
                    update.replace_range(start..end, ".");
                }
            }
        }

        previous_line_opt = Some(current_line);
        current_line_opt = Some(next_line);
    }

    accessible_count
}