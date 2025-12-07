mod shared;
use shared::{ Manifold, PUZZLE_INPUT };

fn main() {
    let mut manifold = Manifold::from(PUZZLE_INPUT);
    println!("{}", manifold.fire_beam())
}
