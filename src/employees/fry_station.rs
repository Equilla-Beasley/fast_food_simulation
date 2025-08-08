///Builder used to prepare Fries
use super::Builder;
use crate::foods::FoodStates;
use crate::foods::fries::{Fries, FryKinds, FrySizes};

#[derive(Default)]

pub struct FryBuilder{
    kind: Option<FryKinds>,
    size: Option<FrySizes>,
    state: Option<FoodStates>
}

impl FryBuilder {
    pub fn set_kind(&mut self, kind: FryKinds) {
        self.kind = Some(kind);
    }
    pub fn set_size(&mut self, size: FrySizes) {
        self.size = Some(size);
    }
    pub fn set_state(&mut self, state: FoodStates) {
        self.state = Some(state);
    }
}

impl Builder for FryBuilder {
    type OutputFood = Fries;

    fn prepare(self) -> Self::OutputFood {
        Fries::create(
            self.kind.expect("Incorrect Kind of Fries Given"),
            self.size.expect("Incorrect Size of Fries Given"),
            self.state.expect("Incorrect Food State Given"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Builder, FryBuilder};
    use crate::foods::FoodStates;
    use crate::foods::fries::{Fries, FryKinds, FrySizes};

    #[test]
    fn build_fries() {
        let mut fry_builder = FryBuilder::default();
        let kind = FryKinds::Normal;
        let size = FrySizes::Medium;
        let state = FoodStates::Cooked;

        fry_builder.set_size(size);
        fry_builder.set_kind(kind);
        fry_builder.set_state(state);

        let normal_medium_fries = fry_builder.prepare();
        let normal_medium_fries_2 = Fries::create(kind,size,state);

        assert_eq!(normal_medium_fries,normal_medium_fries_2);
    }
}