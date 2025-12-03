#![allow(
  clippy::needless_return,
  clippy::format_collect,
  unused
)]

use std::collections::HashMap;
use std::io::prelude::*;

mod blockentity;
mod entities;
mod items;

fn main() {
  blockentity::generate();
  entities::generate();
  items::generate();
  //do_blocks();
  //do_block_types();
  //do_properties();
  //do_get_block_state_id_from_raw();
  //do_get_raw_properties_from_block_state_id();
  //do_blockentity_types();
  //get_type_from_block_state_id();
}

fn do_blocks() {
  let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
  let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

  println!("pub fn get_blocks() -> HashMap<String, Block> {{");
  println!("\tlet mut output: HashMap<String, Block> = HashMap::new();");

  for x in blocks_json.as_object().unwrap().iter() {
    let block = x.1.as_object().unwrap();
    let key = x.0;
    let block_type = convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap());
    let properties: String = if block["properties"].is_object() {
      block["properties"].as_object().unwrap().iter().flat_map(|x| x.1.as_array().unwrap().iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(x.0), block_type, convert_to_upper_camel_case(x.0), if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap())) } else { convert_to_upper_camel_case(y.as_str().unwrap()) } ))).collect()
    } else {
      String::new()
    };

    println!("\tfn add_{}(map: &mut HashMap<String, Block>) {{", convert_to_upper_camel_case(key).to_lowercase());
    println!("\t\tlet mut block = Block {{ block_type: Type::{block_type}, properties: vec![{properties}], states: vec![] }};");
    for x in block["states"].as_array().unwrap().iter() {
      println!("\t\tblock.states.push(State {{ id: {}, properties: vec![ {}], default: {} }});", x.as_object().unwrap()["id"].as_i32().unwrap(), x.as_object().unwrap()["properties"].as_object().unwrap_or(jzon::object! {}.as_object().unwrap()).iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(y.0), block_type, convert_to_upper_camel_case(y.0), if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.1.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.1.as_str().unwrap())) } else { convert_to_upper_camel_case(y.1.as_str().unwrap()) } )).collect::<String>(), if x.as_object().unwrap()["default"].is_boolean() { "true" } else { "false" } )
    }
    println!("\t\tmap.insert(\"{key}\".to_string(), block);");
    println!("\t}}");
    println!("\tadd_{}(&mut output);", convert_to_upper_camel_case(key).to_lowercase());

  }
  println!("\treturn output;");
  println!("}}");
}

fn do_block_types() {
  let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");

  let mut block_types: Vec<String> = Vec::new();
  for x in blocks_file.lines(){
    if x.trim().starts_with("\"type\":") {
      block_types.push(convert_to_upper_camel_case(&x.trim().replace("\"type\": \"", "").replace("\",", "")));
    }
  }
  block_types.sort();
  block_types.dedup();
  block_types.into_iter().filter(|x| x != "Type: [").for_each(|x| println!("{x},"));
}

fn do_properties() {
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
    println!("#[derive(Debug, Clone, PartialEq, Eq)]\npub enum {} {{", property.0);
    for variant in property.1 {
      let mut variant = convert_to_upper_camel_case(&variant);
      if (u8::MIN..u8::MAX).map(|x| x.to_string()).collect::<Vec<String>>().contains(&variant) {
        variant = format!("Num{variant}")
      }
      println!("\t{variant},");
    }
    println!("}}");
  }

  println!("#[derive(Debug, Clone, PartialEq, Eq)]\npub enum Property {{");
  properties.into_iter().for_each(|x| println!("\t{}({}),", x.0, x.0));
  println!("}}");
}

