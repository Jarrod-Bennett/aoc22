use crate::utils;
use itertools::Itertools;

/// A `Rucksack` of two compartments, each containing items represented by a
/// single upper or lowercase letter. An item cannot be in both compartments.
struct Rucksack<'a> {
    comp1: &'a str,
    comp2: &'a str,
}

impl<'a> Rucksack<'a> {
    /// Create a `Rucksack` from a stream of characters. The provided input is
    /// split into the two compartments
    pub fn with(sack: &'a str) -> Self {
        let half_len = sack.len() / 2;
        Self {
            comp1: &sack[0..half_len],
            comp2: &sack[half_len..],
        }
    }

    // Create a `Rucksack` with arbitrary compartments.
    pub fn _with_compartments(comp1: &'a str, comp2: &'a str) -> Self {
        Self { comp1, comp2 }
    }

    pub fn _three_sacks_intersection(a: &str, b: &str, c: &str) -> Option<char> {
        a.chars()
            .into_iter()
            .filter(|ch| b.contains(*ch))
            .find(|ch| c.contains(*ch))
    }

    /// Parse two compartments and return the first item in both compartments,
    /// if any exist, returning Some(item). Otherwise None is returned.
    pub fn first_intersection(&self) -> Option<char> {
        self.comp1
            .chars()
            .into_iter()
            .find(|c| self.comp2.contains(*c))
    }
}

struct Priority(u8);

impl From<char> for Priority {
    fn from(c: char) -> Self {
        if c.is_ascii_uppercase() {
            Self(c as u8 - b'A' + 26 + 1)
        } else {
            Self(c as u8 - b'a' + 1)
        }
    }
}

pub fn solve_part1() {
    let input = utils::io::read(3);

    let score: u32 = input
        .lines()
        .map(|l| {
            Rucksack::with(l)
                .first_intersection()
                .unwrap_or_else(|| panic!("No items in both compartments in line {}", l))
        })
        .map(|common| Priority::from(common).0 as u32)
        .sum();

    println!("Part 1 :: Score: {}", score);
}

pub fn solve_part2() {
    let input = utils::io::read(3);

    let chunks = input.lines().chunks(3);
    let score: u32 = chunks
        .into_iter()
        .map(|mut chunk| {
            let (a, b, c) = (
                chunk.next().unwrap(),
                chunk.next().unwrap(),
                chunk.next().unwrap(),
            );
            Priority::from(
                a.chars()
                    .into_iter()
                    .filter(|ch| b.contains(*ch))
                    .find(|ch| c.contains(*ch))
                    .unwrap_or_else(|| panic!("No value in all three Rucksacks")),
            )
            .0 as u32
        })
        .sum();

    println!("Part 2 :: Score: {}", score);
}
