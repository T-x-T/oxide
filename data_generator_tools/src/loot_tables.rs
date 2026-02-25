use std::collections::BTreeMap;
use std::fs::{self, DirEntry, OpenOptions, ReadDir};
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

use jzon::JsonValue;

use crate::{convert_to_upper_camel_case, read_dir_recursively};

pub fn generate() {
	let mut output = r#"#![allow(clippy::needless_return)]
use basic_types::data_component_predicate::*;
use basic_types::item_modifier::*;
use basic_types::loot_table::*;
use basic_types::predicate::*;
use basic_types::*;
use std::collections::HashMap;
"#
	.to_string();

	for dir in fs::read_dir("../official_server/generated/data/minecraft/loot_table").expect("couldnt open loot_table directory") {
		let name = dir.unwrap().file_name().into_string().unwrap();
		do_dir(name.clone());
		output += format!("mod {name};\n").as_str();
	}

	println!("{output}");

	//let path = PathBuf::from("../data/loot_tables/src/lib.rs");
	//let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	//file.write_all(output.as_bytes()).unwrap();
	//file.flush().unwrap();
}

fn do_dir(name: String) {
	let mut output: BTreeMap<String, JsonValue> = BTreeMap::new();

	let mut path = PathBuf::from_str("../official_server/generated/data/minecraft/loot_table").unwrap();
	path.push(name.clone());

	for file in read_dir_recursively(path).unwrap() {
		let file_content = fs::read_to_string(file.path()).unwrap();
		let file_json = jzon::parse(&file_content).unwrap();
		output.insert(
			"minecraft:".to_string()
				+ file
					.path()
					.to_str()
					.unwrap()
					.to_string()
					.replace(format!("../official_server/generated/data/minecraft/loot_table/{name}/").as_str(), "")
					.replace(".json", "")
					.as_str(),
			file_json,
		);
	}

	//create output file contents
	let mut file_content = "use super::*;\n".to_string();
	file_content += format!("pub fn get_{name}() -> HashMap<&'static str, LootTable> {{\n").as_str();
	file_content += "\tlet mut output: HashMap<&'static str, LootTable> = HashMap::new();\n";
	for (k, v) in output {
		let loot_table = turn_json_into_loot_table(v, k.as_str());
		file_content += format!("\toutput.insert(\"{k}\", {loot_table});\n").as_str();
	}
	file_content += "\treturn output;\n";
	file_content += "}";

	//write output file
	let path = PathBuf::from(format!("../data/loot_tables/src/{name}.rs"));
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(file_content.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn turn_json_into_loot_table(input: JsonValue, block_name: &str) -> String {
	let mut output = "LootTable {\n".to_string();

	let loot_table_type = convert_to_upper_camel_case(input["type"].as_str().unwrap());
	output += format!("\t\tloot_table_type: LootTableType::{loot_table_type},\n").as_str();

	let functions: Vec<String> =
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone())).collect();
	if functions.is_empty() {
		output += "\t\tfunctions: vec![],\n"
	} else {
		output += "\t\tfunctions: vec![\n";
		for function in functions {
			for line in function.lines() {
				output += format!("\t\t\t{line}\n").as_str();
			}
		}
		output += "\t\t],\n";
	}

	let pools: Vec<String> =
		input["pools"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_pool(x.clone(), block_name)).collect();
	output += "\t\tpools: vec![\n";
	for pool in pools {
		for line in pool.lines() {
			output += format!("\t\t\t{line}\n").as_str();
		}
	}
	output += "\t\t],\n";

	let random_sequence = match input["random_sequence"].as_str() {
		Some(x) => format!("Some(\"{x}\")"),
		None => "None".to_string(),
	};
	output += format!("\t\trandom_sequence: {random_sequence},\n").as_str();

	output += "\t}";
	return output;
}

