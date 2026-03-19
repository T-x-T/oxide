use std::collections::BTreeMap;
use std::fs::{self, DirEntry, OpenOptions, ReadDir};
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

use jzon::JsonValue;

use crate::{convert_to_upper_camel_case, read_dir_recursively};

pub fn generate() {
	let mut output = r#"#![allow(clippy::needless_return)]
use basic_types::blocks::*;
use basic_types::item_modifier::*;
use basic_types::loot_table::*;
use basic_types::nbt::*;
use basic_types::predicate::*;
use basic_types::*;
use std::collections::HashMap;
"#
	.to_string();

	let mut function_calls = String::new();

	for dir in fs::read_dir("../official_server/generated/data/minecraft/loot_table").expect("couldnt open loot_table directory") {
		let name = dir.unwrap().file_name().into_string().unwrap();
		do_dir(name.clone());
		output += format!("mod {name};\n").as_str();

		function_calls += format!("\toutput.insert(\"{name}\", {name}::get_{name}());\n").as_str();
	}

	output += "pub fn get_loot_tables() -> HashMap<&'static str, HashMap<&'static str, LootTable>> {
	let mut output: HashMap<&'static str, HashMap<&'static str, LootTable>> = HashMap::new();\n";
	output += function_calls.as_str();
	output += "\treturn output;\n}";

	let path = PathBuf::from("../data/loot_tables/src/lib.rs");
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
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

fn turn_json_into_loot_table(input: JsonValue, block_name: &str) -> (String) {
	let mut output = "LootTable {\n".to_string();

	if input["type"].is_string() {
		let loot_table_type = convert_to_upper_camel_case(input["type"].as_str().unwrap());
		output += format!("\t\tloot_table_type: LootTableType::{loot_table_type},\n").as_str();
	} else {
		output += "\t\tloot_table_type: LootTableType::Custom,\n".to_string().as_str();
	}

	let functions: Vec<String> =
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone(), block_name)).collect();
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

