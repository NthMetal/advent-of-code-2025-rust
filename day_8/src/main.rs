use std::collections::HashSet;
use std::f32;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use glam::{UVec3, Vec3};

// pub const MAX_CONNECTIONS: usize = 4499;
pub const MAX_CONNECTIONS: usize = 4500;

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
    let mut pair_distances = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let a = &points[i];
            let b = &points[j];

            let distance = a.as_vec3().distance(b.as_vec3());
            pair_distances.push((*a, *b, distance));
        }
    }
    pair_distances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    let mut processed_pair_distances = 0;
    let mut circuits: Vec<Vec<UVec3>> = vec![];
    let mut last_a = UVec3::ZERO;
    let mut last_b = UVec3::ZERO;
    while pair_distances.len() > 0 && processed_pair_distances < MAX_CONNECTIONS {
        let (shortest_a, shortest_b, _shortest_distance) = pair_distances.pop().expect("shortest to exist");
        
        let mut a_exists = None;
        let mut b_exists = None;
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&shortest_a) {
                a_exists = Some(index);
            }
            if circuit.contains(&shortest_b) {
                b_exists = Some(index);
            }
        }
        last_a = shortest_a;
        last_b = shortest_b;
        match (a_exists, b_exists) {
            (None, None) => {
                circuits.push(vec![shortest_a, shortest_b]);
            },
            (None, Some(i)) => {
                circuits[i].push(shortest_a);
            },
            (Some(i), None) => {
                circuits[i].push(shortest_b);
            },
            (Some(i), Some(j)) => {
                if i != j {
                    let combine_me = circuits.remove(j);
                    if i > j {
                        circuits[i - 1].extend(combine_me);
                    } else {
                        circuits[i].extend(combine_me);
                    }
                }
            },
        }
        processed_pair_distances += 1;
    }
    println!("last_a: {:?}", last_a);
    println!("last_b: {:?}", last_b);
    println!("x*x: {:?}", last_a.x * last_b.x);
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
