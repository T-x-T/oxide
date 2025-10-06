use super::*;

#[derive(Debug, Clone)]
pub struct Chicken {
  pub position: EntityPosition,
  pub uuid: u128,
  pub entity_id: i32,
}

impl CreatableEntity for Chicken {
  fn new(position: EntityPosition, uuid: u128, entity_id: i32, _extra_nbt: NbtListTag) -> Self {
    return Self { position, uuid, entity_id };
  }
}

impl SaveableEntity for Chicken {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![];
  }
}

impl Entity for Chicken {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:chicken");
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
}
