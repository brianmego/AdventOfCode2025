#![allow(dead_code)]

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(PartialEq, Eq, Debug)]
pub struct InstructionSet(Vec<BatteryBank>);

impl InstructionSet {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        Self(value.lines().map(BatteryBank::from).collect())
    }
}
impl InstructionSet {
    pub fn calculate_joltage(&self) -> usize {
        self.0.iter().map(|b| b.calculate_joltage() as usize).sum()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct BatteryBank(Vec<u8>);
impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        Self(
            value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        )
    }
}
impl BatteryBank {
    fn calculate_joltage(&self) -> u8 {
        let mut max: (usize, u8) = (0, 0);
        for (idx, i) in self.0[0..self.0.len() - 1].iter().enumerate() {
            if i > &max.1 {
                max = (idx, *i)
            }
        }
        let mut second: (usize, u8) = (0, 0);
        for (idx, i) in self.0[max.0 + 1..self.0.len()].iter().enumerate() {
            if i > &second.1 {
                second = (idx, *i)
            }
        }
        (max.1 * 10) + second.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_input() {
        let actual = InstructionSet::from(PUZZLE_INPUT);
        assert_eq!(actual.len(), 4);
    }
    #[test_case("987654321111111", 98)]
    #[test_case("811111111111119", 89)]
    #[test_case("234234234234278", 78)]
    #[test_case("818181911112111", 92)]
    fn test_bank_joltage(inp: &str, exp: u8) {
        let bank = BatteryBank::from(inp);
        assert_eq!(bank.calculate_joltage(), exp);
    }

    #[test]
    fn test_inp_joltage() {
        let actual = InstructionSet::from(PUZZLE_INPUT); assert_eq!(actual.calculate_joltage(), 357);
    }
}
