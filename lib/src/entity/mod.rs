use crate::types::*;

pub mod cat;
pub mod creeper;

pub use cat::*;
pub use creeper::*;

pub struct CommonEntity {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}