fn turn_json_into_function(input: JsonValue, block_name: &str) -> String {
	let function_type = convert_to_upper_camel_case(input["function"].as_str().unwrap());
	let function = match function_type.as_str() {
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
			format!("Function::ApplyBonus({data_struct}),\n")
		}
		"ExplosionDecay" => "Function::ExplosionDecay,\n".to_string(),
		"SetCount" => {
			let count = turn_json_into_number_provider(input["count"].clone());

			let add = if input["add"].is_boolean() { format!("Some({})", input["add"].as_bool().unwrap()) } else { "None".to_string() };

			let data_struct = format!("SetCountData {{\n\tcount: {count}\n\tadd: {add}\n}}");
			format!("Function::SetCount({data_struct}),\n")
		}
		"SetPotion" => format!("Function::SetPotion(\"{}\"),\n", input["id"].as_str().unwrap()),
		"SetInstrument" => format!("Function::SetInstrument(\"{}\"),\n", input["options"].as_str().unwrap()),
		"CopyComponents" => {
			let source = input["source"].as_str().unwrap();

			let mut include = String::new();
			let raw_include: Vec<String> =
				input["include"].as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
			if raw_include.is_empty() {
				include += "vec![],\n"
			} else {
				include += "vec![\n";
				for function in raw_include {
					for line in function.lines() {
						include += format!("\t\t\"{line}\",\n").as_str();
					}
				}
				include += "\t]";
			}

			let mut exclude = String::new();
			let raw_exlude: Vec<String> =
				input["exlucde"].as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
			if raw_exlude.is_empty() {
				exclude += "vec![],\n"
			} else {
				exclude += "vec![\n";
				for function in raw_exlude {
					for line in function.lines() {
						exclude += format!("\t\t\"{line}\",\n").as_str();
					}
				}
				exclude += "\t]";
			}

			let data_struct = format!("CopyComponentsData {{\n\tsource: \"{source}\",\n\tinclude: {include},\n\texclude: {exclude}\n}}");
			format!("Function::CopyComponents({data_struct}),\n")
		}
		"CopyState" => {
			let block = input["block"].as_str().unwrap();

			let mut properties = String::new();
			let raw_properties: Vec<String> =
				input["properties"].as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
			if raw_properties.is_empty() {
				properties += "vec![]\n"
			} else {
				properties += "vec![\n";
				for function in raw_properties {
					for line in function.lines() {
						properties += format!("\t\t\"{line}\",\n").as_str();
					}
				}
				properties += "\t]";
			}

			let data_struct = format!("CopyStateData {{\n\tblock: \"{block}\",\n\tproperties: {properties},\n}}");
			format!("Function::CopyState({data_struct}),\n")
		}
		"SetStewEffect" => {
			let mut stew_effect = String::new();
			let raw_stew_effect: Vec<String> = input["effects"]
				.as_array()
				.unwrap_or(&Vec::new())
				.iter()
				.map(|x| {
					let effect_type = x["type"].as_str().unwrap();
					let duration = turn_json_into_number_provider(x["duration"].clone());
					format!("SetStewEffectDataEffect {{\n\teffect_type: \"{effect_type}\",\n\tduration: {duration}\n}},")
				})
				.collect();
			if raw_stew_effect.is_empty() {
				stew_effect += "\tvec![],\n"
			} else {
				stew_effect += "\tvec![\n";
				for function in raw_stew_effect {
					for line in function.lines() {
						stew_effect += format!("\t\t{line}\n").as_str();
					}
				}
				stew_effect += "\t]";
			}

			format!("Function::SetStewEffect({stew_effect}),\n")
		}
		"SetDamage" => {
			let damage = turn_json_into_number_provider(input["damage"].clone());
			let add = if input["add"].is_boolean() { format!("Some({})", input["add"].as_bool().unwrap()) } else { "None".to_string() };

			let data_struct = format!("SetDamageData {{\n\tdamage: {damage}\n\tadd: {add}\n}}");
			format!("Function::SetDamage({data_struct}),\n")
		}
		"EnchantWithLevels" => {
			let levels = turn_json_into_number_provider(input["levels"].clone());

			let mut options = String::new();
			let raw_options: Vec<String> =
				input["options"].as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
			if raw_options.is_empty() {
				options += "\tvec![]"
			} else {
				options += "\tvec![\n";
				for function in raw_options {
					for line in function.lines() {
						options += format!("\t\t{line},\n").as_str();
					}
				}
				options += "\t]";
			}

			let include_additional_cost_component = if input["include_additional_cost_component"].is_boolean() {
				format!("Some({})", input["add"].as_bool().unwrap())
			} else {
				"None".to_string()
			};

			let data_struct = format!(
				"EnchantWithLevelsData {{\n\tlevels: {levels}\n\toptions: {options},\n\tinclude_additional_cost_component: {include_additional_cost_component},\n}}"
			);
			format!("Function::EnchantWithLevels({data_struct}),\n")
		}
		"EnchantRandomly" => {
			let mut options = String::new();
			let raw_options: Vec<String> =
				input["options"].as_array().unwrap_or(&Vec::new()).iter().map(|x| x.as_str().unwrap().to_string()).collect();
			if raw_options.is_empty() {
				options += "vec![]"
			} else {
				options += "vec![\n";
				for function in raw_options {
					for line in function.lines() {
						options += format!("\t\t\"{line}\",\n").as_str();
					}
				}
				options += "\t]";
			}

			let only_compatible =
				if input["only_compatible"].is_boolean() { format!("Some({})", input["add"].as_bool().unwrap()) } else { "None".to_string() };

			let include_additional_cost_component = if input["include_additional_cost_component"].is_boolean() {
				format!("Some({})", input["add"].as_bool().unwrap())
			} else {
				"None".to_string()
			};

			let data_struct = format!(
				"EnchantRandomlyData {{\n\toptions: {options},\n\tonly_compatible: {only_compatible},\n\tinclude_additional_cost_component: {include_additional_cost_component}\n}}"
			);
			format!("Function::EnchantRandomly({data_struct}),\n")
		}
		"ExplorationMap" => {
			let destination =
				if input["destination"].is_string() { format!("Some({})", input["destination"].as_str().unwrap()) } else { "None".to_string() };
			let decoration =
				if input["decoration"].is_string() { format!("Some(\"{}\")", input["decoration"].as_str().unwrap()) } else { "None".to_string() };
			let zoom = if input["zoom"].is_number() { format!("Some({})", input["zoom"].as_number().unwrap()) } else { "None".to_string() };
			let search_radius = if input["search_radius"].is_number() {
				format!("Some({})", input["search_radius"].as_number().unwrap())
			} else {
				"None".to_string()
			};
			let skip_existing_chunks = if input["skip_existing_chunks"].is_boolean() {
				format!("Some({})", input["skip_existing_chunks"].as_bool().unwrap())
			} else {
				"None".to_string()
			};

			let data_struct = format!(
				"ExplorationMapData {{\n\tdestination: {destination},\n\tdecoration: {decoration},\n\tzoom: {zoom},\n\tsearch_radius: {search_radius},\n\tskip_existing_chunks: {skip_existing_chunks}\n}}"
			);
			format!("Function::ExplorationMap({data_struct}),\n")
		}
		"SetName" => {
			//TODO: finish implementation
			let data_struct = "SetNameData {\n\tname: NbtTag::default(),\n\tentity: None,\n\ttarget: None\n}".to_string();
			format!("Function::SetName({data_struct}),\n")
		}
		"SetOminousBottleAmplifier" => {
			format!("Function::SetOminousBottleAmplifier({}),\n", turn_json_into_number_provider(input["amplifier"].clone()))
		}
		"SetEnchantments" => {
			//TODO: finish implementation
			let data_struct = "SetEnchantmentsData {\n\tenchantments: HashMap::new(),\n\tadd: None\n}".to_string();
			format!("Function::SetEnchantments({data_struct}),\n")
		}
		"SetComponents" => {
			//TODO: finish implementation
			"Function::SetComponents(vec![]),\n".to_string()
		}
		"EnchantedCountIncrease" => {
			let count = turn_json_into_number_provider(input["count"].clone());
			let limit = if input["limit"].is_number() { format!("Some({})", input["limit"].as_number().unwrap()) } else { "None".to_string() };
			let enchantment = input["enchantment"].as_str().unwrap().to_string();
			let data_struct = format!("EnchantCountIncreaseData {{\n\tcount: {count}\n\tlimit: {limit},\n\tenchantment:\"{enchantment}\"\n}}");
			format!("Function::EnchantedCountIncrease({data_struct}),\n")
		}
		"FurnaceSmelt" => "Function::FurnaceSmelt,\n".to_string(),
		"LimitCount" => {
			let min = if input["limit"].is_number() {
				input["limit"].as_number().unwrap().to_string()
			} else {
				if input["limit"].has_key("min") {
					format!("Some({})", turn_json_into_number_provider(input["limit"]["min"].clone()))
				} else {
					"None".to_string()
				}
			};
			let max = if input["limit"].is_number() {
				input["limit"].as_number().unwrap().to_string()
			} else {
				if input["limit"].has_key("max") {
					format!("Some({})", turn_json_into_number_provider(input["limit"]["max"].clone()))
				} else {
					"None".to_string()
				}
			};

			let data_struct = format!("LimitCountData {{\n\tmin: {min},\n\tmax: {max}\n}}");
			format!("Function::LimitCount({data_struct}),\n")
		}
		x => panic!("unknown function_type {x}"),
	};

	let mut conditions = String::new();
	let raw_conditions: Vec<String> =
		input["conditions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
	if raw_conditions.is_empty() {
		conditions += "\tconditions: vec![],\n"
	} else {
		conditions += "\tconditions: vec![\n";
		for condition in raw_conditions {
			for line in condition.lines() {
				conditions += format!("\t\t{line}\n").as_str();
			}
			conditions.pop();
			conditions += ",\n";
		}
		conditions += "\t],\n";
	}
	return format!("ItemModifier {{\n\tfunction: {function}\n\t{conditions}\n}},");
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
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone(), block_name)).collect();
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

	let entries: Vec<String> =
		input["entries"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_entry(x.clone(), block_name)).collect();
	if entries.is_empty() {
		output += "\tentries: vec![],\n"
	} else {
		output += "\tentries: vec![\n";
		for entry in entries {
			for line in entry.lines() {
				output += format!("\t\t{line}\n").as_str();
			}
		}
		output += "\t],\n";
	}

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
		"Binomial" => {
			let n = turn_json_into_number_provider(input["n"].clone());
			let p = turn_json_into_number_provider(input["p"].clone());

			return format!("NumberProvider::Binomial(Box::new({n}), Box::new({p})),");
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
		"TableBonus" => {
			let enchantment = input["enchantment"].as_str().unwrap();
			let mut chances = String::new();
			let raw_chances: Vec<String> =
				input["chances"].as_array().unwrap_or(&Vec::new()).iter().map(|x| f64::from(x.as_number().unwrap()).to_string()).collect();
			if raw_chances.is_empty() {
				chances += "\tvec![],\n"
			} else {
				chances += "\tvec![\n";
				for mut chance in raw_chances {
					if !chance.contains(".") {
						chance += ".0";
					}
					chances += format!("\t\t{chance},\n").as_str();
				}
				chances += "\t]";
			}

			return format!("Predicate::TableBonus(PredicateTableBonus {{\nenchantment: \"{enchantment}\",\nchances: {chances},\n}})");
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

pub fn turn_json_into_entry(input: JsonValue, block_name: &str) -> String {
	return if input.has_key("children") {
		turn_json_into_entry_composite(input, block_name)
	} else if input.has_key("expand") {
		turn_json_into_entry_tag(input, block_name)
	} else {
		turn_json_into_entry_singleton(input, block_name)
	};
}

pub fn turn_json_into_entry_singleton(input: JsonValue, block_name: &str) -> String {
	let entry_type = match input["type"].as_str().unwrap() {
		"minecraft:item" => format!("LootTablePoolEntrySingletonType::Item(\"{}\")", input["name"].as_str().unwrap()),
		"minecraft:loot_table" => {
			if input["value"].is_string() {
				format!("LootTablePoolEntrySingletonType::LootTableId(\"{}\")", input["value"].as_str().unwrap())
			} else {
				format!("LootTablePoolEntrySingletonType::LootTableCustom({})", turn_json_into_loot_table(input["value"].clone(), block_name))
			}
		}
		"minecraft:dynamic" => format!("LootTablePoolEntrySingletonType::Dynamic(\"{}\")", input["name"].as_str().unwrap()),
		_ => "LootTablePoolEntrySingletonType::Empty".to_string(),
	};

	let mut conditions = String::new();
	let raw_conditions: Vec<String> =
		input["conditions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
	if raw_conditions.is_empty() {
		conditions += "vec![]"
	} else {
		conditions += "vec![\n";
		for condition in raw_conditions {
			for line in condition.lines() {
				conditions += format!("\t\t{line}\n").as_str();
			}
			conditions.pop();
			conditions += ",\n";
		}
		conditions += "]";
	}
	let mut functions = String::new();
	let raw_functions: Vec<String> =
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone(), block_name)).collect();
	if raw_functions.is_empty() {
		functions += "vec![]"
	} else {
		functions += "vec![\n";
		for function in raw_functions {
			for line in function.lines() {
				functions += format!("\t\t{line}\n").as_str();
			}
		}
		functions += "]";
	}

	let weight = if input["weight"].is_number() { format!("Some({})", input["weight"].as_number().unwrap()) } else { "None".to_string() };
	let quality = if input["quality"].is_number() { format!("Some({})", input["quality"].as_number().unwrap()) } else { "None".to_string() };

	return format!(
		"LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {{
entry_type: {entry_type},
conditions: {conditions},
functions: {functions},
weight: {weight},
quality: {quality},
}}),\n"
	);
}

