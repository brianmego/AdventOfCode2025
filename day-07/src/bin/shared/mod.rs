#![allow(dead_code)]
use aoc_utils::{Collection, Direction, Loc, ParseableCharacters, Tile, parse_collection};

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Activated,
    Deactivated,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ManifoldSlot {
    Empty,
    Start,
    Splitter(Status),
    Beam,
}

impl ParseableCharacters for ManifoldSlot {
    fn valid_chars() -> Vec<char> {
        vec!['.', 'S', '^', '|']
    }
}

impl From<char> for ManifoldSlot {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'S' => Self::Start,
            '^' => Self::Splitter(Status::Deactivated),
            '|' => Self::Beam,
            _ => unreachable!(),
        }
    }
}

pub struct Manifold(Collection<ManifoldSlot>);

impl From<&str> for Manifold {
    fn from(value: &str) -> Self {
        Self(parse_collection(value).unwrap().1)
    }
}

impl Manifold {
    fn count(&self, manifold_type: ManifoldSlot) -> usize {
        self.0.count_tile_type(&manifold_type)
    }

    fn find_start(&self) -> Option<&Tile<ManifoldSlot>> {
        self.0
            .tiles()
            .iter()
            .find(|t| t.get_type() == &ManifoldSlot::Start)
    }

    pub fn fire_beam(&mut self) -> usize {
        let start = self.find_start().unwrap();
        let mut beams: Vec<&Tile<ManifoldSlot>> = vec![start];
        let mut activated_splitters: Vec<Loc> = vec![];

        while !beams.is_empty() {
            (beams, activated_splitters) = self.move_beams(beams, activated_splitters);
        }
        activated_splitters.len()
    }

    fn move_beams<'a>(
        &'a self,
        beams: Vec<&Tile<ManifoldSlot>>,
        activated_splitters: Vec<Loc>,
    ) -> (Vec<&'a Tile<ManifoldSlot>>, Vec<Loc>) {
        let mut new_beams: Vec<Loc> = vec![];
        let mut activated_splitters: Vec<Loc> = activated_splitters.clone();
        for beam in beams {
            let next_tile = self
                .0
                .get_tile(beam.loc().get_nearby(Direction::South, 1).unwrap());
            if let Some(t) = next_tile {
                match t.get_type() {
                    ManifoldSlot::Splitter(Status::Deactivated) => {
                        if !activated_splitters.contains(t.loc()) {
                            activated_splitters.push(*t.loc());
                        }
                        let left = t.loc().get_nearby(Direction::West, 1).unwrap();
                        let right = t.loc().get_nearby(Direction::East, 1).unwrap();

                        if !new_beams.contains(&left) {
                            new_beams.push(left);
                        }
                        if !new_beams.contains(&right) {
                            new_beams.push(right);
                        }
                    }
                    ManifoldSlot::Empty => new_beams.push(*t.loc()),
                    _ => continue,
                }
            }
        }
        (
            new_beams.iter().map(|l| self.0.get_tile(*l).unwrap()).collect(),
            activated_splitters,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = Manifold::from(PUZZLE_INPUT);
        assert_eq!(actual.count(ManifoldSlot::Start), 1);
        assert_eq!(
            actual.count(ManifoldSlot::Splitter(Status::Deactivated)),
            22
        );
        assert_eq!(actual.count(ManifoldSlot::Splitter(Status::Activated)), 0);
        assert_eq!(actual.count(ManifoldSlot::Beam), 0);
    }

    #[test]
    fn test_fire_beam() {
        let mut actual = Manifold::from(PUZZLE_INPUT);
        assert_eq!(actual.fire_beam(), 21);
    }
}
