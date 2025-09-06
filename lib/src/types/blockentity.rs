use std::error::Error;

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
  CopperGolemStatue,
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
      BlockEntityId::CopperGolemStatue => "minecraft:copper_golem_statue",
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
       "minecraft:copper_golem_statue" => Ok(BlockEntityId::CopperGolemStatue),
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
  #[default]
  NoData, //TODO: remove when everything is implemented
}

#[derive(Debug, Clone, Default)]
pub struct BlockEntityDataItem {
  pub id: String,
  pub count: u8,
  pub components: Vec<SlotComponent>,
}

impl From<BlockEntity> for NbtTag {
  fn from(value: BlockEntity) -> Self {
    println!("{value:?}");
    let mut items: Vec<NbtTag> = vec![
      NbtTag::String(Some("id".to_string()), Into::<&str>::into(value.id).to_string()),
      NbtTag::Int(Some("x".to_string()), value.position.x),
      NbtTag::Int(Some("y".to_string()), value.position.y as i32),
      NbtTag::Int(Some("z".to_string()), value.position.z),
    ];

    if let Some(block_entity_data) = value.data {
      match block_entity_data {
        BlockEntityData::Banner(_items) => (),
        BlockEntityData::Chest(block_entity_data_items) => items.push(NbtTag::TagCompound(Some("Items".to_string()), block_entity_data_items.iter().map(Into::into).collect())),
        BlockEntityData::NoData => (),
      }

    }

    return NbtTag::TagCompound(None, items);
  }
}

impl From<&BlockEntityData> for Vec<NbtTag> {
  fn from(value: &BlockEntityData) -> Self {
    return match value {
      BlockEntityData::Banner(data) => vec![NbtTag::List(Some("patterns".to_string()), data.iter().map(|x| NbtTag::TagCompound(None, vec![NbtTag::String(Some("color".to_string()), x.1.clone()), NbtTag::String(Some("pattern".to_string()), x.0.clone())])).collect())],
      BlockEntityData::Chest(block_entity_data_items) => block_entity_data_items.iter().map(|x| x.into()).collect(),
      BlockEntityData::NoData => Vec::new(),
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
