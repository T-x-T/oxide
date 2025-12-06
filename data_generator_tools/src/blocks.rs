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

  let mut file = std::fs::OpenOptions::new()
   	.read(true)
   	.write(true)
    .truncate(true)
    .create(true)
    .open(path)
    .unwrap();

  file.write_all(output.as_bytes()).unwrap();
  file.flush().unwrap();
}

fn get_blocks() {
  let mut output = String::new();

  output += "#![allow(clippy::needless_return)]\n";

  let mut cargo_toml_contents = "[package]
name = \"block_get_blocks\"
version = \"0.4.0\"
edition = \"2024\"
description = \"\"

[dependencies]
block_types = {path = \"../types\", version = \"*\"}\n".to_string();

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
      block["properties"].as_object().unwrap().iter().flat_map(|x| x.1.as_array().unwrap().iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(x.0), block_type, convert_to_upper_camel_case(x.0), if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap())) } else { convert_to_upper_camel_case(y.as_str().unwrap()) } ))).collect()
    } else {
      String::new()
    };

    output += format!("\tadd_{}(&mut output);\n", convert_to_upper_camel_case(key).to_lowercase()).as_str();
  }
  output += "\treturn output;\n";
  output += "}\n";

  let path = std::path::PathBuf::from("../data/blocks/get_blocks/src/lib.rs");

  let mut file = std::fs::OpenOptions::new()
   	.read(true)
   	.write(true)
    .truncate(true)
    .create(true)
    .open(path)
    .unwrap();

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
      block["properties"].as_object().unwrap().iter().flat_map(|x| x.1.as_array().unwrap().iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(x.0), block_type, convert_to_upper_camel_case(x.0), if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap())) } else { convert_to_upper_camel_case(y.as_str().unwrap()) } ))).collect()
    } else {
      String::new()
    };
    outputs[output_index] += format!("pub fn add_{}(map: &mut HashMap<String, Block>) {{\n", convert_to_upper_camel_case(key).to_lowercase()).as_str();
    outputs[output_index] += format!("\tlet mut block = Block {{ block_type: Type::{block_type}, properties: vec![{properties}], states: vec![] }};\n").as_str();
    for x in block["states"].as_array().unwrap().iter() {
      outputs[output_index] += format!("\tblock.states.push(State {{ id: {}, properties: vec![ {}], default: {} }});\n", x.as_object().unwrap()["id"].as_i32().unwrap(), x.as_object().unwrap()["properties"].as_object().unwrap_or(jzon::object! {}.as_object().unwrap()).iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(y.0), block_type, convert_to_upper_camel_case(y.0), if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.1.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.1.as_str().unwrap())) } else { convert_to_upper_camel_case(y.1.as_str().unwrap()) } )).collect::<String>(), if x.as_object().unwrap()["default"].is_boolean() { "true" } else { "false" } ).as_str();
    }
    outputs[output_index] += format!("\tmap.insert(\"{key}\".to_string(), block);\n").as_str();
    outputs[output_index] += "}\n";

    i += 1;
  }

  std::fs::remove_dir_all(std::path::PathBuf::from("../data/blocks/add_functions"));
  #[allow(clippy::needless_range_loop)]
  for i in 0..outputs.len() {
    std::fs::create_dir_all(std::path::PathBuf::from(format!("../data/blocks/add_functions/add_fn_{i}/src")));

    let cargo_toml_contents = format!("[package]
name = \"blocks_add_fn_{i}\"
version = \"0.4.0\"
edition = \"2024\"
description = \"\"

[dependencies]
block_types = {{path = \"../../types\", version = \"*\"}}");
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
  for x in blocks_file.lines(){
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

  let mut file = std::fs::OpenOptions::new()
   	.read(true)
   	.write(true)
    .truncate(true)
    .create(true)
    .open(path)
    .unwrap();

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
      let property_entry = format!("{}{}", convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), convert_to_upper_camel_case(property.0));
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
  for x in blocks_file.lines(){
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
      let property_entry = format!("{}{}", convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), convert_to_upper_camel_case(property.0));
      properties.entry((convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), property.0.to_string())).or_insert(property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
    }
  }

	output += "pub fn get_block_state_id_from_raw(block_states: &HashMap<String, Block>, block_name: &str, properties: &[(String, String)]) -> u16 {\n";
	output += "\tlet block = block_states.get(block_name).unwrap();\n";
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
		      output += format!("\t\t\t.filter(|x| match properties.iter().find(|y| y.0.as_str() == \"{}\").unwrap().1.as_str() {{\n", prop.1).as_str();
					for prop_val in properties.get(prop).unwrap() {
						let property_name = format!("{block_type}{}", convert_to_upper_camel_case(&prop.1));
						let property_value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(prop_val) {format!("Num{}", convert_to_upper_camel_case(prop_val))} else {convert_to_upper_camel_case(prop_val)};
						output += format!("\t\t\t\t\"{prop_val}\" => x.properties.contains(&Property::{property_name}({property_name}::{property_value})),\n").as_str();
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
  for x in blocks_file.lines(){
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
      let property_entry = format!("{}{}", convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), convert_to_upper_camel_case(property.0));
      properties.entry((convert_to_upper_camel_case(block.1["definition"]["type"].as_str().unwrap()), property.0.to_string())).or_insert(property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
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
      let variant = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&option) {format!("Num{}", convert_to_upper_camel_case(&option))} else {convert_to_upper_camel_case(&option)};
      output += format!("\t\t\tProperty::{enum_variant}({enum_variant}::{variant}) => output.push((\"{}\".to_string(), \"{option}\".to_string())),\n", property.0.1).as_str();
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
      output += format!("\t\t{} => Type::{},\n", state["id"], convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap())).as_str();
    }
  }
  output += "\t\t_ => panic!(\"block_state_id {} doesnt exist\", block_state_id)\n";

  output += "\t};\n";
  output += "}\n";

  let path = std::path::PathBuf::from("../data/blocks/get_type_from_block_state_id/src/lib.rs");

  let mut file = std::fs::OpenOptions::new()
   	.read(true)
   	.write(true)
    .truncate(true)
    .create(true)
    .open(path)
    .unwrap();

  file.write_all(output.as_bytes()).unwrap();
  file.flush().unwrap();
}

