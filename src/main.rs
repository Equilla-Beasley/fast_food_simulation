#![allow(unused)]

use fast_food_simulation::fries::{Fries, FryKinds, FrySizes};

fn main() {
    let normal_medium_fry = Fries::create(FryKinds::Normal, FrySizes::Medium);
    println!("{:?}", normal_medium_fry);
}
