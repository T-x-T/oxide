use super::*;

#[derive(Debug, Clone)]
pub struct Creeper {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}

impl SaveableEntity for Creeper {
  fn to_nbt(&self) -> NbtListTag {
    return NbtListTag::TagCompound(vec![
      NbtTag::String("Id".to_string(), "minecraft:creeper".to_string()),
      NbtTag::TagCompound("Pos".to_string(), vec![
        NbtTag::Double("X".to_string(), self.x),
        NbtTag::Double("Y".to_string(), self.y),
        NbtTag::Double("Z".to_string(), self.z),
      ]),
      NbtTag::TagCompound("Rotation".to_string(), vec![
        NbtTag::Float("0".to_string(), self.yaw),
        NbtTag::Float("1".to_string(), self.pitch),
      ]),
      NbtTag::IntArray("UUID".to_string(), vec![
        (self.uuid >> 96) as i32,
        (self.uuid << 32 >> 96) as i32,
        (self.uuid << 64 >> 96) as i32,
        (self.uuid << 96 >> 96) as i32,
      ]),
    ]);
  }
}

impl Entity for Creeper {
  fn get_type(&self) -> i32 {
    return 30;
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

  fn get_position(&self) -> Position {
	 	return Position {
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
}
