use super::*;
pub fn get_banner_pattern() -> HashMap<&'static str, Vec<&'static str>> {
	let mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	output.insert("pattern_item/creeper", vec!["minecraft:creeper"]);
	output.insert("pattern_item/globe", vec!["minecraft:globe"]);
	output.insert("pattern_item/flow", vec!["minecraft:flow"]);
	output.insert("pattern_item/flower", vec!["minecraft:flower"]);
	output.insert("pattern_item/mojang", vec!["minecraft:mojang"]);
	output.insert("pattern_item/field_masoned", vec!["minecraft:bricks"]);
	output.insert("pattern_item/skull", vec!["minecraft:skull"]);
	output.insert("pattern_item/piglin", vec!["minecraft:piglin"]);
	output.insert("pattern_item/bordure_indented", vec!["minecraft:curly_border"]);
	output.insert("no_item_required", vec!["minecraft:square_bottom_left", "minecraft:square_bottom_right", "minecraft:square_top_left", "minecraft:square_top_right", "minecraft:stripe_bottom", "minecraft:stripe_top", "minecraft:stripe_left", "minecraft:stripe_right", "minecraft:stripe_center", "minecraft:stripe_middle", "minecraft:stripe_downright", "minecraft:stripe_downleft", "minecraft:small_stripes", "minecraft:cross", "minecraft:straight_cross", "minecraft:triangle_bottom", "minecraft:triangle_top", "minecraft:triangles_bottom", "minecraft:triangles_top", "minecraft:diagonal_left", "minecraft:diagonal_up_right", "minecraft:diagonal_up_left", "minecraft:diagonal_right", "minecraft:circle", "minecraft:rhombus", "minecraft:half_vertical", "minecraft:half_horizontal", "minecraft:half_vertical_right", "minecraft:half_horizontal_bottom", "minecraft:border", "minecraft:gradient", "minecraft:gradient_up"]);
	output.insert("pattern_item/guster", vec!["minecraft:guster"]);
	return output;
}