pub fn turn_json_into_entry_tag(input: JsonValue, block_name: &str) -> String {
	let name = input["name"].as_str().unwrap().to_string();
	let expand = input["expand"].as_bool().unwrap().to_string();

	let mut conditions = String::new();
	let raw_conditions: Vec<String> =
		input["conditions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
	if raw_conditions.is_empty() {
		conditions += "vec![]"
	} else {
		conditions += "vec![\n";
		for condition in raw_conditions {
			for line in condition.lines() {
				conditions += format!("\t\t{line}\n").as_str();
			}
			conditions.pop();
			conditions += ",\n";
		}
		conditions += "]";
	}
	let mut functions = String::new();
	let raw_functions: Vec<String> =
		input["functions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_function(x.clone(), block_name)).collect();
	if raw_functions.is_empty() {
		functions += "vec![]"
	} else {
		functions += "vec![\n";
		for function in raw_functions {
			for line in function.lines() {
				functions += format!("\t\t{line}\n").as_str();
			}
		}
		functions += "]";
	}

	let weight = if input["weight"].is_number() { format!("Some({})", input["weight"].as_number().unwrap()) } else { "None".to_string() };
	let quality = if input["quality"].is_number() { format!("Some({})", input["quality"].as_number().unwrap()) } else { "None".to_string() };

	return format!(
		"LootTablePoolEntry::Tag(LootTablePoolEntryTag {{
	name: \"{name}\",
	expand: {expand},
	conditions: {conditions},
	functions: {functions},
	weight: {weight},
	quality: {quality},
}}),\n"
	);
}

