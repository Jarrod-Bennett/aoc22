use std::collections::HashSet;

use crate::utils;

/// Find the index after the first stream of `n` unique characters in the input
fn position_after_consecutive_unique_n(input: &str, n: usize) -> usize {
    input
        .char_indices()
        .flat_map(move |(from, _)| {
            input[from..]
                .char_indices()
                .nth(n - 1)
                .map(|(to, c)| &input[from..from + to + c.len_utf8()])
        })
        .position(|chars| chars.chars().collect::<HashSet<char>>().len() == n)
        .unwrap()
        + n // iterator starts at position 3, so need to add 4 to find end of
            // packet.
}

pub fn solve_part1() {
    let input = utils::io::read(6);

    let score = position_after_consecutive_unique_n(&input, 4);

    println!("Part 1 :: Packet starts at: {}", score);
}

pub fn solve_part2() {
    let input = utils::io::read(6);

    let score = position_after_consecutive_unique_n(&input, 14);

    println!("Part 2 :: Message starts at: {}", score);
}

#[cfg(test)]
mod tests {
    use super::position_after_consecutive_unique_n;

    #[test]
    fn test_find_start_of_packet() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (data, start_location) in inputs {
            assert_eq!(position_after_consecutive_unique_n(data, 4), start_location)
        }
    }

    #[test]
    fn test_start_of_messages() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (data, start_location) in inputs {
            assert_eq!(
                position_after_consecutive_unique_n(data, 14),
                start_location
            )
        }
    }
}
