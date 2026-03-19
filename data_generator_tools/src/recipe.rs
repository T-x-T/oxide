use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use crate::*;
use jzon::JsonValue;

pub fn generate() {
	let mut output = r#"#![allow(clippy::needless_return)]
use basic_types::recipe::*;
use std::collections::HashMap;
"#
	.to_string();

	output += "pub fn get_recipes() -> HashMap<&'static str, Recipe> {
	let mut output: HashMap<&'static str, Recipe> = HashMap::new();\n";

	for file in read_dir_recursively(PathBuf::from_str("../official_server/generated/data/minecraft/recipe").unwrap()).unwrap() {
		let file_content = fs::read_to_string(file.path()).unwrap();
		let file_json = jzon::parse(&file_content).unwrap();

		let key = "minecraft:".to_string()
			+ file
				.path()
				.to_str()
				.unwrap()
				.to_string()
				.replace("../official_server/generated/data/minecraft/recipe/", "")
				.replace(".json", "")
				.as_str();

		output += format!("\toutput.insert(\"{key}\", {});\n", turn_json_into_recipe(file_json)).as_str();
	}

	output += "\treturn output;\n}";

	let path = PathBuf::from("../data/recipes/src/lib.rs");
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn turn_json_into_recipe(input: JsonValue) -> String {
	return match input["type"].as_str().unwrap() {
		"minecraft:blasting" => {
			let furnace_data = turn_json_into_furnace_data(input);
			format!("Recipe::Blasting(Box::new({furnace_data}))")
		}
		"minecraft:campfire_cooking" => {
			let ingredient = turn_json_into_string_vec(input["ingredient"].clone());
			let cooking_time = turn_json_into_optional_number(input["cookingtime"].clone());
			let result_id = input["result"]["id"].as_str().unwrap();
			//TODO: missing data_components for result_components
			let campfire_cooking_data = format!(
				"CampfireCookingData{{
ingredient: {ingredient},
cooking_time: {cooking_time},
result_id: \"{result_id}\",
result_components: None,
}}"
			);
			format!("Recipe::CampfireCooking(Box::new({campfire_cooking_data}))")
		}
		"minecraft:crafting_shaped" => {
			let category = turn_json_into_category(input["category"].clone());
			let group = turn_json_into_optional_string(input["group"].clone());
			let show_notification = turn_json_into_optional_bool(input["show_notification"].clone());
			let pattern = turn_json_into_string_vec(input["pattern"].clone());

			let mut key = "vec![\n".to_string();
			for (k, v) in input["key"].as_object().unwrap().iter() {
				if v.is_string() {
					key += format!("\t(\"{k}\", vec![\"{}\"]),\n", v.as_str().unwrap()).as_str();
				} else {
					key += format!("\t(\"{k}\", vec![\n").as_str();
					for v2 in v.as_array().unwrap() {
						key += format!("\t\t\"{}\",\n", v2.as_str().unwrap()).as_str();
					}
					key += "\t]),\n";
				}
			}
			key += "].into_iter().collect()";

			let result_id = input["result"]["id"].as_str().unwrap();
			//TODO: missing data_components for result_components
			let result_count = turn_json_into_optional_number(input["result"]["count"].clone());

			let crafting_shaped_data = format!(
				"CraftingShapedData{{
category: {category},
group: {group},
show_notification: {show_notification},
pattern: {pattern},
key: {key},
result_id: \"{result_id}\",
result_count: {result_count},
result_components: None,
}}"
			);
			format!("Recipe::CraftingShaped(Box::new({crafting_shaped_data}))")
		}
		"minecraft:crafting_shapeless" => {
			let category = turn_json_into_category(input["category"].clone());
			let group = turn_json_into_optional_string(input["group"].clone());

			let mut ingredients = "vec![\n".to_string();
			for v in input["ingredients"].as_array().unwrap().iter() {
				if v.is_string() {
					ingredients += format!("\tvec![\"{}\"],\n", v.as_str().unwrap()).as_str();
				} else {
					ingredients += "\tvec![\n";
					for v2 in v.as_array().unwrap() {
						ingredients += format!("\t\t\"{}\",\n", v2.as_str().unwrap()).as_str();
					}
					ingredients += "\t],\n";
				}
			}
			ingredients += "]";

			let result_id = input["result"]["id"].as_str().unwrap();
			//TODO: missing data_components for result_components
			let result_count = turn_json_into_optional_number(input["result"]["count"].clone());
			let crafting_shapeless_data = format!(
				"CraftingShapelessData{{
category: {category},
group: {group},
ingredients: {ingredients},
result_id: \"{result_id}\",
result_count: {result_count},
result_components: None,
}}"
			);
			format!("Recipe::CraftingShapeless(Box::new({crafting_shapeless_data}))")
		}
		"minecraft:crafting_transmute" => {
			let category = turn_json_into_category(input["category"].clone());
			let group = turn_json_into_optional_string(input["group"].clone());
			let input_val = turn_json_into_string_vec(input["input"].clone());
			let material = turn_json_into_string_vec(input["material"].clone());
			let result_id = input["result"]["id"].as_str().unwrap();
			let crafting_transmute_data = format!(
				"CraftingTransmuteData{{
category: {category},
group: {group},
input: {input_val},
material: {material},
result_id: \"{result_id}\",
}}"
			);

			format!("Recipe::CraftingTransmute(Box::new({crafting_transmute_data}))")
		}
		"minecraft:crafting_special_armordye"
		| "minecraft:crafting_special_bookcloning"
		| "minecraft:crafting_special_bannerduplicate"
		| "minecraft:crafting_special_firework_rocket"
		| "minecraft:crafting_special_firework_star"
		| "minecraft:crafting_special_firework_star_fade"
		| "minecraft:crafting_special_mapcloning"
		| "minecraft:crafting_special_mapextending"
		| "minecraft:crafting_special_repairitem"
		| "minecraft:crafting_special_shielddecoration"
		| "minecraft:crafting_special_tippedarrow" => {
			let special_type = turn_json_into_special_type(input["type"].clone());
			let category = turn_json_into_category(input["category"].clone());
			let crafting_special_data = format!(
				"CraftingSpecialData{{
special_type: {special_type},
category: {category},
}}"
			);

			format!("Recipe::CraftingSpecial({crafting_special_data})")
		}
		"minecraft:crafting_decorated_pot" => {
			let category = turn_json_into_category(input["category"].clone());
			format!("Recipe::CraftingDecoratedPot({category})")
		}
		"minecraft:smelting" => {
			let furnace_data = turn_json_into_furnace_data(input);
			format!("Recipe::Smelting(Box::new({furnace_data}))")
		}
		"minecraft:smithing_transform" => {
			let template = turn_json_into_string_vec(input["template"].clone());
			let base = turn_json_into_string_vec(input["base"].clone());
			let addition = turn_json_into_string_vec(input["addition"].clone());
			let result_id = input["result"]["id"].as_str().unwrap();
			//TODO: missing data_components for result_components
			let result_count = turn_json_into_optional_number(input["result"]["count"].clone());
			let smithing_transform_data = format!(
				"SmithingTransformData{{
template: {template},
base: {base},
addition: {addition},
result_id: \"{result_id}\",
result_count: {result_count},
result_components: None,
}}"
			);

			format!("Recipe::SmithingTransform(Box::new({smithing_transform_data}))")
		}
		"minecraft:smithing_trim" => {
			let template = turn_json_into_string_vec(input["template"].clone());
			let base = turn_json_into_string_vec(input["base"].clone());
			let addition = turn_json_into_string_vec(input["addition"].clone());
			let smithing_trim_data = format!(
				"SmithingTrimData{{
template: {template},
base: {base},
addition: {addition},
}}"
			);

			format!("Recipe::SmithingTrim({smithing_trim_data})")
		}
		"minecraft:smoking" => {
			let furnace_data = turn_json_into_furnace_data(input);
			format!("Recipe::Smoking(Box::new({furnace_data}))")
		}
		"minecraft:stonecutting" => {
			let ingredient = turn_json_into_string_vec(input["ingredient"].clone());
			let result_id = input["result"]["id"].as_str().unwrap();
			//TODO: missing data_components for result_components
			let result_count = turn_json_into_optional_number(input["result"]["count"].clone());
			let crafting_stone_cutting_data = format!(
				"StoneCuttingData{{
ingredient: {ingredient},
result_id: \"{result_id}\",
result_count: {result_count},
result_components: None,
}}"
			);

			format!("Recipe::StoneCutting(Box::new({crafting_stone_cutting_data}))")
		}
		x => panic!("no idea what recipe type {x} is"),
	};
}

