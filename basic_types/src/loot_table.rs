use crate::item_modifier::ItemModifier;
use crate::predicate::Predicate;
use crate::*;

//https://minecraft.wiki/w/Loot_table
#[derive(Debug, Clone, PartialEq)]
pub struct LootTable {
	pub loot_table_type: LootTableType,
	pub functions: Vec<ItemModifier>,
	pub pools: Vec<LootTablePool>,
	pub random_sequence: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LootTableType {
	Archaeology,
	Block,
	EntityInteract,
	BlockInteract,
	ChargedCreeper,
	Chest,
	Dispenser,
	Entity,
	Equipment,
	Gameplay,
	Harvest,
	Pot,
	Shearing,
	Spawner,
	Gift,
	Fishing,
	Barter,
	Custom,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LootTablePool {
	pub conditions: Vec<Predicate>,
	pub functions: Vec<ItemModifier>,
	pub rolls: NumberProvider,
	pub bonus_rolls: NumberProvider,
	pub entries: Vec<LootTablePoolEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LootTablePoolEntry {
	Singleton(LootTablePoolEntrySingleton),
	Tag(LootTablePoolEntryTag),
	Composite(LootTablePoolEntryComposite),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LootTablePoolEntrySingleton {
	pub entry_type: LootTablePoolEntrySingletonType,
	pub conditions: Vec<Predicate>,
	pub functions: Vec<ItemModifier>,
	pub weight: Option<i32>,
	pub quality: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LootTablePoolEntryTag {
	pub name: &'static str,
	pub expand: bool,
	pub conditions: Vec<Predicate>,
	pub functions: Vec<ItemModifier>,
	pub weight: Option<i32>,
	pub quality: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LootTablePoolEntryComposite {
	pub entry_type: LootTablePoolEntryCompositeType,
	pub children: Vec<LootTablePoolEntry>,
	pub conditions: Vec<Predicate>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LootTablePoolEntryCompositeType {
	Group,
	Alternatives,
	Sequence,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LootTablePoolEntrySingletonType {
	Item(&'static str),
	LootTableId(&'static str),
	LootTableCustom(LootTable),
	Dynamic(&'static str),
	Empty,
}
