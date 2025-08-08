#![allow(unused)]

use fast_food_simulation::food::fries::{Fries, FryKinds, FrySizes};
use fast_food_simulation::inventory::{fried_food_fridge::FriedFoodFridge, deep_freezer::DeepFreezer, fry_staging_area::FryStagingArea, Inventory, FoodBags, FoodBoxes, UpdateToggle};

fn main() {
    let normal_fries = Fries::create(FryKinds::Normal, FrySizes::None);
    let bag_of_fries = FoodBags::BagOfFries(normal_fries);
    let box_of_fries = FoodBoxes::BoxOfFries(bag_of_fries);
    let mut fry_staging_area = FryStagingArea::initiate_inventory();
    let mut fridge = FriedFoodFridge::initiate_inventory();
    let mut deep_freezer = DeepFreezer::initiate_inventory();

    fry_staging_area.update_inventory(normal_fries, 100, UpdateToggle::Add);
    fridge.update_inventory(bag_of_fries, 8, UpdateToggle::Add);
    deep_freezer.update_inventory(box_of_fries, 12, UpdateToggle::Add);

    println!("The fridge starts with {} bags of fries inside", fridge.check_stock(&bag_of_fries));
    fridge.update_inventory(bag_of_fries,2,UpdateToggle::Subtract);
    println!("The fridge now has {} bags of fries inside", fridge.check_stock(&bag_of_fries));

}
