use super::*;

#[derive(Debug)]
pub struct ItemEntity {
  pub common: CommonEntity,
  pub age: i16,
  pub health: i16,
  pub item: Item,
  pub owner: u128,
  pub pickup_delay: i16,
  pub thrower: u128,
}

impl CreatableEntity for ItemEntity {
  fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
    return Self {
      common: data,
      age: extra_nbt.get_child("Age").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short(),
      health: extra_nbt.get_child("Health").unwrap_or(&NbtTag::Short(String::new(), 5)).as_short(),
      item: Item {
        id: extra_nbt.get_child("Item").unwrap_or(&NbtTag::TagCompound(String::new(), Vec::new())).get_child("id").unwrap_or(&NbtTag::String(String::new(), "minecraft:stone".to_string())).as_string().to_string(),
        count: extra_nbt.get_child("Item").unwrap_or(&NbtTag::TagCompound(String::new(), Vec::new())).get_child("count").unwrap_or(&NbtTag::Int(String::new(), 1)).as_int() as u8,
        components: Vec::new()
      },
      owner: extra_nbt.get_child("Owner").unwrap_or(&NbtTag::IntArray(String::new(), vec![0;4])).as_int_array()
        .into_iter()
        .enumerate()
        .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
        .reduce(|a, b| a | b)
        .unwrap(),
      pickup_delay: extra_nbt.get_child("PickupDelay").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short(),
      thrower: extra_nbt.get_child("Thrower").unwrap_or(&NbtTag::IntArray(String::new(), vec![0;4])).as_int_array()
        .into_iter()
        .enumerate()
        .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
        .reduce(|a, b| a | b)
        .unwrap(),
    };
  }
}

impl SaveableEntity for ItemEntity {
  fn to_nbt_extras(&self) -> Vec<NbtTag> {
    return vec![
      NbtTag::Short("Age".to_string(), self.age),
      NbtTag::Short("Health".to_string(), self.health),
      NbtTag::TagCompound("Item".to_string(), vec![
        NbtTag::String("id".to_string(), self.item.id.clone()),
        NbtTag::Int("count".to_string(), self.item.count as i32),
        NbtTag::TagCompound("components".to_string(), Vec::new()),
      ]),
      NbtTag::IntArray("Owner".to_string(), vec![
        (self.owner >> 96) as i32,
        (self.owner << 32 >> 96) as i32,
        (self.owner << 64 >> 96) as i32,
        (self.owner << 96 >> 96) as i32,
      ]),
      NbtTag::Short("PickupDelay".to_string(), self.pickup_delay),
      NbtTag::IntArray("Thrower".to_string(), vec![
        (self.thrower >> 96) as i32,
        (self.thrower << 32 >> 96) as i32,
        (self.thrower << 64 >> 96) as i32,
        (self.thrower << 96 >> 96) as i32,
      ]),
    ];
  }
}

impl Entity for ItemEntity {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:item");
  }

  fn get_metadata(&self) -> Vec<EntityMetadata> {
    return vec![
      EntityMetadata {
        index: 8,
        value: EntityMetadataValue::Slot(Slot {
          item_count: self.item.count as i32,
          item_id: data::items::get_items().get(&self.item.id).unwrap().id,
          components_to_add: Vec::new(),
          components_to_remove: Vec::new()
        }),
      },
    ];
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

  //(height, width) https://minecraft.wiki/w/Hitbox
  fn get_hitbox(&self) -> (f64, f64) {
    return (0.25, 0.25);
  }
}
