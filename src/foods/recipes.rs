///Holds the Recipes for all Food Items
use crate::foods::FoodStates;
use crate::foods::fries::{Fries, FryKinds, FrySizes};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Recipes {
    SmallFries(Fries),
    MediumFries(Fries),
    LargeFries(Fries),
    CurlyFries(Fries),
    WaffleFries(Fries),
}

impl Recipes {
    pub fn small_fries() -> Self {
        Self::SmallFries(Fries::create(FryKinds::Normal,FrySizes::Small,FoodStates::Cooked))
    }
    pub fn medium_fries() -> Self {
        Self::MediumFries(Fries::create(FryKinds::Normal,FrySizes::Medium,FoodStates::Cooked))
    }
    pub fn large_fries() -> Self {
        Self::LargeFries(Fries::create(FryKinds::Normal,FrySizes::Large,FoodStates::Cooked))
    }
    pub fn curly_fries() -> Self {
        Self::CurlyFries(Fries::create(FryKinds::Curly,FrySizes::Medium,FoodStates::Cooked))
    }
    pub fn waffle_fries() -> Self {
        Self::WaffleFries(Fries::create(FryKinds::Waffle,FrySizes::Medium,FoodStates::Cooked))
    }
}

#[cfg(test)]
mod tests {
    use crate::employees::Builder;
    use crate::employees::fry_station::FryBuilder;
    use crate::foods::recipes::Recipes;
    use crate::menu::Menu;

    fn small_fries_employee_vs_recipe() {
        let small_fries = Recipes::small_fries();
        let mut underpaid_worker = FryBuilder::default();

        Menu::order_small_fries(&mut underpaid_worker);

        let small_fries_2 = underpaid_worker.prepare();

        assert_eq!(small_fries,Recipes::SmallFries(small_fries_2));
    }
}