//This module use the builders to create specific orders that the client can call upon
use crate::{
    builders::FryBuilder,
    products::ingredients::*,
    products::ingredients::Fry::*,
};

//The Register knows each kind of item that can be ordered
//and therefore is the struct that is called for different
//food orders.
pub struct Register;

impl Register {
    //Fries
    pub fn make_small_fry(builder: &mut impl FryBuilder) {
        builder.set_fry_type(FryType::Small);
        builder.set_fry(Normal);
    }
    
    pub fn make_medium_fry(builder: &mut impl FryBuilder) {
        builder.set_fry_type(FryType::Medium);
        builder.set_fry(Normal);
        
    }
    
    pub fn make_large_fry(builder: &mut impl FryBuilder) {
        builder.set_fry_type(FryType::Large);
        builder.set_fry(Normal);
    }
    
    pub fn make_curly_fry(builder: &mut impl FryBuilder) {
        builder.set_fry_type(FryType::Curly);
        builder.set_fry(Normal);
    }
    
    //Burgers
}