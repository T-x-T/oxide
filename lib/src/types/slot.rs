use std::error::Error;

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
		};
	}
}

impl Default for Item {
	fn default() -> Self {
		Self {
			id: "minecraft:air".to_string(),
			count: 0,
			components: Vec::new(),
		}
	}
}

impl From<Vec<Item>> for NbtTag {
	fn from(value: Vec<Item>) -> Self {
		return NbtTag::List(
			"Items".to_string(),
			value
				.iter()
				.enumerate()
				.filter(|(_, item)| item.id != "minecraft:air" && item.count != 0)
				.map(|(i, item)| {
					NbtListTag::TagCompound(vec![
						NbtTag::Byte("Slot".to_string(), i as u8),
						NbtTag::String("id".to_string(), item.id.clone()),
						NbtTag::Int("count".to_string(), item.count as i32),
						NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion
					])
				})
				.collect(),
		);
	}
}

impl From<Item> for NbtTag {
	fn from(value: Item) -> Self {
		return NbtTag::TagCompound(
			"Item".to_string(),
			vec![
				NbtTag::Byte("Slot".to_string(), 1),
				NbtTag::String("id".to_string(), value.id.clone()),
				NbtTag::Int("count".to_string(), value.count as i32),
				NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion]
			],
		);
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
		};
	}
}

impl From<&Item> for Slot {
	fn from(value: &Item) -> Self {
		return Slot {
			item_count: value.count as i32,
			item_id: data::items::get_items().iter().find(|y| y.0 == &value.id).unwrap().1.id,
			components_to_add: value.components.clone(),
			components_to_remove: Vec::new(),
		};
	}
}

impl From<Slot> for Item {
	fn from(value: Slot) -> Self {
		return Self {
			id: data::items::get_item_name_by_id(value.item_id).to_string(),
			count: value.item_count as u8,
			components: value.components_to_add,
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
			};
		};

		return Self {
			id: data::items::get_item_name_by_id(value.item_id).to_string(),
			count: value.item_count as u8,
			components: value.components_to_add,
		};
	}
}

