use geo::Line;
use geo::line_intersection::line_intersection;
use geo_types::coord;
use itertools::Itertools;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../input_day9");

    let input = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect_vec();

    let mut size = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let area = area(input[i], input[j]);

            if area > size {
                size = area;
            }
        }
    }

    println!("Part1: {}", size);

    let mut hull2 = Vec::new();

    for i in 0..input.len() - 1 {
        let start = input[i];
        let end = input[i + 1];
        let line_1 = Line::new(
            coord! {x: start.0 as f64, y:start.1 as f64},
            coord! {x: end.0 as f64, y:end.1 as f64},
        );
        hull2.push(line_1);
    }
    let line_1 = Line::new(
        coord! {x: input[input.len() - 1].0 as f64, y:input[input.len() - 1].1 as f64},
        coord! {x: input[0].0 as f64, y:input[0].1 as f64},
    );
    hull2.push(line_1);

    let mut size = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let p1 = (min(input[i].0, input[j].0), min(input[i].1, input[j].1));
            let p2 = (max(input[i].0, input[j].0), max(input[i].1, input[j].1));
            let p1 = (p1.0 as f64 + 0.1, p1.1 as f64 + 0.1);
            let p2 = (p2.0 as f64 - 0.1, p2.1 as f64 - 0.1);
            let p3 = (p1.0, p2.1);
            let p4 = (p2.0, p1.1);
            let line_1 = Line::new(coord! {x: p1.0, y:p1.1}, coord! {x: p3.0, y:p3.1});
            let line_2 = Line::new(coord! {x: p3.0, y:p3.1}, coord! {x: p2.0, y:p2.1});
            let line_3 = Line::new(coord! {x: p2.0, y:p2.1}, coord! {x: p4.0, y:p4.1});
            let line_4 = Line::new(coord! {x: p4.0, y:p4.1}, coord! {x: p1.0, y:p1.1});

            let area = area(input[i], input[j]);
            if area > size
                && line_in_hull(&hull2, line_1)
                && line_in_hull(&hull2, line_2)
                && line_in_hull(&hull2, line_3)
                && line_in_hull(&hull2, line_4)
            {
                size = area;
            }
        }
    }

    println!("Part2: {}", size);
}

fn area(start: (i64, i64), end: (i64, i64)) -> i64 {
    let x = (start.0 - end.0).abs() + 1;
    let y = (start.1 - end.1).abs() + 1;
    x * y
}

fn line_in_hull(hull: &Vec<Line<f64>>, line: Line<f64>) -> bool {
    for h in hull {
        if line_intersection(*h, line).is_some() {
            return false;
        }
    }
    true
}
