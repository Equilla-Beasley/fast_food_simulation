///Logic for the usage of Inventories
pub mod deep_freezer;
pub mod fried_food_fridge;
pub mod fry_staging_area;

use crate::foods::fries::Fries;

pub enum UpdateToggle {
    Add,
    Subtract,
}

//Food Bags are only used in refrigerators
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodBags{
    BagOfFries(Fries),
    EmptyBag,
}
//Food Boxes are only used in the deep freezer
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodBoxes{
    BoxOfFries(FoodBags),
    BoxOfNothing,
}

pub trait Inventory {
    type Container;
    fn initiate_inventory() -> Self;
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle);
    fn check_stock(&mut self, container: &Self::Container) -> &u32;
}
