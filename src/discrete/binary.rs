use crate::prelude::*;
use std::{fmt};

/// Type representing binary (base-2) values.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Binary;

impl Space for Binary {
    type Value = bool;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Finite(2) }
}

impl BoundedSpace for Binary {
    fn inf(&self) -> Option<bool> { Some(false) }

    fn sup(&self) -> Option<bool> { Some(true) }

    fn contains(&self, _: bool) -> bool { true }
}

impl FiniteSpace for Binary {
    type Iter = Self::IntoIter;

    fn iter(&self) -> Self::Iter {
        self.clone().into_iter()
    }
}

impl IntoIterator for Binary {
    type Item = <Self as Space>::Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![false, true].into_iter()
    }
}

impl_union_intersect!(Binary, Binary);

impl Surjection<bool, bool> for Binary {
    fn map_onto(&self, val: bool) -> bool { val }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{0, 1}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    extern crate serde_test;
    #[cfg(feature = "serialize")]
    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn test_dim() {
        let d = Binary;

        assert_eq!(d.dim(), Dim::one());
    }

    #[test]
    fn test_card() {
        let d = Binary;

        assert_eq!(d.card(), Card::Finite(2));
    }

    #[test]
    fn test_bounds() {
        let d = Binary;

        assert_eq!(d.inf().unwrap(), false);
        assert_eq!(d.sup().unwrap(), true);

        assert!(d.contains(false));
        assert!(d.contains(true));
    }

    #[test]
    fn test_surjection() {
        let d = Binary;

        assert_eq!(d.map_onto(true), true);
        assert_eq!(d.map_onto(false), false);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Binary;

        assert_tokens(&d, &[Token::UnitStruct { name: "Binary" }]);
    }
}
