///Logic for the usage of Inventories
mod deep_freezer;

use crate::food::Food;
use crate::food::fries::Fries;

pub enum UpdateToggle {
    Add,
    Subtract,
}

//Work In Progress
pub struct BoxOfFries {
    container: Box<Fries>,
}

pub trait Inventory {
    type Storage;

    fn initiate_inventory() -> Self;
    fn update_inventory(&mut self, food: Fries, amount: u32, toggle: UpdateToggle);
}
