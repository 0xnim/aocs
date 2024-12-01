use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part_one(input: &str) -> u32 {
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

    sum as u32
}

#[aoc(day1, part2)]
pub fn part_two(input: &str) -> u32 {
    let mut first_column: Vec<i32> = Vec::new();
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        let first = split_line.next().unwrap().parse::<i32>().unwrap();
        let second = split_line.next().unwrap().parse::<i32>().unwrap();
        first_column.push(first);

        *frequency_map.entry(second).or_insert(0) += 1;
    }

    let mut total_score: i32 = 0;
    for &key in &first_column {
        if let Some(&count) = frequency_map.get(&key) {
            total_score += key * count;
        }
    }

    total_score as u32
}
