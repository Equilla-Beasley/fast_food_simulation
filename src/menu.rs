///Stores all Menu Items and the Recipes for them
use crate::employees::fry_station::FryBuilder;
use crate::foods::FoodStates;
use crate::foods::fries::{FryKinds, FrySizes};

pub struct Menu {}

impl Menu {
    //Fried Items
    pub fn order_small_fries(builder: &mut FryBuilder) {
        builder.set_kind(FryKinds::Normal);
        builder.set_size(FrySizes::Small);
        builder.set_state(FoodStates::Cooked);
    }
    pub fn order_medium_fries(builder: &mut FryBuilder) {
        builder.set_kind(FryKinds::Normal);
        builder.set_size(FrySizes::Medium);
        builder.set_state(FoodStates::Cooked);
    }
    pub fn order_large_fries(builder: &mut FryBuilder) {
        builder.set_kind(FryKinds::Normal);
        builder.set_size(FrySizes::Large);
        builder.set_state(FoodStates::Cooked);
    }
    pub fn order_curly_fries(builder: &mut FryBuilder) {
        builder.set_kind(FryKinds::Curly);
        builder.set_size(FrySizes::Medium);
        builder.set_state(FoodStates::Cooked);
    }
    pub fn order_waffle_fries(builder: &mut FryBuilder) {
        builder.set_kind(FryKinds::Waffle);
        builder.set_size(FrySizes::Medium);
        builder.set_state(FoodStates::Cooked);
    }
}

#[cfg(test)]
mod tests {
    use crate::employees::Builder;
    use crate::employees::fry_station::FryBuilder;
    use crate::foods::FoodStates;
    use crate::foods::fries::{Fries, FryKinds, FrySizes};
    use crate::menu;

    #[test]
    fn order_medium_fries(){
        let mut underpaid_worker = FryBuilder::default();
        menu::Menu::order_medium_fries(&mut underpaid_worker);

        let normal_medium_fries = underpaid_worker.prepare();
        let normal_medium_fries_2 = Fries::create(FryKinds::Normal, FrySizes::Medium, FoodStates::Cooked);

        assert_eq!(normal_medium_fries, normal_medium_fries_2);
    }
}
