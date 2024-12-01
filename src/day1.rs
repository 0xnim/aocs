use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // TODO: Implement part one

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        let first = split_line.next().unwrap().parse::<i32>().unwrap();
        let second = split_line.next().unwrap().parse::<i32>().unwrap();
        first_column.push(first);
        second_column.push(second);
    }

    first_column.sort_unstable();
    second_column.sort_unstable();

    let mut sum: i32 = 0;
    for i in 0..first_column.len() {
        sum += (first_column[i] - second_column[i]).abs() as i32;
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // TODO: Implement part one

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        let first = split_line.next().unwrap().parse::<i32>().unwrap();
        let second = split_line.next().unwrap().parse::<i32>().unwrap();
        first_column.push(first);
        second_column.push(second);
    }

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &value in &second_column {
        *frequency_map.entry(value).or_insert(0) += 1;
    }

    let mut total_score: i32 = 0;
    for &key in &first_column {
        if let Some(&count) = frequency_map.get(&key) {
            total_score += key * count;
        }
    }

    Some(total_score as u32)
}