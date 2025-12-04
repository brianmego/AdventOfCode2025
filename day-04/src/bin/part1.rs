mod shared;
use shared::{PUZZLE_INPUT, Warehouse};

fn main() {
    let warehouse = Warehouse::from(PUZZLE_INPUT);
    println!("{}", warehouse.accessible_slots().len())
}