impl From<Item> for Slot {
	fn from(value: Item) -> Self {
		return Self {
			item_count: value.count as i32,
			item_id: data::items::get_items().get(value.id.as_str()).unwrap().id,
			components_to_add: value.components,
			components_to_remove: Vec::new(),
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
				item_id: data::items::get_items().get(value.id.as_str()).unwrap().id,
				components_to_add: value.components,
				components_to_remove: Vec::new(),
			})
		};
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct BlockPredicate {
	blocks: Vec<String>,
	properties: Vec<BlockPredicateProperty>,
	nbt: Option<NbtTag>,
	data_components: Vec<NbtTag>,
	partial_data_component_predicates: Vec<NbtTag>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockPredicateProperty {
	name: String,
	is_exact_match: bool,
	exact_value: Option<String>,
	min_value: Option<String>,
	max_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumeEffect {
	ApplyEffects(Vec<PotionEffect>, f32),
	RemoveEffects(Vec<IdSet>),
	ClearAllEffects,
	TeleportRandomly(f32),
	PlaySound(String, bool, Option<f32>), //sound_name, has_fixed_range?, fixed_range //the bool is probably unneccessary?
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdSet {
	ByName(Option<String>),
	ById(Vec<i32>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PotionEffect {
	type_id: i32,
	amplifier: i32,
	duration: i32,
	ambient: bool,
	show_particles: bool,
	show_icon: bool,
	hidden_effect: Option<(i32, i32, bool, bool, bool)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FireworkExplosion {
	shape: FireworkExplosionShape,
	colors: Vec<i32>,
	fade_colors: Vec<i32>,
	has_trail: bool,
	has_twinkle: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum FireworkExplosionShape {
	SmallBall,
	LargeBall,
	Star,
	Creeper,
	Burst,
}

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
	CanPlaceOn(Vec<BlockPredicate>),
	CanBreak(Vec<BlockPredicate>),
	AttributeModifiers(Vec<(i32, String, f64, i32, i32)>), //attribute_id, modifier_id, value, operation, slot
	CustomModelData(Vec<f32>, Vec<bool>, Vec<String>, Vec<i32>),
	TooltipDisplay(bool, Vec<i32>),
	RepairCost(i32),
	CreativeSlotLock,
	EnchantmentGlintOverride(bool),
	IntangibleProjectile(NbtTag),
	Food(i32, f32, bool),
	Consumable(f32, i32, String, bool, Vec<ConsumeEffect>),
	UseRemainder(Option<Slot>),
	UseCooldown(f32, Option<String>),
	DamageResistant(String),
	Tool(Vec<(IdSet, Option<f32>, Option<bool>)>, f32, i32, bool), //blocks, speed, correct_drop_for_blocks, default_mining_speed, can_destroy_blocks_in_creative
	Weapon(i32, f32),
	Enchantable(i32),
	Equippable(i32, String, Option<String>, Option<String>, Option<IdSet>, bool, bool, bool), //slot, equip_sound, model, camera_overlay, allowed_entities, dispensable, swappable, damage_on_hurt
	Repairable(IdSet),
	Glider,
	TooltipStyle(String),
	DeathProtection(Vec<ConsumeEffect>),
	BlockAttacks(f32, f32, Vec<(f32, Option<IdSet>, f32, f32)>, f32, f32, f32, Option<String>, Option<String>, Option<String>),
	StoredEnchantments(Vec<(i32, i32)>),
	DyedColor(i32),
	MapColor(i32),
	MapId(i32),
	MapDecorations(NbtTag),
	MapPostProcessing(u8),
	ChargedProjectiles(Vec<Option<Slot>>),
	BundleContents(Vec<Option<Slot>>),
	PotionContents(Option<i32>, Option<i32>, Vec<PotionEffect>, String),
	PotionDurationScale(f32),
	SuspiciousStewEffects(Vec<(i32, i32)>),
	WritableBookContent(Vec<(String, Option<String>)>),
	WrittenBookContent(Vec<(String, Option<String>)>),
	Trim(String, String),
	DebugStickState(NbtTag),
	EntityData(NbtTag),
	BucketEntityData(NbtTag),
	BlockEntityData(NbtTag),
	Instrument(String),
	ProvidesTrimMaterial(u8, String),
	OminousBottleAmplifier(u8),
	JukeboxPlayable(u8, String),
	ProvidesBannerPatterns(String),
	Recipes(NbtTag),
	LodestoneTracker(bool, String, crate::BlockPosition, bool),
	FireworkExplosion(FireworkExplosion),
	Fireworks(i32, Vec<FireworkExplosion>),
	Profile(Option<String>, Option<u128>, Vec<(String, String, Option<String>)>),
	NoteblockSound(String),
	BannerPatterns(Vec<(String, i32)>),
	BaseColor(u8),
	PotDecorations(Vec<i32>),
	Container(Vec<i32>),
	BlockState(Vec<(String, String)>),
	Bees(Vec<(NbtTag, i32, i32)>),
	Lock(NbtTag),
	ContainerLoot(NbtTag),
	BreakSound(String),
	VillagerVariant(String),
	WolfVariant(String),
	WolfSoundVariant(String),
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
	ChickenVariant(u8, String),
	FrogVariant(i32),
	HorseVariant(u8),
	PaintingVariant(i32, i32, String, Option<NbtTag>, Option<NbtTag>),
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
			SlotComponent::CanPlaceOn(_) => 11,
			SlotComponent::CanBreak(_) => 12,
			SlotComponent::AttributeModifiers(_) => 13,
			SlotComponent::CustomModelData(_, _, _, _) => 14,
			SlotComponent::TooltipDisplay(_, _) => 15,
			SlotComponent::RepairCost(_) => 16,
			SlotComponent::CreativeSlotLock => 17,
			SlotComponent::EnchantmentGlintOverride(_) => 18,
			SlotComponent::IntangibleProjectile(_) => 19,
			SlotComponent::Food(_, _, _) => 20,
			SlotComponent::Consumable(..) => 21,
			SlotComponent::UseRemainder(_) => 22,
			SlotComponent::UseCooldown(_, _) => 23,
			SlotComponent::DamageResistant(_) => 24,
			SlotComponent::Tool(..) => 25,
			SlotComponent::Weapon(_, _) => 26,
			SlotComponent::Enchantable(_) => 27,
			SlotComponent::Equippable(..) => 28,
			SlotComponent::Repairable(_) => 29,
			SlotComponent::Glider => 30,
			SlotComponent::TooltipStyle(_) => 31,
			SlotComponent::DeathProtection(_) => 32,
			SlotComponent::BlockAttacks(..) => 33,
			SlotComponent::StoredEnchantments(_) => 34,
			SlotComponent::DyedColor(_) => 35,
			SlotComponent::MapColor(_) => 36,
			SlotComponent::MapId(_) => 37,
			SlotComponent::MapDecorations(_) => 38,
			SlotComponent::MapPostProcessing(_) => 39,
			SlotComponent::ChargedProjectiles(_) => 40,
			SlotComponent::BundleContents(_) => 41,
			SlotComponent::PotionContents(..) => 42,
			SlotComponent::PotionDurationScale(_) => 43,
			SlotComponent::SuspiciousStewEffects(_) => 44,
			SlotComponent::WritableBookContent(_) => 45,
			SlotComponent::WrittenBookContent(_) => 46,
			SlotComponent::Trim(..) => 47,
			SlotComponent::DebugStickState(_) => 48,
			SlotComponent::EntityData(_) => 49,
			SlotComponent::BucketEntityData(_) => 50,
			SlotComponent::BlockEntityData(_) => 51,
			SlotComponent::Instrument(_) => 52,
			SlotComponent::ProvidesTrimMaterial(..) => 53,
			SlotComponent::OminousBottleAmplifier(_) => 54,
			SlotComponent::JukeboxPlayable(..) => 55,
			SlotComponent::ProvidesBannerPatterns(_) => 56,
			SlotComponent::Recipes(_) => 57,
			SlotComponent::LodestoneTracker(_, _, _, _) => 58,
			SlotComponent::FireworkExplosion(_) => 59,
			SlotComponent::Fireworks(_, _) => 60,
			SlotComponent::Profile(_, _, _) => 61,
			SlotComponent::NoteblockSound(_) => 62,
			SlotComponent::BannerPatterns(_) => 63,
			SlotComponent::BaseColor(_) => 64,
			SlotComponent::PotDecorations(_) => 65,
			SlotComponent::Container(_) => 66,
			SlotComponent::BlockState(_) => 67,
			SlotComponent::Bees(_) => 68,
			SlotComponent::Lock(_) => 69,
			SlotComponent::ContainerLoot(_) => 70,
			SlotComponent::BreakSound(_) => 71,
			SlotComponent::VillagerVariant(_) => 72,
			SlotComponent::WolfVariant(_) => 73,
			SlotComponent::WolfSoundVariant(_) => 74,
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
			SlotComponent::ChickenVariant(_, _) => 86,
			SlotComponent::FrogVariant(_) => 87,
			SlotComponent::HorseVariant(_) => 88,
			SlotComponent::PaintingVariant(..) => 89,
			SlotComponent::LlamaVariant(_) => 90,
			SlotComponent::AxolotlVariant(_) => 91,
			SlotComponent::CatVariant(_) => 92,
			SlotComponent::CatCollar(_) => 93,
			SlotComponent::SheepColor(_) => 94,
			SlotComponent::ShulkerColor(_) => 95,
		};
	}
}


pub fn deserialize_slot(data: &mut Vec<u8>) -> Result<Option<Slot>, Box<dyn Error>> {
	let item_count = crate::deserialize::varint(data)?;

	if item_count == 0 {
		return Ok(None);
	}

	let item_id = crate::deserialize::varint(data)?;
	let number_of_components_to_add = crate::deserialize::varint(data)?;
	let number_of_components_to_remove = crate::deserialize::varint(data)?;

	let mut components_to_add: Vec<SlotComponent> = Vec::new();
	for _ in 0..number_of_components_to_add {
		let component_id = crate::deserialize::varint(data)?;
		components_to_add.push(match component_id {
			0 => SlotComponent::CustomData(crate::deserialize::nbt_network(data)?),
			1 => SlotComponent::MaxStackSize(crate::deserialize::varint(data)?),
			2 => SlotComponent::MaxDamage(crate::deserialize::varint(data)?),
			3 => SlotComponent::Damage(crate::deserialize::varint(data)?),
			4 => SlotComponent::Unbreakable,
			5 => SlotComponent::CustomName(crate::deserialize::nbt_network(data)?),
			6 => SlotComponent::ItemName(crate::deserialize::nbt_network(data)?),
			7 => SlotComponent::ItemModel(crate::deserialize::string(data)?),
			8 => SlotComponent::Lore((0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::nbt_network(data).unwrap()).collect()),
			9 => SlotComponent::Rarity(data.remove(0)),
			10 => SlotComponent::Enchantments(
				(0..crate::deserialize::varint(data)?)
					.map(|_| (crate::deserialize::varint(data).unwrap(), crate::deserialize::varint(data).unwrap()))
					.collect(),
			),
			//11 => todo!(), //SlotComponent::CanPlaceOn,
			//12 => todo!(), //SlotComponent::CanBreak,
			//13 => todo!(), //SlotComponent::AttributeModifiers,
			14 => SlotComponent::CustomModelData(
				(0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::float(data).unwrap()).collect(),
				(0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::boolean(data).unwrap()).collect(),
				(0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::string(data).unwrap()).collect(),
				(0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::int(data).unwrap()).collect(),
			),
			15 => SlotComponent::TooltipDisplay(
				crate::deserialize::boolean(data)?,
				(0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::varint(data).unwrap()).collect(),
			),
			16 => SlotComponent::RepairCost(crate::deserialize::varint(data)?),
			17 => SlotComponent::CreativeSlotLock,
			18 => SlotComponent::EnchantmentGlintOverride(crate::deserialize::boolean(data)?),
			19 => SlotComponent::IntangibleProjectile(crate::deserialize::nbt_network(data)?),
			20 => SlotComponent::Food(crate::deserialize::varint(data)?, crate::deserialize::float(data)?, crate::deserialize::boolean(data)?),
			//21 => todo!(), //SlotComponent::Consumable,
			22 => SlotComponent::UseRemainder(deserialize_slot(data)?),
			23 => SlotComponent::UseCooldown(
				crate::deserialize::float(data)?,
				if crate::deserialize::boolean(data)? { Some(crate::deserialize::string(data)?) } else { None },
			),
			24 => SlotComponent::DamageResistant(crate::deserialize::string(data)?),
			//25 => todo!(), //SlotComponent::Tool,
			26 => SlotComponent::Weapon(crate::deserialize::varint(data)?, crate::deserialize::float(data)?),
			27 => SlotComponent::Enchantable(crate::deserialize::varint(data)?),
			//28 => todo!(), //SlotComponent::Equippable,
			//29 => todo!(), //SlotComponent::Repairable,
			30 => SlotComponent::Glider,
			31 => SlotComponent::TooltipStyle(crate::deserialize::string(data)?),
			//32 => todo!(), //SlotComponent::DeathProtection,
			//33 => todo!(), //SlotComponent::BlockAttacks,
			34 => SlotComponent::StoredEnchantments(
				(0..crate::deserialize::varint(data)?)
					.map(|_| (crate::deserialize::varint(data).unwrap(), crate::deserialize::varint(data).unwrap()))
					.collect(),
			),
			35 => SlotComponent::DyedColor(crate::deserialize::int(data)?),
			36 => SlotComponent::MapColor(crate::deserialize::int(data)?),
			37 => SlotComponent::MapId(crate::deserialize::varint(data)?),
			38 => SlotComponent::MapDecorations(crate::deserialize::nbt_network(data)?),
			39 => SlotComponent::MapPostProcessing(data.remove(0)),
			40 => SlotComponent::ChargedProjectiles((0..crate::deserialize::varint(data)?).map(|_| deserialize_slot(data).unwrap()).collect()),
			41 => SlotComponent::BundleContents((0..crate::deserialize::varint(data)?).map(|_| deserialize_slot(data).unwrap()).collect()),
			//42 => todo!(), //SlotComponent::PotionContents,
			43 => SlotComponent::PotionDurationScale(crate::deserialize::float(data)?),
			44 => SlotComponent::SuspiciousStewEffects(
				(0..crate::deserialize::varint(data)?)
					.map(|_| (crate::deserialize::varint(data).unwrap(), crate::deserialize::varint(data).unwrap()))
					.collect(),
			),
			45 => SlotComponent::WritableBookContent(
				(0..crate::deserialize::varint(data)?)
					.map(|_| {
						(
							crate::deserialize::string(data).unwrap(),
							if crate::deserialize::boolean(data).unwrap() { Some(crate::deserialize::string(data).unwrap()) } else { None },
						)
					})
					.collect(),
			),
			46 => SlotComponent::WrittenBookContent(
				(0..crate::deserialize::varint(data)?)
					.map(|_| {
						(
							crate::deserialize::string(data).unwrap(),
							if crate::deserialize::boolean(data).unwrap() { Some(crate::deserialize::string(data).unwrap()) } else { None },
						)
					})
					.collect(),
			),
			//47 => todo!(), //SlotComponent::Trim,
			48 => SlotComponent::DebugStickState(crate::deserialize::nbt_network(data)?),
			49 => SlotComponent::EntityData(crate::deserialize::nbt_network(data)?),
			50 => SlotComponent::BucketEntityData(crate::deserialize::nbt_network(data)?),
			51 => SlotComponent::BlockEntityData(crate::deserialize::nbt_network(data)?),
			//52 => todo!(), //SlotComponent::Instrument,
			//53 => todo!(), //SlotComponent::ProvidesTrimMaterial,
			54 => SlotComponent::OminousBottleAmplifier(data.remove(0)),
			//55 => todo!(), //SlotComponent::JukeboxPlayable,
			56 => SlotComponent::ProvidesBannerPatterns(crate::deserialize::string(data)?),
			57 => SlotComponent::Recipes(crate::deserialize::nbt_network(data)?),
			58 => SlotComponent::LodestoneTracker(
				crate::deserialize::boolean(data)?,
				crate::deserialize::string(data)?,
				crate::deserialize::position(data)?,
				crate::deserialize::boolean(data)?,
			),
			//59 => todo!(), //SlotComponent::FireworkExplosion,
			//60 => todo!(), //SlotComponent::Fireworks,
			61 => SlotComponent::Profile(
				if crate::deserialize::boolean(data)? { Some(crate::deserialize::string(data)?) } else { None },
				if crate::deserialize::boolean(data)? { Some(crate::deserialize::uuid(data)?) } else { None },
				(0..crate::deserialize::varint(data)?)
					.map(|_| {
						(
							crate::deserialize::string(data).unwrap(),
							crate::deserialize::string(data).unwrap(),
							if crate::deserialize::boolean(data).unwrap() { Some(crate::deserialize::string(data).unwrap()) } else { None },
						)
					})
					.collect(),
			),
			62 => SlotComponent::NoteblockSound(crate::deserialize::string(data)?),
			//63 => todo!(), //SlotComponent::BannerPatterns,
			64 => SlotComponent::BaseColor(data.remove(0)),
			65 => {
				SlotComponent::PotDecorations((0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::varint(data).unwrap()).collect())
			}
			66 => SlotComponent::Container((0..crate::deserialize::varint(data)?).map(|_| crate::deserialize::varint(data).unwrap()).collect()),
			67 => SlotComponent::BlockState(
				(0..crate::deserialize::varint(data)?)
					.map(|_| (crate::deserialize::string(data).unwrap(), crate::deserialize::string(data).unwrap()))
					.collect(),
			),
			68 => SlotComponent::Bees(
				(0..crate::deserialize::varint(data)?)
					.map(|_| {
						(
							crate::deserialize::nbt_network(data).unwrap(),
							crate::deserialize::varint(data).unwrap(),
							crate::deserialize::varint(data).unwrap(),
						)
					})
					.collect(),
			),
			69 => SlotComponent::Lock(crate::deserialize::nbt_network(data)?),
			70 => SlotComponent::ContainerLoot(crate::deserialize::nbt_network(data)?),
			//71 => todo!(), //SlotComponent::BreakSound,
			//72 => todo!(), //SlotComponent::VillagerVariant,
			//73 => todo!(), //SlotComponent::WolfVariant,
			//74 => todo!(), //SlotComponent::WolfSoundVariant,
			75 => SlotComponent::WolfCollar(data.remove(0)),
			76 => SlotComponent::FoxVariant(data.remove(0)),
			77 => SlotComponent::SalmonSize(data.remove(0)),
			78 => SlotComponent::ParrotVariant(crate::deserialize::varint(data)?),
			79 => SlotComponent::TropicalFishPattern(data.remove(0)),
			80 => SlotComponent::TropicalFishBaseColor(data.remove(0)),
			81 => SlotComponent::TropicalFishPatternColor(data.remove(0)),
			82 => SlotComponent::MooshroomVariant(data.remove(0)),
			83 => SlotComponent::RabbitVariant(data.remove(0)),
			84 => SlotComponent::PigVariant(data.remove(0)),
			85 => SlotComponent::CowVariant(data.remove(0)),
			//86 => todo!(), //SlotComponent::ChickenVariant,
			87 => SlotComponent::FrogVariant(crate::deserialize::varint(data)?),
			88 => SlotComponent::HorseVariant(data.remove(0)),
			//89 => todo!(), //SlotComponent::PaintingVariant,
			90 => SlotComponent::LlamaVariant(data.remove(0)),
			91 => SlotComponent::AxolotlVariant(data.remove(0)),
			92 => SlotComponent::CatVariant(crate::deserialize::varint(data)?),
			93 => SlotComponent::CatCollar(data.remove(0)),
			94 => SlotComponent::SheepColor(data.remove(0)),
			95 => SlotComponent::ShulkerColor(data.remove(0)),
			x => {
				println!("I cant deserialize the SlotComponent with id {x}, because I dont know it");
				return Ok(None);
			}
		});
	}
	let mut components_to_remove: Vec<i32> = Vec::new();
	for _ in 0..number_of_components_to_remove {
		components_to_remove.push(crate::deserialize::varint(data)?);
	}

	return Ok(Some(Slot {
		item_count,
		item_id,
		components_to_add,
		components_to_remove,
	}));
}

pub fn serialize_slot(input: Option<&Slot>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	let Some(input) = input else {
		output.append(&mut crate::serialize::varint(0));
		return output;
	};

	if input.item_count == 0 {
		output.append(&mut crate::serialize::varint(0));
		return output;
	}

	output.append(&mut crate::serialize::varint(input.item_count));
	output.append(&mut crate::serialize::varint(input.item_id));
	output.append(&mut crate::serialize::varint(input.components_to_add.len() as i32));
	output.append(&mut crate::serialize::varint(input.components_to_remove.len() as i32));
	for component_to_add in &input.components_to_add {
		output.append(&mut crate::serialize::varint(component_to_add.into()));
		output.append(&mut match component_to_add.clone() {
			SlotComponent::CustomData(a) => crate::serialize::nbt_network(a),
			SlotComponent::MaxStackSize(a) => crate::serialize::varint(a),
			SlotComponent::MaxDamage(a) => crate::serialize::varint(a),
			SlotComponent::Damage(a) => crate::serialize::varint(a),
			SlotComponent::Unbreakable => vec![],
			SlotComponent::CustomName(a) => crate::serialize::nbt_network(a),
			SlotComponent::ItemName(a) => crate::serialize::nbt_network(a),
			SlotComponent::ItemModel(a) => crate::serialize::string(&a),
			SlotComponent::Lore(a) => a.into_iter().flat_map(crate::serialize::nbt_network).collect(),
			SlotComponent::Rarity(a) => vec![a],
			SlotComponent::Enchantments(a) => {
				a.into_iter().flat_map(|(x, y)| vec![crate::serialize::varint(x), crate::serialize::varint(y)]).flatten().collect()
			}
			SlotComponent::CanPlaceOn(..) => todo!(),
			SlotComponent::CanBreak(..) => todo!(),
			SlotComponent::AttributeModifiers(..) => todo!(),
			SlotComponent::CustomModelData(a, b, c, d) => vec![
				a.into_iter().flat_map(crate::serialize::float).collect::<Vec<u8>>(),
				b.into_iter().flat_map(crate::serialize::boolean).collect(),
				c.into_iter().flat_map(|x| crate::serialize::string(&x)).collect(),
				d.into_iter().flat_map(crate::serialize::int).collect(),
			]
			.into_iter()
			.flatten()
			.collect(),
			SlotComponent::TooltipDisplay(a, b) => {
				vec![crate::serialize::boolean(a), b.into_iter().flat_map(crate::serialize::varint).collect()].into_iter().flatten().collect()
			}
			SlotComponent::RepairCost(a) => crate::serialize::varint(a),
			SlotComponent::CreativeSlotLock => vec![],
			SlotComponent::EnchantmentGlintOverride(a) => crate::serialize::boolean(a),
			SlotComponent::IntangibleProjectile(a) => crate::serialize::nbt_network(a),
			SlotComponent::Food(a, b, c) => {
				vec![crate::serialize::varint(a), crate::serialize::float(b), crate::serialize::boolean(c)].into_iter().flatten().collect()
			}
			SlotComponent::Consumable(..) => todo!(),
			SlotComponent::UseRemainder(a) => serialize_slot(a.as_ref()),
			SlotComponent::UseCooldown(a, b) => vec![
				crate::serialize::float(a),
				if let Some(b) = b { vec![vec![1], crate::serialize::string(&b)].into_iter().flatten().collect() } else { vec![0] },
			]
			.into_iter()
			.flatten()
			.collect(),
			SlotComponent::DamageResistant(a) => crate::serialize::string(&a),
			SlotComponent::Tool(..) => todo!(),
			SlotComponent::Weapon(a, b) => vec![crate::serialize::varint(a), crate::serialize::float(b)].into_iter().flatten().collect(),
			SlotComponent::Enchantable(a) => crate::serialize::varint(a),
			SlotComponent::Equippable(..) => todo!(),
			SlotComponent::Repairable(..) => todo!(),
			SlotComponent::Glider => vec![],
			SlotComponent::TooltipStyle(a) => crate::serialize::string(&a),
			SlotComponent::DeathProtection(..) => todo!(),
			SlotComponent::BlockAttacks(..) => todo!(),
			SlotComponent::StoredEnchantments(a) => {
				a.into_iter().flat_map(|(x, y)| vec![crate::serialize::varint(x), crate::serialize::varint(y)]).flatten().collect()
			}
			SlotComponent::DyedColor(a) => crate::serialize::int(a),
			SlotComponent::MapColor(a) => crate::serialize::int(a),
			SlotComponent::MapId(a) => crate::serialize::varint(a),
			SlotComponent::MapDecorations(a) => crate::serialize::nbt_network(a),
			SlotComponent::MapPostProcessing(a) => vec![a],
			SlotComponent::ChargedProjectiles(a) => a.into_iter().flat_map(|x| serialize_slot(x.as_ref())).collect(),
			SlotComponent::BundleContents(a) => a.into_iter().flat_map(|x| serialize_slot(x.as_ref())).collect(),
			SlotComponent::PotionContents(..) => todo!(),
			SlotComponent::PotionDurationScale(a) => crate::serialize::float(a),
			SlotComponent::SuspiciousStewEffects(a) => {
				a.into_iter().flat_map(|(x, y)| vec![crate::serialize::varint(x), crate::serialize::varint(y)]).flatten().collect()
			}
			SlotComponent::WritableBookContent(a) => a
				.into_iter()
				.flat_map(|(x, y)| {
					vec![
						crate::serialize::string(&x),
						if let Some(y) = y { vec![vec![0x01], crate::serialize::string(&y)].into_iter().flatten().collect() } else { vec![0x00] },
					]
				})
				.flatten()
				.collect(),
			SlotComponent::WrittenBookContent(a) => a
				.into_iter()
				.flat_map(|(x, y)| {
					vec![
						crate::serialize::string(&x),
						if let Some(y) = y { vec![vec![0x01], crate::serialize::string(&y)].into_iter().flatten().collect() } else { vec![0x00] },
					]
				})
				.flatten()
				.collect(),
			SlotComponent::Trim(..) => todo!(),
			SlotComponent::DebugStickState(a) => crate::serialize::nbt_network(a),
			SlotComponent::EntityData(a) => crate::serialize::nbt_network(a),
			SlotComponent::BucketEntityData(a) => crate::serialize::nbt_network(a),
			SlotComponent::BlockEntityData(a) => crate::serialize::nbt_network(a),
			SlotComponent::Instrument(..) => todo!(),
			SlotComponent::ProvidesTrimMaterial(..) => todo!(),
			SlotComponent::OminousBottleAmplifier(a) => vec![a],
			SlotComponent::JukeboxPlayable(..) => todo!(),
			SlotComponent::ProvidesBannerPatterns(a) => crate::serialize::string(&a),
			SlotComponent::Recipes(a) => crate::serialize::nbt_network(a),
			SlotComponent::LodestoneTracker(a, b, c, d) => {
				vec![crate::serialize::boolean(a), crate::serialize::string(&b), crate::serialize::position(&c), crate::serialize::boolean(d)]
					.into_iter()
					.flatten()
					.collect()
			}
			SlotComponent::FireworkExplosion(..) => todo!(),
			SlotComponent::Fireworks(..) => todo!(),
			SlotComponent::Profile(a, b, c) => vec![
				if let Some(a) = a { vec![vec![0x01], crate::serialize::string(&a)].into_iter().flatten().collect() } else { vec![0x00] },
				if let Some(b) = b { vec![vec![0x01], crate::serialize::uuid(&b)].into_iter().flatten().collect() } else { vec![0x00] },
				c.into_iter()
					.flat_map(|(x, y, z)| {
						vec![
							crate::serialize::string(&x),
							crate::serialize::string(&y),
							if let Some(z) = z { vec![vec![0x01], crate::serialize::string(&z)].into_iter().flatten().collect() } else { vec![0x00] },
						]
					})
					.flatten()
					.collect(),
			]
			.into_iter()
			.flatten()
			.collect(),
			SlotComponent::NoteblockSound(a) => crate::serialize::string(&a),
			SlotComponent::BannerPatterns(..) => todo!(),
			SlotComponent::BaseColor(a) => vec![a],
			SlotComponent::PotDecorations(a) => a.into_iter().flat_map(crate::serialize::varint).collect(),
			SlotComponent::Container(a) => a.into_iter().flat_map(crate::serialize::varint).collect(),
			SlotComponent::BlockState(a) => {
				a.into_iter().flat_map(|(x, y)| vec![crate::serialize::string(&x), crate::serialize::string(&y)]).flatten().collect()
			}
			SlotComponent::Bees(a) => a
				.into_iter()
				.flat_map(|(x, y, z)| vec![crate::serialize::nbt_network(x), crate::serialize::varint(y), crate::serialize::varint(z)])
				.flatten()
				.collect(),
			SlotComponent::Lock(a) => crate::serialize::nbt_network(a),
			SlotComponent::ContainerLoot(a) => crate::serialize::nbt_network(a),
			SlotComponent::BreakSound(..) => todo!(),
			SlotComponent::VillagerVariant(..) => todo!(),
			SlotComponent::WolfVariant(..) => todo!(),
			SlotComponent::WolfSoundVariant(..) => todo!(),
			SlotComponent::WolfCollar(a) => vec![a],
			SlotComponent::FoxVariant(a) => vec![a],
			SlotComponent::SalmonSize(a) => vec![a],
			SlotComponent::ParrotVariant(a) => crate::serialize::varint(a),
			SlotComponent::TropicalFishPattern(a) => vec![a],
			SlotComponent::TropicalFishBaseColor(a) => vec![a],
			SlotComponent::TropicalFishPatternColor(a) => vec![a],
			SlotComponent::MooshroomVariant(a) => vec![a],
			SlotComponent::RabbitVariant(a) => vec![a],
			SlotComponent::PigVariant(a) => vec![a],
			SlotComponent::CowVariant(a) => vec![a],
			SlotComponent::ChickenVariant(..) => todo!(),
			SlotComponent::FrogVariant(a) => crate::serialize::varint(a),
			SlotComponent::HorseVariant(a) => vec![a],
			SlotComponent::PaintingVariant(..) => todo!(),
			SlotComponent::LlamaVariant(a) => vec![a],
			SlotComponent::AxolotlVariant(a) => vec![a],
			SlotComponent::CatVariant(a) => crate::serialize::varint(a),
			SlotComponent::CatCollar(a) => vec![a],
			SlotComponent::SheepColor(a) => vec![a],
			SlotComponent::ShulkerColor(a) => vec![a],
		});
	}

	for component_to_remove in &input.components_to_remove {
		output.append(&mut crate::serialize::varint(*component_to_remove));
	}

	return output;
}


pub fn deserialize_hashed_slot(data: &mut Vec<u8>) -> Result<Option<Slot>, Box<dyn Error>> {
	let has_item = crate::deserialize::boolean(data)?;

	if !has_item {
		return Ok(None);
	}

	let item_id = crate::deserialize::varint(data)?;
	let item_count = crate::deserialize::varint(data)?;

	let components_to_add_len = crate::deserialize::varint(data)?;
	let mut components_to_add: Vec<(i32, i32)> = Vec::new(); //(varint, int)
	for _ in 0..components_to_add_len {
		components_to_add.push((crate::deserialize::varint(data)?, crate::deserialize::int(data)?));
	}

	let components_to_remove_len = crate::deserialize::varint(data)?;
	let mut components_to_remove: Vec<i32> = Vec::new();
	for _ in 0..components_to_remove_len {
		components_to_remove.push(crate::deserialize::varint(data)?);
	}

	//might have to do something about the components_to_add but probably not(?)
	return Ok(Some(Slot {
		item_count,
		item_id,
		components_to_add: Vec::new(),
		components_to_remove,
	}));
}

pub fn serialize_hashed_slot(input: Option<&Slot>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	let Some(input) = input else {
		output.append(&mut crate::serialize::boolean(false));
		return output;
	};

	if input.item_count == 0 {
		output.append(&mut crate::serialize::boolean(false));
		return output;
	}

	output.append(&mut crate::serialize::varint(input.item_count));
	output.append(&mut crate::serialize::varint(input.item_id));
	output.append(&mut crate::serialize::varint(input.components_to_add.len() as i32));
	for component in &input.components_to_add {
		output.append(&mut crate::serialize::varint(component.into()));
		output.append(&mut crate::serialize::int(0)); //there should be a CRC32C calculation of some sorts here... https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Hashed_Format
	}
	output.append(&mut crate::serialize::varint(input.components_to_remove.len() as i32));
	for component in &input.components_to_remove {
		output.append(&mut crate::serialize::varint(*component));
	}

	return output;
}
