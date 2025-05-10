use std::{collections::HashMap, fmt::format};

fn main() {
  //do_items();
  do_blocks();
  //do_block_types();
  //do_properties();
}

fn do_items() {
  let items_file = std::fs::read_to_string("../official_server/generated/reports/items.json").expect("failed to read items.json report");
  let items_json = jzon::parse(&items_file).expect("failed to parse items.json report");
  let registries_file = std::fs::read_to_string("../official_server/generated/reports/registries.json").expect("failed to read registries.json report");
  let registries_json = jzon::parse(&registries_file).expect("failed to parse registries.json report");
  let items_registry = registries_json.as_object().unwrap()["minecraft:item"]["entries"].as_object().unwrap();

  for x in items_json.as_object().unwrap().iter() {
    let components = x.1.as_object().unwrap()["components"].clone();

    let key = x.0;
    let max_stack_size = components["minecraft:max_stack_size"].as_i32().unwrap();
    let rarity: String = components["minecraft:rarity"].as_str().unwrap().chars().enumerate().map(|i| if i.0 == 0 {i.1.to_ascii_uppercase()} else {i.1}).collect();
    let repair_cost = components["minecraft:repair_cost"].as_i32().unwrap();
    let id = items_registry[key]["protocol_id"].as_i32().unwrap();

    //("minecraft:stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1 }),
    println!("(\"{key}\", Item {{ max_stack_size: {max_stack_size}, rarity: ItemRarity::{rarity}, repair_cost: {repair_cost}, id: {id} }}),");
  }
}

fn do_blocks() {
  let blocks_file = std::fs::read_to_string("../official_server/generated/reports/blocks.json").expect("failed to read blocks.json report");
  let blocks_json = jzon::parse(&blocks_file).expect("failed to parse blocks.json report");

  for x in blocks_json.as_object().unwrap().iter() {
    let block = x.1.as_object().unwrap();
    let key = x.0;
    let block_type = convert_to_upper_camel_case(block["definition"]["type"].as_str().unwrap());
    let properties: String = if block["properties"].is_object() {
      block["properties"].as_object().unwrap().iter().map(|x| x.1.as_array().unwrap().into_iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(x.0), block_type, convert_to_upper_camel_case(x.0), if (u8::MIN..u8::MAX).into_iter().map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.as_str().unwrap())) } else { convert_to_upper_camel_case(y.as_str().unwrap()) } ))).flatten().collect()
    } else {
      String::new()
    };
    let states: String = block["states"].as_array().unwrap().into_iter().map(|x| format!("State {{ id: {}, properties: vec![ {}], default: {} }},", x.as_object().unwrap()["id"].as_i32().unwrap(), x.as_object().unwrap()["properties"].as_object().unwrap_or(jzon::object! {}.as_object().unwrap()).iter().map(|y| format!("Property::{}{}({}{}::{}),", block_type, convert_to_upper_camel_case(y.0), block_type, convert_to_upper_camel_case(y.0), if (u8::MIN..u8::MAX).into_iter().map(|z| z.to_string()).collect::<Vec<String>>().contains(&y.1.as_str().unwrap().to_string()) { format!("Num{}", convert_to_upper_camel_case(y.1.as_str().unwrap())) } else { convert_to_upper_camel_case(y.1.as_str().unwrap()) } )).collect::<String>(), if x.as_object().unwrap()["default"].is_boolean() { "true" } else { "false" } )).collect();

    println!("(\"{key}\", Block {{ block_type: Type::{block_type}, properties: vec![{properties}], states: vec![{states}] }}),");
  }
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
      if !properties.contains_key(&property_entry) {
        properties.insert(property_entry, property.1.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect());
      }
    }
  }

  for property in properties.clone() {
    println!("#[derive(Debug, Clone, PartialEq, Eq)]\npub enum {} {{", property.0);
    for variant in property.1 {
      let mut variant = convert_to_upper_camel_case(&variant);
      if (u8::MIN..u8::MAX).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().contains(&variant) {
        variant = format!("Num{variant}")
      }
      println!("\t{},", variant);
    }
    println!("}}");
  }

  println!("#[derive(Debug, Clone, PartialEq, Eq)]\npub enum Property {{");
  properties.into_iter().for_each(|x| println!("\t{}({}),", x.0, x.0));
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
