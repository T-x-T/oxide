use super::*;

pub fn generate() {
	let mut output = String::new();

	let items_file = std::fs::read_to_string("../official_server/generated/reports/items.json").expect("failed to read items.json report");
	let items_json = jzon::parse(&items_file).expect("failed to parse items.json report");
	let registries_file =
		std::fs::read_to_string("../official_server/generated/reports/registries.json").expect("failed to read registries.json report");
	let registries_json = jzon::parse(&registries_file).expect("failed to parse registries.json report");
	let items_registry = registries_json.as_object().unwrap()["minecraft:item"]["entries"].as_object().unwrap();

	output += "#![allow(clippy::needless_return)]\n";
	output += "use std::collections::HashMap;\n\n";
	output += "#[derive(Debug, Clone)]\n";
	output += "pub enum ItemRarity {\n";
	output += "\tCommon,\n";
	output += "\tUncommon,\n";
	output += "\tRare,\n";
	output += "\tEpic,\n";
	output += "}\n\n";
	output += "#[derive(Debug, Clone)]\n";
	output += "pub struct Item {\n";
	output += "\tpub max_stack_size: u8,\n";
	output += "\tpub rarity: ItemRarity,\n";
	output += "\tpub repair_cost: u8,\n";
	output += "\tpub id: i32,\n";
	output += "}\n\n";
	output += "pub fn get_item_name_by_id(id: i32) -> String {\n";
	output += "\treturn get_items().into_iter().find(|x| x.1.id == id).unwrap_or(get_items().into_iter().next().unwrap()).0;\n";
	output += "}\n\n";
	output += "pub fn get_items() -> HashMap<String, Item> {\n";
	output += "\treturn vec![\n";


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
			"\t\t(\"{key}\", Item {{ max_stack_size: {max_stack_size}, rarity: ItemRarity::{rarity}, repair_cost: {repair_cost}, id: {id} }}),\n"
		)
		.as_str();
	}

	output += "\t].into_iter().map(|x| (x.0.to_string(), x.1)).collect();\n";
	output += "}\n";


	let path = std::path::PathBuf::from("../data/items/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}
