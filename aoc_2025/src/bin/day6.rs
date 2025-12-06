fn main() {
    let input = include_str!("../../input_day6");

    let mut problems = Vec::new();
    let lines = input.lines();
    let mut operators = Vec::new();

    for (i, line) in lines.enumerate() {
        if !line.contains("*") {
            problems.push(Vec::new());
            for number in line.split_whitespace() {
                let number = number.parse::<i64>().unwrap();
                problems[i].push(number);
            }
        } else {
            for operator in line.split_whitespace() {
                operators.push(operator);
            }
        }
    }

    let mut part1 = 0;

    for i in 0..operators.len() {
        if operators[i] == "+" {
            let mut res = 0;
            for j in 0..problems.len() {
                res += problems[j][i];
            }
            part1 += res;
        } else if operators[i] == "*" {
            let mut res = 1;
            for j in 0..problems.len() {
                res *= problems[j][i];
            }
            part1 += res;
        }
    }

    println!("part1: {}", part1);

    let mut problems = Vec::new();

    for line in input.lines() {
        problems.push(line.chars().collect::<Vec<char>>());
    }

    let max_index = problems.iter().map(|line| line.len()).max().unwrap();
    let mut part2 = 0;
    let mut numbers = Vec::new();
    for i in (0..max_index).rev() {
        let mut number = String::from("");
        for j in 0..problems.len() - 1 {
            let c = problems[j].get(i);
            if c.is_some() {
                number.push(*c.unwrap());
            }
        }

        let number = number.trim();
        if number.is_empty() {
            numbers.clear();
            continue;
        }
        numbers.push(number.parse::<i64>().unwrap());
        part2 += match problems[problems.len() - 1].get(i) {
            Some('+') => numbers.iter().sum(),
            Some('*') => numbers.iter().product(),
            _ => 0,
        };
    }

    println!("part2: {}", part2);
}
