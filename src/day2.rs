use rayon::prelude::*; // Importing rayon's parallel iterator

/// Represents a single report of levels.
#[derive(Debug)]
struct Report {
    levels: Vec<u8>, // Using u8 since level values are between 0 and 100
}

impl Report {
    /// Constructs a new Report from a string of levels.
    fn new(line: &str) -> Self {
        let levels = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u8>().ok()) // Using u8 instead of i32 for efficiency
            .collect();
        Self { levels }
    }

    /// Checks if the report is safe.
    fn is_safe(&self) -> bool {
        let levels = &self.levels;
        if levels.len() < 2 {
            return false; // Reports with fewer than 2 levels cannot be checked.
        }

        // Parallelizing adjacent pair comparisons without using windows or zip
        (0..levels.len() - 1)
            .into_par_iter() // Parallelize iteration over indices
            .all(|i| {
                let diff = (levels[i + 1] as i8) - (levels[i] as i8);
                diff.abs() <= 3 && diff != 0 // Return true if the difference is valid
            })
    }
}

#[aoc(day2, part1)]
pub fn part_one(input: &str) -> u32 {
    input
        .par_lines() // Use rayon's parallel iterator for lines
        .map(Report::new) // Convert each line to a Report
        .filter(|report| report.is_safe()) // Filter out unsafe reports
        .count() as u32 // Count the number of safe reports
}



/*#[aoc(day2, part2)]
pub fn part_two(input: &str) -> u32 {
    0
    /// TODO: redo
}*/
