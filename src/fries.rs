///Logic for the creation of Fries
#[derive(Debug, PartialEq)]
pub enum FryKinds {
    Normal,
    Curly,
    Waffle,
}

#[derive(Debug, PartialEq)]
pub enum FrySizes {
    Small,
    Medium,
    Large,
}
#[derive(Debug, PartialEq)]
pub struct Fries {
    kind: FryKinds,
    size: FrySizes,
}

impl Fries {
    pub fn create(kind: FryKinds, size: FrySizes) -> Fries {
        Fries {
            kind: kind,
            size: size,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::fries::{Fries, FryKinds, FrySizes};

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