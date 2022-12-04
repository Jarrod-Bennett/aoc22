//! Handle reading input files in various forms

use std::fs;

pub fn read(day: u32) -> String {
    let filename = format!("src/inputs/day{}.txt", day);
    fs::read_to_string(filename).expect("Unable to read file.")
}
