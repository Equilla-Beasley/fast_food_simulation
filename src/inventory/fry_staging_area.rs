///Logic for the Staging Area for Fries
use std::collections::HashMap;
use crate::food::fries::Fries;
use super::{Inventory, UpdateToggle};

#[derive(Debug, PartialEq)]
pub struct FryStagingArea {
    storage: HashMap<Fries,u32>,
}

impl Inventory for FryStagingArea{
    type Container = Fries;
    fn initiate_inventory() -> FryStagingArea {
        FryStagingArea {
            storage: HashMap::new(),
        }
    }
    /*
        If the inventory doesn't have the chosen entry, the inputted amount is set to a max value.
        Otherwise, it will add or subtract the inputted amount from the current amount.
     */
    fn update_inventory(&mut self, container: Self::Container, amount: u32, toggle: UpdateToggle) {
        self.storage
            .entry(container)
            .and_modify(|x| match toggle {
                UpdateToggle::Add => *x += amount,
                UpdateToggle::Subtract => *x -= amount,
            })
            .or_insert(100);
    }
    fn check_stock(&mut self, container: &Self::Container) -> &u32 {
        self.storage.get(container).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::food::fries::{Fries, FryKinds, FrySizes};
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
        let normal_fries = Fries::create(FryKinds::Normal, FrySizes::None);
        let normal_fries_2 = normal_fries.clone();

        inventory.update_inventory(normal_fries,100, UpdateToggle::Add);
        let amount = inventory.storage.get(&normal_fries_2).unwrap();

        assert_eq!(*amount, 100);
    }
}