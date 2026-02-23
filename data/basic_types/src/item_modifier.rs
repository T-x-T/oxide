use std::collections::HashMap;

use crate::*;

//https://minecraft.wiki/w/Item_modifier
#[derive(Debug, Clone, PartialEq)]
pub enum ItemModifier {
	ApplyBonus(ApplyBonusData),
	CopyComponents(CopyComponentsData),
	CopyCustomData(CopyCustomDataData),
	CopyName(&'static str),
	CopyState(CopyStateData),
	EnchantRandomly(EnchantRandomlyData),
	EnchantWithLevels(EnchantWithLevelsData),
	EnchantedCountIncrease(EnchantCountIncreaseData),
	ExplorationMap(ExplorationMapData),
	ExplosionDecay,
	FillPlayerHead(EntityLootContext),
	Filtered(FilteredData),
	FurnaceSmelt,
	LimitCount(NumberProvider),
	ModifyContents(ModifyContentsData),
	Reference(Vec<ItemModifier>),
	Sequence(Vec<ItemModifier>),
	SetAttributes(SetAttributesData),
	SetBannerPattern(SetBannerPatternData),
	SetBookCover(SetBookCoverData),
	SetComponents(Vec<DataComponent>),
	SetContents(SetContentsData),
	SetCount(SetCountData),
	SetCustomData(&'static str),
	SetCustomModelData(SetCustomModelData),
	SetDamage(SetDamageData),
	SetEnchantments(SetEnchantmentsData),
	SetFireworks(SetFireworksData),
	SetFireworkExplosion(SetFireworksExplosionData),
	SetInstrument(&'static str),
	SetItem(&'static str),
	SetLootTable(SetLootTableData),
	SetLore(SetLoreData),
	SetName(SetNameData),
	SetOminousBottleAmplifier(NumberProvider),
	SertPotion(&'static str),
	SetRandomDyes(NumberProvider),
	SetRandomPotion(Vec<&'static str>),
	SetStewEffect(SetStewEffectData),
	SetWritableBookPages(SetWritableBookPagesData),
	SetWrittenBookPages(SetWrittenBookPagesData),
	ToggleTooltips(ToggleTooltipsData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentModifierMode {
	Append,
	Insert(Option<i32>),
	ReplaceAll,
	ReplaceSection(Option<i32>, Option<i32>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityLootContext {
	This,
	Attacker,
	DirectAttacker,
	AttackingPlayer,
	TargetEntity,
	InteractingEntity,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ApplyBonusData {
	pub enchantment: &'static str,
	pub formula: &'static str,
	pub extra: Option<i32>,
	pub probability: Option<f32>,
	pub bonus_multiplier: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyComponentsData {
	pub source: &'static str,
	pub include: Vec<&'static str>,
	pub exclude: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyCustomDataData {
	pub source: CopyCustomDataType,
	pub operations: Vec<CopyCustomDataOperation>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CopyCustomDataType {
	Context(CopyCustomDataContext),
	Storage(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CopyCustomDataContext {
	BlockEntity,
	This,
	Attacker,
	DirectAttacker,
	AttackingPlayer,
	TargetEntity,
	InteractingEntity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyCustomDataOperation {
	pub source: &'static str,
	pub target: &'static str,
	pub operation_type: CopyCustomDataOperationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CopyCustomDataOperationType {
	Replace,
	Append,
	Merge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyStateData {
	pub block: &'static str,
	pub properties: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantRandomlyData {
	pub enchant_randomly: Vec<&'static str>,
	pub only_compatible: bool,
	pub include_additional_cost_component: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantWithLevelsData {
	pub levels: NumberProvider,
	pub options: Vec<&'static str>,
	pub include_additional_cost_component: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantCountIncreaseData {
	pub count: NumberProvider,
	pub limit: i32,
	pub enchantment: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationMapData {
	pub destination: &'static str,
	pub decoration: &'static str,
	pub zoom: i32,
	pub search_radius: i32,
	pub skip_existing_chunks: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct FilteredData {
	pub item_filter: ItemPredicate,
	pub modifiert: Vec<ItemModifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyContentsData {
	pub component: ModifyContentsDataComponent,
	pub modifier: Vec<ItemModifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModifyContentsDataComponent {
	BundleContents,
	ChargedProjectiles,
	Container,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributesData {
	pub modifiers: Vec<SetAttributesDataModifier>,
	pub replace: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributesDataModifier {
	pub attribute: &'static str,
	pub operation: SetAttributesDataModifierOperation,
	pub amount: NumberProvider,
	pub id: &'static str,
	pub slot: Vec<SetAttributesDataModifierSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAttributesDataModifierOperation {
	AddValue,
	AddMultipliedBase,
	AddMultipliedTotal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAttributesDataModifierSlot {
	MainHand,
	OffHand,
	Feet,
	Legs,
	Chest,
	Head,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetBannerPatternData {
	pub patterns: Vec<SetBannerPatternDataPattern>,
	pub append: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetBannerPatternDataPattern {
	pub pattern: &'static str,
	pub color: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetBookCoverData {
	pub author: Option<&'static str>,
	pub generation: Option<u8>,
	pub title: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetContentsData {
	pub entries: Vec<LootTable>,
	pub component: SetContentsDataComponent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetContentsDataComponent {
	Container,
	BundleContents,
	ChargedProjectiles,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCountData {
	pub count: NumberProvider,
	pub add: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomModelData {
	pub floats: SetCustomModelDataFloats,
	pub flags: SetCustomModelDataFlags,
	pub strings: SetCustomModelDataStrings,
	pub colors: SetCustomModelDataColors,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomModelDataFloats {
	pub values: Vec<NumberProvider>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomModelDataFlags {
	pub values: Vec<bool>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomModelDataStrings {
	pub values: Vec<&'static str>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomModelDataColors {
	pub values: Vec<NumberProvider>,
	pub mode: ComponentModifierMode,
}


#[derive(Debug, Clone, PartialEq)]
pub struct SetDamageData {
	pub damage: NumberProvider,
	pub add: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetEnchantmentsData {
	pub enchantments: HashMap<i32, NumberProvider>,
	pub add: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetFireworksData {
	pub explosions: Vec<SetFireworksDataExplosion>,
	pub flight_duration: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetFireworksDataExplosion {
	pub values: Vec<SetFireworksExplosionData>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetFireworksExplosionData {
	pub shape: SetFireworksDataExplosionValuesShape,
	pub colors: Vec<i32>,
	pub fade_colors: Vec<i32>,
	pub has_trail: bool,
	pub has_twinkle: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetFireworksDataExplosionValuesShape {
	SmallBall,
	LargeBall,
	Star,
	Creeper,
	Burst,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetLootTableData {
	pub name: &'static str,
	pub seed: Option<i32>,
	pub blockentity_type: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetLoreData {
	pub lore: Vec<&'static str>,
	pub entity: EntityLootContext,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetNameData {
	pub name: &'static str,
	pub entity: EntityLootContext,
	pub target: SetNameDataTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetNameDataTarget {
	CustomName,
	ItemName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStewEffectData {
	pub effects: Vec<SetStewEffectDataEffect>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStewEffectDataEffect {
	pub effect_type: &'static str,
	pub duration: NumberProvider,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetWritableBookPagesData {
	pub pages: Vec<SetWritableBookPagesDataPage>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetWritableBookPagesDataPage {
	pub raw: &'static str,
	pub filtered: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetWrittenBookPagesData {
	pub pages: Vec<SetWrittenBookPagesDataPage>,
	pub mode: ComponentModifierMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetWrittenBookPagesDataPage {
	pub raw: &'static str,
	pub filtered: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleTooltipsData {
	pub attribute_modifier: Option<bool>,
	pub can_break: Option<bool>,
	pub can_place_on: Option<bool>,
	pub dyed_color: Option<bool>,
	pub enchantments: Option<bool>,
	pub stored_enchantments: Option<bool>,
	pub trim: Option<bool>,
	pub unbreakable: Option<bool>,
}
