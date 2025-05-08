fn main() {
  //do_items();
  //do_blocks();
  do_block_types();
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
    //let properties: String = block["properties"].as_array().unwrap().into_iter().map(|i| convert_to_upper_camel_case(i.as_str().unwrap())).collect();
    let id = block["states"].as_array().unwrap().into_iter().find(|x| x.as_object().unwrap()["default"].as_bool().unwrap_or_default()).unwrap().as_object().unwrap()["id"].as_i32().unwrap();

    // ("minecraft:oak_log", Block { block_type: Type::Block, properties: vec![Property::X, Property::Y, Property::Z], states: vec![State { id: 136, default: false, properties: vec![Property::X] }, State { id: 137, default: true, properties: vec![Property::Y] }, State { id: 138, default: false, properties: vec![Property::Z] }] }),
    println!("(\"{key}\", Block {{ block_type: Type::{block_type}, properties: vec![], states: vec![State {{ id: {id}, default: true, properties: vec![] }}] }}),");
  }
}

fn do_block_types() {
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
  block_types.into_iter().filter(|x| x != "Type: [").for_each(|x| println!("{x},"));
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
