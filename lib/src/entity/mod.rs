use std::collections::HashMap;

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

#[derive(Debug, Default)]
pub struct CommonEntity {
  pub position: EntityPosition,
  pub velocity: EntityPosition,
  pub uuid: u128,
  pub entity_id: i32,
  pub air: i16,
  pub custom_name: NbtTag,
  pub custom_name_visible: bool,
  pub data: NbtTag,
  pub fall_distance: f64,
  pub ticks_until_fire_is_out: i16,
  pub is_glowing: bool,
  pub has_visual_fire: bool,
  pub invulnerable: bool,
  pub no_gravity: bool,
  pub on_ground: bool,
  pub passengers: Vec<Box<dyn SaveableEntity + Send>>,
  pub portal_cooldown: i32,
  pub is_silent: bool,
  pub scoreboard_tags: Vec<NbtListTag>,
  pub ticks_frozen: i32,
}

#[derive(Debug, Default)]
pub struct CommonMob {
  pub absorption_amount: f32,
  pub active_effects: Vec<NbtListTag>,
  pub attributes: Vec<NbtListTag>,
  pub brain: NbtTag,
  pub can_pick_up_loot: bool,
  pub death_loot_table: Option<String>,
  pub death_loot_table_seed: Option<i64>,
  pub death_time: i16,
  pub drop_chances: NbtTag,
  pub equipment: HashMap<String, Item>,
  pub fall_flying: u8,
  pub health: f32,
  pub home_location: (i32, i32, i32),
  pub home_radius: i32,
  pub hurt_by_timestamp: i32,
  pub hurt_time: i16,
  pub leashed_block: Option<(i32, i32, i32)>,
  pub leashed_entity: Option<u128>,
  pub is_left_handed: bool,
  pub locator_bar_icon_color: Option<i32>,
  pub locator_bar_icon_style: Option<String>,
  pub has_no_ai: bool,
  pub is_persistance_required: bool,
  pub sleeping_location: Option<(i32, i32, i32)>,
  pub team: Option<String>,
  pub alive_for_ticks: i32,
  pub wander_to: Option<BlockPosition>,
  pub wandered_for: u16,
}
