
fn main() {
    let lines = include_str!("../../input_day3").lines();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let parsed = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let mut index = 0;
        for i in 0..parsed.len() - 1 {
            if parsed[i] > parsed[index] {
                index = i;
            }
        }

        let mut index2 = index + 1;
        for i in index + 1..parsed.len() {
            if parsed[i] > parsed[index2] {
                index2 = i;
            }
        }

        let jolt = parsed[index] * 10 + parsed[index2];
        part1 += jolt;

        let mut indexes = [0usize; 12];

        for i in 0..12 {
            let start_index = if i == 0 { 0 } else { indexes[i - 1] + 1 };
            let mut index = start_index;

            for j in start_index..parsed.len() - (11 - i) {
                if parsed[j] > parsed[index] {
                    index = j;
                }
            }
            indexes[i] = index;
        }

        let mut jolt = 0;
        for i in 0..12 {
            jolt += parsed[indexes[i]] * 10u64.pow((11 - i) as u32)
        }
        part2 += jolt;
    }

    println!("{}", part1);
    println!("{}", part2);
}