pub fn turn_json_into_entry_composite(input: JsonValue, block_name: &str) -> String {
	let entry_type = match input["type"].as_str().unwrap() {
		"minecraft:group" => "LootTablePoolEntryCompositeType::Group",
		"minecraft:alternatives" => "LootTablePoolEntryCompositeType::Alternatives",
		"minecraft:sequence" => "LootTablePoolEntryCompositeType::Sequence",
		x => panic!("no idea what the LootTablePoolEntrySingletonType {x} is supposed to be"),
	};

	let mut conditions = String::new();
	let raw_conditions: Vec<String> =
		input["conditions"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_predicate(x.clone(), block_name)).collect();
	if raw_conditions.is_empty() {
		conditions += "vec![]"
	} else {
		conditions += "vec![\n";
		for condition in raw_conditions {
			for line in condition.lines() {
				conditions += format!("\t\t{line}\n").as_str();
			}
			conditions.pop();
			conditions += ",\n";
		}
		conditions += "]";
	}

	let mut children = String::new();
	let raw_children: Vec<String> =
		input["children"].as_array().unwrap_or(&Vec::new()).iter().map(|x| turn_json_into_entry(x.clone(), block_name)).collect();
	if raw_children.is_empty() {
		children += "vec![]"
	} else {
		children += "vec![\n";
		for condition in raw_children {
			for line in condition.lines() {
				children += format!("\t\t{line}\n").as_str();
			}
			children.pop();
			children += "\n";
		}
		children += "]";
	}

	return format!(
		"LootTablePoolEntry::Composite(LootTablePoolEntryComposite {{
	entry_type: {entry_type},
	children: {children},
	conditions: {conditions},
 }}),\n"
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
