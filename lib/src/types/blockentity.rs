use std::error::Error;

use data::blocks::Type;

use super::*;

#[derive(Debug, Clone)]
pub struct BlockEntity {
  pub id: BlockEntityId,
  pub position: Position, //global position, NOT within the chunk
  pub components: Option<Vec<SlotComponent>>, //At least I think so?
  pub data: Option<BlockEntityData>,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockEntityId {
  Banner,
  Barrel,
  Beacon,
  Bed,
  Beehive,
  Bell,
  BlastFurnace,
  BrewingStand,
  BrushableBlock,
  CalibratedSculkSensor,
  Campfire,
  Chest,
  ChiseledBookshelf,
  Comperator,
  CommandBlock,
  Conduit,
  Crafter,
  CreakingHeart,
  DaylightDetector,
  DecoratedPot,
  Dispenser,
  Dropper,
  EnchantingTable,
  EnderChest,
  EndGateway,
  EndPortal,
  Furnace,
  HangingSign,
  Hopper,
  Jigsaw,
  Jukebox,
  Lectern,
  MobSpawner,
  Piston,
  ShulkerBox,
  Sign,
  Skull,
  SculkCatalyst,
  SculkSensor,
  SculkShrieker,
  Smoker,
  SoulCampfire,
  StructureBlock,
  TrappedChest,
  TrialSpawner,
  Vault,
}

impl From<BlockEntityId> for &str {
  fn from(value: BlockEntityId) -> Self {
    match value {
      BlockEntityId::Banner => "minecraft:banner",
      BlockEntityId::Barrel => "minecraft:barrel",
      BlockEntityId::Beacon => "minecraft:beacon",
      BlockEntityId::Bed => "minecraft:bed",
      BlockEntityId::Beehive => "minecraft:beehive",
      BlockEntityId::Bell => "minecraft:bell",
      BlockEntityId::BlastFurnace => "minecraft:blast_furnace",
      BlockEntityId::BrewingStand => "minecraft:brewing_stand",
      BlockEntityId::BrushableBlock => "minecraft:brushable_block",
      BlockEntityId::CalibratedSculkSensor => "minecraft:calibrated_sculk_sensor",
      BlockEntityId::Campfire => "minecraft:campfire",
      BlockEntityId::Chest => "minecraft:chest",
      BlockEntityId::ChiseledBookshelf => "minecraft:chiseled_bookshelf",
      BlockEntityId::Comperator => "minecraft:comperator",
      BlockEntityId::CommandBlock => "minecraft:command_block",
      BlockEntityId::Conduit => "minecraft:conduit",
      BlockEntityId::Crafter => "minecraft:crafter",
      BlockEntityId::CreakingHeart => "minecraft:creaking_heart",
      BlockEntityId::DaylightDetector => "minecraft:daylight_detector",
      BlockEntityId::DecoratedPot => "minecraft:decorated_pot",
      BlockEntityId::Dispenser => "minecraft:dispenser",
      BlockEntityId::Dropper => "minecraft:dropper",
      BlockEntityId::EnchantingTable => "minecraft:enchanting_table",
      BlockEntityId::EnderChest => "minecraft:ender_chest",
      BlockEntityId::EndGateway => "minecraft:end_gateway",
      BlockEntityId::EndPortal => "minecraft:end_portal",
      BlockEntityId::Furnace => "minecraft:furnace",
      BlockEntityId::HangingSign => "minecraft:hanging_sign",
      BlockEntityId::Hopper => "minecraft:hopper",
      BlockEntityId::Jigsaw => "minecraft:jigsaw",
      BlockEntityId::Jukebox => "minecraft:jukebox",
      BlockEntityId::Lectern => "minecraft:lectern",
      BlockEntityId::MobSpawner => "minecraft:mob_spawner",
      BlockEntityId::Piston => "minecraft:piston",
      BlockEntityId::ShulkerBox => "minecraft:shulker_box",
      BlockEntityId::Sign => "minecraft:sign",
      BlockEntityId::Skull => "minecraft:skull",
      BlockEntityId::SculkCatalyst => "minecraft:sculk_catalyst",
      BlockEntityId::SculkSensor => "minecraft:sculk_sensor",
      BlockEntityId::SculkShrieker => "minecraft:sculk_shrieker",
      BlockEntityId::Smoker => "minecraft:smoker",
      BlockEntityId::SoulCampfire => "minecraft:soul_campfire",
      BlockEntityId::StructureBlock => "minecraft:structure_block",
      BlockEntityId::TrappedChest => "minecraft:trapped_chest",
      BlockEntityId::TrialSpawner => "minecraft:trial_spawner",
      BlockEntityId::Vault => "minecraft:vault",
    }
  }
}

impl TryFrom<&str> for BlockEntityId {
  type Error = Box<dyn Error>;

  fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
    match value {
       "minecraft:banner" => Ok(BlockEntityId::Banner),
       "minecraft:barrel" => Ok(BlockEntityId::Barrel),
       "minecraft:beacon" => Ok(BlockEntityId::Beacon),
       "minecraft:bed" => Ok(BlockEntityId::Bed),
       "minecraft:beehive" => Ok(BlockEntityId::Beehive),
       "minecraft:bell" => Ok(BlockEntityId::Bell),
       "minecraft:blast_furnace" => Ok(BlockEntityId::BlastFurnace),
       "minecraft:brewing_stand" => Ok(BlockEntityId::BrewingStand),
       "minecraft:brushable_block" => Ok(BlockEntityId::BrushableBlock),
       "minecraft:calibrated_sculk_sensor" => Ok(BlockEntityId::CalibratedSculkSensor),
       "minecraft:campfire" => Ok(BlockEntityId::Campfire),
       "minecraft:chest" => Ok(BlockEntityId::Chest),
       "minecraft:chiseled_bookshelf" => Ok(BlockEntityId::ChiseledBookshelf),
       "minecraft:comperator" => Ok(BlockEntityId::Comperator),
       "minecraft:command_block" => Ok(BlockEntityId::CommandBlock),
       "minecraft:conduit" => Ok(BlockEntityId::Conduit),
       "minecraft:crafter" => Ok(BlockEntityId::Crafter),
       "minecraft:creaking_heart" => Ok(BlockEntityId::CreakingHeart),
       "minecraft:daylight_detector" => Ok(BlockEntityId::DaylightDetector),
       "minecraft:decorated_pot" => Ok(BlockEntityId::DecoratedPot),
       "minecraft:dispenser" => Ok(BlockEntityId::Dispenser),
       "minecraft:dropper" => Ok(BlockEntityId::Dropper),
       "minecraft:enchanting_table" => Ok(BlockEntityId::EnchantingTable),
       "minecraft:ender_chest" => Ok(BlockEntityId::EnderChest),
       "minecraft:end_gateway" => Ok(BlockEntityId::EndGateway),
       "minecraft:end_portal" => Ok(BlockEntityId::EndPortal),
       "minecraft:furnace" => Ok(BlockEntityId::Furnace),
       "minecraft:hanging_sign" => Ok(BlockEntityId::HangingSign),
       "minecraft:hopper" => Ok(BlockEntityId::Hopper),
       "minecraft:jigsaw" => Ok(BlockEntityId::Jigsaw),
       "minecraft:jukebox" => Ok(BlockEntityId::Jukebox),
       "minecraft:lectern" => Ok(BlockEntityId::Lectern),
       "minecraft:mob_spawner" => Ok(BlockEntityId::MobSpawner),
       "minecraft:piston" => Ok(BlockEntityId::Piston),
       "minecraft:shulker_box" => Ok(BlockEntityId::ShulkerBox),
       "minecraft:sign" => Ok(BlockEntityId::Sign),
       "minecraft:skull" => Ok(BlockEntityId::Skull),
       "minecraft:sculk_catalyst" => Ok(BlockEntityId::SculkCatalyst),
       "minecraft:sculk_sensor" => Ok(BlockEntityId::SculkSensor),
       "minecraft:sculk_shrieker" => Ok(BlockEntityId::SculkShrieker),
       "minecraft:smoker" => Ok(BlockEntityId::Smoker),
       "minecraft:soul_campfire" => Ok(BlockEntityId::SoulCampfire),
       "minecraft:structure_block" => Ok(BlockEntityId::StructureBlock),
       "minecraft:trapped_chest" => Ok(BlockEntityId::TrappedChest),
       "minecraft:trial_spawner" => Ok(BlockEntityId::TrialSpawner),
       "minecraft:vault" => Ok(BlockEntityId::Vault),
      _ => Err(Box::new(crate::CustomError::InvalidInput(value.to_string())))
    }
  }
}

