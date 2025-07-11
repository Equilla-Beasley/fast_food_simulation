//Connects code for all the builders
pub mod fries;

use crate::products::fries::Fries;
use crate::products::ingredients::{Fry, FryType};

//Builder Blueprints
pub trait FryBuilder {
    fn set_fry_type(&mut self, fry_type: FryType);
    fn set_fry(&mut self, kind: Fry);
    fn build(self) -> Fries;
}