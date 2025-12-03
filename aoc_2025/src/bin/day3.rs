fn main() {
    let lines = include_str!("../../input_day3").lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>()
    });

    let mut part1 = 0;
    let mut part2 = 0;

    for bank in lines {
        part1 += calc_jolt(&bank, 2);
        part2 += calc_jolt(&bank, 12);
    }

    println!("{}", part1);
    println!("{}", part2);
}

fn calc_jolt(bank: &Vec<u64>, number_of_batteries: usize) -> u64 {
    let mut indexes = vec![0usize; number_of_batteries];

    for i in 0..number_of_batteries {
        let mut index = if i == 0 { 0 } else { indexes[i - 1] + 1 };
        for j in index..bank.len() - (number_of_batteries - 1 - i) {
            if bank[j] > bank[index] {
                index = j;
            }
        }
        indexes[i] = index;
    }

    (0..number_of_batteries)
        .map(|i| bank[indexes[i]] * 10u64.pow((number_of_batteries - 1 - i) as u32))
        .sum::<u64>()
}
