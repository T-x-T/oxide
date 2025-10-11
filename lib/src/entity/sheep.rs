use super::*;

#[derive(Debug)]
pub struct Sheep {
  pub common: CommonEntity,
  pub mob: CommonMob,
}

impl CreatableEntity for Sheep {
  fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
    let mob = CommonMob::from_nbt(extra_nbt);

    return Self { common: data, mob };
  }
}

impl SaveableEntity for Sheep {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![];
  }
}

impl Entity for Sheep {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:sheep");
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

  fn is_mob(&self) -> bool {
    return true;
  }

  fn get_mob_data(&self) -> &CommonMob {
    return &self.mob;
  }

  fn get_mob_data_mut(&mut self) -> &mut CommonMob {
    return &mut self.mob;
  }

  fn set_mob_data(&mut self, common_mob_data: CommonMob) {
    self.mob = common_mob_data;
  }

  //(height, width) https://minecraft.wiki/w/Hitbox
  fn get_hitbox(&self) -> (f64, f64) {
    return (1.3, 0.9);
  }
}