fn turn_json_into_function(input: JsonValue) -> String {
	let function_type = convert_to_upper_camel_case(input["function"].as_str().unwrap());
	match function_type.as_str() {
		"ApplyBonus" => {
			let enchantment = input["enchantment"].as_str().unwrap();
			let formula = input["formula"].as_str().unwrap();

			let parameters = input["parameters"].clone();
			let extra =
				if parameters["extra"].is_number() { format!("Some({})", parameters["extra"].as_number().unwrap()) } else { "None".to_string() };
			let probability = if parameters["probability"].is_number() {
				format!("Some({})", parameters["probability"].as_number().unwrap())
			} else {
				"None".to_string()
			};
			let bonus_multiplier = if parameters["bonus_multiplier"].is_number() {
				format!("Some({})", parameters["bonus_multiplier"].as_number().unwrap())
			} else {
				"None".to_string()
			};

			let data_struct = format!(
				"ApplyBonusData {{\n\tenchantment: \"{enchantment}\",\n\tformula: \"{formula}\",\textra: {extra},\n\tprobability: {probability},\n\tbonus_multiplier: {bonus_multiplier}\n}}"
			);
			return format!("ItemModifier::ApplyBonus({data_struct}),\n");
		}
		"ExplosionDecay" => return "ItemModifier::ExplosionDecay,\n".to_string(),
		"SetCount" => {
			let count = turn_json_into_number_provider(input["count"].clone());

			let add = if input["add"].is_boolean() { format!("Some({})", input["add"].as_bool().unwrap()) } else { "None".to_string() };

			let data_struct = format!("SetCountData {{\n\tcount: {count}\n\tadd: {add}\n}}");
			return format!("ItemModifier::SetCount({data_struct}),\n");
		}
		"SetPotion" => return format!("ItemModifier::SetPotion(\"{}\"),\n", input["id"].as_str().unwrap()),
		"SetInstrument" => return format!("ItemModifier::SetInstrument(\"{}\"),\n", input["options"].as_str().unwrap()),
		x => panic!("unknown function_type {x}"),
	}

	//TODO: handle functions with parameters
}

fn turn_json_into_pool(input: JsonValue, block_name: &str) -> String {
	let mut output = "LootTablePool {\n".to_string();

	let conditions: Vec<String> =
		input["conditions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
	if conditions.is_empty() {
		output += "\tconditions: vec![],\n"
	} else {
		output += "\tconditions: vec![\n";
		for condition in conditions {
			for line in condition.lines() {
				output += format!("\t\t{line}\n").as_str();
			}
			output.pop();
			output += ",\n";
		}
		output += "\t],\n";
	}

	let functions: Vec<String> =
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone())).collect();
	if functions.is_empty() {
		output += "\tfunctions: vec![],\n"
	} else {
		output += "\tfunctions: vec![\n";
		for function in functions {
			for line in function.lines() {
				output += format!("\t\t{line}\n").as_str();
			}
		}
		output += "\t],\n";
	}

	let rolls = input["rolls"].clone();
	output += format!("\trolls: {}\n", turn_json_into_number_provider(rolls)).as_str();

	let rolls = input["bonus_rolls"].clone();
	output += format!("\tbonus_rolls: {}\n", turn_json_into_number_provider(rolls)).as_str();

	output += "\tentries: vec![],\n";

	output += "},";
	return output;
}

fn turn_json_into_number_provider(input: JsonValue) -> String {
	if input.is_number() {
		let mut number = input.as_number().unwrap().to_string();
		if !number.contains(".") {
			number += ".0";
		}
		return format!("NumberProvider::Constant({number}),");
	}
	let number_provider_type = convert_to_upper_camel_case(input["type"].as_str().unwrap());
	match number_provider_type.as_str() {
		"Uniform" => {
			let min = turn_json_into_number_provider(input["min"].clone());
			let max = turn_json_into_number_provider(input["max"].clone());

			return format!("NumberProvider::Uniform(Box::new({min}), Box::new({max})),");
		}
		x => panic!("unknown number_provider_type {x}"),
	}
	//TODO: handle other variants
}

