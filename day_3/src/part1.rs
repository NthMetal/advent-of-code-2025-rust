use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "assets/day_3_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);

    let mut bank_joltages = vec![];

    for (line_index, line_result) in reader.lines().enumerate() {
        println!("--------------------------------------------");
        let Ok(bank) = line_result else {
            println!("Error reading line: {:?}", line_index);
            return;
        };
        println!("BANK: {:?}", bank);
        let mut voltages = vec![];
        for (index, c) in bank.chars().enumerate() {
            let num = c.to_digit(10).expect("its made of valid digits");
            voltages.push(num);
        }
        let mut highest = 0;
        let mut highest_index = 0;
        let mut second_highest = 0;
        let mut second_highest_index = 0;
        for (index, &num) in voltages.iter().enumerate() {
            if num > highest {
                highest = num;
                highest_index = index;
            }
        }
        for (index, &num) in voltages.iter().enumerate() {
            if num > second_highest && index > highest_index {
                second_highest = num;
                second_highest_index = index;
            }
        }
        if second_highest == 0 {
            second_highest = highest;
            second_highest_index = highest_index;
            highest = 0;
            highest_index = 0;
            for (index, &num) in voltages.iter().enumerate() {
                if num > highest && index != second_highest_index {
                    highest = num;
                    highest_index = index;
                }
            }
        }
        println!("highest: {:?} ({:?}), second_highest: {:?} ({:?})", highest, highest_index, second_highest, second_highest_index);
        let highest_joltage = highest * 10 + second_highest;
        bank_joltages.push(highest_joltage);
    }
    println!("======================================");
    println!("bank_joltages: {:?}", bank_joltages);
    // bank_joltages.sort_unstable();
    // bank_joltages.dedup();
    println!("sum: {:?}", bank_joltages.iter().sum::<u32>());
}
