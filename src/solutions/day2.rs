use crate::utils;

#[derive(PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<char> for RockPaperScissors {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use RockPaperScissors::*;
        match value.to_ascii_uppercase() {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err(format!(
                "Input {} is not a valid Rock/Paper/Scissors value.",
                value
            )),
        }
    }
}

impl From<RockPaperScissors> for u32 {
    fn from(rps: RockPaperScissors) -> Self {
        use RockPaperScissors::*;
        match rps {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

struct Match {
    opponent_move: RockPaperScissors,
    response: RockPaperScissors,
}

impl Match {
    fn score(&self) -> u32 {
        use RockPaperScissors::*;

        let shape_score: u32 = self.response.into();

        // Draw
        if self.opponent_move == self.response {
            return shape_score + Outcome::Draw as u32;
        }

        // Win/loss
        shape_score
            + match self.opponent_move {
                Rock => (self.response == Paper) as u32 * 6,
                Paper => (self.response == Scissors) as u32 * 6,
                Scissors => (self.response == Rock) as u32 * 6,
            }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Loss = 0, // X
    Draw = 3, // Y
    Win = 6,  // Z
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Outcome::*;
        match value.to_ascii_uppercase() {
            'X' => Ok(Loss),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(format!("Input {} is not a valid outcome value.", value)),
        }
    }
}

impl From<Outcome> for u32 {
    fn from(outcome: Outcome) -> Self {
        use Outcome::*;
        match outcome {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

#[derive(PartialEq)]
struct Match2 {
    opponent_move: RockPaperScissors,
    outcome: Outcome,
}

impl TryFrom<&str> for Match2 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let opponent_move: RockPaperScissors = value
            .chars()
            .next()
            .ok_or_else(|| "Missing opponent move".to_string())?
            .try_into()?;

        let outcome: Outcome = value
            .chars()
            .nth(2)
            .ok_or_else(|| "Missing required outcome".to_string())?
            .try_into()?;

        Ok(Match2 {
            opponent_move,
            outcome,
        })
    }
}

impl Match2 {
    fn score(&self) -> u32 {
        use Outcome::*;

        let opponent_piece = self.opponent_move as u32;

        match self.outcome {
            Win => (opponent_piece % 3 + 1) + Win as u32,
            Loss => (opponent_piece + 1) % 3 + 1 + Loss as u32,
            Draw => opponent_piece + Draw as u32,
        }
    }
}

pub fn solve_part1() {
    let input = utils::io::read(2);

    let score1: u32 = input
        .lines()
        .map(|l| Match {
            // Can improve this probably with split and tuples and filter map
            opponent_move: RockPaperScissors::try_from(l.to_string().chars().next().unwrap())
                .unwrap(),
            response: RockPaperScissors::try_from(l.to_string().chars().nth(2).unwrap()).unwrap(),
        })
        .map(|m| m.score())
        .sum();

    println!("Part 1 :: Score: {}", score1);
}

pub fn solve_part2() {
    let input = utils::io::read(2);

    let score2: u32 = input
        .lines()
        .map(|l| Match2::try_from(l).expect("Unable to parse line"))
        .map(|m| m.score())
        .sum();

    println!("Part 2 :: Score: {}", score2);
}