fn turn_json_into_predicate(input: JsonValue, block_name: &str) -> String {
	let condition = convert_to_upper_camel_case(input["condition"].as_str().unwrap());
	match condition.as_str() {
		"SurvivesExplosion" => return "Predicate::SurvivesExplosion".to_string(),
		"KilledByPlayer" => return "Predicate::KilledByPlayer".to_string(),
		"Inverted" => {
			return format!("Predicate::Inverted(Box::new({}))", turn_json_into_predicate(input["term"].clone(), block_name));
		}
		"AnyOf" => {
			let predicates: Vec<String> =
				input["terms"].as_array().unwrap().iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
			return format!("Predicate::AnyOf(vec![{}])", predicates.join(",\n"));
		}
		"MatchTool" => {
			let predicate: JsonValue = input["predicate"].clone();

			let items = if predicate["items"].is_array() {
				let items = predicate["items"].as_array().unwrap();
				let mut items_code = "vec![".to_string();
				for item in items {
					items_code += format!("\"{}\", ", item.as_str().unwrap()).as_str();
				}
				items_code += "]";
				items_code
			} else if predicate["items"].is_string() {
				format!("vec![\"{}\"]", predicate["items"].as_str().unwrap())
			} else {
				"vec![]".to_string()
			};

			let count =
				if predicate["count"].is_number() { format!("Some({})", predicate["count"].as_number().unwrap()) } else { "None".to_string() };
			let count_min = if predicate["count_min"].is_number() {
				format!("Some({})", predicate["count_min"].as_number().unwrap())
			} else {
				"None".to_string()
			};
			let count_max = if predicate["count_max"].is_number() {
				format!("Some({})", predicate["count_max"].as_number().unwrap())
			} else {
				"None".to_string()
			};

			//TODO: missing DataComponentPredicate
			return format!(
				"Predicate::MatchTool(ItemPredicate{{items: {items},\ncount: {count},\ncount_min: {count_min},\ncount_max: {count_max},\ndata_component_predicates: vec![]\n}})"
			);
		}
		"BlockStateProperty" => {
			let block = input["block"].as_str().unwrap();
			let properties = input["properties"].as_object().unwrap();

			let mut properties_code = "vec![".to_string();
			for (property, value) in properties.iter() {
				if value.is_string() {
					let block_type = convert_to_upper_camel_case(get_block_type(block_name).as_str());
					let property_name = convert_to_upper_camel_case(property);
					let property_name = block_type + property_name.as_str();

					let value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&value.as_str().unwrap().to_string()) {
						format!("Num{}", convert_to_upper_camel_case(value.as_str().unwrap()))
					} else {
						convert_to_upper_camel_case(value.as_str().unwrap())
					};

					properties_code +=
						format!("(Property::{property_name}({property_name}::{value}), Property::{property_name}({property_name}::{value}))").as_str();
				} else {
					panic!("cant handle predicate BlockStateProperty with min and max yet");
				}
			}
			properties_code += "]";
			//TODO: missing DataComponentPredicate
			return format!(
				"Predicate::BlockStateProperty(PredicateBlockStateProperty{{\n\tblock: \"{block}\",\n\tproperties: {properties_code}\n}})"
			);
		}
		"LocationCheck" => {
			let offset_x =
				if input["offset_x"].is_number() { format!("Some({})", input["offset_x"].as_number().unwrap()) } else { "None".to_string() };
			let offset_y =
				if input["offset_y"].is_number() { format!("Some({})", input["offset_y"].as_number().unwrap()) } else { "None".to_string() };
			let offset_z =
				if input["offset_z"].is_number() { format!("Some({})", input["offset_z"].as_number().unwrap()) } else { "None".to_string() };

			let predicate = if input.has_key("predicate") {
				format!("Some({})", turn_json_into_location_predicate(input["predicate"].clone(), block_name))
			} else {
				"None".to_string()
			};

			return format!(
				"Predicate::LocationCheck(Box::new(PredicateLocationCheck {{\noffset_x: {offset_x},\noffset_y: {offset_y},\noffset_z: {offset_z},\npredicate: {predicate}}}))"
			);
		}
		"EntityProperties" => {
			let entity_loot_context = convert_to_upper_camel_case(input["entity"].as_str().unwrap());
			let entity_loot_context = format!("EntityLootContext::{entity_loot_context}");
			let entity_predicate = "None"; //TODO: implement entity predicate

			return format!(
				"Predicate::EntityProperties(Box::new(PredicateEntityProperties {{\nentity: {entity_loot_context},\npredicate: {entity_predicate}\n}}))"
			);
		}
		"RandomChance" => {
			return format!("Predicate::RandomChance({})", turn_json_into_number_provider(input["chance"].clone()));
		}
		"RandomChanceWithEnchantedBonus" => {
			//TODO: needs proper implementation
			let unenchanted_chance = "None";
			let enchanted_chance = "None";
			let enchantment = "None";
			return format!(
				"Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {{\nunenchanted_chance: {unenchanted_chance},\nenchanted_chance: {enchanted_chance},\nenchantment: {enchantment}\n}})"
			);
		}
		"DamageSourceProperties" => {
			//TODO: needs proper implementation
			let direct_entity = "None";
			let source_entity = "None";
			let is_direct = "None";
			let tags = "vec![]";
			return format!(
				"Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {{\ndirect_entity: {direct_entity},\nsource_entity: {source_entity},\nis_direct: {is_direct},\ntags: {tags}\n}}))"
			);
		}
		x => panic!("unknown predicate {x}"),
	}
	//TODO: handle other variants
}