fn turn_json_into_furnace_data(input: JsonValue) -> String {
	let category = turn_json_into_category(input["category"].clone());
	let group = turn_json_into_optional_string(input["group"].clone());
	let ingredient = turn_json_into_string_vec(input["ingredient"].clone());
	let cooking_time = turn_json_into_optional_number(input["cookingtime"].clone());
	let result_id = input["result"]["id"].as_str().unwrap();
	//TODO: missing data_components for result_components
	let experience = turn_json_into_optional_number(input["experience"].clone());

	format!(
		"FurnaceData{{
category: {category},
group: {group},
ingredient: {ingredient},
cooking_time: {cooking_time},
result_id: \"{result_id}\",
result_components: None,
experience: {experience},
}}"
	)
}

fn turn_json_into_category(input: JsonValue) -> String {
	return match input.as_str().unwrap() {
		"equipment" => "Some(Category::Equipment)".to_string(),
		"building" => "Some(Category::Building)".to_string(),
		"misc" => "Some(Category::Misc)".to_string(),
		"redstone" => "Some(Category::Redstone)".to_string(),
		"blocks" => "Some(Category::Blocks)".to_string(),
		"food" => "Some(Category::Food)".to_string(),
		x => {
			println!("no idea what category {x} is supposed to be");
			"None".to_string()
		}
	};
}