fn get_block_from_block_state_id() -> String {
  return "pub fn get_block_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> Block {
\treturn block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().1.clone();
}\n".to_string();
}


fn get_block_state_from_block_state_id() -> String {
  return "pub fn get_block_state_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> State {
\treturn block_states.iter()
\t\t.filter(|x| x.1.states.iter().any(|y| y.id == block_state_id))
\t\t.map(|x| x.1.states.iter().find(|y| y.id == block_state_id).unwrap())
\t\t.collect::<Vec<&State>>().first_mut().unwrap().clone();
}\n".to_string();
}

fn get_block_name_from_block_state_id() -> String {
  return "pub fn get_block_name_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> String {
\treturn block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().0.clone();
}\n ".to_string();
}

fn get_block_from_name() -> String {
  return "pub fn get_block_from_name(name: &str, block_states: &HashMap<String, Block>) -> Block {
\tlet air = block_states.get(\"minecraft:air\").unwrap();
\tlet block = block_states.get(name).unwrap_or(air);
\treturn block.clone();
}\n".to_string();
}

fn impl_type() -> String {
  return
  "impl Type {
\t#[allow(clippy::match_like_matches_macro)]
\tpub fn has_right_click_behavior(&self) -> bool {
\t\treturn match self {
\t\t\tType::Anvil => true,
\t\t\tType::Barrel => true,
\t\t\tType::Beacon => true,
\t\t\tType::Bed => true,
\t\t\tType::Beehive => true,
\t\t\tType::Bell => true,
\t\t\tType::BlastFurnace => true,
\t\t\tType::BrewingStand => true,
\t\t\tType::Button => true,
\t\t\tType::Cake => true,
\t\t\tType::CalibratedSculkSensor => true,
\t\t\tType::Campfire => true,
\t\t\tType::Candle => true,
\t\t\tType::CandleCake => true,
\t\t\tType::Carrot => true,
\t\t\tType::CartographyTable => true,
\t\t\tType::Cauldron => true,
\t\t\tType::CeilingHangingSign => true,
\t\t\tType::Chest => true,
\t\t\tType::Comparator => true,
\t\t\tType::Composter => true,
\t\t\tType::Crafter => true,
\t\t\tType::CraftingTable => true,
\t\t\tType::Dispenser => true,
\t\t\tType::Door => true,
\t\t\tType::DragonEgg => true,
\t\t\tType::Dropper => true,
\t\t\tType::EnchantmentTable => true,
\t\t\tType::EndGateway => true,
\t\t\tType::EndPortal => true,
\t\t\tType::EndPortalFrame => true,
\t\t\tType::EnderChest => true,
\t\t\tType::FenceGate => true,
\t\t\tType::FlowerPot => true,
\t\t\tType::Furnace => true,
\t\t\tType::Grindstone => true,
\t\t\tType::Hopper => true,
\t\t\tType::Jigsaw => true,
\t\t\tType::Jukebox => true,
\t\t\tType::LavaCauldron => true,
\t\t\tType::LayeredCauldron => true,
\t\t\tType::Lever => true,
\t\t\tType::Loom => true,
\t\t\tType::NetherPortal => true,
\t\t\tType::Repeater => true,
\t\t\tType::SmithingTable => true,
\t\t\tType::Smoker => true,
\t\t\tType::StandingSign => true,
\t\t\tType::Stonecutter => true,
\t\t\tType::Trapdoor => true,
\t\t\tType::TrappedChest => true,
\t\t\tType::WallSign => true,
\t\t\tType::Lectern => true,
\t\t\tType::ShulkerBox => true,
\t\t\t_ => false,
\t\t}
\t}
\t
\t#[allow(clippy::match_like_matches_macro)]
\tpub fn is_solid(&self) -> bool {
\t\treturn match self {
\t\t\tType::Air => false,
\t\t\tType::SugarCane => false,
\t\t\tType::Liquid => false,
\t\t\tType::BubbleColumn => false,
\t\t\tType::KelpPlant => false,
\t\t\tType::CoralPlant => false,
\t\t\tType::DoublePlant => false,
\t\t\tType::BaseCoralPlant => false,
\t\t\tType::CaveVinesPlant => false,
\t\t\tType::WeepingVines => false,
\t\t\tType::WeepingVinesPlant => false,
\t\t\tType::TwistingVinesPlant => false,
\t\t\tType::Sapling => false,
\t\t\tType::BambooSapling => false,
\t\t\tType::Mushroom => false,
\t\t\tType::TallGrass => false,
\t\t\tType::TallDryGrass => false,
\t\t\tType::ShortDryGrass => false,
\t\t\tType::DryVegetation => false,
\t\t\tType::Fire => false,
\t\t\tType::SoulFire => false,
\t\t\tType::WallBanner => false,
\t\t\tType::WallSign => false,
\t\t\tType::StandingSign => false,
\t\t\tType::Torch => false,
\t\t\tType::TorchflowerCrop => false,
\t\t\tType::WallTorch => false,
\t\t\tType::RedstoneTorch => false,
\t\t\tType::RedstoneWallTorch => false,
\t\t\tType::PressurePlate => false,
\t\t\tType::WeightedPressurePlate => false,
\t\t\tType::Light => false,
\t\t\tType::Lever => false,
\t\t\t_ => true,
\t\t}
\t}
}\n".to_string();
}

fn structs() -> String {
  return
  "#[derive(Debug, Clone)]
pub struct Block {
\tpub block_type: Type,
\tpub states: Vec<State>,
\tpub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct State {
\tpub id: u16,
\tpub default: bool,
\tpub properties: Vec<Property>,
}\n".to_string();
}
