pub mod world;
pub mod position;

pub use world::*;
pub use position::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardinalDirection {
  North,
  East,
  South,
  West,
}
