//Establishes recipes for fries

use crate::products::ingredients::{FryType, Fry};

pub struct Fries {
    fry_type: FryType,
    kind: Fry,
}

impl Fries {
    //Constructor
    pub fn new (fry_type: FryType, kind: Fry) -> Self {
        Self {
            fry_type,
            kind,
        }
    }

    //Getters
    pub fn fry_type(&self) -> FryType {
        self.fry_type
    }

    pub fn kind(&self) -> &Fry {
        &self.kind
    }
}