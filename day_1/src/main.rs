use std::fs::File;
use std::io::{prelude::*, BufReader};

const _TEST_ROTATIONS: [&str; 10] = [
    "L68",
    "L30",
    "R48",
    "L05",
    "R60",
    "L55",
    "L01",
    "L99",
    "R14",
    "L82",
    // "R1000",
    // "L1000",
];


fn main() {
    let input_path = "assets/day_1_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);

    let mut total_times_at_zero = 0;
    let mut current_rotation: i32 = 50;

    for (line_index, line_result) in reader.lines().enumerate() {
        let Ok(rotation) = line_result else {
            println!("Error reading line: {:?}", line_index);
            return;
        };
        count_zero_passes(rotation, &mut total_times_at_zero, &mut current_rotation);
    }
    // for rotation in _TEST_ROTATIONS {
    //     count_zero_passes(rotation.to_owned(), &mut total_times_at_zero, &mut current_rotation);
    // }

    println!("Total times at zero: {:?}", total_times_at_zero);
}

fn count_zero_clicks(rotation: String, total_times_at_zero: &mut i32, current_rotation: &mut i32) {
    let (direction_str, amount_str) = rotation.split_at(1);
    let direction = match direction_str {
        "L" => -1i32,
        "R" => 1i32,
        _ => 1
    };
    let Ok(amount) = amount_str.parse::<u32>() else {
        println!("Error parsing: {:?}", amount_str);
        return;
    };
    *current_rotation += direction * amount as i32;
    *current_rotation = *current_rotation % 100;
    let signum = current_rotation.signum();
    match signum {
        -1 => *current_rotation = 100 + *current_rotation,
        0 => *total_times_at_zero += 1,
        _ => {}
    }
    println!("The dial is rotated {:?} ({:?}, {:?}) to point at {:?}", rotation, direction, amount, *current_rotation);
}

fn count_zero_passes(rotation_str: String, total_times_at_zero: &mut i32, current_rotation: &mut i32) {
    let (direction_str, amount_str) = rotation_str.split_at(1);
    let direction = match direction_str {
        "L" => -1i32,
        "R" => 1i32,
        _ => 1
    };
    let Ok(amount) = amount_str.parse::<u32>() else {
        println!("Error parsing: {:?}", amount_str);
        return;
    };
    let rotation = direction * amount as i32;
    let new_rotation = *current_rotation + rotation;
    let mut zero_passes = (new_rotation / 100).abs();

    if *current_rotation != 0 && new_rotation <= 0 {
        zero_passes += 1;
    }
    *current_rotation = *current_rotation % 100;
    *current_rotation = new_rotation.rem_euclid(100);
    *total_times_at_zero += zero_passes;
    
    println!("The dial is rotated {:?} ({:?}, {:?}) to point at {:?}", rotation_str, direction, amount, *current_rotation);
}