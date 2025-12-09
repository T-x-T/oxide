use super::*;

pub fn generate() {
	let mut output = String::new();

	let registries_file =
		std::fs::read_to_string("../official_server/generated/reports/registries.json").expect("failed to read registries.json report");
	let registries_json = jzon::parse(&registries_file).expect("failed to parse registries.json report");
	let registry = registries_json.as_object().unwrap()["minecraft:entity_type"]["entries"].as_object().unwrap();

	output += "#![allow(clippy::needless_return)]\n";
	output += "pub fn get_id_from_name(name: &str) -> i32 {\n";
	output += "\treturn match name {\n";

	for entry in registry.iter() {
		output += format!("\t\t\"{}\" => {},\n", entry.0, entry.1.as_object().unwrap()["protocol_id"].as_i32().unwrap()).as_str();
	}

	output += "\t\t_ => panic!(\"get_id_from_name encountered entity with name {name} that doesnt exist\"),\n";
	output += "\t};\n";
	output += "}\n";
	output += "\n";
	output += "pub fn get_name_from_id(id: i32) -> String {\n";
	output += "\treturn match id {\n";

	for entry in registry.iter() {
		output += format!("\t\t{} => \"{}\".to_string(),\n", entry.1.as_object().unwrap()["protocol_id"].as_i32().unwrap(), entry.0).as_str();
	}

	output += "\t\t_ => panic!(\"get_name_from_id encountered entity with id {id} that doesnt exist\"),\n";
	output += "\t};\n";
	output += "}\n";

	let path = std::path::PathBuf::from("../data/entities/src/lib.rs");

	let mut file = std::fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();

	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}
