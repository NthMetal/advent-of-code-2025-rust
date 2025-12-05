use std::ops::RangeInclusive;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "assets/day_5_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);

    let mut ranges = vec![];
    let mut fresh = 0;
    let mut finished_processing_ranges = false;
    for (line_index, line_result) in reader.lines().enumerate() {
        let Ok(line) = line_result else {
            println!("Error reading line: {:?}", line_index);
            return;
        };
        
        if line == "".to_owned() {
            println!("Blank at: {:?}", line_index);
            finished_processing_ranges = true;
            continue;
        }
        match finished_processing_ranges {
            false => {
                println!("line: {:?}", line);
                process_range(&mut ranges, line)
            },
            true => {
                let is_fresh = process_item(&ranges, line.as_str());
                println!("line: {:?} -- {:?}", line, is_fresh);
                fresh += is_fresh
            },
        }
    }

    println!("fresh: {:?}", fresh);
}

pub fn process_range(ranges: &mut Vec<RangeInclusive<u64>>, line: String) {
    let mut split = line.split("-");
    let left_str = split.next().expect(format!("left String: {:?} must be splitable by a -", line).as_str());
    let right_str = split.next().expect(format!("right String: {:?} must be splitable by a -", line).as_str());
    let left = left_str.parse::<u64>().expect(format!("left String: {:?} must be parsable to a u64", left_str).as_str());
    let right = right_str.parse::<u64>().expect(format!("right String: {:?} must be parsable to a u64", right_str).as_str());
    ranges.push(left..=right);
}

pub fn process_item(ranges: &Vec<RangeInclusive<u64>>, line: &str) -> u32 {
    let id = line.parse::<u64>().expect(format!("String: {:?} must be parsable to a u64", line).as_str());
    for range in ranges {
        if range.contains(&id) {
            return 1;
        }
    }
    return 0;
}