///Connects all types of foods using the Food traits
pub mod fries;
pub mod recipes;
pub mod cooked_items;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodStates {
    Frozen,
    Cooked,
    Inedible,
}
