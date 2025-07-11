//Concrete Builder for Fries
use crate::{
    products::fries::Fries,
    products::ingredients::{FryType,Fry},
};
use super::FryBuilder;

//creates a default value for this struct 
// that can be created
#[derive(Default)]
pub struct FriesBuilder {
    fry_type: Option<FryType>,
    kind: Option<Fry>,
}

impl FryBuilder for FriesBuilder{
    fn set_fry_type(&mut self, fry_type: FryType) {
        self.fry_type = Some(fry_type);
    }

    fn set_fry(&mut self, kind: Fry) {
        self.kind = Some(kind);
    }
    fn build(self) -> Fries {
        Fries::new (
            self.fry_type.expect("Please choose a type of fry"),
            self.kind.expect("Please choose a kind of fry"),
        )
    }
}