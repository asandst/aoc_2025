

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

            for size in (1..=i_str.len()/2).rev() {
                if i_str.len()%size != 0{
                    continue;
                }
                
                let parts = i_str.as_bytes()
                    .chunks(size)
                    .collect::<Vec<&[u8]>>();

                let first = parts[0];
                if parts.iter().all(|&item| item == first) {
                    part2 += i;

                    if i_str.len()%2 == 0 && i_str.len()/2 == size {
                        part1 += i;
                    }
                    break;
                }
            }
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}