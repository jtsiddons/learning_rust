use crate::garden::vegetables::Onion;

pub mod garden;

fn main() {
    let plant = Onion {};
    println!("I'm growing {:?}!", plant);
}