fn do_get_block_state_id_from_raw() {
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

	println!("pub fn get_block_state_id_from_raw(block_states: &HashMap<String, Block>, block_name: &str, properties: &[(String, String)]) -> u16 {{");
	println!("\tlet block = block_states.get(block_name).unwrap();");
	println!("\treturn match block.block_type {{");

	for block_type in block_types {
		println!("\t\tType::{} => {{", block_type.replace("\"", ""));
		if properties.iter().filter(|x| x.0.0 == block_type).count() == 0 {
			println!("\t\t\tblock.states.first().unwrap().id");
			println!("\t\t}},");
		} else {
		  println!("\t\t\tlet block_states: Vec<&State> = block.states.iter()");
			for prop in properties.keys() {
				if prop.0 == block_type {
		      println!("\t\t\t.filter(|x| match properties.iter().find(|y| y.0.as_str() == \"{}\").unwrap().1.as_str() {{", prop.1);
					for prop_val in properties.get(prop).unwrap() {
						let property_name = format!("{block_type}{}", convert_to_upper_camel_case(&prop.1));
						let property_value = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(prop_val) {format!("Num{}", convert_to_upper_camel_case(prop_val))} else {convert_to_upper_camel_case(prop_val)};
						println!("\t\t\t\t\"{prop_val}\" => x.properties.contains(&Property::{property_name}({property_name}::{property_value})),");
					}
					println!("\t\t\t\t_ => false,");

					println!("\t\t\t\t}})");
				}
			}
			println!("\t\t\t\t.collect();");
			println!("\t\t\tlet block_state_id = block_states.first().unwrap().id;");
			println!("\t\t\tassert_eq!(block_states.len(), 1);");
			println!("\t\t\tblock_state_id");
			println!("\t\t}},");
		}
	}

	println!("\t}};");
	println!("}}");
}

fn do_get_raw_properties_from_block_state_id() {
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


  println!("pub fn get_raw_properties_from_block_state_id(block_states: &HashMap<String, Block>, block_state_id: u16) -> Vec<(String, String)> {{");
  println!("\tlet state = block_states.iter().find(|x| x.1.states.iter().any(|x| x.id == block_state_id)).unwrap().1.states.iter().find(|x| x.id == block_state_id).unwrap().clone();");
  println!("\tlet mut output: Vec<(String, String)> = Vec::new();\n");
  println!("\tfor property in state.properties {{");
  println!("\t\tmatch property {{");
  for property in properties {
    let enum_variant = convert_to_upper_camel_case(&format!("{}{}", property.0.0, convert_to_upper_camel_case(&property.0.1)));
    for option in property.1 {
      let variant = if (u8::MIN..u8::MAX).map(|z| z.to_string()).collect::<Vec<String>>().contains(&option) {format!("Num{}", convert_to_upper_camel_case(&option))} else {convert_to_upper_camel_case(&option)};
      println!("\t\t\tProperty::{enum_variant}({enum_variant}::{variant}) => output.push((\"{}\".to_string(), \"{option}\".to_string())),", property.0.1);
    }
  }
  println!("\t\t}}");
  println!("\t}}");
  println!("\treturn output;");
  println!("}}");
}

fn get_type_from_block_state_id() {
  let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
  let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");
  println!("pub fn get_type_from_block_state_id(block_state_id: u16) -> Type {{");
  println!("\treturn match block_state_id {{");

  for x in blocks_json.as_object().unwrap().iter() {
    let block = x.1.as_object().unwrap();
    for state in block["states"].as_array().unwrap() {
      println!("\t\t{} => Type::{},", state["id"], convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap()));
    }
  }
  println!("\t\t_ => panic!(\"block_state_id {{}} doesnt exist\", block_state_id)");

  println!("\t}};");
  println!("}}");
}


fn convert_to_upper_camel_case(input: &str) -> String {
  let mut found_underscore = false;
  return input
    .replace("minecraft:", "")
    .chars()
    .enumerate()
    .map(|i| {
      if i.1 == '_' {
        found_underscore = true;
        return i.1;
      }
      if i.0 == 0 || found_underscore {
        found_underscore = false;
        return i.1.to_ascii_uppercase();
      } else {
        return i.1;
      }
    }
  )
  .filter(|i| *i != '_')
  .collect();
}
