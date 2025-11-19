use crate::entity::ItemEntity;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
  pub item_count: i32,
  pub item_id: i32,
  pub components_to_add: Vec<SlotComponent>,
  pub components_to_remove: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Item {
  pub id: String,
  pub count: u8,
  pub components: Vec<SlotComponent>,
}

impl Item {
  pub fn get_entity(&self, position: EntityPosition, entity_id: i32) -> ItemEntity {
    return ItemEntity {
      common: crate::entity::CommonEntity {
        position,
     		velocity: EntityPosition::default(),
        uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
        entity_id,
        ..Default::default()
      },
      age: 0,
      health: 5,
      item: self.clone(),
      owner: 0,
      pickup_delay: 0,
      thrower: 0,
    }
  }
}

impl Default for Item {
  fn default() -> Self {
    Self { id: "minecraft:air".to_string(), count: 0, components: Vec::new() }
  }
}

impl From<Vec<Item>> for NbtTag {
  fn from(value: Vec<Item>) -> Self {
    return NbtTag::List("Items".to_string(), value.iter().enumerate().filter(|(_, item)| item.id != "minecraft:air" && item.count != 0).map(|(i, item)| {
      NbtListTag::TagCompound(vec![
        NbtTag::Byte("Slot".to_string(), i as u8),
        NbtTag::String("id".to_string(), item.id.clone()),
        NbtTag::Int("count".to_string(), item.count as i32),
        NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion
      ])
    }).collect());
  }
}

impl From<Item> for Vec<NbtTag> {
  fn from(value: Item) -> Self {
    return vec![
      NbtTag::String("id".to_string(), value.id.clone()),
      NbtTag::Int("count".to_string(), value.count as i32),
      NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion
    ];
  }
}

impl From<NbtTag> for Item {
  fn from(value: NbtTag) -> Self {
    return Self {
      id: value.get_child("id").unwrap_or(&NbtTag::String("".to_string(), "minecraft:air".to_string())).as_string().to_string(),
      count: value.get_child("count").unwrap_or(&NbtTag::Int("".to_string(), 0)).as_int() as u8,
      components: Vec::new(), //missing nbt to SlotComponent conversion
    }
  }
}

impl From<&Item> for Slot {
  fn from(value: &Item) -> Self {
    return Slot {
      item_count: value.count as i32,
      item_id: data::items::get_items().iter().find(|y| y.0.clone() == value.id).unwrap().1.id,
      components_to_add: value.components.clone(),
      components_to_remove: Vec::new()
    };
  }
}

impl From<Slot> for Item {
  fn from(value: Slot) -> Self {
    return Self {
      id: data::items::get_item_name_by_id(value.item_id),
      count: value.item_count as u8,
      components: value.components_to_add
    };
  }
}

impl From<Option<Slot>> for Item {
  fn from(value: Option<Slot>) -> Self {
    let Some(value) = value else {
      return Self {
        id: "minecraft:air".to_string(),
        count: 0,
        components: Vec::new(),
      }
    };

    return Self {
      id: data::items::get_item_name_by_id(value.item_id),
      count: value.item_count as u8,
      components: value.components_to_add
    };
  }
}

impl From<Item> for Slot {
  fn from(value: Item) -> Self {
    return Self {
      item_count: value.count as i32,
      item_id: data::items::get_items().get(&value.id).unwrap().id,
      components_to_add: value.components,
      components_to_remove: Vec::new()
    };
  }
}

impl From<Item> for Option<Slot> {
  fn from(value: Item) -> Self {
    return if value.count == 0 {
      None
    } else {
      Some(Slot {
        item_count: value.count as i32,
        item_id: data::items::get_items().get(&value.id).unwrap().id,
        components_to_add: value.components,
        components_to_remove: Vec::new()
      })
    }
  }
}


