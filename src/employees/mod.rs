///Logic for the Usage of Builders
//TODO: Turn Employees into Threads
pub mod fry_station;

pub trait Builder: Default {
    type OutputFood;
    fn prepare(self) -> Self::OutputFood;
}