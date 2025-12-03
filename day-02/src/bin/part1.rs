mod shared;

use crate::shared::{InstructionSet, PUZZLE_INPUT};

fn main() {
    let instruction_set = InstructionSet::from(PUZZLE_INPUT);
    let sum: u64 = instruction_set.find_duplicates().iter().sum();
    println!("{sum}");
}
