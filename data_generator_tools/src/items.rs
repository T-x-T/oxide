use super::*;

pub fn generate() {
	let mut output = String::new();

	let items_file = fs::read_to_string("../official_server/generated/reports/items.json").expect("failed to read items.json report");
	let items_json = jzon::parse(&items_file).expect("failed to parse items.json report");
	let registries_file =
		fs::read_to_string("../official_server/generated/reports/registries.json").expect("failed to read registries.json report");
	let registries_json = jzon::parse(&registries_file).expect("failed to parse registries.json report");
	let items_registry = registries_json.as_object().unwrap()["minecraft:item"]["entries"].as_object().unwrap();

	output += r#"#![allow(clippy::needless_return)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ItemRarity {
	Common,
	Uncommon,
	Rare,
	Epic,
}

#[derive(Debug, Clone)]
pub struct Item {
	pub max_stack_size: u8,
	pub rarity: ItemRarity,
	pub repair_cost: u8,
	pub id: i32,
}

pub fn get_item_name_by_id(id: i32) -> &'static str {
	return get_items().into_iter().find(|x| x.1.id == id).unwrap_or(get_items().into_iter().next().unwrap()).0;
}

pub fn get_items() -> HashMap<&'static str, Item> {
	let mut items = HashMap::new();
"#;

	for x in items_json.as_object().unwrap().iter() {
		let components = x.1.as_object().unwrap()["components"].clone();

		let key = x.0;
		let max_stack_size = components["minecraft:max_stack_size"].as_i32().unwrap();
		let rarity: String = components["minecraft:rarity"]
			.as_str()
			.unwrap()
			.chars()
			.enumerate()
			.map(|i| if i.0 == 0 { i.1.to_ascii_uppercase() } else { i.1 })
			.collect();
		let repair_cost = components["minecraft:repair_cost"].as_i32().unwrap();
		let id = items_registry[key]["protocol_id"].as_i32().unwrap();

		output += format!(
			"\titems.insert(\"{key}\", Item {{ max_stack_size: {max_stack_size}, rarity: ItemRarity::{rarity}, repair_cost: {repair_cost}, id: {id} }});\n"
		)
		.as_str();
	}

	output += r#"
	return items;
}
"#;


	let path = PathBuf::from("../data/items/src/lib.rs");
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}
