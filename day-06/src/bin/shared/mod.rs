#![allow(dead_code)]

use std::str::FromStr;

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Homework(Vec<Problem>);

impl From<&str> for Homework {
    fn from(value: &str) -> Self {
        let problem_numbers: Vec<Vec<usize>> = value
            .lines()
            .rev()
            .skip(1)
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let problem_ops: Vec<Operation> = value
            .lines()
            .last()
            .map(|l| {
                l.split_whitespace()
                    .map(|op| Operation::from_str(op).unwrap())
                    .collect()
            })
            .unwrap();
        let mut problems: Vec<Problem> = vec![];
        for (i, op) in problem_ops.iter().enumerate() {
            let numbers: Vec<usize> = problem_numbers.iter().map(|row| row[i]).collect();
            problems.push(Problem::new(numbers, op.clone()))

        }
        Self(problems)
    }
}
impl Homework {
    pub fn solve(&self) -> usize {
        self.0.iter().map(|p| p.solve()).sum()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct InvalidChar;

impl FromStr for Operation {
    type Err = InvalidChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(InvalidChar),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Problem {
    numbers: Vec<usize>,
    op: Operation,
}

impl Problem {
    fn new(numbers: Vec<usize>, op: Operation) -> Self {
        Self { numbers, op }
    }
    fn solve(&self) -> usize {
        match self.op {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = Homework::from(PUZZLE_INPUT);
        assert_eq!(actual.solve(), 4277556);
    }
}
