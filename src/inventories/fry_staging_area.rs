///Logic for the Staging Area for Fries
use std::collections::HashMap;
use crate::foods::cooked_items::CookedItems;
use super::{Inventory, UpdateToggle};

const MAX_FRIES: u32 = 2000;

#[derive(Debug, PartialEq)]
pub struct FryStagingArea {
    storage: HashMap<CookedItems,u32>,
}

impl Inventory for FryStagingArea{
    type Container = CookedItems;
    fn initiate_inventory() -> FryStagingArea {
        FryStagingArea {
            storage: HashMap::new(),
        }
    }
    /*
        If the inventory doesn't have the chosen entry, the inputted amount is set to a max value.
        Otherwise, it will add or subtract the inputted amount from the current amount.
     */
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) -> Result<(),&str>{
        match container {
            CookedItems::CookedNormalFries(_) => {},
            CookedItems::CookedCurlyFries(_) => {},
            CookedItems::CookedWaffleFries(_) => {},
            _ => return Err("Incorrect Cooked Item inputted into Fry Staging Area"),
        }

        self.storage
            .entry(container)
            .and_modify(|x| match toggle {
                UpdateToggle::Add => {
                    if *x + amount < MAX_FRIES {
                        *x += amount
                    }
                },
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(100);
        Ok(())
    }
    //Returns stock of a given item. If the item is not in stock, returns 0
    fn check_stock(&mut self, container: &Self::Container) -> &u32 {
        self.storage.get(container).unwrap_or(&(0u32))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::foods::cooked_items::CookedItems;
    use super::{FryStagingArea, Inventory, UpdateToggle};

    #[test]
    fn create_inventory() {
        let inventory = FryStagingArea::initiate_inventory();
        let inventory_test = FryStagingArea {
            storage: HashMap::new(),
        };

        assert_eq!(inventory, inventory_test);
    }

    #[test]
    fn update_inventory() {
        let mut inventory = FryStagingArea::initiate_inventory();
        let normal_fries = CookedItems::cook_normal_fries();
        let normal_fries_2 = normal_fries.clone();

        inventory.update_inventory(normal_fries, 100, UpdateToggle::Add).expect("Incorrect Cooked Item inputted into Fry Staging Area");
        let amount = inventory.storage.get(&normal_fries_2).unwrap();
        assert_eq!(*amount, 100);
    }

    //TODO: Figure out how to correctly write this test
    #[test]
    fn input_wrong_cooked_food() {
        let mut inventory = FryStagingArea::initiate_inventory();
        let test = CookedItems::cook_test();
        let result =inventory.update_inventory(test, 100, UpdateToggle::Add).expect("Incorrect Cooked Item inputted into Fry Staging Area");
        assert_eq!(result, ());
    }
}