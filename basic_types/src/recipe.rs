use std::collections::HashMap;

use crate::data_component::DataComponent;

#[derive(Debug, Clone, PartialEq)]
pub enum Recipe {
	Blasting(Box<FurnaceData>),
	CampfireCooking(Box<CampfireCookingData>),
	CraftingShaped(Box<CraftingShapedData>),
	CraftingShapeless(Box<CraftingShapelessData>),
	CraftingTransmute(Box<CraftingTransmuteData>),
	CraftingSpecial(CraftingSpecialData),
	CraftingDecoratedPot(Option<Category>),
	Smelting(Box<FurnaceData>),
	SmithingTransform(Box<SmithingTransformData>),
	SmithingTrim(SmithingTrimData),
	Smoking(Box<FurnaceData>),
	StoneCutting(Box<StoneCuttingData>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
	Equipment,
	Building,
	Misc,
	Redstone,
	Blocks,
	Food,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FurnaceData {
	pub category: Option<Category>,
	pub group: Option<&'static str>,
	pub ingredient: Vec<&'static str>,
	pub cooking_time: Option<i32>,
	pub result_id: &'static str,
	pub result_components: Option<DataComponent>,
	pub experience: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CampfireCookingData {
	pub ingredient: Vec<&'static str>,
	pub cooking_time: Option<i32>,
	pub result_id: &'static str,
	pub result_components: Option<DataComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftingShapedData {
	pub category: Option<Category>,
	pub group: Option<&'static str>,
	pub show_notification: Option<bool>,
	pub pattern: Vec<&'static str>,
	pub key: HashMap<&'static str, Vec<&'static str>>,
	pub result_id: &'static str,
	pub result_count: Option<i32>,
	pub result_components: Option<DataComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftingShapelessData {
	pub category: Option<Category>,
	pub group: Option<&'static str>,
	pub ingredients: Vec<Vec<&'static str>>,
	pub result_id: &'static str,
	pub result_count: Option<i32>,
	pub result_components: Option<DataComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftingTransmuteData {
	pub category: Option<Category>,
	pub group: Option<&'static str>,
	pub input: Vec<&'static str>,
	pub material: Vec<&'static str>,
	pub result_id: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftingSpecialData {
	pub special_type: CraftingSpecialType,
	pub category: Option<Category>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CraftingSpecialType {
	Armordye,
	Bannerduplicate,
	Bookcloning,
	FireworkRocket,
	FireworkStar,
	FireworkStarFade,
	Mapcloning,
	Mapextending,
	Repairitem,
	Shielddecoration,
	Tippedarrow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmithingTransformData {
	pub template: Vec<&'static str>,
	pub base: Vec<&'static str>,
	pub addition: Vec<&'static str>,
	pub result_id: &'static str,
	pub result_count: Option<i32>,
	pub result_components: Option<DataComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmithingTrimData {
	pub template: Vec<&'static str>,
	pub base: Vec<&'static str>,
	pub addition: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoneCuttingData {
	pub ingredient: Vec<&'static str>,
	pub result_id: &'static str,
	pub result_count: Option<i32>,
	pub result_components: Option<DataComponent>,
}
