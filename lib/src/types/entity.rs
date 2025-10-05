use crate::types::*;

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
