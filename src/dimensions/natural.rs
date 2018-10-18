use rand::Rng;
use {BoundedSpace, Space, Card};

/// The set of all natural numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Natural;

impl Space for Natural {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> u64 { unimplemented!() }
}

impl BoundedSpace for Natural {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<u64> { Some(0) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, _: Self::BoundValue) -> bool { true }
}
