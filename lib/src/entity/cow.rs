use super::*;

#[derive(Debug, Clone)]
pub struct Cow {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}

impl CreatableEntity for Cow {
  fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, uuid: u128, entity_id: i32, _extra_nbt: NbtListTag) -> Self {
    return Self { x, y, z, yaw, pitch, uuid, entity_id };
  }
}

impl SaveableEntity for Cow {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![];
  }
}

impl Entity for Cow {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:cow");
  }

  fn get_x(&self) -> f64 {
  	return self.x;
  }

  fn get_y(&self) -> f64 {
  	return self.y;
  }

  fn get_z(&self) -> f64 {
  	return self.z;
  }

  fn get_yaw(&self) -> f32 {
  	return self.yaw;
  }

  fn get_pitch(&self) -> f32 {
  	return self.pitch;
  }

  fn get_position(&self) -> BlockPosition {
	 	return BlockPosition {
		  x: self.get_x() as i32,
			y: self.get_y() as i16,
			z: self.get_z() as i32,
	  };
  }

  fn get_uuid(&self) -> u128 {
    return self.uuid;
  }

  fn get_id(&self) -> i32 {
    return self.entity_id;
  }

  fn get_metadata(&self) -> Vec<EntityMetadata> {
    return Vec::new();
  }

  fn set_yaw(&mut self, yaw: f32) {
    self.yaw = yaw;
  }

  fn set_pitch(&mut self, pitch: f32) {
    self.pitch = pitch;
  }

  fn set_position(&mut self, position: BlockPosition) {
    self.x = position.x as f64;
    self.y = position.y as f64;
    self.z = position.z as f64;
  }
}
