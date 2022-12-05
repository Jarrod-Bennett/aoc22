use crate::utils;

#[derive(Debug)]
/// A collection of CrateStacks storing the Elve's supplies.
struct Stacks {
    stacks: Vec<CrateStack>,
}

impl Stacks {
    fn with_capacity(n: usize) -> Self {
        Self {
            stacks: (0..n).map(|_| CrateStack::default()).collect(),
        }
    }

    /// Add a row of crates to the respective Stacks
    fn parse_row(&mut self, line: &str) -> Result<(), String> {
        // why do we have to allocate?
        let line = line.chars().collect::<Vec<char>>();

        let mut column: usize = 0;
        for i in (0..line.len()).step_by(4) {
            match &line[i..i + 3] {
                ['[', cargo, ']'] => {
                    self.stacks.get_mut(column).unwrap().0.push((*cargo).into());
                }
                [' ', ' ', ' '] => {} // empty row is fine, do nothing
                _ => return Err(format!("Could not parse element {}", "e")),
            }
            column += 1;
        }

        Ok(())
    }

    fn move_crates_individually(&mut self, movement: &Movement) -> Result<(), String> {
        // should add error checking
        for _ in 0..movement.quantity {
            let moved_crate = self
                .stacks
                .get_mut(movement.origin)
                .unwrap()
                .0
                .pop()
                .unwrap();
            self.stacks
                .get_mut(movement.destination)
                .unwrap()
                .0
                .push(moved_crate);
        }

        Ok(())
    }

    fn move_crates_together(&mut self, movement: &Movement) -> Result<(), String> {
        let mut moved_crates = Vec::with_capacity(movement.quantity);
        for _ in 0..movement.quantity {
            moved_crates.push(
                self.stacks
                    .get_mut(movement.origin)
                    .unwrap()
                    .0
                    .pop()
                    .unwrap(),
            )
        }
        for _ in 0..movement.quantity {
            self.stacks
                .get_mut(movement.destination)
                .unwrap()
                .0
                .push(moved_crates.pop().unwrap())
        }

        Ok(())
    }
}

#[derive(Debug)]
struct CrateStack(Vec<Crate>);

impl Default for CrateStack {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<&str> for CrateStack {
    fn from(s: &str) -> Self {
        Self(s.chars().map(Crate::from).collect::<Vec<Crate>>())
    }
}

impl CrateStack {
    /// Move `n` crates from this stack to another.
    /// Returns an error if `n` exceeds the number of crates in the stack
    fn move_crates(&mut self, other: &mut Self, n: usize) -> Result<(), String> {
        if self.0.len() < n {
            return Err(format!(
                "Attempted to move {} crates but CrateStack has {} crates (attempted to
                move more crates than exist in the stack)",
                n,
                self.0.len()
            ));
        }

        // Repeatedly pop/pushing is probably faster than take n/reverse/push n
        for _ in 0..n {
            other.0.push(self.0.pop().unwrap())
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);

impl From<char> for Crate {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Into<char> for Crate {
    fn into(self) -> char {
        self.0
    }
}

impl Default for Crate {
    fn default() -> Self {
        Self(' ')
    }
}

#[derive(Debug)]
struct Movement {
    origin: usize,
    destination: usize,
    quantity: usize,
}

impl TryFrom<&str> for Movement {
    type Error = String;

    /// Parse a `Movement` from a line formatted as:
    /// 'move n from a to b' where `n` is the quantity of crates to move from
    /// stack `a` to stack `b`.
    /// Origin and destination cannot be the same.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(' ');

        match split.collect::<Vec<&str>>()[..] {
            ["move", quantity, "from", origin, "to", destination] => {
                let quantity = quantity
                    .parse()
                    .map_err(|_| format!("Unable to parse {} as quantity", quantity))?;
                let origin: usize = origin
                    .parse()
                    .map_err(|_| format!("Unable to parse {} as quantity", origin))?;
                let destination: usize = destination
                    .parse()
                    .map_err(|_| format!("Unable to parse {} as quantity", destination))?;
                if origin == destination {
                    Err("Origin and destination are the same".to_string())
                } else {
                    Ok(Self {
                        origin: origin - 1,
                        destination: destination - 1,
                        quantity,
                    })
                }
            }
            _ => Err(format!("Unable to parse input {:?}", value)),
        }
    }
}

pub fn solve_part1() {
    let input = utils::io::read(5);

    // find the row that separates the stacks and movements and use it to find
    // the number of stacks. Number of stacks is len/4 + 1 since each number
    // occupies 3 spaces + a separater between numbers.
    // This may not work if there are >= 10 stacks.
    let rows_of_crates = input
        .lines()
        .position(|l| l.is_empty())
        .expect("Could not find empty row to split stacks and movements")
        - 1;
    let num_stacks = input
        .lines()
        .nth(rows_of_crates)
        .expect("Could not find row with number of stacks")
        .len()
        / 4
        + 1;

    // Can now create the stacks and iterate in reverse to populate the crates.
    let mut stacks = Stacks::with_capacity(num_stacks);
    for row in (0..rows_of_crates).rev() {
        stacks.parse_row(input.lines().nth(row).unwrap()).unwrap();
    }

    // number of stacks can be found by inspecting the length of the line of
    // stack numbers (there are 3 characters per number).

    let moves: Vec<Movement> = input
        .lines()
        .skip(10)
        .map(|l| Movement::try_from(l).unwrap())
        .collect();

    for mov in moves {
        stacks.move_crates_individually(&mov).unwrap();
    }

    let top_crates = stacks
        .stacks
        .iter()
        .map(|stack| stack.0.last().map_or(' ', |c| (*c).into()))
        .collect::<String>();

    println!("Part 1 :: Crates on top: {}", top_crates);
}

pub fn solve_part2() {
    let input = utils::io::read(5);

    let rows_of_crates = input
        .lines()
        .position(|l| l.is_empty())
        .expect("Could not find empty row to split stacks and movements")
        - 1;
    let num_stacks = input
        .lines()
        .nth(rows_of_crates)
        .expect("Could not find row with number of stacks")
        .len()
        / 4
        + 1;

    // Can now create the stacks and iterate in reverse to populate the crates.
    let mut stacks = Stacks::with_capacity(num_stacks);
    for row in (0..rows_of_crates).rev() {
        stacks.parse_row(input.lines().nth(row).unwrap()).unwrap();
    }

    // number of stacks can be found by inspecting the length of the line of
    // stack numbers (there are 3 characters per number).

    let moves: Vec<Movement> = input
        .lines()
        .skip(10)
        .map(|l| Movement::try_from(l).unwrap())
        .collect();

    for mov in moves {
        stacks.move_crates_together(&mov).unwrap();
    }

    let top_crates = stacks
        .stacks
        .iter()
        .map(|stack| stack.0.last().map_or(' ', |c| (*c).into()))
        .collect::<String>();

    println!("Part 2 :: Crates on top: {}", top_crates);
}
