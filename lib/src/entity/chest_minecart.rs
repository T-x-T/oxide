use super::*;

#[derive(Debug, Clone)]
pub struct ChestMinecart {
  pub position: EntityPosition,
  pub velocity: EntityPosition,
  pub uuid: u128,
  pub entity_id: i32,
}

impl CreatableEntity for ChestMinecart {
  fn new(position: EntityPosition, velocity: EntityPosition, uuid: u128, entity_id: i32, _extra_nbt: NbtListTag) -> Self {
    return Self { position, velocity, uuid, entity_id };
  }
}

impl SaveableEntity for ChestMinecart {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![];
  }
}

impl Entity for ChestMinecart {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:chest_minecart");
  }

  fn get_position(&self) -> EntityPosition {
	 	return self.position;
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

  fn set_position(&mut self, position: EntityPosition) {
    self.position = position;
  }

  fn get_velocity(&self) -> EntityPosition {
    return self.velocity;
  }

  fn set_velocity(&mut self, velocity: EntityPosition) {
    self.velocity = velocity;
  }
}
