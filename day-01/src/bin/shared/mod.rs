#![allow(dead_code)]

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

const MIN_FACE: u8 = 0;
const MAX_FACE: u8 = 99;

#[derive(PartialEq, Debug)]
pub enum Direction {
    L,
    R,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::L,
            "R" => Self::R,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    direction: Direction,
    count: u16,
}

#[derive(PartialEq, Debug)]
pub struct Results {
    exact_mins: usize,
    passed_mins: usize,
}
pub struct Dial {
    min: u8,
    max: u8,
    start: u8,
}

impl Dial {
    pub fn new(min: u8, max: u8, start: u8) -> Self {
        Self { min, max, start }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn run(&self, instructions: &InstructionSet) -> Results {
        let mut exact_mins: usize = 0;
        let mut passed_mins: usize = 0;
        let mut current_val: isize = self.start.into();
        for inst in &instructions.0 {
            let full_rotations: usize = (inst.count / u16::from(self.max + 1)).into();
            passed_mins += full_rotations;
            let count = inst.count % u16::from(self.max+1);
            match inst.direction {
                Direction::L => {
                    current_val -= isize::try_from(count).unwrap();
                    if current_val < self.min.into() {
                        passed_mins += 1;
                        current_val += self.max as isize + 1;
                    }
                }
                Direction::R => {
                    current_val += isize::try_from(count).unwrap();
                    if current_val > self.max.into() {
                        passed_mins += 1;
                        current_val -= self.max as isize + 1;
                    }
                }
            }
            if current_val == self.min.into() {
                exact_mins += 1;
                // passed_mins += 1;
            }
            dbg!(inst.count, current_val, exact_mins, passed_mins, "");
        }
        Results {
            exact_mins,
            passed_mins,
        }
    }
}

pub struct InstructionSet(Vec<Instruction>);

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        let mut set = vec![];
        for line in value.lines() {
            let (direction, count) = line.split_at(1);
            set.push(Instruction {
                direction: Direction::from(direction),
                count: count.parse::<u16>().unwrap(),
            });
        }
        InstructionSet(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let inp: InstructionSet = PUZZLE_INPUT.into();
        assert_eq!(inp.0.len(), 10);
        assert_eq!(
            inp.0[0],
            Instruction {
                direction: Direction::L,
                count: 68
            }
        );
        assert_eq!(
            inp.0[2],
            Instruction {
                direction: Direction::R,
                count: 48
            }
        );
    }

    #[test]
    fn test_exact_mins() {
        let inp: InstructionSet = PUZZLE_INPUT.into();
        let dial = Dial::new(MIN_FACE, MAX_FACE, 50);
        let actual = dial.run(&inp);
        assert_eq!(actual.exact_mins, 3);
    }

    #[test]
    fn test_passed_mins() {
        let inp: InstructionSet = PUZZLE_INPUT.into();
        let dial = Dial::new(MIN_FACE, MAX_FACE, 50);
        let actual = dial.run(&inp);
        assert_eq!(actual.passed_mins, 6);
    }
}
