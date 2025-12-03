#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let inp = InputList::new(PUZZLE_INPUT);
        let actual = "";
        let expected = "";
        assert_eq!(actual, expected);
    }
}
