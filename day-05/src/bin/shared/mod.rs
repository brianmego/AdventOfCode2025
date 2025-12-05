#![allow(dead_code)]

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd)]
pub struct IngredientId(usize);

#[derive(PartialEq, Eq, Clone, Debug)]
struct Range {
    min: IngredientId,
    max: IngredientId,
}

impl Range {
    fn new(min: usize, max: usize) -> Self {
        Self {
            min: IngredientId(min),
            max: IngredientId(max),
        }
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InstructionSet {
    fresh_ranges: Vec<Range>,
    available_ids: Vec<IngredientId>,
}

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        let parts = value.split_once("\n\n").expect("known puzzle input");
        let fresh_ranges = parts
            .0
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .map(|(min, max)| {
                Range::new(min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap())
            })
            .collect();
        let available_ids = parts
            .1
            .lines()
            .map(|l| IngredientId(l.parse::<usize>().unwrap()))
            .collect();
        Self::new(fresh_ranges, available_ids)
    }
}
impl InstructionSet {
    fn new(fresh_ranges: Vec<Range>, available_ids: Vec<IngredientId>) -> Self {
        Self {
            fresh_ranges,
            available_ids,
        }
    }

    pub fn get_fresh(&self) -> Vec<IngredientId> {
        self.available_ids
            .iter()
            .filter(|id| {
                for range in self.fresh_ranges.iter() {
                    let is_valid = **id >= range.min && **id <= range.max;
                    if is_valid {
                        return true;
                    }
                }
                false
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
        let actual = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(actual.get_fresh().len(), 3);
    }
}
