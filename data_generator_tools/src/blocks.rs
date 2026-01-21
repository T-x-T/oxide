use super::*;

static ADD_FNS_COUNT: usize = 8;

pub fn generate() {
	block_types();
	get_blocks();
	get_type_from_block_state_id();
	get_blocks_add_functions();

	let mut output = String::new();

	output += "#![allow(unused_mut)]\n#![allow(clippy::needless_return)]\nuse std::collections::HashMap;\n";
	output += "pub use block_types::*;\n";
	output += "pub use block_get_blocks::*;\n";
	output += "pub use block_get_type_from_block_state_id::*;\n";
	output += get_block_from_block_state_id().as_str();
	output += get_block_state_from_block_state_id().as_str();
	output += get_block_name_from_block_state_id().as_str();
	output += get_block_from_name().as_str();
	output += get_raw_properties_from_block_state_id().as_str();
	output += get_block_state_id_from_raw().as_str();

	let path = std::path::PathBuf::from("../data/blocks/main/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn get_blocks() {
	let mut output = String::new();

	output += "#![allow(clippy::needless_return)]\n";

	let mut cargo_toml_contents = "[package]
name = \"block_get_blocks\"
version = \"0.5.0\"
edition = \"2024\"
description = \"\"

[dependencies]
block_types = {path = \"../types\", version = \"*\"}\n"
		.to_string();

	for i in 0..ADD_FNS_COUNT {
		cargo_toml_contents += format!("blocks_add_fn_{i} = {{path = \"../add_functions/add_fn_{i}\", version = \"*\"}}\n").as_str();
		output += format!("use blocks_add_fn_{i}::*;\n").as_str();
	}
	let mut file = std::fs::OpenOptions::new()
		.read(true)
		.write(true)
		.truncate(true)
		.create(true)
		.open(std::path::PathBuf::from("../data/blocks/get_blocks/Cargo.toml"))
		.unwrap();
	file.write_all(cargo_toml_contents.as_bytes()).unwrap();
	file.flush().unwrap();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	output += "use std::collections::HashMap;\n";
	output += "use block_types::*;\n";
	output += "pub fn get_blocks() -> HashMap<String, Block> {\n";
	output += "\tlet mut output: HashMap<String, Block> = HashMap::new();\n";

	for x in blocks_json.as_object().unwrap().iter() {
		let block = x.1.as_object().unwrap();
		let key = x.0;
		let block_type = convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap());
		let properties: String = if block["properties"].is_object() {
			block["properties"]
				.as_object()
				.unwrap()
				.iter()
				.flat_map(|x| {
					x.1.as_array().unwrap().iter().map(|y| {
						format!(
							"Property::{}{}({}{}::{}),",
							block_type,
							convert_to_upper_camel_case(x.0),
							block_type,
							convert_to_upper_camel_case(x.0),
							if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) {
								format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap()))
							} else {
								convert_to_upper_camel_case(y.as_str().unwrap())
							}
						)
					})
				})
				.collect()
		} else {
			String::new()
		};

		output += format!("\tadd_{}(&mut output);\n", convert_to_upper_camel_case(key).to_lowercase()).as_str();
	}
	output += "\treturn output;\n";
	output += "}\n";

	let path = std::path::PathBuf::from("../data/blocks/get_blocks/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn get_blocks_add_functions() {
	let mut outputs: Vec<String> = Vec::new();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	let mut i = 0;
	#[allow(clippy::explicit_counter_loop)]
	for x in blocks_json.as_object().unwrap().iter() {
		let output_index = i % ADD_FNS_COUNT;
		if outputs.len() <= output_index {
			outputs.push("use block_types::*;\nuse std::collections::HashMap;\n".to_string());
		}

		let block = x.1.as_object().unwrap();
		let key = x.0;
		let block_type = convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap());
		let properties: String = if block["properties"].is_object() {
			block["properties"]
				.as_object()
				.unwrap()
				.iter()
				.flat_map(|x| {
					x.1.as_array().unwrap().iter().map(|y| {
						format!(
							"Property::{}{}({}{}::{}),",
							block_type,
							convert_to_upper_camel_case(x.0),
							block_type,
							convert_to_upper_camel_case(x.0),
							if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) {
								format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap()))
							} else {
								convert_to_upper_camel_case(y.as_str().unwrap())
							}
						)
					})
				})
				.collect()
		} else {
			String::new()
		};
		let default_state = block["states"].as_array().unwrap().iter().position(|x| x.as_object().unwrap()["default"].is_boolean()).unwrap();
		outputs[output_index] +=
			format!("pub fn add_{}(map: &mut HashMap<String, Block>) {{\n", convert_to_upper_camel_case(key).to_lowercase()).as_str();
		outputs[output_index] +=
			format!("\tlet mut block = Block {{ block_type: Type::{block_type}, properties: vec![{properties}], states: vec![], default_state: {default_state} }};\n").as_str();
		for x in block["states"].as_array().unwrap().iter() {
			outputs[output_index] += format!(
				"\tblock.states.push(State {{ id: {}, properties: vec![ {}] }});\n",
				x.as_object().unwrap()["id"].as_i32().unwrap(),
				x.as_object().unwrap()["properties"]
					.as_object()
					.unwrap_or(jzon::object! {}.as_object().unwrap())
					.iter()
					.map(|y| format!(
						"Property::{}{}({}{}::{}),",
						block_type,
						convert_to_upper_camel_case(y.0),
						block_type,
						convert_to_upper_camel_case(y.0),
						if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.1.as_str().unwrap().to_string()) {
							format!("Num{}", convert_to_upper_camel_case(y.1.as_str().unwrap()))
						} else {
							convert_to_upper_camel_case(y.1.as_str().unwrap())
						}
					))
					.collect::<String>()
			)
			.as_str();
		}
		outputs[output_index] += format!("\tmap.insert(\"{key}\".to_string(), block);\n").as_str();
		outputs[output_index] += "}\n";

		i += 1;
	}

	std::fs::remove_dir_all(std::path::PathBuf::from("../data/blocks/add_functions"));
	#[allow(clippy::needless_range_loop)]
	for i in 0..outputs.len() {
		std::fs::create_dir_all(std::path::PathBuf::from(format!("../data/blocks/add_functions/add_fn_{i}/src")));

		let cargo_toml_contents = format!(
			"[package]
name = \"blocks_add_fn_{i}\"
version = \"0.5.0\"
edition = \"2024\"
description = \"\"

[dependencies]
block_types = {{path = \"../../types\", version = \"*\"}}"
		);
		let mut file = std::fs::OpenOptions::new()
			.read(true)
			.write(true)
			.truncate(true)
			.create(true)
			.open(std::path::PathBuf::from(format!("../data/blocks/add_functions/add_fn_{i}/Cargo.toml")))
			.unwrap();
		file.write_all(cargo_toml_contents.as_bytes()).unwrap();
		file.flush().unwrap();

		let mut file = std::fs::OpenOptions::new()
			.read(true)
			.write(true)
			.truncate(true)
			.create(true)
			.open(std::path::PathBuf::from(format!("../data/blocks/add_functions/add_fn_{i}/src/lib.rs")))
			.unwrap();
		file.write_all(outputs[i].as_bytes()).unwrap();
		file.flush().unwrap();
	}
}

fn block_types() {
	let mut output = String::new();
	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");

	output += "#![allow(clippy::needless_return)]\n";
	output += structs().as_str();
	output += impl_type().as_str();
	output += property_enums().as_str();
	output += "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n";
	output += "pub enum Type {\n";

	let mut block_types: Vec<String> = Vec::new();
	for x in blocks_file.lines() {
		if x.trim().starts_with("\"type\":") {
			block_types.push(convert_to_upper_camel_case(&x.trim().replace("\"type\": \"", "").replace("\",", "")));
		}
	}
	block_types.sort();
	block_types.dedup();
	block_types.into_iter().filter(|x| !x.contains("[")).for_each(|x| output += format!("\t{x},\n").as_str());

	output += "}\n";

	output = output.replace("\"", "");

	let path = std::path::PathBuf::from("../data/blocks/types/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn property_enums() -> String {
	let mut output = String::new();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	let mut properties: HashMap<String, Vec<String>> = HashMap::new();
	for block in blocks_json.as_object().unwrap().iter() {
		if !block.1["properties"].is_object() {
			continue;
		}
		for property in block.1["properties"].as_object().unwrap().iter() {
			let property_entry = format!(
				"{}{}",
				convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()),
				convert_to_upper_camel_case(property.0)
			);
			properties.entry(property_entry).or_insert(property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
		}
	}

	for property in properties.clone() {
		output += format!("#[derive(Debug, Clone, PartialEq, Eq)]\npub enum {} {{ ", property.0).as_str();
		for variant in property.1 {
			let mut variant = convert_to_upper_camel_case(&variant);
			if (u8::MIN..u8::MAX).map(|x| x.to_string()).collect::<Vec<String>>().contains(&variant) {
				variant = format!("Num{variant}")
			}
			output += format!("\t{variant}, ").as_str();
		}
		output += "}\n";
	}

	output += "#[derive(Debug, Clone, PartialEq, Eq)]\npub enum Property {\n";
	properties.into_iter().for_each(|x| output += format!("\t{}({}),\n", x.0, x.0).as_str());
	output += "}\n";

	return output;
}

fn get_block_state_id_from_raw() -> String {
	let mut output = String::new();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	let mut block_types: Vec<String> = Vec::new();
	for x in blocks_file.lines() {
		if x.trim().starts_with("\"type\":") {
			block_types.push(convert_to_upper_camel_case(&x.trim().replace("\"type\": \"", "").replace("\",", "")));
		}
	}
	block_types.sort();
	block_types.dedup();
	let block_types: Vec<String> = block_types.into_iter().filter(|x| x != "\"type\": [").collect();

	//The key is the type and then the property, because properties can have different values depending on their type
	let mut properties: HashMap<(String, String), Vec<String>> = HashMap::new();
	for block in blocks_json.as_object().unwrap().iter() {
		if !block.1["properties"].is_object() {
			continue;
		}
		for property in block.1["properties"].as_object().unwrap().iter() {
			let property_entry = format!(
				"{}{}",
				convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()),
				convert_to_upper_camel_case(property.0)
			);
			properties
				.entry((convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), property.0.to_string()))
				.or_insert(property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
		}
	}

	output += "pub fn get_block_state_id_from_raw(block_states: &HashMap<String, Block>, block_name: &str, properties: &[(String, String)]) -> u16 {\n";
	output += "\tlet Some(block) = block_states.get(block_name) else {
\t\tprintln!(\"get_block_state_id_from_raw couldnt find block {block_name} with properties {properties:?}\");
\t\treturn 0;
	};\n";
	output += "\treturn match block.block_type {\n";

	for block_type in block_types {
		output += format!("\t\tType::{} => {{\n", block_type.replace("\"", "")).as_str();
		if properties.iter().filter(|x| x.0.0 == block_type).count() == 0 {
			output += "\t\t\tblock.states.first().unwrap().id\n";
			output += "\t\t},\n";
		} else {
			output += "\t\t\tlet block_states: Vec<&State> = block.states.iter()\n";
			for prop in properties.keys() {
				if prop.0 == block_type {
					output +=
						format!("\t\t\t.filter(|x| match properties.iter().find(|y| y.0.as_str() == \"{}\").unwrap().1.as_str() {{\n", prop.1).as_str();
					for prop_val in properties.get(prop).unwrap() {
						let property_name = format!("{block_type}{}", convert_to_upper_camel_case(&prop.1));
						let property_value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(prop_val) {
							format!("Num{}", convert_to_upper_camel_case(prop_val))
						} else {
							convert_to_upper_camel_case(prop_val)
						};
						output +=
							format!("\t\t\t\t\"{prop_val}\" => x.properties.contains(&Property::{property_name}({property_name}::{property_value})),\n")
								.as_str();
					}
					output += "\t\t\t\t_ => false,\n";
					output += "\t\t\t\t})\n";
				}
			}
			output += "\t\t\t\t.collect();\n";
			output += "\t\t\tlet block_state_id = block_states.first().unwrap().id;\n";
			output += "\t\t\tassert_eq!(block_states.len(), 1);\n";
			output += "\t\t\tblock_state_id\n";
			output += "\t\t},\n";
		}
	}

	output += "\t};\n";
	output += "}\n";

	return output;
}

fn get_raw_properties_from_block_state_id() -> String {
	let mut output = String::new();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	let mut block_types: Vec<String> = Vec::new();
	for x in blocks_file.lines() {
		if x.trim().starts_with("\"type\":") {
			block_types.push(convert_to_upper_camel_case(&x.trim().replace("\"type\": \"", "").replace("\",", "")));
		}
	}
	block_types.sort();
	block_types.dedup();
	let block_types: Vec<String> = block_types.into_iter().filter(|x| x != "\"type\": [").collect();

	//The key is the type and then the property, because properties can have different values depending on their type
	let mut properties: HashMap<(String, String), Vec<String>> = HashMap::new();
	for block in blocks_json.as_object().unwrap().iter() {
		if !block.1["properties"].is_object() {
			continue;
		}
		for property in block.1["properties"].as_object().unwrap().iter() {
			let property_entry = format!(
				"{}{}",
				convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()),
				convert_to_upper_camel_case(property.0)
			);
			properties
				.entry((convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), property.0.to_string()))
				.or_insert(property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
		}
	}


	output += "pub fn get_raw_properties_from_block_state_id(block_states: &HashMap<String, Block>, block_state_id: u16) -> Vec<(String, String)> {\n";
	output += "\tlet state = block_states.iter().find(|x| x.1.states.iter().any(|x| x.id == block_state_id)).unwrap().1.states.iter().find(|x| x.id == block_state_id).unwrap().clone();\n";
	output += "\tlet mut output: Vec<(String, String)> = Vec::new();\n\n";
	output += "\tfor property in state.properties {\n";
	output += "\t\tmatch property {\n";
	for property in properties {
		let enum_variant = convert_to_upper_camel_case(&format!("{}{}", property.0.0, convert_to_upper_camel_case(&property.0.1)));
		for option in property.1 {
			let variant = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&option) {
				format!("Num{}", convert_to_upper_camel_case(&option))
			} else {
				convert_to_upper_camel_case(&option)
			};
			output += format!(
				"\t\t\tProperty::{enum_variant}({enum_variant}::{variant}) => output.push((\"{}\".to_string(), \"{option}\".to_string())),\n",
				property.0.1
			)
			.as_str();
		}
	}
	output += "\t\t}\n";
	output += "\t}\n";
	output += "\treturn output;\n";
	output += "}\n";

	return output;
}

