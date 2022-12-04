use std::str::FromStr;

use crate::utils;

/// Food for the elves.
struct Food {
    food: u32,
}

impl FromStr for Food {
    type Err = String;

    /// Parse food from a string where each `food` is separated by '\n'.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .trim()
            .split('\n')
            .into_iter()
            .map(|l| l.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum();

        Ok(Food { food: x })
    }
}

impl From<Food> for u32 {
    fn from(f: Food) -> Self {
        f.food
    }
}

impl AsRef<u32> for Food {
    fn as_ref(&self) -> &u32 {
        &self.food
    }
}

pub fn solve_part1() {
    let input = utils::io::read(1);
    let max: u32 = input
        .trim()
        .split("\n\n")
        .map(|lines| lines.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("Part 1 :: max value: {}", max);
}

pub fn solve_part2() {
    let input = utils::io::read(1);
    let mut sums: Vec<u32> = input
        .trim()
        .split("\n\n")
        .map(|lines| lines.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| a.cmp(b).reverse());
    sums.truncate(3);
    let max3: u32 = sums.iter().sum();

    println!("Part 2 :: max value: {}", max3);
}
