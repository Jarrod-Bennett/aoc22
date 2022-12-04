use crate::utils;
use std::{fmt::Debug, str::FromStr};

struct Range<T> {
    start: T,
    end: T,
}

impl<T: Ord + FromStr + Debug> Range<T> {
    /// Create a range from text formed as '{Start}-{End}'
    fn parse(s: &str) -> Result<Self, String> {
        let mut nums = s.split('-');
        let start: T = nums
            .next()
            .ok_or_else(|| "Could not find start".to_string())?
            .parse()
            .map_err(|_| "Could not parse start".to_string())?;
        let end = nums
            .next()
            .ok_or_else(|| "Could not find end".to_string())?
            .parse()
            .map_err(|_| "Could not parse end".to_string())?;

        Ok(Self { start, end })
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn contains(&self, other: &T) -> bool {
        self.start <= *other && self.end >= *other
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(&other.start)
            || self.contains(&other.end)
            || other.contains(&self.start)
            || other.contains(&self.end)
    }
}

pub fn solve_part1() {
    let input = utils::io::read(4);

    let score: i32 = input
        .lines()
        .map(|l| {
            let mut ranges = l.split(',');
            let first: Range<i32> = Range::parse(ranges.next().unwrap()).unwrap();
            let second: Range<i32> = Range::parse(ranges.next().unwrap()).unwrap();

            (first.contains_range(&second) || second.contains_range(&first)) as i32
        })
        .sum();

    println!("Part 1 :: Score: {}", score);
}

pub fn solve_part2() {
    let input = utils::io::read(4);

    let score: i32 = input
        .lines()
        .map(|l| {
            let mut ranges = l.split(',');
            let first: Range<i32> = Range::parse(ranges.next().unwrap()).unwrap();
            let second: Range<i32> = Range::parse(ranges.next().unwrap()).unwrap();

            first.overlaps(&second) as i32
        })
        .sum();
    println!("Part 2 :: Score: {}", score);
}