fn get_type_from_block_state_id() {
	let mut output = String::new();

	let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
	let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

	output += "#![allow(clippy::needless_return)]\n";
	output += "use block_types::*;\n";
	output += "pub fn get_type_from_block_state_id(block_state_id: u16) -> Type {\n";
	output += "\treturn match block_state_id {\n";

	for x in blocks_json.as_object().unwrap().iter() {
		let block = x.1.as_object().unwrap();
		for state in block["states"].as_array().unwrap() {
			output +=
				format!("\t\t{} => Type::{},\n", state["id"], convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap())).as_str();
		}
	}
	output += "\t\t_ => panic!(\"block_state_id {} doesnt exist\", block_state_id)\n";

	output += "\t};\n";
	output += "}\n";

	let path = std::path::PathBuf::from("../data/blocks/get_type_from_block_state_id/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn get_block_from_block_state_id() -> String {
	return "pub fn get_block_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> Block {
\treturn block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().1.clone();
}\n"
		.to_string();
}


fn get_block_state_from_block_state_id() -> String {
	return "pub fn get_block_state_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> State {
\treturn block_states.iter()
\t\t.filter(|x| x.1.states.iter().any(|y| y.id == block_state_id))
\t\t.map(|x| x.1.states.iter().find(|y| y.id == block_state_id).unwrap())
\t\t.collect::<Vec<&State>>().first_mut().unwrap().clone();
}\n"
		.to_string();
}

