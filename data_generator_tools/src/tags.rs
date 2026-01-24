use std::fs::{DirEntry, ReadDir};
use std::io;
use std::str::FromStr;

use super::*;

pub fn generate() {
	let mut output = "#![allow(clippy::needless_return)]\nuse std::collections::HashMap;\n".to_string();

	for dir in fs::read_dir("../official_server/generated/data/minecraft/tags").expect("couldnt open tags directory") {
		let name = dir.unwrap().file_name().into_string().unwrap();
		do_dir(name.clone());
		output += format!("mod {name};\n").as_str();
		output += format!("pub use {name}::*;\n").as_str();
	}

	let path = PathBuf::from("../data/tags/src/lib.rs");
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(output.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn do_dir(name: String) {
	let mut output: HashMap<String, Vec<String>> = HashMap::new();

	let mut path = PathBuf::from_str("../official_server/generated/data/minecraft/tags").unwrap();
	path.push(name.clone());

	for file in read_dir_recursively(path).unwrap() {
		let file_content = fs::read_to_string(file.path()).unwrap();
		let file_json = jzon::parse(&file_content).unwrap();
		let values = file_json["values"].as_array().unwrap();
		output.insert(
			file
				.path()
				.to_str()
				.unwrap()
				.to_string()
				.replace(format!("../official_server/generated/data/minecraft/tags/{name}/").as_str(), "")
				.replace(".json", ""),
			values.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
		);
	}

	//Now we just resolve all sub tags recursively (prepended by a #)
	while sub_tags_left(&output) {
		let output_clone = output.clone();
		for entry in output.values_mut() {
			for element in entry.clone() {
				if element.starts_with("#") {
					let mut output_clone_clone = output_clone.clone();
					let mut default: Vec<String> = Vec::new();
					let mut values_to_append = output_clone_clone.get_mut(&element.replace("#minecraft:", "")).unwrap_or(&mut default);
					entry.append(values_to_append);
				}
			}
			entry.retain(|x| !x.starts_with("#"));
			entry.sort();
			entry.dedup();
		}
	}

	//create output file contents
	let mut file_content = "use super::*;\n".to_string();
	file_content += format!("pub fn get_{name}() -> HashMap<&'static str, Vec<&'static str>> {{\n").as_str();
	file_content += "\tlet mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();\n";
	for (k, v) in output {
		file_content += format!("\toutput.insert(\"{k}\", vec!{v:?});\n").as_str();
	}
	file_content += "\treturn output;\n";
	file_content += "}";

	//write output file
	let path = PathBuf::from(format!("../data/tags/src/{name}.rs"));
	let mut file = fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).unwrap();
	file.write_all(file_content.as_bytes()).unwrap();
	file.flush().unwrap();
}

fn sub_tags_left(input: &HashMap<String, Vec<String>>) -> bool {
	for entry in input.values() {
		for element in entry {
			if element.starts_with("#") {
				return true;
			}
		}
	}

	return false;
}

fn read_dir_recursively(path: PathBuf) -> io::Result<Vec<DirEntry>> {
	let mut output: Vec<DirEntry> = Vec::new();

	for entry in fs::read_dir(path).unwrap() {
		let entry = entry.unwrap();
		if entry.metadata().unwrap().is_file() {
			output.push(entry);
		} else {
			output.append(&mut read_dir_recursively(entry.path()).unwrap());
		}
	}

	return Ok(output);
}
