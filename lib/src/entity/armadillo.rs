use super::*;

#[derive(Debug)]
pub struct Armadillo {
  pub common: CommonEntity,
  pub mob: CommonMob,
}

impl CreatableEntity for Armadillo {
  fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
    let mob = CommonMob::from_nbt(extra_nbt);

    return Self { common: data, mob };
  }
}

impl SaveableEntity for Armadillo {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![];
  }
}

impl Entity for Armadillo {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:armadillo");
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
