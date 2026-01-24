use super::*;
pub fn get_instrument() -> HashMap<&'static str, Vec<&'static str>> {
	let mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	output.insert("goat_horns", vec!["minecraft:admire_goat_horn", "minecraft:call_goat_horn", "minecraft:dream_goat_horn", "minecraft:feel_goat_horn", "minecraft:ponder_goat_horn", "minecraft:seek_goat_horn", "minecraft:sing_goat_horn", "minecraft:yearn_goat_horn"]);
	output.insert("screaming_goat_horns", vec!["minecraft:admire_goat_horn", "minecraft:call_goat_horn", "minecraft:dream_goat_horn", "minecraft:yearn_goat_horn"]);
	output.insert("regular_goat_horns", vec!["minecraft:feel_goat_horn", "minecraft:ponder_goat_horn", "minecraft:seek_goat_horn", "minecraft:sing_goat_horn"]);
	return output;
}