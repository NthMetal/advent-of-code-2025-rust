use std::collections::HashSet;
use std::f32;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use glam::{UVec3, Vec3};

pub const MAX_CONNECTIONS: usize = 500;

fn main() {
    let input_path = "assets/day_8_input.txt";
    let Ok(file) = File::open(input_path) else {
        println!("Error reading file: {:?}", input_path);
        return;
    };
    let reader = BufReader::new(file);
    let mut points = vec![];
    for (line_index, read_line) in reader.lines().enumerate() {
        let Ok(line) = read_line else {
            println!("Error reading line");
            continue;
        };
        let mut point = UVec3::ZERO;
        let point_strs = line.split(",");
        for (index, point_str) in point_strs.enumerate() {
            match index {
                0 => point.x = point_str.parse().expect("parsable number"),
                1 => point.y = point_str.parse().expect("parsable number"),
                2 => point.z = point_str.parse().expect("parsable number"),
                _ => panic!("3 points per line")
            }
        }
        
        points.push(point);
    }
    let mut circuits = points.iter().map(|point| vec![*point]).collect::<Vec<Vec<UVec3>>>();
    let mut pairs = HashSet::new();
    let mut pairs_made = 0;
    for _i in 0..(10) {
        // find the shortest pair
        // n^2 search?
        let mut shortest_distance = f32::MAX;
        let mut point_a_circuit_index = 0;
        let mut point_b_circuit_index = 0;
        let mut point_a_item = UVec3::ZERO;
        let mut point_b_item = UVec3::ZERO;
        // let mut visited_pairs = vec![];
        let mut manual_mapped_points: Vec<(usize, usize, UVec3)> = vec![];
        for (cindex, circuit) in circuits.iter().enumerate() {
            for (pindex, point) in circuit.iter().enumerate() {
                manual_mapped_points.push((cindex, pindex, *point));
            }
        }
        // let mapped_points = circuits.iter()
        //     .enumerate()
        //     .flat_map(|(circuit_index, circuit)| circuit.iter()
        //         .map(|point| (circuit_index, *point)).collect::<Vec<(usize, Vec3)>>());
        // println!("manual_mapped_points: {:?} {:?}", manual_mapped_points.len(), manual_mapped_points);
        for (circuit_a_index, _a_index, point_a) in &manual_mapped_points {
            for (circuit_b_index, _b_index, point_b) in &manual_mapped_points {
                let visited_pair_a = (*point_a, *point_b);
                let visited_pair_b = (*point_b, *point_a);
                // let visited = pairs.contains(&visited_pair_a) || pairs.contains(&visited_pair_b);
                if point_a != point_b {
                    let distance = point_a.as_vec3().distance(point_b.as_vec3());
                    if distance <= shortest_distance {
                        shortest_distance = distance;
                        point_a_circuit_index = *circuit_a_index;
                        point_b_circuit_index = *circuit_b_index;
                        point_a_item = *point_a;
                        point_b_item = *point_b;

                        // println!("point_a_circuit_index: {:?} point_b_circuit_index: {:?}", point_a_circuit_index, point_b_circuit_index);
                        // visited_pairs.push((point_a, point_b));
                    }
                }
            }
        }
        println!("_: {:?} -- {:?} | {:?} -> {:?}", _i, pairs.len(), pairs_made, (point_a_item, point_b_item));
        
        if point_a_circuit_index == point_b_circuit_index {
            pairs_made += 1;
            pairs.insert((point_a_item, point_b_item));
            continue;
        }

        let mut circuit_a = circuits.remove(point_a_circuit_index);
        let circuit_b = if point_a_circuit_index > point_b_circuit_index {
            circuits.remove(point_b_circuit_index)
        } else {
            circuits.remove(point_b_circuit_index - 1)
        };
        circuit_a.extend(circuit_b);
        circuits.push(circuit_a);
        pairs_made += 1;
    }
    
    circuits.sort_by_key(|inner| std::cmp::Reverse(inner.len()));
    let mut count = 0;
    let mut total = 1;
    for circuit in circuits {
        println!("circuit: {:?}", circuit.len());
        if count < 3 {
            total *= circuit.len();
            count += 1;
        }
    }

    println!("total: {:?}", total);
}
