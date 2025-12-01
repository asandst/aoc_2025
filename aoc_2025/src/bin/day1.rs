fn main() {
    let lines = include_str!("../../input_day1")
        .lines();

    let mut absolute_dir = 50;

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let (dir, amount) = line.split_at(1);
        let amount = amount.parse::<i32>().unwrap();

        let dir_as_mul = match dir {
            "L" => -1,
            "R" => 1,
            &_ => panic!("invalid input")
        };

        if absolute_dir == 0 && dir_as_mul == -1 {
            part2 -= 1;
        }

        absolute_dir = absolute_dir + dir_as_mul * amount;

        while absolute_dir < 0 {
            absolute_dir += 100;
            part2 +=1;
        }

        while absolute_dir > 99 {
            absolute_dir -= 100;
            part2 +=1;
        }

        if absolute_dir == 0 && dir_as_mul == -1 {
            part2 += 1;
        }

        if absolute_dir == 0 {
            part1 += 1;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);


}