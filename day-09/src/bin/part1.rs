mod shared;
use shared::{ InstructionSet, PUZZLE_INPUT };

fn main() {
    let out = InstructionSet::from(PUZZLE_INPUT).get_largest_area();
    println!("{}", out);
}
