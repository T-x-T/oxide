use super::*;

#[derive(Debug, Clone)]
pub struct BlockEntity {
  pub id: String,
  pub position: Position,
  pub components: Option<Vec<SlotComponent>>, //At least I think so?
  pub data: Option<BlockEntityData>,
}

#[derive(Debug, Clone)]
pub enum BlockEntityData {
  Chest(Vec<BlockEntityDataItem>),
}

#[derive(Debug, Clone)]
pub struct BlockEntityDataItem {
  pub id: String,
  pub count: u8,
  pub components: Vec<SlotComponent>,
}


impl From<BlockEntityData> for NbtTag {
  fn from(value: BlockEntityData) -> Self {
    return match value {
      BlockEntityData::Chest(block_entity_data_items) => NbtTag::TagCompound(None, block_entity_data_items.iter().map(|x| x.into()).collect()),
    };
  }
}

impl From<&BlockEntityDataItem> for NbtTag {
  fn from(value: &BlockEntityDataItem) -> Self {
    return NbtTag::TagCompound(None, vec![
      //NbtTag::Byte(Some("Slot".to_string()), value.slot), //doesn't seem to do anything so Im not keeping track of it for now, instead I always have the right sized vec and then just take the slot from its index
      NbtTag::String(Some("id".to_string()), value.id.clone()),
      NbtTag::Int(Some("count".to_string()), value.count as i32),
      NbtTag::TagCompound(Some("components".to_string()), Vec::new()), //TODO: missing SlotComponent to nbt conversion
    ]);
  }
}
