use std::collections::{HashMap, HashSet};
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

    // let mut beam_indicies = HashSet::new();
    let mut beam_universes: HashMap<usize, u64> = HashMap::new();
    for (line_index, read_line) in reader.lines().enumerate() {
        let Ok(line) = read_line else {
            println!("Error reading line");
            continue;
        };
        let line_length = line.len();
        for (index, char) in line.chars().enumerate() {
            match char.to_string().as_str() {
                "S" => { 
                    beam_universes.insert(index, 1); 
                },
                "^" => {
                    if let Some(count) = beam_universes.remove(&index) {
                        splits += 1;

                        let left_index = index.saturating_sub(1);
                        let right_index = index + 1;

                        if index > 0 {
                            *beam_universes.entry(left_index).or_insert(0) += count;
                        }
                        if index < line_length - 1 {
                            *beam_universes.entry(right_index).or_insert(0) += count;
                        }
                    }
                },
                "." => {},
                _ => panic!()
            }
        }
    }

    // count unique universes
    println!("beam_universes: {:?}", beam_universes);
    let row_universe_count = beam_universes.values().sum::<u64>();
    // let result = count_trailing_duplicates(&beam_universes);
    // let row_universe_count = beam_universes.values().map(|(total_possible_paths, ..)| total_possible_paths).sum::<u32>();
    // for (beam_id, universes_a) in &beam_universes {
    //     let mut universes = universes_a.clone();
    //     // println!("Beam: {:?} Universes: {:?}", beam_id, universes);
    //     let Some(last_universe) = universes.pop() else {
    //         continue;
    //     };
    //     universes.push(last_universe); // eh easier than borrowing properly
    //     println!("Beam: {:?} Last Universe: {:?}", beam_id, last_universe);
    //     row_universe_count += universes.iter().filter(|item| !counted_universes.contains(*item)).count();
    //     counted_universes.insert(last_universe);
    // }
    println!("row_universe_count: {:?}", row_universe_count);

    println!("splits: {:?}", splits);
}

// .......S.......
// .......|.......
// ......|^.......
// ......|........
// .....|^.^......
// 4


// .......S.......
// .......|.......
// ......1^2...... 2 = 1,2
// ......|.|...... 2^1
// .....3^4^5..... 4 = 13,14,24,25
// .....|.|.|..... 2^2
// ....6^7^8^9.... 8 = 136,137,147,148,247,248,258,259
// ....|.|.|.|.... 2^3
// ...a^b^cde^f... 13 = 136a,136b,137b,137c,147b,147c,148d,247b,247c,248d,258d,259e,259f
// ...|.|.|||.|... 2^4 = 16 - 3
// ..|^|^|||^|^|.. _ = 
// ..|.|.|||.|.|..
// .|^|||^||.||^|.
// .|.|||.||.||.|.
// |^|^|^|^|^|||^|
// |.|.|.|.|.|||.|

//     1                1
//    1 1              1 1
//   1 2 1            1 2 1
//  1 3 3 1          1 3 3 1
// 1 4 6 4 1        1 4 3 3 1 1
// 


// .......S.......
// .......|.......
// ......a^b...... 2 = a,b
// ......|.|...... 2^1
// .....c^d^e..... 4 = ac,ad,bd,be

// S=7
// a=37
// b=39
// c=66
// d=68
// e=70

//  0>".......S......."< {7: []}
// 15>".......|......."< {7: []}
// 30>"......a^b......"< {8: [b], 6: [a]}
// 45>"..............."< {8: [b], 6: [a]}
// 60>".....c^d^e....."< {5: [a, c], 7: [a, d, d], 9: [b, e]}
// 75>"..............."< {5: [a, c], 7: [a, d, d], 9: [b, e]}
// 90>"....f^g^h^i...."< {6: [a, c, g, g], 4: [a, c, f], 8: [a, d, d, h, h], 10: [b, e, i]}

// /**
//  * acf
//  * acg
//  * adg
//  * adh
//  * bdg
//  * bdh
//  * beh
//  * bei
//  */

