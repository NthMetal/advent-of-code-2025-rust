use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "assets/day_7_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);
    let mut splits = 0;

    let mut beam_indicies = HashSet::new();
    for read_line in reader.lines() {
        let Ok(line) = read_line else {
            println!("Error reading line");
            continue;
        };
        for (index, char) in line.chars().enumerate() {
            match char.to_string().as_str() {
                "S" => { beam_indicies.insert(index); },
                "^" => {
                    if beam_indicies.contains(&index) {
                        beam_indicies.remove(&index);
                        splits += 1;
                    }
                    // let pre_length = beam_indicies.len();
                    beam_indicies.insert(index + 1);
                    beam_indicies.insert(index.saturating_sub(1));
                    // let post_length = beam_indicies.len();
                    // splits += if post_length - pre_length > 0 { 1 } else { 0 };
                },
                "." => {},
                _ => panic!()
            }
        }
    }

    println!("splits: {:?}", splits);
}


// .|.|||.||.||.|.
// |^|^|^|^|^|||^|
// |.|.|.|.|.|||.|

// .....|^|^|.....
// .....|.|.|.....
// ....|^|^|^|....2 