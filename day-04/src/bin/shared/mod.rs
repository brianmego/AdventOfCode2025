#![allow(dead_code)]
use rayon::prelude::*;
use aoc_utils::{Collection, Direction, ParseableCharacters, Tile, parse_collection};

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WarehouseSlot {
    Paper,
    Empty,
}
impl ParseableCharacters for WarehouseSlot {
    fn valid_chars() -> Vec<char> {
        vec!['@', '.']
    }
}
impl From<char> for WarehouseSlot {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

pub struct Warehouse(Collection<WarehouseSlot>);
impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        Self(parse_collection(value).unwrap().1)
    }
}
impl Warehouse {
    pub fn accessible_slots(&self) -> Vec<Tile<WarehouseSlot>> {
        self.0
            .tiles()
            .par_iter()
            .filter(|t| t.get_type() == &WarehouseSlot::Paper)
            .filter(|t| {
                Direction::get_all()
                    .par_iter()
                    .filter(|d| {
                        t.loc().get_nearby(**d, 1).is_some_and(|x| {
                            self.0.get_tile(x).is_some_and(|neighbor| {
                                neighbor.get_type() == &WarehouseSlot::Paper
                            })
                        })
                    })
                    .count()
                    < 4
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = Warehouse::from(PUZZLE_INPUT);
        // assert_eq!(actual.accessible_slots().len(), 13);
    }
}
