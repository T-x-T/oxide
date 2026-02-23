#![allow(clippy::needless_return)]

pub mod data_component;
pub mod data_component_predicate;
pub mod item_modifier;
pub mod loot_table;
pub mod nbt;
pub mod predicate;

pub use data_component::*;
pub use data_component_predicate::*;
pub use item_modifier::*;
pub use loot_table::*;
pub use nbt::*;
pub use predicate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NumberProvider {
	Constant(f32),
	Uniform(f32, f32),
	Binomial(i32, f32),
	Score(&'static str, &'static str, &'static str, &'static str, f32),
	Storage(&'static str, &'static str),
	EnchantmentLevel(&'static str),
	Sum(Vec<NumberProvider>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemPredicate {
	pub items: Vec<&'static str>,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
	pub data_component_predicates: Vec<DataComponentPredicate>,
}