//implement missing SlotComponent variants https://git.thetxt.io/thetxt/oxide/issues/18
#[derive(Debug, Clone, PartialEq)]
pub enum SlotComponent {
  CustomData(NbtTag),
  MaxStackSize(i32),
  MaxDamage(i32),
  Damage(i32),
  Unbreakable,
  CustomName(NbtTag),
  ItemName(NbtTag),
  ItemModel(String),
  Lore(Vec<NbtTag>),
  Rarity(u8),
  Enchantments(Vec<(i32, i32)>),
  CanPlaceOn, //Missing block predicate array
  CanBreak, //Missing block predicate array
  AttributeModifiers, //Missing attribute modifiers
  CustomModelData(Vec<f32>, Vec<bool>, Vec<String>, Vec<i32>),
  TooltipDisplay(bool, Vec<i32>),
  RepairCost(i32),
  CreativeSlotLock,
  EnchantmentGlintOverride(bool),
  IntangibleProjectile(NbtTag),
  Food(i32, f32, bool),
  Consumable, //Missing consume effects implementation
  UseRemainder(Option<Slot>),
  UseCooldown(f32, Option<String>),
  DamageResistant(String),
  Tool, //missing Rules implementation
  Weapon(i32, f32),
  Enchantable(i32),
  Equippable, //couldnt be bothered yet
  Repairable, //Missing ID Set implementation
  Glider,
  TooltipStyle(String),
  DeathProtection, //Missing consume effects implementation
  BlockAttacks, //couldnt be bothered
  StoredEnchantments(Vec<(i32, i32)>),
  DyedColor(i32),
  MapColor(i32),
  MapId(i32),
  MapDecorations(NbtTag),
  MapPostProcessing(u8),
  ChargedProjectiles(Vec<Option<Slot>>),
  BundleContents(Vec<Option<Slot>>),
  PotionContents, //wont be doing that rn lol
  PotionDurationScale(f32),
  SuspiciousStewEffects(Vec<(i32, i32)>),
  WritableBookContent(Vec<(String, Option<String>)>),
  WrittenBookContent(Vec<(String, Option<String>)>),
  Trim, //yeah this also needs data still
  DebugStickState(NbtTag),
  EntityData(NbtTag),
  BucketEntityData(NbtTag),
  BlockEntityData(NbtTag),
  Instrument, //not important enough for now
  ProvidesTrimMaterial, //also still missing shit
  OminousBottleAmplifier(u8),
  JukeboxPlayable, //still missing some stuffs
  ProvidesBannerPatterns(String),
  Recipes(NbtTag),
  LodestoneTracker(bool, String, crate::BlockPosition, bool),
  FireworkExplosion, //Missing firework explosion implementation
  Fireworks, //Missing firework explosion implementation
  Profile(Option<String>, Option<u128>, Vec<(String, String, Option<String>)>),
  NoteblockSound(String),
  BannerPatterns, //figure out later
  BaseColor(u8),
  PotDecorations(Vec<i32>),
  Container(Vec<i32>),
  BlockState(Vec<(String, String)>),
  Bees(Vec<(NbtTag, i32, i32)>),
  Lock(NbtTag),
  ContainerLoot(NbtTag),
  BreakSound, //will be implemented in the future
  VillagerVariant, //will be implemented in the future
  WolfVariant, //will be implemented in the future
  WolfSoundVariant, //will be implemented in the future
  WolfCollar(u8),
  FoxVariant(u8),
  SalmonSize(u8),
  ParrotVariant(i32),
  TropicalFishPattern(u8),
  TropicalFishBaseColor(u8),
  TropicalFishPatternColor(u8),
  MooshroomVariant(u8),
  RabbitVariant(u8),
  PigVariant(u8),
  CowVariant(u8),
  ChickenVariant, //will be implemented in the future
  FrogVariant(i32),
  HorseVariant(u8),
  PaintingVariant, //will maybe be bothered in the future, idk
  LlamaVariant(u8),
  AxolotlVariant(u8),
  CatVariant(i32),
  CatCollar(u8),
  SheepColor(u8),
  ShulkerColor(u8),
}

