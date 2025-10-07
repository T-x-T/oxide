use super::*;

#[derive(Debug)]
pub struct Chicken {
  pub common: CommonEntity,
}

impl CreatableEntity for Chicken {
  fn new(data: CommonEntity, _extra_nbt: NbtListTag) -> Self {
    return Self { common: data };
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

  fn get_metadata(&self) -> Vec<EntityMetadata> {
    return Vec::new();
  }

  fn get_common_entity_data(&self) -> &CommonEntity {
    return &self.common;
  }

  fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity {
    return &mut self.common;
  }

  fn set_common_entity_data(&mut self, common_entity_data: CommonEntity) {
    self.common = common_entity_data;
  }
}
