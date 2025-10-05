use crate::types::*;
use crate::entity::*;

pub trait CreatableEntity {
  fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, uuid: u128, entity_id: i32) -> Self;
}

pub trait Entity: std::fmt::Debug {
  fn get_type(&self) -> i32;
  fn get_x(&self) -> f64;
  fn get_y(&self) -> f64;
  fn get_z(&self) -> f64;
  fn get_yaw(&self) -> f32;
  fn get_pitch(&self) -> f32;
  fn get_position(&self) -> Position;
  fn get_uuid(&self) -> u128;
  fn get_id(&self) -> i32;
  fn get_yaw_u8(&self) -> u8 {
    return if self.get_yaw() < 0.0 {
      (((self.get_yaw() / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_yaw() / 90.0) * 64.0) as u8
    };
  }

  fn get_pitch_u8(&self) -> u8 {
    return if self.get_pitch() < 0.0 {
      (((self.get_pitch() / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_pitch() / 90.0) * 64.0) as u8
    };
  }
}

pub trait SaveableEntity: Entity + Send {
  fn to_nbt(&self) -> NbtListTag;
}

pub fn new(entity_type: &str, x: f64, y: f64, z: f64, yaw: f32, pitch: f32, uuid: u128, entity_id: i32) -> Option<Box<dyn SaveableEntity + Send>> {
  return match entity_type {
	  "minecraft:armadillo" => Some(Box::new(Armadillo::new(x, y, z, yaw, pitch, uuid, entity_id))),
	  "minecraft:cat" => Some(Box::new(Cat::new(x, y, z, yaw, pitch, uuid, entity_id))),
	  "minecraft:chest_minecart" => Some(Box::new(ChestMinecart::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:chicken" => Some(Box::new(Chicken::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:cow" => Some(Box::new(Cow::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:creeper" => Some(Box::new(Creeper::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:donkey" => Some(Box::new(Donkey::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:horse" => Some(Box::new(Horse::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:item" => Some(Box::new(ItemEntity::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:parrot" => Some(Box::new(Parrot::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:pig" => Some(Box::new(Pig::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:rabbit" => Some(Box::new(Rabbit::new(x, y, z, yaw, pitch, uuid, entity_id))),
    "minecraft:sheep" => Some(Box::new(Sheep::new(x, y, z, yaw, pitch, uuid, entity_id))),
			_ => None,
  };
}
