mod shared;
use shared::{PUZZLE_INPUT, Homework};

fn main() {
    println!("{}", Homework::from(PUZZLE_INPUT).solve());
}