#[derive(Debug, Clone, Default)]
pub enum BlockEntityData {
  Banner(Vec<(String, String)>), //patterns: <pattern, color>
  Chest(Vec<BlockEntityDataItem>),
  Furnace(Vec<BlockEntityDataItem>), //0: item being smelted 1: fuel 2: result
  BrewingStand(Vec<BlockEntityDataItem>), //0: left, 1: middle, 2: right, 3: ingredient, 4: fuel
  Crafter(Vec<BlockEntityDataItem>), //len 9
  Dispenser(Vec<BlockEntityDataItem>), //len 9
  Hopper(Vec<BlockEntityDataItem>), //len 5
  #[default]
  NoData, //TODO: remove when everything is implemented
}

#[derive(Debug, Clone)]
pub struct BlockEntityDataItem {
  pub id: String,
  pub count: u8,
  pub components: Vec<SlotComponent>,
}

impl Default for BlockEntityDataItem {
  fn default() -> Self {
    Self { id: "minecraft:air".to_string(), count: 0, components: Vec::new() }
  }
}

impl From<BlockEntity> for NbtListTag {
  fn from(value: BlockEntity) -> Self {
    let mut items: Vec<NbtTag> = vec![
      NbtTag::String("id".to_string(), Into::<&str>::into(value.id).to_string()),
      NbtTag::Int("x".to_string(), value.position.x),
      NbtTag::Int("y".to_string(), value.position.y as i32),
      NbtTag::Int("z".to_string(), value.position.z),
    ];

    if let Some(block_entity_data) = value.data {
      items.append(&mut block_entity_data.into());
    }

    return NbtListTag::TagCompound(items);
  }
}

