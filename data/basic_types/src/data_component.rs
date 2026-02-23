use std::collections::HashMap;

use crate::*;

//https://minecraft.wiki/w/Data_component_format
#[derive(Debug, Clone, PartialEq)]
pub enum DataComponent {
	AttackRange(AttackDamageData),
	AttributeModifiers(Vec<AttributeModifierData>),
	BannerPatterns(Vec<BannerPatternData>),
	BaseColor(&'static str),
	Bees(Vec<BeeData>),
	BlockEntityData(NbtTag),
	BlockState(blocks::Property),
	BlocksAttacks(BlocksAttacksData),
	BreakSound(Sound),
	BucketEntityData(NbtTag),
	BundleContents(Vec<ItemStack>),
	CanBreak(Vec<BlockPredicate>),
	CanPlaceOn(Vec<BlockPredicate>),
	ChargedProjectiles(ItemStack),
	Consumable(ConsumableData),
	Container(Vec<ContainerDataItem>),
	ContainerLoot(ContainerLootData),
	CustomData(NbtTag),
	CustomModelData(CustomModelDataData),
	CustomName(NbtTag),
	Damage(i32),
	DamageResistant(&'static str),
	DamageType(&'static str),
	DeathProtection(Vec<ConsumeEffect>),
	DebugStickState(Vec<blocks::Property>),
	Dye(&'static str),
	DyedColor(i32),
	Enchantable(i32),
	EnchantmentGlintOverride(bool),
	Enchantments(HashMap<&'static str, i32>),
	EntityData(NbtTag),
	Equippable(EquippableData),
	FireworkExplosion,
	Fireworks,
	Food,
	Glider,
	Instrument,
	IntangibleProjectile,
	ItemModel,
	ItemName,
	JukeboxPlayable,
	KineticWeapon,
	Lock,
	LodestoneTracker,
	Lore,
	MapColor,
	MapDecorations,
	MapId,
	MaxDamage,
	MaxStackSize,
	MinimumAttackCharge,
	NoteBlockSound,
	OminousBottleAmplifier,
	PiercingWeapon,
	PotDecorations,
	PotionContents,
	PotionDurationScale,
	Profile,
	ProvidesBannerPatterns,
	ProvidesTrimMaterial,
	Rarity,
	Recipes,
	RepairCost,
	Repairable,
	StoredEnchantments,
	SuspiciousStewEffects,
	SwingAnimation,
	Tool,
	TooltipDisplay,
	TooltipStyle,
	Trim,
	Unbreakable,
	UseCooldown,
	UseEffects,
	UseRemainder,
	Weapon,
	WritableBookContent,
	WrittenBookContent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
	pub id: &'static str,
	pub count: i32,
	pub components: Vec<DataComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sound {
	Id(&'static str),
	Custom(SoundEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoundEvent {
	pub id: &'static str,
	pub range: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockPredicate {
	pub blocks: Vec<&'static str>,
	pub nbt: NbtTag,
	pub state: blocks::Property,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsumeEffect {
	pub consume_effect_type: ConsumeEffectType,
	pub effects: Vec<Effect>,
	pub probability: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumeEffectType {
	ApplyEffects,
	RemoveEffects(Vec<&'static str>),
	ClearAllEffects,
	TeleportRandomly(Option<f32>),
	PlaySound(Sound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
	pub id: &'static str,
	pub amplifier: Option<u8>,
	pub duration: Option<i32>,
	pub ambient: Option<bool>,
	pub show_particles: Option<bool>,
	pub show_icon: Option<bool>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct AttackDamageData {
	pub min_reach: Option<f32>,
	pub max_reach: Option<f32>,
	pub min_creative_reach: Option<f32>,
	pub max_creative_reach: Option<f32>,
	pub hitbox_margin: Option<f32>,
	pub mob_factor: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifierData {
	pub id: &'static str,
	pub attribute_type: &'static str,
	pub slot: AttributeModifiersDataSlot,
	pub operation: AttributeModifiersDataOperation,
	pub amount: f64,
	pub display: AttributeModifiersDataDisplay,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeModifiersDataSlot {
	Any,
	Hand,
	Armor,
	Mainhand,
	Offhand,
	Head,
	Chest,
	Legs,
	Feet,
	Body,
	Saddle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeModifiersDataOperation {
	AddValue,
	AddMultipliedBase,
	AddMultipliedTotal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeModifiersDataDisplay {
	Default,
	Hidden,
	Override(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BannerPatternData {
	pub color: &'static str,
	pub id: Option<&'static str>,
	pub asset_id: Option<&'static str>,
	pub translation_key: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BeeData {
	pub entity_data: NbtTag,
	pub min_ticks_in_hive: i32,
	pub ticks_in_hive: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlocksAttacksData {
	pub block_delay_seconds: Option<f32>,
	pub disable_cooldown_scale: Option<f32>,
	pub damage_reductions: Vec<BlocksAttacksDataDamageReduction>,
	pub threshold: Option<f32>,
	pub base: Option<f32>,
	pub factor: Option<f32>,
	pub block_sound: Option<Sound>,
	pub disabled_sound: Option<Sound>,
	pub bypassed_by: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlocksAttacksDataDamageReduction {
	pub damage_types: Vec<&'static str>,
	pub base: f32,
	pub factor: f32,
	pub horizontal_blocking_angle: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsumableData {
	pub consume_seconds: Option<f32>,
	pub animation: Option<ConsumableDataAnimation>,
	pub sound: Option<Sound>,
	pub has_consume_particles: Option<bool>,
	pub on_consume_effects: Vec<ConsumeEffect>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumableDataAnimation {
	None,
	Eat,
	Drink,
	Block,
	Bow,
	Spear,
	Crossbow,
	Spyglass,
	TootHorn,
	Brush,
	Bundle,
	Trident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerDataItem {
	pub item: ItemStack,
	pub slot: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerLootData {
	pub loot_table_id: &'static str,
	pub seed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomModelDataData {
	pub floats: Vec<f32>,
	pub flags: Vec<bool>,
	pub strings: Vec<&'static str>,
	pub colors: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EquippableData {
	pub slot: EquippableDataSlot,
	pub equip_sound: Sound,
	pub asset_id: Option<&'static str>,
	pub allowed_entities: Vec<&'static str>,
	pub dispensable: Option<bool>,
	pub swappable: Option<bool>,
	pub damage_on_hurt: Option<bool>,
	pub equip_on_interact: Option<bool>,
	pub camera_overlay: Option<&'static str>,
	pub can_be_sheared: Option<bool>,
	pub shearing_sound: Option<Sound>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EquippableDataSlot {
	Head,
	Chest,
	Legs,
	Feet,
	Body,
	Mainhand,
	Offhand,
	Saddle,
}
