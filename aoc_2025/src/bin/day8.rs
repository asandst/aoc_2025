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

        //println!(
        //    "Connect {:?} {:?}",
        //    input[shortest_pair_index.0], input[shortest_pair_index.1]
        //);
        connected.insert(shortest_pair_index);
        let last_circuit_number = circuit_number;

        let mut c = 0;
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
    println!("{:?}", circuit);
    let grouped = circuit.clone().into_iter().counts_by(|e| e.1);
    let res = grouped.values().k_largest(3);
    let debug = circuit.values().sorted();
    println!("{:?}", res);
    println!("{:?}", res.product::<usize>());
    println!("{:?}", debug);
}
fn dist((x1, y1, z1): &(i64, i64, i64), (x2, y2, z2): &(i64, i64, i64)) -> i64 {
    let p = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
    (p as f64).sqrt() as i64
}
