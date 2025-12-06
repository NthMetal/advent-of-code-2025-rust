use std::ops::RangeInclusive;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "assets/day_6_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let mut reader = BufReader::new(file);
    // let Ok(line_length) = reader.skip_until(b'\n') else {
    //     println!("Unable to read until newline");
    //     return;
    // };
    // let _ = reader.seek(std::io::SeekFrom::Start(0));
    let mut totals= vec![];
    let mut sums = vec![];
    let mut products = vec![];

    for (line_index, line_result) in reader.lines().enumerate() {
        let Ok(line) = line_result else {
            println!("Error reading line: {:?}", line_index);
            return;
        };
        let split_lines = line.split_whitespace();
        // println!("---------------");
        for (split_index, split) in split_lines.enumerate() {
            // println!("split: {:?}", split);
            let parsed_str_result = split.parse::<u64>();
            let number_result = match parsed_str_result {
                Ok(r) => r,
                Err(_) => {
                    match split {
                        "*" => totals.push(products[split_index]),
                        "+" => totals.push(sums[split_index]),
                        _ => continue,
                    }
                    continue;
                },
            };
            if split_index < products.len() {
                products[split_index] *= number_result;
            } else {
                products.push(number_result);
            }
            if split_index < sums.len() {
                sums[split_index] += number_result;
            } else {
                sums.push(number_result);
            }
        }
    }

    // let mut current_horizontal_offset = 0;
    // 'horizontal: while current_horizontal_offset < line_length {
    //     let mut next_seek_successful = true;
    //     let mut line_index = 0;
    //     let mut first_number_length = 0;
    //     'vertical: while next_seek_successful {
    //         next_seek_successful = reader.seek(std::io::SeekFrom::Start((
    //             current_horizontal_offset * (line_length + line_index)
    //         ) as u64)).is_ok();
    //         let mut number_buffer = vec![];
    //         let number_read_result = reader.read_until(b' ', &mut number_buffer);
    //         let number_length = match number_read_result {
    //             Ok(s) => s,
    //             Err(_) => {
    //                 match reader.read_until(b'\n', &mut number_buffer) {
    //                     Ok(s) => s,
    //                     Err(_) => break 'vertical,
    //                 }
    //             },
    //         };
    //         let buffer_str = str::from_utf8(&number_buffer).unwrap();
    //         if line_index == 0 && buffer_str.ends_with(" ") &&  buffer_str.len() > 0 {
    //             first_number_length = number_length;
    //         }
    //         let parsed_str_result = buffer_str.replace(" ", "").parse::<u32>();
    //         let first_number = match parsed_str_result {
    //             Ok(r) => r,
    //             Err(_) => {
    //                 current_horizontal_offset += number_length;
    //                 continue 'horizontal;
    //             },
    //         };
    //         line_index += 1;
    //         println!("Number Buffer: {:?}", first_number);
    //         println!("first_number_length: {:?}", first_number_length);
    //     }

    //     current_horizontal_offset += first_number_length;
    // }

    // for (line_index, line_result) in reader.lines().enumerate() {
    //     println!("Line: {:?}", line_result);
    // }

    println!("total: {:?}", totals.iter().sum::<u64>());
}


