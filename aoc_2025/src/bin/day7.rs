use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input_day7");

    let mut beam_indexes = HashSet::new();
    let mut part1 = 0;

    let mut beam_indexes_with_hist = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if i % 2 == 1 {
            continue;
        }

        if beam_indexes.is_empty() {
            let start_index = line.find("S").unwrap();
            beam_indexes.insert(start_index);
            beam_indexes_with_hist.insert(start_index, 1);
        } else {
            for (x, c) in line.chars().enumerate() {
                if c == '^' && beam_indexes.contains(&x) {
                    beam_indexes.remove(&x);
                    beam_indexes.insert(x - 1);
                    beam_indexes.insert(x + 1);

                    let x_value = beam_indexes_with_hist[&x];

                    beam_indexes_with_hist
                        .entry(x + 1)
                        .and_modify(|e| *e += x_value)
                        .or_insert(x_value);

                    beam_indexes_with_hist
                        .entry(x - 1)
                        .and_modify(|e| *e += x_value)
                        .or_insert(x_value);

                    beam_indexes_with_hist.remove(&x);
                    part1 += 1;
                }
            }
        }
    }

    let part2 = beam_indexes_with_hist.values().sum::<i64>();

    println!("{}", part1);
    println!("{}", part2);
}
