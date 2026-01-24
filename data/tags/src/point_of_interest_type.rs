use super::*;
pub fn get_point_of_interest_type() -> HashMap<&'static str, Vec<&'static str>> {
	let mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	output.insert("acquirable_job_site", vec!["minecraft:armorer", "minecraft:butcher", "minecraft:cartographer", "minecraft:cleric", "minecraft:farmer", "minecraft:fisherman", "minecraft:fletcher", "minecraft:leatherworker", "minecraft:librarian", "minecraft:mason", "minecraft:shepherd", "minecraft:toolsmith", "minecraft:weaponsmith"]);
	output.insert("village", vec!["minecraft:armorer", "minecraft:butcher", "minecraft:cartographer", "minecraft:cleric", "minecraft:farmer", "minecraft:fisherman", "minecraft:fletcher", "minecraft:home", "minecraft:leatherworker", "minecraft:librarian", "minecraft:mason", "minecraft:meeting", "minecraft:shepherd", "minecraft:toolsmith", "minecraft:weaponsmith"]);
	output.insert("bee_home", vec!["minecraft:bee_nest", "minecraft:beehive"]);
	return output;
}