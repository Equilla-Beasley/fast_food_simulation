///Logic for the usage of the Deep Freezer
use std::collections::HashMap;
use super::{FoodBoxes, Inventory, UpdateToggle};

#[derive(Debug, PartialEq)]
pub struct DeepFreezer {
    storage: HashMap<FoodBoxes,u32>,
}

impl Inventory for DeepFreezer{
    type Container = FoodBoxes;
    fn initiate_inventory() -> DeepFreezer {
        DeepFreezer {
            storage: HashMap::new(),
        }
    }
    /*
        If the inventory doesn't have the chosen entry, the inputted amount is set to 8 (Max Number of Any Kind of Boxes).
        Otherwise, it will add or subtract the inputted amount from the current amount.
     */
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) {
        self.storage
            .entry(match container {
                FoodBoxes::BoxOfFries(_) => container,
                _ => panic!("Incorrect Food Bag Inputted"),
            })
            .and_modify(|x| match toggle {
                UpdateToggle::Add => *x += amount,
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(8);
    }

    fn check_stock(&mut self, container: &Self::Container) -> &u32 {
        self.storage.get(container).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::foods::FoodStates;
    use crate::foods::fries::{Fries, FryKinds, FrySizes};
    use crate::inventories::FoodBags; //I don't know why I can't use super for this.
    use super::{Inventory, UpdateToggle, DeepFreezer, FoodBoxes};

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
        let normal_medium_fry = Fries::create(FryKinds::Normal, FrySizes::Medium, FoodStates::Cooked);

        let fry_bag = FoodBags::BagOfFries(normal_medium_fry);
        let fry_bag_2 = FoodBags::BagOfFries(normal_medium_fry);

        let fry_box = FoodBoxes::BoxOfFries(fry_bag);
        let fry_box_2 = FoodBoxes::BoxOfFries(fry_bag_2);

        inventory.update_inventory(fry_box,100, UpdateToggle::Add);
        let amount = inventory.storage.get(&fry_box_2).unwrap();

        assert_eq!(*amount, 8);
    }
}