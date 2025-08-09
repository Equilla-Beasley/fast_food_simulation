///Logic for the usage of the Deep Freezer
use std::collections::HashMap;
use super::{FoodBoxes, Inventory, UpdateToggle};

const MAX_BOXES: u32 = 12;
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
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) -> Result<(),&str> {
        match container {
            FoodBoxes::BoxOfNormalFries(_) => {}
            FoodBoxes::BoxOfCurlyFries(_) => {}
            FoodBoxes::BoxOfWaffleFries(_) => {}
            _ => return Err("Incorrect Food Box inputted into Deep Freezer"),
        }
        
        self.storage
            .entry(container)
            .and_modify(|x| match toggle {
                UpdateToggle::Add => {
                    if *x + amount < MAX_BOXES {
                        *x += amount
                    }
                },
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(12);
        Ok(())
    }

    fn check_stock(&mut self, container: &Self::Container) -> &u32 {
        self.storage.get(container).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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

        let fry_box = FoodBoxes::box_of_normal_fries();
        let fry_box_2 = fry_box.clone();

        inventory.update_inventory(fry_box,100, UpdateToggle::Add).expect("Incorrect Food Box inputted into Deep Freezer");
        let amount = inventory.storage.get(&fry_box_2).unwrap();

        assert_eq!(*amount, 12);
    }
}