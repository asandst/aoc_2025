use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input_day7");
    let mut beam_history = HashMap::new();
    let mut part1 = 0;

    for line in input.lines() {
        if beam_history.is_empty() {
            beam_history.insert(line.find("S").unwrap(), 1);
        } else {
            for (x, c) in line.chars().enumerate() {
                if c == '^' && beam_history.contains_key(&x) {
                    [x + 1, x - 1].iter().for_each(|key| {
                        let value = beam_history[&x];
                        beam_history
                            .entry(*key)
                            .and_modify(|e| *e += value)
                            .or_insert(value);
                    });
                    beam_history.remove(&x);
                    part1 += 1;
                }
            }
        }
    }

    let part2 = beam_history.values().sum::<i64>();

    println!("{}", part1);
    println!("{}", part2);
}
