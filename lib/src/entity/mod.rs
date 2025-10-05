use crate::types::*;

pub mod armadillo;
pub mod cat;
pub mod chicken;
pub mod cow;
pub mod creeper;
pub mod horse;
pub mod item;
pub mod rabbit;
pub mod sheep;

pub use armadillo::*;
pub use cat::*;
pub use chicken::*;
pub use cow::*;
pub use creeper::*;
pub use horse::*;
pub use item::*;
pub use rabbit::*;
pub use sheep::*;

pub struct CommonEntity {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}
