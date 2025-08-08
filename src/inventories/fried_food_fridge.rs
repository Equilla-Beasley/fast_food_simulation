///Logic for the usage of the Fridge for Fried Foods
use std::collections::HashMap;
use super::{FoodBags, Inventory, UpdateToggle};

#[derive(Debug, PartialEq)]
pub struct FriedFoodFridge {
    storage: HashMap<FoodBags,u32>,
}

impl Inventory for FriedFoodFridge{
    type Container = FoodBags;
    fn initiate_inventory() -> FriedFoodFridge {
        FriedFoodFridge {
            storage: HashMap::new(),
        }
    }
    /*
        If the inventory doesn't have the chosen entry, the inputted amount is set to 8 (Max number of bags of Fries allowed).
        Otherwise, it will add or subtract the inputted amount from the current amount.
     */
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) {
        self.storage
            .entry(match container {
                FoodBags::BagOfFries(_) => container,
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
    use super::{FriedFoodFridge, Inventory, UpdateToggle, FoodBags};

    #[test]
    fn create_inventory() {
        let inventory = FriedFoodFridge::initiate_inventory();
        let inventory_test = FriedFoodFridge {
            storage: HashMap::new(),
        };

        assert_eq!(inventory, inventory_test);
    }

    #[test]
    fn update_inventory() {
        let mut inventory = FriedFoodFridge::initiate_inventory();
        let normal_medium_fry = Fries::create(FryKinds::Normal, FrySizes::Medium, FoodStates::Cooked);

        let fry_bag = FoodBags::BagOfFries(normal_medium_fry);
        let fry_bag_2 = FoodBags::BagOfFries(normal_medium_fry);

        inventory.update_inventory(fry_bag,100, UpdateToggle::Add);
        let amount = inventory.storage.get(&fry_bag_2).unwrap();

        assert_eq!(*amount, 8);
    }
}