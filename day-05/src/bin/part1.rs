mod shared;
use shared::{InstructionSet, PUZZLE_INPUT};

fn main() {
    let actual = InstructionSet::from(PUZZLE_INPUT);
    println!("{}", actual.get_fresh().len());
}
