fn main() {
    let mut input_separator_found = false;

    let mut fresh_ranges = Vec::new();
    let mut ingredients = Vec::new();

    include_str!("../../input_day5").lines().for_each(|line| {
        if line.is_empty() {
            input_separator_found = true;
            return;
        }

        if !input_separator_found {
            let (start, end) = line.split_once("-").unwrap();

            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            fresh_ranges.push((start, end));
        } else {
            let id = line.parse::<i64>().unwrap();
            ingredients.push(id);
        }
    });

    let mut part1 = 0;

    for id in ingredients {
        for range in &fresh_ranges {
            if id >= range.0 && id <= range.1 {
                part1 += 1;
                break;
            }
        }
    }
    println!("part1: {}", part1);

    let mut part2 = 0;

    for i in 0..fresh_ranges.len() {
        println!("processing {:?}", fresh_ranges[i]);
        let mut ranges = vec![(fresh_ranges[i]); 1];
        for j in i + 1..fresh_ranges.len() {
            let comp_range = fresh_ranges[j];
            println!("Comparing to {:?}", comp_range);
            let mut added = vec![(fresh_ranges[i]); 0];
            for range in &mut ranges {
                if comp_range.0 <= range.0 && comp_range.1 >= range.0 && comp_range.1 < range.1 {
                    range.0 = comp_range.1 + 1;
                    println!("Increased start to {:?} because {:?}", range, comp_range);
                } else if comp_range.0 <= range.0 && comp_range.1 >= range.1 {
                    println!("Remove range {:?} because {:?}", range, comp_range);
                    range.0 = -1;
                    range.1 = -1;
                } else if comp_range.0 > range.0
                    && comp_range.0 <= range.1
                    && comp_range.1 >= range.1
                {
                    range.1 = comp_range.0 - 1;
                    println!("Lowered end to {:?} because {:?}", range, comp_range);
                } else if comp_range.0 > range.0 && comp_range.1 < range.1 {
                    added.push((range.0, comp_range.0 - 1));
                    added.push((comp_range.1 + 1, range.1));
                    println!("Split range {:?} because {:?}", range, comp_range);
                    range.0 = -1;
                    range.1 = -1;
                }
            }
            ranges.append(&mut added);
        }

        for r in ranges {
            println!("Sum {:?}", r);
            part2 += r.1 - r.0;
            if r.1 != -1 {
                part2 += 1;
            }
        }
    }
    println!("part2: {}", part2);
}
