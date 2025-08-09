///Contains all Possible Cooked Items
use crate::foods::FoodStates;
use crate::foods::fries::{Fries, FryKinds, FrySizes};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CookedItems{
    CookedNormalFries(Fries),
    CookedCurlyFries(Fries),
    CookedWaffleFries(Fries),
    CookedTBD,
}

impl CookedItems {
    pub fn cook_normal_fries() -> Self {
        Self::CookedNormalFries(Fries::create(FryKinds::Normal,FrySizes::None,FoodStates::Cooked))
    }
    pub fn cook_curly_fries() -> Self {
        Self::CookedCurlyFries(Fries::create(FryKinds::Curly,FrySizes::None,FoodStates::Cooked))
    }
    pub fn cook_waffle_fries() -> Self {
        Self::CookedWaffleFries(Fries::create(FryKinds::Waffle,FrySizes::None,FoodStates::Cooked))
    }
    pub fn cook_test() -> Self {
        Self::CookedTBD
    }
}