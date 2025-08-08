///Logic for the creation of Fries
use super::FoodStates;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FryKinds {
    Normal,
    Curly,
    Waffle,
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum FrySizes {
    Small,
    Medium,
    Large,
    None,
}
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Fries{
    kind: FryKinds,
    size: FrySizes,
    state: FoodStates,
}

impl Fries {
    pub fn create(kind: FryKinds, size: FrySizes, state: FoodStates) -> Fries {
        Fries {
            kind,
            size,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::foods::FoodStates;
    use super::{Fries, FryKinds, FrySizes};

    #[test]
    fn create_fries() {
        let test_fry = Fries::create(FryKinds::Normal, FrySizes::Medium, FoodStates::Cooked);
        let test_fry_direct = Fries {
            kind: FryKinds::Normal,
            size: FrySizes::Medium,
            state: FoodStates::Cooked,
        };

        assert_eq!(test_fry, test_fry_direct);
    }
}