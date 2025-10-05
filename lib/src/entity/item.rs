use super::*;

#[derive(Debug, Clone)]
pub struct ItemEntity {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}

impl CreatableEntity for ItemEntity {
  fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, uuid: u128, entity_id: i32) -> Self {
    return Self { x, y, z, yaw, pitch, uuid, entity_id };
  }
}

impl SaveableEntity for ItemEntity {
  fn to_nbt(&self) -> NbtListTag {
    return NbtListTag::TagCompound(vec![
      NbtTag::String("id".to_string(), "minecraft:item".to_string()),
      NbtTag::List("Pos".to_string(), vec![
        NbtListTag::Double(self.x),
        NbtListTag::Double(self.y),
        NbtListTag::Double(self.z),
      ]),
      NbtTag::List("Rotation".to_string(), vec![
        NbtListTag::Float(self.yaw),
        NbtListTag::Float(self.pitch),
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

impl Entity for ItemEntity {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:item");
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

impl ItemEntity {
  pub fn from_nbt(value: NbtListTag, next_entity_id: i32) -> Self {
    return Self {
      x: value.get_child("Pos").unwrap().as_list()[0].as_double(),
      y: value.get_child("Pos").unwrap().as_list()[1].as_double(),
      z: value.get_child("Pos").unwrap().as_list()[2].as_double(),
      yaw: value.get_child("Rotation").unwrap().as_list()[0].as_float(),
      pitch: value.get_child("Rotation").unwrap().as_list()[1].as_float(),
      uuid: value.get_child("UUID").unwrap().as_int_array()
        .into_iter()
        .enumerate()
        .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
        .reduce(|a, b| a | b)
        .unwrap(),
      entity_id: next_entity_id,
    };
  }
}
