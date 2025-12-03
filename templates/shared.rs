#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

struct InstructionSet;

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = InstructionSet::from(PUZZLE_INPUT);
        let expected = "";
        assert_eq!(actual, expected);
    }
}
