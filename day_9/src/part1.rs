use std::collections::HashSet;
use std::f32;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use glam::UVec2;

fn main() {
    let input_path = "assets/day_9_input.txt";
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
        let mut point = UVec2::ZERO;
        let point_strs = line.split(",");
        for (index, point_str) in point_strs.enumerate() {
            match index {
                0 => point.x = point_str.parse().expect("parsable number"),
                1 => point.y = point_str.parse().expect("parsable number"),
                _ => panic!("2 points per line")
            }
        }
        
        points.push(point);
    }
    let mut pair_areas = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let &a = &points[i];
            let &b = &points[j];

            let width = (a.x.max(b.x) - a.x.min(b.x)).max(1) + 1;
            let height = (a.y.max(b.y) - a.y.min(b.y)).max(1) + 1;
            let area = (width as u64) * (height as u64);
            pair_areas.push((a, b, area));
        }
    }
    pair_areas.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    println!("pair_distances: {:?}", pair_areas[0]);
}
