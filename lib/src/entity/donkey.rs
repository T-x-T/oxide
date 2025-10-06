use super::*;

#[derive(Debug, Clone)]
pub struct Donkey {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub uuid: u128,
  pub entity_id: i32,
}

impl CreatableEntity for Donkey {
  fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, uuid: u128, entity_id: i32) -> Self {
    return Self { x, y, z, yaw, pitch, uuid, entity_id };
  }
}

impl SaveableEntity for Donkey {}

impl Entity for Donkey {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:donkey");
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
