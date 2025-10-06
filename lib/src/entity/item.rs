use super::*;

#[derive(Debug, Clone)]
pub struct ItemEntity {
  pub position: EntityPosition,
  pub uuid: u128,
  pub entity_id: i32,
  pub age: i16,
  pub health: i16,
  pub item: Item,
  pub owner: u128,
  pub pickup_delay: i16,
  pub thrower: u128,
}

impl CreatableEntity for ItemEntity {
  fn new(position: EntityPosition, uuid: u128, entity_id: i32, extra_nbt: NbtListTag) -> Self {
    return Self {
      position, uuid, entity_id,
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
        (self.get_uuid() >> 96) as i32,
        (self.get_uuid() << 32 >> 96) as i32,
        (self.get_uuid() << 64 >> 96) as i32,
        (self.get_uuid() << 96 >> 96) as i32,
      ]),
      NbtTag::Short("PickupDelay".to_string(), self.pickup_delay),
      NbtTag::IntArray("Thrower".to_string(), vec![
        (self.get_uuid() >> 96) as i32,
        (self.get_uuid() << 32 >> 96) as i32,
        (self.get_uuid() << 64 >> 96) as i32,
        (self.get_uuid() << 96 >> 96) as i32,
      ]),
    ];
  }
}

impl Entity for ItemEntity {
  fn get_type(&self) -> i32 {
    return data::entities::get_id_from_name("minecraft:item");
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

  fn set_position(&mut self, position: EntityPosition) {
    self.position = position;
  }
}
