pub mod fry_station;

///Logic for the Usage of Builders
pub trait Builder: Default {
    type OutputFood;
    fn prepare(self) -> Self::OutputFood;
}