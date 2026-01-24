use super::*;
pub fn get_fluid() -> HashMap<&'static str, Vec<&'static str>> {
	let mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	output.insert("lava", vec!["minecraft:lava", "minecraft:flowing_lava"]);
	output.insert("water", vec!["minecraft:water", "minecraft:flowing_water"]);
	return output;
}