fn get_block_name_from_block_state_id() -> String {
	return "pub fn get_block_name_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> String {
\treturn block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().0.clone();
}\n "
		.to_string();
}

fn get_block_from_name() -> String {
	return "pub fn get_block_from_name(name: &str, block_states: &HashMap<String, Block>) -> Block {
\tlet air = block_states.get(\"minecraft:air\").unwrap();
\tlet block = block_states.get(name).unwrap_or(air);
\treturn block.clone();
}\n"
		.to_string();
}

fn impl_type() -> String {
	return r#"impl Type {
	#[allow(clippy::match_like_matches_macro)]
	pub fn has_right_click_behavior(&self) -> bool {
		return match self {
			Type::Anvil => true,
			Type::Barrel => true,
			Type::Beacon => true,
			Type::Bed => true,
			Type::Beehive => true,
			Type::Bell => true,
			Type::BlastFurnace => true,
			Type::BrewingStand => true,
			Type::Button => true,
			Type::Cake => true,
			Type::CalibratedSculkSensor => true,
			Type::Campfire => true,
			Type::Candle => true,
			Type::CandleCake => true,
			Type::Carrot => true,
			Type::CartographyTable => true,
			Type::Cauldron => true,
			Type::CeilingHangingSign => true,
			Type::Chest => true,
			Type::Comparator => true,
			Type::Composter => true,
			Type::Crafter => true,
			Type::CraftingTable => true,
			Type::Dispenser => true,
			Type::Door => true,
			Type::DragonEgg => true,
			Type::Dropper => true,
			Type::EnchantmentTable => true,
			Type::EndGateway => true,
			Type::EndPortal => true,
			Type::EndPortalFrame => true,
			Type::EnderChest => true,
			Type::FenceGate => true,
			Type::FlowerPot => true,
			Type::Furnace => true,
			Type::Grindstone => true,
			Type::Hopper => true,
			Type::Jigsaw => true,
			Type::Jukebox => true,
			Type::LavaCauldron => true,
			Type::LayeredCauldron => true,
			Type::Lever => true,
			Type::Loom => true,
			Type::NetherPortal => true,
			Type::Repeater => true,
			Type::SmithingTable => true,
			Type::Smoker => true,
			Type::StandingSign => true,
			Type::Stonecutter => true,
			Type::Trapdoor => true,
			Type::TrappedChest => true,
			Type::WallSign => true,
			Type::Lectern => true,
			Type::ShulkerBox => true,
			_ => false,
		}
	}

	#[allow(clippy::match_like_matches_macro)]
	pub fn is_solid(&self) -> bool {
		return match self {
			Type::Air => false,
			Type::SugarCane => false,
			Type::Liquid => false,
			Type::BubbleColumn => false,
			Type::KelpPlant => false,
			Type::CoralPlant => false,
			Type::DoublePlant => false,
			Type::BaseCoralPlant => false,
			Type::CaveVinesPlant => false,
			Type::WeepingVines => false,
			Type::WeepingVinesPlant => false,
			Type::TwistingVinesPlant => false,
			Type::Sapling => false,
			Type::BambooSapling => false,
			Type::Mushroom => false,
			Type::TallGrass => false,
			Type::TallDryGrass => false,
			Type::ShortDryGrass => false,
			Type::DryVegetation => false,
			Type::Fire => false,
			Type::SoulFire => false,
			Type::WallBanner => false,
			Type::WallSign => false,
			Type::StandingSign => false,
			Type::Torch => false,
			Type::TorchflowerCrop => false,
			Type::WallTorch => false,
			Type::RedstoneTorch => false,
			Type::RedstoneWallTorch => false,
			Type::PressurePlate => false,
			Type::WeightedPressurePlate => false,
			Type::Light => false,
			Type::Lever => false,
			_ => true,
		}
	}
}
"#
	.to_string();
}

fn structs() -> String {
	return r#"#[derive(Debug, Clone)]
pub struct Block {
	pub block_type: Type,
	pub states: Vec<State>,
	pub default_state: usize,
	pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct State {
	pub id: u16,
	pub properties: Vec<Property>,
}
"#
	.to_string();
}
