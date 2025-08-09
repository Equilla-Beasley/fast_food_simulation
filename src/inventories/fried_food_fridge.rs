///Logic for the usage of the Fridge for Fried Foods
use std::collections::HashMap;
use super::{FoodBags, Inventory, UpdateToggle};

const MAX_BAGS: u32 = 8;

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
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) -> Result<(),&str>{
        match container {
            FoodBags::BagOfNormalFries(_) => {}
            FoodBags::BagOfCurlyFries(_) => {}
            FoodBags::BagOfWaffleFries(_) => {}
            _ => return Err("Incorrect Bag of Food inputted into Fridge"),
        }
        
        self.storage
            .entry(container)
            .and_modify(|x| match toggle {
                UpdateToggle::Add => {
                    if *x + amount < MAX_BAGS {
                        *x += amount
                    }
                },
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(8);
        Ok(())
    }
    fn check_stock(&mut self, container: &Self::Container) -> &u32 {
        self.storage.get(container).unwrap_or(&(0u32))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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

        let fry_bag = FoodBags::bag_of_normal_fries();
        let fry_bag_2 = fry_bag.clone();

        inventory.update_inventory(fry_bag, 100, UpdateToggle::Add).expect("Incorrect Bag of Food inputted into Fridge");
        let amount = inventory.storage.get(&fry_bag_2).unwrap();

        assert_eq!(*amount, 8);
    }
}