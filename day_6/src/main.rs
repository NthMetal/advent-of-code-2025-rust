use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Default, Debug)]
pub enum Operation {
    #[default]
    Unknown,
    Add,
    Multiply
}

#[derive(Default, Debug)]
pub struct MathGroup {
    pub number_strings: Vec<String>,
    pub operation: Operation
}
impl MathGroup {
    pub fn from_one(str: String) -> Self {
        MathGroup { number_strings: vec![str], ..Default::default() }
    }
}

fn main() {
    let input_path = "assets/day_6_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);
    // let Ok(line_length) = reader.skip_until(b'\n') else {
    //     println!("Unable to read until newline");
    //     return;
    // };
    // let _ = reader.seek(std::io::SeekFrom::Start(0));
    let mut totals= vec![];
    let mut numbers: Vec<MathGroup>= vec![];
    let mut read_lines= vec![];
    // let mut sums = vec![];
    // let mut products = vec![];

    for read_line in reader.lines() {
        let Ok(line) = read_line else {
            println!("Error reading line");
            continue;
        };
        read_lines.push(line);
    }
    read_lines.reverse();
    for (operation, nums) in split_first_as_header(&read_lines) {
        let operation = match operation.as_str() {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => continue,
        };
        println!("OP: {:?} NUMS: {:?}", operation, nums);
        let mut owned_nums = nums.to_owned();
        owned_nums.reverse();
        numbers.push(MathGroup {
            number_strings: owned_nums,
            operation: operation,
        });


    }

    for number_group in numbers {
        println!("----------------------");
        println!("number_group: {:?}", number_group);
        let parsed_numbers = parse_vertical(&number_group.number_strings);
        let calculated = match number_group.operation {
            Operation::Add =>  parsed_numbers.iter().sum::<u64>(),
            Operation::Multiply => parsed_numbers.iter().product::<u64>(),
            Operation::Unknown => panic!(""),
        };
        println!("parsed_numbers: {:?} = {:?}", parsed_numbers, calculated);
        totals.push(calculated);
    }

    println!("total: {:?}", totals.iter().sum::<u64>());
}


fn parse_vertical(nums: &[String]) -> Vec<u64> {
    let max_len = nums.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut cols: Vec<String> = vec![String::new(); max_len];
    for line in nums {
        for (i, ch) in line.chars().enumerate() {
            if !ch.is_whitespace() {
                cols[i].push(ch);
            }
        }
    }
    cols.into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn split_first_as_header(lines: &[String]) -> Vec<(String, Vec<String>)> {
    if lines.is_empty() {
        return vec![];
    }

    let first_line = &lines[0];
    let chars: Vec<char> = first_line.chars().collect();
    let mut ranges = Vec::new();
    let mut start = 0;
    let mut in_token = !chars[0].is_whitespace();

    // split: " +   "
    //          ^^^
    // split: "   1 "
    // split: " 387 "
    // split: " 215 "
    for i in 1..=chars.len() {
        let c = if i < chars.len() { chars[i] } else { ' ' };
        let is_space = c.is_whitespace();

        if in_token && is_space {
            let mut end = i;
            while end < chars.len() && chars[end].is_whitespace() {
                end += 1;
            }
            println!("start..end: {:?}", start..end);
            ranges.push(start..end);
            start = end;
            in_token = false;
        } else if !in_token && !is_space {
            start = i;
            in_token = true;
        }
    }
    // println!("ranges: {:?}", ranges);
    if start < chars.len() {
        ranges.push(start..chars.len());
    }

    let mut result = Vec::new();
    for range in ranges {
        let header_str = first_line.get(range.clone()).unwrap();
        let header = header_str.trim().to_string();
        let mut rest = Vec::new();
        println!("header: {:?} lines: {:?}", header, lines);
        for row in &lines[1..] {
            let cell = row.get(range.clone()).unwrap_or("").to_string();
            // println!("row: {:?} cell: {:?}", row, cell);
            rest.push(cell);
        }
        result.push((header, rest));
    }

    result
}


// ---------------
// split: "123"
// split: "328"
// split: "  1"
// split: "64"
// ---------------
// split: " 45"
// split: "64"
// split: " 387"
// split: "23"
// ---------------
// split: "  6"
// split: "98"
// split: " 215"
// split: "314"
// ---------------
// split: "*"
// split: "  +"
// split: "  *"
// split: "  +"
// ----------------------

// split: "  1"
// split: " 387"
// split: " 215"