use std::fs::File;
use std::io::{prelude::*, BufReader};

use geo::{Contains, ContainsProperly, Coord, Intersects, Line, LineString, Point, Polygon, Rect, point, polygon};
use glam::{IVec2, UVec2};

fn main() {
    let input_path = "../assets/day_9_input.txt";
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);
    let mut points = vec![];
    for (line_index, read_line) in reader.lines().enumerate() {
        let Ok(line) = read_line else {
            println!("Error reading line");
            continue;
        };
        let mut point = IVec2::ZERO;
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
    if points.len() < 4 { panic!("what do you want? a three sided square?"); }
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
    pair_areas.retain(|(current_a, current_b, _)| {
        is_valid_pair(&points, (*current_a, *current_b))
    });
    pair_areas.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("pair_distances: {:?}", pair_areas[0]);
}

pub fn is_valid_pair(points: &Vec<IVec2>, rect: (IVec2, IVec2)) -> bool {
    let mut edges = vec![];
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        edges.push((a, b));
    }
    let mut is_valid = true;
    for edge in edges {
            let (rect_left, rect_right) = (rect.0.x.min(rect.1.x), rect.0.x.max(rect.1.x));
            let (rect_bottom, rect_top) = (rect.0.y.min(rect.1.y), rect.0.y.max(rect.1.y));

            let (edge_left, edge_right) = (edge.0.x.min(edge.1.x), edge.0.x.max(edge.1.x));
            let (edge_bottom, edge_top) = (edge.0.y.min(edge.1.y), edge.0.y.max(edge.1.y));

            if rect_right > edge_left &&
                rect_left < edge_right &&
                rect_top > edge_bottom &&
                rect_bottom < edge_top {
                    is_valid = false;
                    break;
            }
    }
    is_valid
}