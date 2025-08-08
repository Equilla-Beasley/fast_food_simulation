///Connects all types of foods using the Food traits
pub mod fries;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FoodStates {
    Frozen,
    Cooked,
    Inedible,
}