impl From<BlockEntityData> for Vec<NbtTag> {
  fn from(value: BlockEntityData) -> Self {
    return match value {
      BlockEntityData::Banner(data) => vec![NbtTag::List("patterns".to_string(), data.iter().map(|x| NbtListTag::TagCompound(vec![NbtTag::String("color".to_string(), x.1.clone()), NbtTag::String("pattern".to_string(), x.0.clone())])).collect())],
      BlockEntityData::Chest(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::Furnace(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::BrewingStand(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::Crafter(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::Dispenser(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::Hopper(block_entity_data_items) => {
        vec![
          items_to_nbt(block_entity_data_items),
        ]
      },
      BlockEntityData::NoData => Vec::new(),
    };
  }
}

fn items_to_nbt(block_entity_data_items: Vec<BlockEntityDataItem>) -> NbtTag {
  return NbtTag::List("Items".to_string(), block_entity_data_items.iter().enumerate().map(|(i, item)| {
    NbtListTag::TagCompound(vec![
      NbtTag::Byte("Slot".to_string(), i as u8),
      NbtTag::String("id".to_string(), item.id.clone()),
      NbtTag::Int("count".to_string(), item.count as i32),
      NbtTag::TagCompound("components".to_string(), Vec::new()), //TODO: missing SlotComponent to nbt conversion
    ])
  }).collect());
}

impl From<&BlockEntityDataItem> for Slot {
  fn from(value: &BlockEntityDataItem) -> Self {
    return Slot {
      item_count: value.count as i32,
      item_id: Some(data::items::get_items().iter().find(|y| y.0.clone() == value.id).unwrap().1.id),
      components_to_add: value.components.clone(),
      components_to_remove: Vec::new()
    };
  }
}

pub fn get_blockentity_for_placed_block(position_global: Position, block_state_id: u16) -> Option<BlockEntity> {
  return match data::blocks::get_type_from_block_state_id(block_state_id, &data::blocks::get_blocks()) { //TODO: pass the blocks in from somewhere, recomputing this on every placed block is a bit insane
    Type::Chest => Some(BlockEntity { id: BlockEntityId::Chest, position: position_global, components: None, data: Some(BlockEntityData::Chest(vec![BlockEntityDataItem::default();27])) }),
    Type::TrappedChest => Some(BlockEntity { id: BlockEntityId::TrappedChest, position: position_global, components: None, data: Some(BlockEntityData::Chest(vec![BlockEntityDataItem::default();27])) }),
    Type::Barrel => Some(BlockEntity { id: BlockEntityId::Barrel, position: position_global, components: None, data: Some(BlockEntityData::Chest(vec![BlockEntityDataItem::default();27])) }),
    Type::Banner => Some(BlockEntity { id: BlockEntityId::Banner, position: position_global, components: None, data: None }),
    Type::WallBanner => Some(BlockEntity { id: BlockEntityId::Banner, position: position_global, components: None, data: None }),
    Type::Beacon => Some(BlockEntity { id: BlockEntityId::Beacon, position: position_global, components: None, data: None }),
    Type::Bed => Some(BlockEntity { id: BlockEntityId::Bed, position: position_global, components: None, data: None }),
    Type::Beehive => Some(BlockEntity { id: BlockEntityId::Beehive, position: position_global, components: None, data: None }),
    Type::Bell => Some(BlockEntity { id: BlockEntityId::Bell, position: position_global, components: None, data: None }),
    Type::BlastFurnace => Some(BlockEntity { id: BlockEntityId::BlastFurnace, position: position_global, components: None, data: Some(BlockEntityData::Furnace(vec![BlockEntityDataItem::default();3])) }),
    Type::BrewingStand => Some(BlockEntity { id: BlockEntityId::BrewingStand, position: position_global, components: None, data: Some(BlockEntityData::BrewingStand(vec![BlockEntityDataItem::default();5])) }),
    Type::Brushable => Some(BlockEntity { id: BlockEntityId::BrushableBlock, position: position_global, components: None, data: None }),
    Type::CalibratedSculkSensor => Some(BlockEntity { id: BlockEntityId::CalibratedSculkSensor, position: position_global, components: None, data: None }),
    Type::Campfire => Some(BlockEntity { id: BlockEntityId::Campfire, position: position_global, components: None, data: None }), //TODO: check via block_state_id if this is a regular or soul campfire
    Type::ChiseledBookShelf => Some(BlockEntity { id: BlockEntityId::ChiseledBookshelf, position: position_global, components: None, data: None }),
    Type::Comparator => Some(BlockEntity { id: BlockEntityId::Comperator, position: position_global, components: None, data: None }),
    Type::Command => Some(BlockEntity { id: BlockEntityId::CommandBlock, position: position_global, components: None, data: None }),
    Type::Conduit => Some(BlockEntity { id: BlockEntityId::Conduit, position: position_global, components: None, data: None }),
    Type::Crafter => Some(BlockEntity { id: BlockEntityId::Crafter, position: position_global, components: None, data: Some(BlockEntityData::Crafter(vec![BlockEntityDataItem::default();9])) }),
    Type::CreakingHeart => Some(BlockEntity { id: BlockEntityId::CreakingHeart, position: position_global, components: None, data: None }),
    Type::DaylightDetector => Some(BlockEntity { id: BlockEntityId::DaylightDetector, position: position_global, components: None, data: None }),
    Type::DecoratedPot => Some(BlockEntity { id: BlockEntityId::DecoratedPot, position: position_global, components: None, data: None }),
    Type::Dispenser => Some(BlockEntity { id: BlockEntityId::Dispenser, position: position_global, components: None, data: Some(BlockEntityData::Dispenser(vec![BlockEntityDataItem::default();9])) }),
    Type::Dropper => Some(BlockEntity { id: BlockEntityId::Dropper, position: position_global, components: None, data: Some(BlockEntityData::Dispenser(vec![BlockEntityDataItem::default();9])) }),
    Type::EnchantmentTable => Some(BlockEntity { id: BlockEntityId::EnchantingTable, position: position_global, components: None, data: None }),
    Type::EnderChest => Some(BlockEntity { id: BlockEntityId::EnderChest, position: position_global, components: None, data: None }),
    Type::EndGateway => Some(BlockEntity { id: BlockEntityId::EndGateway, position: position_global, components: None, data: None }),
    Type::EndPortal => Some(BlockEntity { id: BlockEntityId::EndPortal, position: position_global, components: None, data: None }),
    Type::Furnace => Some(BlockEntity { id: BlockEntityId::Furnace, position: position_global, components: None, data: Some(BlockEntityData::Furnace(vec![BlockEntityDataItem::default();3])) }),
    Type::WallHangingSign => Some(BlockEntity { id: BlockEntityId::HangingSign, position: position_global, components: None, data: None }),
    Type::CeilingHangingSign => Some(BlockEntity { id: BlockEntityId::HangingSign, position: position_global, components: None, data: None }),
    Type::Hopper => Some(BlockEntity { id: BlockEntityId::Hopper, position: position_global, components: None, data: Some(BlockEntityData::Hopper(vec![BlockEntityDataItem::default();5])) }),
    Type::Jigsaw => Some(BlockEntity { id: BlockEntityId::Jigsaw, position: position_global, components: None, data: None }),
    Type::Jukebox => Some(BlockEntity { id: BlockEntityId::Jukebox, position: position_global, components: None, data: None }),
    Type::Lectern => Some(BlockEntity { id: BlockEntityId::Lectern, position: position_global, components: None, data: None }),
    Type::Spawner => Some(BlockEntity { id: BlockEntityId::MobSpawner, position: position_global, components: None, data: None }),
    Type::MovingPiston => Some(BlockEntity { id: BlockEntityId::Piston, position: position_global, components: None, data: None }),
    Type::ShulkerBox => Some(BlockEntity { id: BlockEntityId::ShulkerBox, position: position_global, components: None, data: Some(BlockEntityData::Chest(vec![BlockEntityDataItem::default();27])) }),
    Type::WallSign => Some(BlockEntity { id: BlockEntityId::Sign, position: position_global, components: None, data: None }),
    Type::StandingSign => Some(BlockEntity { id: BlockEntityId::Sign, position: position_global, components: None, data: None }),
    Type::Skull => Some(BlockEntity { id: BlockEntityId::Skull, position: position_global, components: None, data: None }),
    Type::WallSkull => Some(BlockEntity { id: BlockEntityId::Skull, position: position_global, components: None, data: None }),
    Type::SculkCatalyst => Some(BlockEntity { id: BlockEntityId::SculkCatalyst, position: position_global, components: None, data: None }),
    Type::SculkSensor => Some(BlockEntity { id: BlockEntityId::SculkSensor, position: position_global, components: None, data: None }),
    Type::SculkShrieker => Some(BlockEntity { id: BlockEntityId::SculkShrieker, position: position_global, components: None, data: None }),
    Type::Smoker => Some(BlockEntity { id: BlockEntityId::Smoker, position: position_global, components: None, data: Some(BlockEntityData::Furnace(vec![BlockEntityDataItem::default();3])) }),
    Type::Structure => Some(BlockEntity { id: BlockEntityId::StructureBlock, position: position_global, components: None, data: None }),
    Type::TrialSpawner => Some(BlockEntity { id: BlockEntityId::TrialSpawner, position: position_global, components: None, data: None }),
    Type::Vault => Some(BlockEntity { id: BlockEntityId::Vault, position: position_global, components: None, data: None }),
    _ => None,
  };
}
