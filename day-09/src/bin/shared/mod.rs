#![allow(dead_code)]

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InstructionSet(Vec<Tile>);

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(x, y)| Tile::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect(),
        )
    }
}

impl InstructionSet {
    pub fn get_largest_area(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                self.0
                    .iter()
                    .skip(i + 1)
                    .map(|t2| t.get_area_between(t2))
                    .collect::<Vec<usize>>()
            })
            .max()
            .unwrap()
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Tile {
    fn get_area_between(&self, other: &Tile) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_input() {
        let actual = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(actual.0.len(), 8);
        assert_eq!(actual.0[0].x, 7);
        assert_eq!(actual.0[0].y, 1);
    }

    #[test_case(Tile::new(2, 5), Tile::new(9, 7), 24)]
    #[test_case(Tile::new(7, 1), Tile::new(11, 7), 35)]
    #[test_case(Tile::new(7, 3), Tile::new(2, 3), 6)]
    #[test_case(Tile::new(2, 5), Tile::new(11, 1), 50)]
    fn test_get_area_between(t1: Tile, t2: Tile, expected: usize) {
        assert_eq!(t1.get_area_between(&t2), expected);
    }

    #[test]
    fn test_get_largest_area() {
        let actual = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(actual.get_largest_area(), 50);
    }
}
