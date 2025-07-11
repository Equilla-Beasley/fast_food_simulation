#![allow(unused)]

mod builders;
mod products;
mod register;

use builders::FryBuilder;
use products::fries::Fries;
use register::Register;

fn main() {
    //creates a default, abstract builder (aka a builder that has knowledge but no current purpose)
    let mut fry_builder = builders::fries::FriesBuilder::default();
    
    //Register creates a concrete (aka takes a builder and gives it a purpose)
    Register::make_curly_fry(&mut fry_builder);
    
    //Tests
    let curly_fry = fry_builder.build();
    println!("What type of fries were created? {:?}\n",curly_fry.fry_type());
    
}
