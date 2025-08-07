///Logic for the creation of Fries
use super::Food;

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
}
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Fries{
    kind: FryKinds,
    size: FrySizes,
}

impl Fries {
    pub fn create(kind: FryKinds, size: FrySizes) -> Fries {
        Fries {
            kind,
            size,
        }
    }
}
impl Food for Fries {}

#[cfg(test)]
mod tests {
    use super::Food;
    use super::{Fries, FryKinds, FrySizes};

    #[test]
    fn create_fries() {
        let test_fry = Fries::create(FryKinds::Normal, FrySizes::Medium);
        let test_fry_direct = Fries {
            kind: FryKinds::Normal,
            size: FrySizes::Medium,
        };

        assert_eq!(test_fry, test_fry_direct);
    }
}