#[allow(clippy::from_over_into)]
impl Into<i32> for &SlotComponent {
  fn into(self) -> i32 {
    return self.clone().into();
  }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for SlotComponent {
  fn into(self) -> i32 {
    return match self {
      SlotComponent::CustomData(_) => 0,
      SlotComponent::MaxStackSize(_) => 1,
      SlotComponent::MaxDamage(_) => 2,
      SlotComponent::Damage(_) => 3,
      SlotComponent::Unbreakable => 4,
      SlotComponent::CustomName(_) => 5,
      SlotComponent::ItemName(_) => 6,
      SlotComponent::ItemModel(_) => 7,
      SlotComponent::Lore(_) => 8,
      SlotComponent::Rarity(_) => 9,
      SlotComponent::Enchantments(_) => 10,
      SlotComponent::CanPlaceOn => 11,
      SlotComponent::CanBreak => 12,
      SlotComponent::AttributeModifiers => 13,
      SlotComponent::CustomModelData(_, _, _, _) => 14,
      SlotComponent::TooltipDisplay(_, _) => 15,
      SlotComponent::RepairCost(_) => 16,
      SlotComponent::CreativeSlotLock => 17,
      SlotComponent::EnchantmentGlintOverride(_) => 18,
      SlotComponent::IntangibleProjectile(_) => 19,
      SlotComponent::Food(_, _, _) => 20,
      SlotComponent::Consumable => 21,
      SlotComponent::UseRemainder(_) => 22,
      SlotComponent::UseCooldown(_, _) => 23,
      SlotComponent::DamageResistant(_) => 24,
      SlotComponent::Tool => 25,
      SlotComponent::Weapon(_, _) => 26,
      SlotComponent::Enchantable(_) => 27,
      SlotComponent::Equippable => 28,
      SlotComponent::Repairable => 29,
      SlotComponent::Glider => 30,
      SlotComponent::TooltipStyle(_) => 31,
      SlotComponent::DeathProtection => 32,
      SlotComponent::BlockAttacks => 33,
      SlotComponent::StoredEnchantments(_) => 34,
      SlotComponent::DyedColor(_) => 35,
      SlotComponent::MapColor(_) => 36,
      SlotComponent::MapId(_) => 37,
      SlotComponent::MapDecorations(_) => 38,
      SlotComponent::MapPostProcessing(_) => 39,
      SlotComponent::ChargedProjectiles(_) => 40,
      SlotComponent::BundleContents(_) => 41,
      SlotComponent::PotionContents => 42,
      SlotComponent::PotionDurationScale(_) => 43,
      SlotComponent::SuspiciousStewEffects(_) => 44,
      SlotComponent::WritableBookContent(_) => 45,
      SlotComponent::WrittenBookContent(_) => 46,
      SlotComponent::Trim => 47,
      SlotComponent::DebugStickState(_) => 48,
      SlotComponent::EntityData(_) => 49,
      SlotComponent::BucketEntityData(_) => 50,
      SlotComponent::BlockEntityData(_) => 51,
      SlotComponent::Instrument => 52,
      SlotComponent::ProvidesTrimMaterial => 53,
      SlotComponent::OminousBottleAmplifier(_) => 54,
      SlotComponent::JukeboxPlayable => 55,
      SlotComponent::ProvidesBannerPatterns(_) => 56,
      SlotComponent::Recipes(_) => 57,
      SlotComponent::LodestoneTracker(_, _, _, _) => 58,
      SlotComponent::FireworkExplosion => 59,
      SlotComponent::Fireworks => 60,
      SlotComponent::Profile(_, _, _) => 61,
      SlotComponent::NoteblockSound(_) => 62,
      SlotComponent::BannerPatterns => 63,
      SlotComponent::BaseColor(_) => 64,
      SlotComponent::PotDecorations(_) => 65,
      SlotComponent::Container(_) => 66,
      SlotComponent::BlockState(_) => 67,
      SlotComponent::Bees(_) => 68,
      SlotComponent::Lock(_) => 69,
      SlotComponent::ContainerLoot(_) => 70,
      SlotComponent::BreakSound => 71,
      SlotComponent::VillagerVariant => 72,
      SlotComponent::WolfVariant => 73,
      SlotComponent::WolfSoundVariant => 74,
      SlotComponent::WolfCollar(_) => 75,
      SlotComponent::FoxVariant(_) => 76,
      SlotComponent::SalmonSize(_) => 77,
      SlotComponent::ParrotVariant(_) => 78,
      SlotComponent::TropicalFishPattern(_) => 79,
      SlotComponent::TropicalFishBaseColor(_) => 80,
      SlotComponent::TropicalFishPatternColor(_) => 81,
      SlotComponent::MooshroomVariant(_) => 82,
      SlotComponent::RabbitVariant(_) => 83,
      SlotComponent::PigVariant(_) => 84,
      SlotComponent::CowVariant(_) => 85,
      SlotComponent::ChickenVariant => 86,
      SlotComponent::FrogVariant(_) => 87,
      SlotComponent::HorseVariant(_) => 88,
      SlotComponent::PaintingVariant => 89,
      SlotComponent::LlamaVariant(_) => 90,
      SlotComponent::AxolotlVariant(_) => 91,
      SlotComponent::CatVariant(_) => 92,
      SlotComponent::CatCollar(_) => 93,
      SlotComponent::SheepColor(_) => 94,
      SlotComponent::ShulkerColor(_) => 95,
    }
  }
}
