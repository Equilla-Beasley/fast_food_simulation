///Logic for the usage of Inventories
pub mod deep_freezer;
pub mod fried_food_fridge;
pub mod fry_staging_area;

use crate::foods::FoodStates;
use crate::foods::fries::{Fries, FryKinds, FrySizes};

pub enum UpdateToggle {
    Add,
    Subtract,
}

//Food Bags are only used in refrigerators
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodBags{
    BagOfNormalFries(Fries),
    BagOfCurlyFries(Fries),
    BagOfWaffleFries(Fries),
    EmptyBag,
}

impl FoodBags {
    pub fn bag_of_normal_fries() -> Self {
        Self::BagOfNormalFries(Fries::create(FryKinds::Normal,FrySizes::None,FoodStates::Frozen))
    }
    pub fn bag_of_curly_fries() -> Self {
        Self::BagOfCurlyFries(Fries::create(FryKinds::Curly,FrySizes::None,FoodStates::Frozen))
    }
    pub fn bag_of_waffle_fries() -> Self {
        Self::BagOfWaffleFries(Fries::create(FryKinds::Waffle,FrySizes::None,FoodStates::Frozen))
    }
}

//Food Boxes are only used in the deep freezer
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodBoxes {
    BoxOfNormalFries(FoodBags),
    BoxOfCurlyFries(FoodBags),
    BoxOfWaffleFries(FoodBags),
    BoxOfNothing,
}

impl FoodBoxes {
    pub fn box_of_normal_fries() -> Self {
        Self::BoxOfNormalFries(FoodBags::bag_of_normal_fries())
    }
    pub fn box_of_curly_fries() -> Self {
        Self::BoxOfCurlyFries(FoodBags::bag_of_curly_fries())
    }
    pub fn box_of_waffle_fries() -> Self {
        Self::BoxOfWaffleFries(FoodBags::bag_of_waffle_fries())
    }
}

pub trait Inventory {
    type Container;
    fn initiate_inventory() -> Self;
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) -> Result<(),&str>;
    fn check_stock(&mut self, container: &Self::Container) -> &u32;
}
