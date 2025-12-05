use super::*;

pub fn generate() {
  let mut output = String::new();

  let registries_file = std::fs::read_to_string("../official_server/generated/reports/registries.json").expect("failed to read registries.json report");
  let registries_json = jzon::parse(&registries_file).expect("failed to parse registries.json report");
  let registry = registries_json.as_object().unwrap()["minecraft:block_entity_type"]["entries"].as_object().unwrap();

  output += "#![allow(clippy::needless_return)]\n";
  output += "pub fn get_block_entity_types() -> std::collections::HashMap<String, u8> {{\n";
  output += "\tlet mut output: std::collections::HashMap<String, u8> = std::collections::HashMap::new();\n\n";

  for entry in registry.iter() {
    output += format!("\toutput.insert(\"{}\".to_string(), {});\n", entry.0, entry.1.as_object().unwrap()["protocol_id"].as_i32().unwrap()).as_str();
  }

  output += "\n\treturn output;\n";
  output += "}}";

  let path = std::path::PathBuf::from("../data/blockentity/src/lib.rs");

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
