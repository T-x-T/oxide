use std::collections::HashMap;

use crate::nbt::NbtTag;
use crate::predicate::ItemPredicate;

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
	FireworkExplosion(FireworkExplosionData),
	Fireworks(FireworksData),
	Food(FoodData),
	Glider,
	Instrument(InstrumentData),
	IntangibleProjectile,
	ItemModel(&'static str),
	ItemName(NbtTag),
	JukeboxPlayable(&'static str),
	KineticWeapon(KineticWeaponData),
	Lock(ItemPredicate),
	LodestoneTracker(LodestoneTrackerData),
	Lore(Vec<NbtTag>),
	MapColor(i32),
	MapDecorations(HashMap<&'static str, MapDecoration>),
	MapId(i32),
	MaxDamage(i32),
	MaxStackSize(i32),
	MinimumAttackCharge(f32),
	NoteBlockSound(&'static str),
	OminousBottleAmplifier(i32),
	PiercingWeapon(PiercingWeaponData),
	PotDecorations(Vec<&'static str>),
	PotionContents(PotionContentsData),
	PotionDurationScale(f32),
	Profile(ProfileData),
	ProvidesBannerPatterns(&'static str),
	ProvidesTrimMaterial(&'static str),
	Rarity(&'static str),
	Recipes(Vec<&'static str>),
	RepairCost(i32),
	Repairable(Vec<&'static str>),
	StoredEnchantments(HashMap<&'static str, i32>),
	SuspiciousStewEffects(Vec<SuspiciousStewEffect>),
	SwingAnimation(SwingAnimationData),
	Tool(ToolData),
	TooltipDisplay(TooltipDisplayData),
	TooltipStyle(&'static str),
	Trim(TrimData),
	Unbreakable,
	UseCooldown(UseCooldownData),
	UseEffects(UseEffectsData),
	UseRemainder(ItemStack),
	Weapon(WeaponData),
	WritableBookContent(WritableBookContentData),
	WrittenBookContent(WrittenBookContentData),
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

#[derive(Debug, Clone, PartialEq)]
pub struct FireworkExplosionData {
	pub shape: FireworkExplosionShape,
	pub colors: Vec<i32>,
	pub fade_colors: Vec<i32>,
	pub has_trail: bool,
	pub has_twinkle: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FireworkExplosionShape {
	SmallBall,
	LargeBall,
	Star,
	Creeper,
	Burst,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FireworksData {
	pub flight_duration: u8,
	pub explosions: Vec<FireworkExplosionData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FoodData {
	pub nutrition: i32,
	pub saturation: f32,
	pub can_always_eat: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstrumentData {
	Id(&'static str),
	Custom(CustomInstrument),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomInstrument {
	pub description: NbtTag,
	pub sound_event: Sound,
	pub use_duration: f32,
	pub range: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KineticWeaponData {
	pub delay_ticks: Option<i32>,
	pub damage_conditions: KineticWeaponCondition,
	pub dismount_conditions: KineticWeaponCondition,
	pub knockback_conditions: KineticWeaponCondition,
	pub forward_movement: Option<f32>,
	pub damage_multiplier: Option<f32>,
	pub sound: Option<Sound>,
	pub hit_sound: Option<Sound>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KineticWeaponCondition {
	pub max_duration_ticks: i32,
	pub min_speed: Option<i32>,
	pub min_relative_speed: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LodestoneTrackerData {
	pub pos: Option<Vec<i32>>,
	pub dimension: Option<&'static str>,
	pub tracked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapDecoration {
	pub decoration_type: &'static str,
	pub x: f64,
	pub z: f64,
	pub rotation: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PiercingWeaponData {
	pub deals_knockback: Option<bool>,
	pub dismounts: Option<bool>,
	pub sound: Option<Sound>,
	pub hit_sound: Option<Sound>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PotionContentsData {
	Id(&'static str),
	Custom(CustomPotionContents),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomPotionContents {
	pub potion: &'static str,
	pub custom_color: i32,
	pub custom_name: &'static str,
	pub custom_effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileData {
	pub name: Option<&'static str>,
	pub id: Option<u128>,
	pub properties: Vec<ProfileDataProperties>,
	pub texture: Option<&'static str>,
	pub cape: Option<&'static str>,
	pub elytra: Option<&'static str>,
	pub model: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileDataProperties {
	pub texture_data: &'static str,
	pub signature: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuspiciousStewEffect {
	pub id: &'static str,
	pub duration: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwingAnimationData {
	pub animation: SwingAnimationType,
	pub duration: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwingAnimationType {
	None,
	Whack,
	Stab,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolData {
	pub default_mining_speed: Option<f32>,
	pub damage_per_block: Option<i32>,
	pub can_destroy_blocks_in_creative: Option<bool>,
	pub rules: Vec<ToolRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolRule {
	pub blocks: Vec<&'static str>,
	pub speed: Option<f32>,
	pub correct_for_drops: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipDisplayData {
	pub hide_tooltip: bool,
	pub hidden_components: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrimData {
	pub pattern: &'static str,
	pub material: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseCooldownData {
	pub seonds: f32,
	pub cooldown_ground: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseEffectsData {
	pub can_sprint: Option<bool>,
	pub speed_mulitplier: Option<f32>,
	pub interact_vibrations: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeaponData {
	pub item_damage_per_attack: Option<i32>,
	pub disable_blocking_for_seonds: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
	pub raw: &'static str,
	pub filtered: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WritableBookContentData {
	pub pages: Vec<Page>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrittenBookContentData {
	pub pages: Vec<Page>,
	pub title: Page,
	pub author: &'static str,
	pub generation: Vec<i32>,
	pub resolved: Option<bool>,
}
