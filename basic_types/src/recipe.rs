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


impl Recipe {
	pub fn get_result_count(&self) -> i32 {
		return match self {
			Recipe::Blasting(_) => 1,
			Recipe::CampfireCooking(_) => 1,
			Recipe::CraftingShaped(crafting_shaped_data) => crafting_shaped_data.result_count.unwrap_or(1),
			Recipe::CraftingShapeless(crafting_shapeless_data) => crafting_shapeless_data.result_count.unwrap_or(1),
			Recipe::CraftingTransmute(_crafting_transmute_data) => 1,
			Recipe::CraftingSpecial(_crafting_special_data) => 1,
			Recipe::CraftingDecoratedPot(_category) => 1,
			Recipe::Smelting(_) => 1,
			Recipe::SmithingTransform(smithing_transform_data) => smithing_transform_data.result_count.unwrap_or(1),
			Recipe::SmithingTrim(_) => 1,
			Recipe::Smoking(_) => 1,
			Recipe::StoneCutting(stone_cutting_data) => stone_cutting_data.result_count.unwrap_or(1),
		};
	}

	pub fn get_result_item_id(&self) -> Option<&'static str> {
		return match self {
			Recipe::Blasting(furnace_data) => Some(furnace_data.result_id),
			Recipe::CampfireCooking(campfire_cooking_data) => Some(campfire_cooking_data.result_id),
			Recipe::CraftingShaped(crafting_shaped_data) => Some(crafting_shaped_data.result_id),
			Recipe::CraftingShapeless(crafting_shapeless_data) => Some(crafting_shapeless_data.result_id),
			Recipe::CraftingTransmute(crafting_transmute_data) => Some(crafting_transmute_data.result_id),
			Recipe::CraftingSpecial(_) => None,
			Recipe::CraftingDecoratedPot(_) => None,
			Recipe::Smelting(furnace_data) => Some(furnace_data.result_id),
			Recipe::SmithingTransform(smithing_transform_data) => Some(smithing_transform_data.result_id),
			Recipe::SmithingTrim(_) => None,
			Recipe::Smoking(furnace_data) => Some(furnace_data.result_id),
			Recipe::StoneCutting(stone_cutting_data) => Some(stone_cutting_data.result_id),
		};
	}
}
