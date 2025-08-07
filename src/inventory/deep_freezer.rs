///Logic for the usage of the Deep Freezer
use std::collections::HashMap;
use crate::food::fries::Fries;
use super::{BoxOfFries, Inventory, UpdateToggle};

#[derive(Debug, PartialEq)]
pub struct DeepFreezer {
    storage: HashMap<Fries,u32>,
}

impl Inventory for DeepFreezer{
    type Storage = BoxOfFries;
    fn initiate_inventory() -> DeepFreezer {
        DeepFreezer {
            storage: HashMap::new(),
        }
    }
    /*
        If the inventory doesn't have the chosen entry, the inputted amount is set as a default value.
        Otherwise, it will add or subtract the inputted amount from the current amount.
     */
    fn update_inventory(&mut self, food: Fries, amount: u32, toggle: UpdateToggle) {
        self.storage
            .entry(food)
            .and_modify(|x| match toggle {
                UpdateToggle::Add => *x += amount,
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(amount);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::food::fries::{Fries, FryKinds, FrySizes};
    use crate::inventory::{Inventory, UpdateToggle};
    use crate::inventory::deep_freezer::DeepFreezer;

    #[test]
    fn create_inventory() {
        let inventory = DeepFreezer::initiate_inventory();
        let inventory_test = DeepFreezer {
            storage: HashMap::new(),
        };

        assert_eq!(inventory, inventory_test);
    }

    #[test]
    fn update_inventory() {
        let mut inventory = DeepFreezer::initiate_inventory();
        let normal_medium_fry = Fries::create(FryKinds::Normal, FrySizes::Medium);
        let normal_medium_fry_2 = normal_medium_fry.clone();

        inventory.update_inventory(normal_medium_fry,100, UpdateToggle::Add);
        let amount = inventory.storage.get(&normal_medium_fry_2).unwrap();

        assert_eq!(*amount, 100);
    }
}