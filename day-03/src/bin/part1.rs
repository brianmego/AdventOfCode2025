mod shared;
use shared::{PUZZLE_INPUT, InstructionSet};

fn main() {
    let out = InstructionSet::from(PUZZLE_INPUT).calculate_joltage();
    println!("{out}");
}
