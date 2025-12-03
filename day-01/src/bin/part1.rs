mod shared;

use shared::{ InstructionSet, Dial, PUZZLE_INPUT };

fn main() {
    let inp: InstructionSet = PUZZLE_INPUT.into();
    let dial = Dial::new(0, 99, 50);
    let out = dial.run(&inp);
    println!("{out:?}");
}
