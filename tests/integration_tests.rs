use fast_food_simulation::employees::{Builder};
use fast_food_simulation::employees::fry_station::FryBuilder;
use fast_food_simulation::foods::{FoodStates};
use fast_food_simulation::foods::cooked_items::CookedItems;
use fast_food_simulation::foods::fries::{Fries, FryKinds, FrySizes};
use fast_food_simulation::inventories::fried_food_fridge::FriedFoodFridge;
use fast_food_simulation::inventories::fry_staging_area::FryStagingArea;
use fast_food_simulation::inventories::{FoodBags, Inventory, UpdateToggle};
use fast_food_simulation::inventories::deep_freezer::DeepFreezer;
use fast_food_simulation::menu::Menu;

///Integration Tests
//TODO: Goal: Create More Integration Tests

//This test is currently very inefficient and will become better once inventories.rs and foods.rs are updated
#[test]
fn order_fries_and_update_inventory() {
    let mut fry_staging_area = FryStagingArea::initiate_inventory();
    let mut fridge = FriedFoodFridge::initiate_inventory();
    let mut underpaid_worker = FryBuilder::default();

    //Stock Fridge and Staging Area with Curly Fries
    fridge.update_inventory(FoodBags::bag_of_curly_fries(), 5, UpdateToggle::Add).expect("Incorrect Bag of Fries chosen");
    let mut num_curly_fries_in_fridge = fridge.check_stock(&FoodBags::bag_of_curly_fries());
    assert_eq!(*num_curly_fries_in_fridge, 8, "{} Bags of Curly Fries were added to the fridge instead of 5", num_curly_fries_in_fridge);
    //TODO: This should be 5 bags of fries, but fries_food_fridge.rs needs to be edited to achieve this.

    fry_staging_area.update_inventory(CookedItems::cook_curly_fries(), 100, UpdateToggle::Add).expect("Incorrect Fries added");
    let mut num_curly_fries_in_staging = fry_staging_area.check_stock(&CookedItems::cook_curly_fries());
    assert_eq!(*num_curly_fries_in_staging, 100, "{} Curly Fries were added to staging area instead of 100", num_curly_fries_in_staging);

    //Order of Fries comes in
    Menu::order_curly_fries(&mut underpaid_worker);
    let curly_fry_order = underpaid_worker.prepare();
    assert_eq!(curly_fry_order, Fries::create(FryKinds::Curly, FrySizes::Medium, FoodStates::Cooked), "Unpaid Worker failed to make correct Curly Fries");

    //Adjust Inventory (subtract from staging area, and a bag of curly fries must be taken out to be cooked)
    fry_staging_area.update_inventory(CookedItems::cook_curly_fries(), 50, UpdateToggle::Subtract).expect("Incorrect Cooked Item added to Fry Staging Area");
    num_curly_fries_in_staging = fry_staging_area.check_stock(&CookedItems::cook_curly_fries());
    assert_eq!(*num_curly_fries_in_staging, 50, "{} Curly Fries were subtracted from the staging area instead of 50", num_curly_fries_in_staging);

    fridge.update_inventory(FoodBags::bag_of_curly_fries(), 1, UpdateToggle::Subtract).expect("Incorrect Bag of Fries Added");
    num_curly_fries_in_fridge = fridge.check_stock(&FoodBags::bag_of_curly_fries());
    assert_eq!(*num_curly_fries_in_fridge, 7, "{} Bags of Curly Fries were subtracted from the fridge instead of 1", num_curly_fries_in_fridge);
    //TODO: This should be 4 bags of fries, but fries_food_fridge.rs needs to be edited to achieve this.

    //Order Complete!
}

//TODO: Decide what to do with this integration test
#[test]
fn improved_integration_test() {
    let mut deep_freezer = DeepFreezer::initiate_inventory(); //Good
    let mut underpaid_employee = FryBuilder::default(); //Good
    //deep_freezer.refill_inventory(); //What I want
}