fn turn_json_into_location_predicate(input: JsonValue, block_name: &str) -> String {
	let biomes = if input["biomes"].is_array() {
		let items = input["biomes"].as_array().unwrap();
		let mut items_code = "vec![".to_string();
		for item in items {
			items_code += format!("\"{}\", ", item.as_str().unwrap()).as_str();
		}
		items_code += "]";
		items_code
	} else if input["biomes"].is_string() {
		format!("vec![{}]", input["biomes"].as_str().unwrap())
	} else {
		"vec![]".to_string()
	};

	let blocks = if input["block"]["blocks"].is_array() {
		let items = input["block"]["blocks"].as_array().unwrap();
		let mut items_code = "vec![".to_string();
		for item in items {
			items_code += format!("\"{}\", ", item.as_str().unwrap()).as_str();
		}
		items_code += "]";
		items_code
	} else if input["block"]["blocks"].is_string() {
		format!("vec![\"{}\"]", input["block"]["blocks"].as_str().unwrap())
	} else {
		"vec![]".to_string()
	};


	let mut block_state_code = "vec![".to_string();
	if input.has_key("block") {
		for (property, value) in input["block"]["state"].as_object().unwrap().iter() {
			if value.is_string() {
				let block_type = convert_to_upper_camel_case(get_block_type(block_name).as_str());
				let property_name = convert_to_upper_camel_case(property);
				let property_name = block_type + property_name.as_str();

				let value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&value.as_str().unwrap().to_string()) {
					format!("Num{}", convert_to_upper_camel_case(value.as_str().unwrap()))
				} else {
					convert_to_upper_camel_case(value.as_str().unwrap())
				};

				block_state_code +=
					format!("(Property::{property_name}({property_name}::{value}), Property::{property_name}({property_name}::{value}))").as_str();
			} else {
				panic!("cant handle predicate BlockStateProperty with min and max yet");
			}
		}
	}
	block_state_code += "]";

	let dimension =
		if input["dimension"].is_string() { format!("Some({})", input["dimension"].as_str().unwrap()) } else { "None".to_string() };

	let fluids = if input["fluid"]["fluids"].is_array() {
		let items = input["fluid"]["fluids"].as_array().unwrap();
		let mut items_code = "vec![".to_string();
		for item in items {
			items_code += format!("\"{}\", ", item.as_str().unwrap()).as_str();
		}
		items_code += "]";
		items_code
	} else if input["fluid"]["fluids"].is_string() {
		format!("vec![\"{}\"]", input["fluid"]["fluids"].as_str().unwrap())
	} else {
		"vec![]".to_string()
	};

	let mut fluid_state_code = "vec![".to_string();
	if input.has_key("fluid") {
		for (property, value) in input["fluid"]["state"].as_object().unwrap().iter() {
			if value.is_string() {
				let block_type = convert_to_upper_camel_case(get_block_type(block_name).as_str());
				let property_name = convert_to_upper_camel_case(property);
				let property_name = block_type + property_name.as_str();

				let value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&value.as_str().unwrap().to_string()) {
					format!("Num{}", convert_to_upper_camel_case(value.as_str().unwrap()))
				} else {
					convert_to_upper_camel_case(value.as_str().unwrap())
				};

				fluid_state_code +=
					format!("(Property::{property_name}({property_name}::{value}), Property::{property_name}({property_name}::{value}))").as_str();
			} else {
				panic!("cant handle predicate BlockStateProperty with min and max yet");
			}
		}
	}
	fluid_state_code += "]";

	let light = if input["light"].is_number() { format!("Some({})", input["light"].as_number().unwrap()) } else { "None".to_string() };
	let light_min =
		if input["light"]["min"].is_number() { format!("Some({})", input["light"]["min"].as_number().unwrap()) } else { "None".to_string() };
	let light_max =
		if input["light"]["max"].is_number() { format!("Some({})", input["light"]["max"].as_number().unwrap()) } else { "None".to_string() };

	let position_x =
		if input["position_x"].is_number() { format!("Some({})", input["position_x"].as_number().unwrap()) } else { "None".to_string() };
	let position_x_min = if input["position_x"]["min"].is_number() {
		format!("Some({})", input["positiob_x"]["min"].as_number().unwrap())
	} else {
		"None".to_string()
	};
	let position_x_max = if input["position_x"]["max"].is_number() {
		format!("Some({})", input["position_x"]["max"].as_number().unwrap())
	} else {
		"None".to_string()
	};

	let position_y =
		if input["position_y"].is_number() { format!("Some({})", input["position_y"].as_number().unwrap()) } else { "None".to_string() };
	let position_y_min = if input["position_y"]["min"].is_number() {
		format!("Some({})", input["positiob_y"]["min"].as_number().unwrap())
	} else {
		"None".to_string()
	};
	let position_y_max = if input["position_y"]["max"].is_number() {
		format!("Some({})", input["position_y"]["max"].as_number().unwrap())
	} else {
		"None".to_string()
	};

	let position_z =
		if input["position_z"].is_number() { format!("Some({})", input["position_z"].as_number().unwrap()) } else { "None".to_string() };
	let position_z_min = if input["position_z"]["min"].is_number() {
		format!("Some({})", input["positiob_z"]["min"].as_number().unwrap())
	} else {
		"None".to_string()
	};
	let position_z_max = if input["position_z"]["max"].is_number() {
		format!("Some({})", input["position_z"]["max"].as_number().unwrap())
	} else {
		"None".to_string()
	};

	let smokey = if input["smokey"].is_boolean() { format!("Some({})", input["smokey"].as_number().unwrap()) } else { "None".to_string() };
	let can_see_sky =
		if input["can_see_sky"].is_boolean() { format!("Some({})", input["can_see_sky"].as_number().unwrap()) } else { "None".to_string() };

	let structures = if input["structures"].is_array() {
		let items = input["structures"].as_array().unwrap();
		let mut items_code = "vec![".to_string();
		for item in items {
			items_code += format!("\"{}\", ", item.as_str().unwrap()).as_str();
		}
		items_code += "]";
		items_code
	} else if input["structures"].is_string() {
		format!("vec![{}]", input["structures"].as_str().unwrap())
	} else {
		"vec![]".to_string()
	};

	//TODO: missing NBT, DataComponent and DataComponentPredicate conversion
	return format!(
		"LocationPredicate {{
\tbiomes: {biomes},
\tblocks: {blocks},
\tblock_nbt: NbtTag::default(),
\tblock_state: {block_state_code},
\tblock_components: vec![],
\tblock_predicates: vec![],
\tdimension: {dimension},
\tfluids: {fluids},
\tfluid_state: {fluid_state_code},
\tfluid_components: vec![],
\tfluid_predicates: vec![],
\tlight: {light},
\tlight_max: {light_max},
\tlight_min: {light_min},
\tposition_x: {position_x},
\tposition_x_max: {position_x_max},
\tposition_x_min: {position_x_min},
\tposition_y: {position_y},
\tposition_y_max: {position_y_max},
\tposition_y_min: {position_y_min},
\tposition_z: {position_z},
\tposition_z_max: {position_z_max},
\tposition_z_min: {position_z_min},
\tsmokey: {smokey},
\tcan_see_sky: {can_see_sky},
\tstructures: {structures},
}}"
	);
}

fn get_block_type(block_name: &str) -> String {
	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	return blocks_json.as_object().unwrap().iter().find(|x| x.0 == block_name).unwrap().1["definition"]["type"]
		.as_str()
		.unwrap()
		.to_string();
}
