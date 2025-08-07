///Connects all types of foods using the Food traits
pub mod fries;

use std::fmt::Debug;
use std::hash::Hash;

pub trait Food: Debug + PartialEq + Hash+ Eq + Copy +  Clone {}