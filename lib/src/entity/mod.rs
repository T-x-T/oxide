use crate::types::*;
use crate::packets::clientbound::play::{EntityMetadata, EntityMetadataValue};

pub mod armadillo;
pub mod cat;
pub mod chest_minecart;
pub mod chicken;
pub mod cow;
pub mod creeper;
pub mod donkey;
pub mod horse;
pub mod item;
pub mod parrot;
pub mod pig;
pub mod rabbit;
pub mod sheep;

pub use armadillo::*;
pub use cat::*;
pub use chest_minecart::*;
pub use chicken::*;
pub use cow::*;
pub use creeper::*;
pub use donkey::*;
pub use horse::*;
pub use item::*;
pub use parrot::*;
pub use pig::*;
pub use rabbit::*;
pub use sheep::*;

pub struct CommonEntity {
  pub position: EntityPosition,
  pub uuid: u128,
  pub entity_id: i32,
}
