use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_LEN: usize = 12;

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
        for c in bank.chars() {
            let num = c.to_digit(10).expect("its made of valid digits");
            voltages.push(num);
        }
        
        let mut numbers: [u32; MAX_LEN] = [0; MAX_LEN];
        let mut registered_indicies = vec![];
        let mut last_added_index = 0;
        for big_index in 0..MAX_LEN {
            
            let current_highest: &mut u32 = &mut numbers[big_index];
            for (index, &voltage) in (&voltages).iter().enumerate() {
                let max = (voltages.len() - 1) - (MAX_LEN - big_index - 1);
                // println!("{:?} index: {:?} - {:?}", voltage, index, max);
                if index > max { continue; }
                if index < last_added_index { continue; }
                // 811111111111119
                //             ^ 0 stop here
                // 811111111119111
                //            ^
                if registered_indicies.contains(&index) { continue; }
                if voltage > *current_highest {
                    *current_highest = voltage;
                    registered_indicies.push(index);
                    last_added_index = index;
                }
            }
            // println!(">>>>>>>>>>>>>>>>>>");

        }
        println!("numbers: {:?}", numbers);
        let mut highest_joltage = 0;
        for (index, value) in numbers.iter().rev().enumerate() {
            highest_joltage += 10u64.pow(index as u32) * (*value as u64);
        }
        bank_joltages.push(highest_joltage);
    }
    println!("======================================");
    println!("bank_joltages: {:?}", bank_joltages);
    // bank_joltages.sort_unstable();
    // bank_joltages.dedup();
    println!("sum: {:?}", bank_joltages.iter().sum::<u64>());
}
