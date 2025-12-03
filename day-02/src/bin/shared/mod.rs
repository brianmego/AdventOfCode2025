#![allow(dead_code)]

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

pub struct InstructionSet(Vec<Range>);

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        let ranges: Vec<Range> = value
            .split(',')
            .filter_map(|v| Range::try_from(v).ok())
            .collect();
        Self(ranges)
    }
}
impl InstructionSet {
    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn find_duplicates(&self) -> Vec<u64> {
        self.0.iter().flat_map(|r| r.find_duplicates()).collect()
    }
}

struct BadRange(String);
#[derive(PartialEq, Eq, Clone, Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    fn find_duplicates(&self) -> Vec<u64> {
        let mut acc = vec![];
        for i in self.min..=self.max {
            let str_i = i.to_string();
            let (left, right) = str_i.split_at(str_i.len() / 2);
            if left == right {
                acc.push(i)
            }
        }
        acc
    }
}
impl TryFrom<&str> for Range {
    type Error = BadRange;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (left, right): (&str, &str) = value.split_once('-').ok_or(BadRange(value.into()))?;
        Ok(Self {
            min: left
                .trim_end()
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Bad Range: {value}")),
            max: right
                .trim_end()
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Bad Range: {value}")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let instruction_set = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(instruction_set.len(), 11);
        assert_eq!(instruction_set.0[0], Range::new(11, 22));
        assert_eq!(instruction_set.0[1], Range::new(95, 115));
        assert_eq!(instruction_set.0[2], Range::new(998, 1012));
        assert_eq!(instruction_set.0[3], Range::new(1188511880, 1188511890));
        assert_eq!(instruction_set.0[4], Range::new(222220, 222224));
        assert_eq!(instruction_set.0[5], Range::new(1698522, 1698528));
        assert_eq!(instruction_set.0[6], Range::new(446443, 446449));
        assert_eq!(instruction_set.0[7], Range::new(38593856, 38593862));
        assert_eq!(instruction_set.0[8], Range::new(565653, 565659));
        assert_eq!(instruction_set.0[9], Range::new(824824821, 824824827));
        assert_eq!(instruction_set.0[10], Range::new(2121212118, 2121212124));
    }

    #[test]
    fn test_find_duplicates() {
        let instruction_set = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(
            instruction_set.find_duplicates().iter().sum::<u64>(),
            1227775554
        );
    }
}
