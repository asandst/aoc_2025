use std::cmp::PartialEq;

fn main() {
    let map = include_str!("../../input_day4")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => Tile::Paper,
                    '.' => Tile::Empty,
                    _ => todo!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1_map = map.clone();
    let part1 = take_rolls(&mut part1_map, false);

    let mut part2 = 0;
    let mut part2_map = map.clone();

    let mut rolls_taken = -1;
    while rolls_taken != 0 {
        rolls_taken = take_rolls(&mut part2_map, true);
        part2 += rolls_taken;
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn take_rolls(map: &mut Vec<Vec<Tile>>, remove_roll: bool) -> i32 {
    let mut papers_taken = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Tile::Empty {
                continue;
            }

            let mut sum = 0;
            for x in -1i32..=1 {
                for y in -1i32..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    match map
                        .get((i as i32 + x) as usize)
                        .map(|row| row.get((j as i32 + y) as usize))
                    {
                        Some(Some(Tile::Paper)) => sum += 1,
                        _ => {}
                    }
                }
            }

            if sum < 4 {
                papers_taken += 1;
                if remove_roll {
                    map[i][j] = Tile::Empty;
                }
            }
        }
    }
    papers_taken
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tile {
    Paper,
    Empty,
}
