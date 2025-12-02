

fn main() {
    let data = include_str!("../../input_day2");

    let mut part1 = 0;
    let mut part2 = 0;

    for range_str in data.split(",") {
        let (lower, upper) = range_str.split_once("-").unwrap();

        let lower_int = lower.parse::<i64>().unwrap();
        let upper_int = upper.parse::<i64>().unwrap();

        for i in lower_int..=upper_int {
            let i_str = i.to_string();
            let (start, end) = i_str.split_at(i_str.len()/2);

            if start == end {
                part1 += i;
            }
        }

        for i in lower_int..=upper_int {
            let i_str = i.to_string();

            for size in 1..=i_str.len()/2 {
                if (i_str.len() / size) * size != i_str.len(){
                    continue;
                }
                let subs = i_str.as_bytes()
                    .chunks(size)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();

                let first = subs[0];
                if subs.iter().all(|&item| item == first) {
                    part2 += i;
                    break;
                }
            }

        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}