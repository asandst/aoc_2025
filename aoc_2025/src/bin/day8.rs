use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input_day8");

    let input = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
        .collect::<Vec<_>>();
    println!("Part1 {}", part1(input.clone()));
    println!("Part2 {}", part2(input));
}

fn part1(input: Vec<(i64, i64, i64)>) -> i64 {
    let mut connected = HashSet::<(usize, usize)>::new();
    let mut circuit = HashMap::<usize, usize>::new();
    let mut circuit_number: usize = 0;

    for _connections in 0..=1000 {
        let mut shortest_dist = i64::MAX;
        let mut shortest_pair_index = (0, 0);
        for i in 0..input.len() {
            for j in i + 1..input.len() {
                let pos1 = input[i];
                let pos2 = input[j];
                if dist(&pos1, &pos2) < shortest_dist && !connected.contains(&(i, j)) {
                    shortest_dist = dist(&pos1, &pos2);
                    shortest_pair_index = (i, j);
                }
            }
        }
        connected.insert(shortest_pair_index);
        let last_circuit_number = circuit_number;

        if circuit.contains_key(&shortest_pair_index.0)
            && circuit.contains_key(&shortest_pair_index.1)
        {
            let c1 = circuit[&shortest_pair_index.0];
            let c2 = circuit[&shortest_pair_index.1];
            for value in circuit.values_mut() {
                if *value == c2 {
                    *value = c1;
                }
            }
        }
        let c;
        if circuit.contains_key(&shortest_pair_index.0) {
            c = *circuit.get(&shortest_pair_index.0).unwrap();
        } else if circuit.contains_key(&shortest_pair_index.1) {
            c = *circuit.get(&shortest_pair_index.1).unwrap();
        } else {
            c = last_circuit_number;
            circuit_number += 1;
        }

        circuit.insert(shortest_pair_index.0, c);
        circuit.insert(shortest_pair_index.1, c);
    }

    let grouped = circuit.clone().into_iter().counts_by(|e| e.1);
    let res = grouped.values().k_largest(3);
    res.product::<usize>() as i64
}

fn part2(input: Vec<(i64, i64, i64)>) -> i64 {
    let mut connected = HashSet::<(usize, usize)>::new();
    let mut circuit = HashMap::<usize, usize>::new();
    let mut circuit_number: usize = 0;

    loop {
        let mut shortest_dist = i64::MAX;
        let mut shortest_pair_index = (0, 0);
        for i in 0..input.len() {
            for j in i + 1..input.len() {
                let pos1 = input[i];
                let pos2 = input[j];
                if dist(&pos1, &pos2) < shortest_dist && !connected.contains(&(i, j)) {
                    shortest_dist = dist(&pos1, &pos2);
                    shortest_pair_index = (i, j);
                }
            }
        }

        connected.insert(shortest_pair_index);
        let last_circuit_number = circuit_number;

        if circuit.contains_key(&shortest_pair_index.0)
            && circuit.contains_key(&shortest_pair_index.1)
        {
            let c1 = circuit[&shortest_pair_index.0];
            let c2 = circuit[&shortest_pair_index.1];
            for value in circuit.values_mut() {
                if *value == c2 {
                    *value = c1;
                }
            }
        }

        let c;
        if circuit.contains_key(&shortest_pair_index.0) {
            c = *circuit.get(&shortest_pair_index.0).unwrap();
        } else if circuit.contains_key(&shortest_pair_index.1) {
            c = *circuit.get(&shortest_pair_index.1).unwrap();
        } else {
            c = last_circuit_number;
            circuit_number += 1;
        }

        circuit.insert(shortest_pair_index.0, c);
        circuit.insert(shortest_pair_index.1, c);

        let grouped = circuit.clone().into_iter().counts_by(|e| e.1);
        let res = grouped.values().k_largest(1).collect::<Vec<_>>();
        if *res[0] == 1000 {
            let pos1 = input[shortest_pair_index.0];
            let pos2 = input[shortest_pair_index.1];
            return pos1.0 * pos2.0;
        }
    }
}

fn dist((x1, y1, z1): &(i64, i64, i64), (x2, y2, z2): &(i64, i64, i64)) -> i64 {
    let p = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
    (p as f64).sqrt() as i64
}