fn turn_json_into_special_type(input: JsonValue) -> String {
	return match input.as_str().unwrap() {
		"minecraft:crafting_special_armordye" => "CraftingSpecialType::Armordye".to_string(),
		"minecraft:crafting_special_bannerduplicate" => "CraftingSpecialType::Bannerduplicate".to_string(),
		"minecraft:crafting_special_bookcloning" => "CraftingSpecialType::Bookcloning".to_string(),
		"minecraft:crafting_special_firework_rocket" => "CraftingSpecialType::FireworkRocket".to_string(),
		"minecraft:crafting_special_firework_star" => "CraftingSpecialType::FireworkStar".to_string(),
		"minecraft:crafting_special_firework_star_fade" => "CraftingSpecialType::FireworkStarFade".to_string(),
		"minecraft:crafting_special_mapcloning" => "CraftingSpecialType::Mapcloning".to_string(),
		"minecraft:crafting_special_mapextending" => "CraftingSpecialType::Mapextending".to_string(),
		"minecraft:crafting_special_repairitem" => "CraftingSpecialType::Repairitem".to_string(),
		"minecraft:crafting_special_shielddecoration" => "CraftingSpecialType::Shielddecoration".to_string(),
		"minecraft:crafting_special_tippedarrow" => "CraftingSpecialType::Tippedarrow".to_string(),
		x => panic!("no idea what special type {x} is supposed to be"),
	};
}

fn turn_json_into_optional_string(input: JsonValue) -> String {
	return match input.as_str() {
		Some(x) => format!("Some(\"{x}\")"),
		None => "None".to_string(),
	};
}

fn turn_json_into_optional_number(input: JsonValue) -> String {
	return match input.as_number() {
		Some(x) => format!("Some({x})"),
		None => "None".to_string(),
	};
}

fn turn_json_into_optional_bool(input: JsonValue) -> String {
	return match input.as_bool() {
		Some(x) => format!("Some({x})"),
		None => "None".to_string(),
	};
}

fn turn_json_into_string_vec(input: JsonValue) -> String {
	let mut output = String::new();
	let mut parts: Vec<String> = input.as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
	if parts.is_empty() && input.is_string() {
		parts.push(input.as_str().unwrap().to_string());
	}
	if parts.is_empty() {
		output += "vec![]"
	} else {
		output += "vec![\n";
		for function in parts {
			for line in function.lines() {
				output += format!("\t\"{line}\",\n").as_str();
			}
		}
		output += "]";
	}

	return output;
}
