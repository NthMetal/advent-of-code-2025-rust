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

    // let mut valid_ranges = HashSet::new();
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

    // let mut processed_ranges: HashSet<(u64, u64)> = HashSet::new();
    let mut ranges = ranges.iter().map(|range| (*range.start(), *range.end())).collect::<Vec<(u64, u64)>>();
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut valid_range_count: u64 = 0;
    let mut current = None::<(u64,u64)>;

    for (start, end) in ranges {
        let (current_range_start, current_range_end) = current.unwrap_or((start, end));

        if start <= current_range_end + 1 {
            current = Some((current_range_start, current_range_end.max(end)));
        } else {
            valid_range_count += current_range_end - current_range_start + 1;
            current = Some((start, end));
        }
    }

    // last
    if let Some((cs, ce)) = current {
        valid_range_count += ce - cs + 1;
    }
    // 19
    println!("fresh: {:?}", fresh);
    println!("valid_range_count: {:?}", valid_range_count);
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

// 342046150371924

// 347822618448856
