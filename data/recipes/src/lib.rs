#![allow(clippy::needless_return)]
use basic_types::recipe::*;
use std::collections::HashMap;
pub fn get_recipes() -> HashMap<&'static str, Recipe> {
	let mut output: HashMap<&'static str, Recipe> = HashMap::new();
	output.insert("minecraft:dropper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"# #",
	"#R#",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:dropper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:stone"]),
].into_iter().collect(),
result_id: "minecraft:stone_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
	vec!["minecraft:bamboo"],
],
result_id: "minecraft:bamboo_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_porkchop", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:porkchop",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_porkchop",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:polished_deepslate_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:polished_deepslate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_pink_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:pink_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:pink_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dune_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:sandstone"]),
	("S", vec!["minecraft:dune_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:dune_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:diorite_stairs_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:diorite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_grate_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_grate"),
ingredients: vec![
	vec!["minecraft:weathered_copper_grate"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_grate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:spruce_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cracked_deepslate_bricks", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:deepslate_bricks",
],
cooking_time: Some(200),
result_id: "minecraft:cracked_deepslate_bricks",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:gray_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:gray_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_blue_orchid", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:blue_orchid"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_slab_from_end_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone",
],
result_id: "minecraft:end_stone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:red_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:red_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:gold_ingot_from_blasting_gold_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:gold_ore",
],
cooking_time: Some(100),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:cobbled_deepslate_stairs_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:cobbled_deepslate_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobbled_deepslate_wall_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:cobbled_deepslate_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_red_sandstone_slab_from_smooth_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_red_sandstone",
],
result_id: "minecraft:smooth_red_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:brown_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:brown_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:brown_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:pink_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:pink_dye"]),
].into_iter().collect(),
result_id: "minecraft:pink_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:sentry_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:sentry_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:smooth_basalt", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:basalt",
],
cooking_time: Some(200),
result_id: "minecraft:smooth_basalt",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:book_cloning", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Bookcloning,
category: Some(Category::Misc),
}));
	output.insert("minecraft:dye_green_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:green_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:furnace_minecart", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:furnace"],
	vec!["minecraft:minecart"],
],
result_id: "minecraft:furnace_minecart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_leggings", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_leggings",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:dark_oak_logs"],
],
result_id: "minecraft:dark_oak_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_stairs_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_tile_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bucket", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	" # ",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:bucket",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:blue_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:blue_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:lime_dye_from_smelting", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:sea_pickle",
],
cooking_time: Some(200),
result_id: "minecraft:lime_dye",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:cobbled_deepslate_slab_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:cobbled_deepslate_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:beehive", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"PPP",
	"HHH",
	"PPP",
],
key: vec![
	("H", vec!["minecraft:honeycomb"]),
	("P", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:beehive",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:campfire", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" S ",
	"SCS",
	"LLL",
],
key: vec![
	("C", vec!["#minecraft:coals"]),
	("L", vec!["#minecraft:logs"]),
	("S", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:campfire",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_lime_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:lime_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:lime_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:white_dye"]),
].into_iter().collect(),
result_id: "minecraft:white_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:chiseled_red_sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:red_sandstone_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_red_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:white_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:white_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:light_gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:powered_rail", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X#X",
	"XRX",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("R", vec!["minecraft:redstone"]),
	("X", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:powered_rail",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper"),
ingredients: vec![
	vec!["minecraft:oxidized_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:oak_log"]),
].into_iter().collect(),
result_id: "minecraft:oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:red_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:red_dye"]),
].into_iter().collect(),
result_id: "minecraft:red_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:lapis_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:lapis_lazuli"]),
].into_iter().collect(),
result_id: "minecraft:lapis_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blackstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:blackstone"]),
].into_iter().collect(),
result_id: "minecraft:blackstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:honey_bottle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:honey_block"],
	vec!["minecraft:glass_bottle"],
	vec!["minecraft:glass_bottle"],
	vec!["minecraft:glass_bottle"],
	vec!["minecraft:glass_bottle"],
],
result_id: "minecraft:honey_bottle",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:blue_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:blue_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:brown_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:brown_dye"]),
].into_iter().collect(),
result_id: "minecraft:brown_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:cobblestone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:cobblestone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cauldron", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:cauldron",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cut_copper_stairs_from_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cut_copper",
],
result_id: "minecraft:cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oak_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:blue_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_smelting_deepslate_gold_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:deepslate_gold_ore",
],
cooking_time: Some(200),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:red_sandstone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:red_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:red_sandstone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_red_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:red_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:stone_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:stone_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:packed_mud", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:mud"],
	vec!["minecraft:wheat"],
],
result_id: "minecraft:packed_mud",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:glass_bottle", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	" # ",
],
key: vec![
	("#", vec!["minecraft:glass"]),
].into_iter().collect(),
result_id: "minecraft:glass_bottle",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:pale_oak_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:gold_nugget", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:gold_ingot"],
],
result_id: "minecraft:gold_nugget",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_chiseled_copper_from_waxed_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_cut_copper",
],
result_id: "minecraft:waxed_weathered_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_stairs_from_tuff_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff_bricks",
],
result_id: "minecraft:tuff_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:spruce_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:gray_dye_from_closed_eyeblossom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("gray_dye"),
ingredients: vec![
	vec!["minecraft:closed_eyeblossom"],
],
result_id: "minecraft:gray_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_slab_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:spruce_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:purple_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:purple_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:purple_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_dye_from_oxeye_daisy", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_gray_dye"),
ingredients: vec![
	vec!["minecraft:oxeye_daisy"],
],
result_id: "minecraft:light_gray_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:leather_chestplate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"XXX",
],
key: vec![
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:leather_chestplate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_light_blue_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:light_blue_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:light_blue_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_carrot", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:gold_nugget"]),
	("X", vec!["minecraft:carrot"]),
].into_iter().collect(),
result_id: "minecraft:golden_carrot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coast_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:coast_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:smooth_quartz_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_quartz"]),
].into_iter().collect(),
result_id: "minecraft:smooth_quartz_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:oxidized_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:deepslate_bricks_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:magenta_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:magenta_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:deepslate_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_bricks"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:fletching_table", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"@@",
	"##",
	"##",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("@", vec!["minecraft:flint"]),
].into_iter().collect(),
result_id: "minecraft:fletching_table",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:glow_item_frame", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:item_frame"],
	vec!["minecraft:glow_ink_sac"],
],
result_id: "minecraft:glow_item_frame",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_dye_from_wildflowers", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("yellow_dye"),
ingredients: vec![
	vec!["minecraft:wildflowers"],
],
result_id: "minecraft:yellow_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_dark_oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_dark_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_dark_oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:white_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:white_dye",
],
result_id: "minecraft:white_shulker_box",
})));
	output.insert("minecraft:waxed_oxidized_copper_grate_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_grate"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_grate"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_grate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:decorated_pot", Recipe::CraftingDecoratedPot(Some(Category::Misc)));
	output.insert("minecraft:stone_bricks_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_stairs_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:bricks"]),
].into_iter().collect(),
result_id: "minecraft:brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:redstone_lamp", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" R ",
	"RGR",
	" R ",
],
key: vec![
	("G", vec!["minecraft:glowstone"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:redstone_lamp",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:brown_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:brown_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:light_gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_stairs_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:blue_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:blue_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crimson_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:clock", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
	" # ",
],
key: vec![
	("#", vec!["minecraft:gold_ingot"]),
	("X", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:clock",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:light_blue_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purpur_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"FF",
	"FF",
],
key: vec![
	("F", vec!["minecraft:popped_chorus_fruit"]),
].into_iter().collect(),
result_id: "minecraft:purpur_block",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cut_red_sandstone_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:cut_red_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sandstone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:sandstone"]),
].into_iter().collect(),
result_id: "minecraft:sandstone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:warped_hyphae", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:warped_stem"]),
].into_iter().collect(),
result_id: "minecraft:warped_hyphae",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_bulb_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_bulb"),
ingredients: vec![
	vec!["minecraft:copper_bulb"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_bulb",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper"),
ingredients: vec![
	vec!["minecraft:oxidized_cut_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_cut_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diorite_slab_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:diorite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_green_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:green_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_bricks_from_moss_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("mossy_stone_bricks"),
ingredients: vec![
	vec!["minecraft:stone_bricks"],
	vec!["minecraft:moss_block"],
],
result_id: "minecraft:mossy_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mangrove_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_mangrove_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cooked_chicken", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:chicken",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_chicken",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_exposed_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper"),
ingredients: vec![
	vec!["minecraft:exposed_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_stairs_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_tile_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wayfinder_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:terracotta"]),
	("S", vec!["minecraft:wayfinder_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:wayfinder_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_exposed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:baked_potato_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:potato",
],
cooking_time: Some(100),
result_id: "minecraft:baked_potato",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:lapis_lazuli", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:lapis_block"],
],
result_id: "minecraft:lapis_lazuli",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:sugar_from_honey_bottle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("sugar"),
ingredients: vec![
	vec!["minecraft:honey_bottle"],
],
result_id: "minecraft:sugar",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_stairs_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:waxed_copper_block"]),
].into_iter().collect(),
result_id: "minecraft:waxed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:observer", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"RRQ",
	"###",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
	("Q", vec!["minecraft:quartz"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:observer",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:pink_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:pink_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:baked_potato", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:potato",
],
cooking_time: Some(200),
result_id: "minecraft:baked_potato",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:dune_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:dune_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:waxed_oxidized_cut_copper_slab_from_waxed_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_cut_copper",
],
result_id: "minecraft:waxed_oxidized_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_from_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_copper",
],
result_id: "minecraft:oxidized_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:smooth_sandstone", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:sandstone",
],
cooking_time: Some(200),
result_id: "minecraft:smooth_sandstone",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:pale_oak_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:pale_oak_logs"],
],
result_id: "minecraft:pale_oak_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_chiseled_copper_from_waxed_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_copper",
],
result_id: "minecraft:waxed_oxidized_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dark_oak_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:redstone_from_smelting_redstone_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: Some("redstone"),
ingredient: vec![
	"minecraft:redstone_ore",
],
cooking_time: Some(200),
result_id: "minecraft:redstone",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:iron_ingot_from_blasting_iron_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:iron_ore",
],
cooking_time: Some(100),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:purple_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:red_dye"],
],
result_id: "minecraft:purple_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_golem_statue_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_golem_statue"),
ingredients: vec![
	vec!["minecraft:exposed_copper_golem_statue"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_golem_statue",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_stairs_from_waxed_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_copper",
],
result_id: "minecraft:waxed_weathered_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:candle", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"S",
	"H",
],
key: vec![
	("H", vec!["minecraft:honeycomb"]),
	("S", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lantern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X#X",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:torch"]),
	("X", vec!["minecraft:iron_nugget"]),
].into_iter().collect(),
result_id: "minecraft:lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_granite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_granite"]),
].into_iter().collect(),
result_id: "minecraft:polished_granite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_weathered_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_wall_from_mossy_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_cobblestone",
],
result_id: "minecraft:mossy_cobblestone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:ward_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:ward_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:light_weighted_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:light_weighted_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:green_dye"]),
].into_iter().collect(),
result_id: "minecraft:green_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:cracked_nether_bricks", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:nether_bricks",
],
cooking_time: Some(200),
result_id: "minecraft:cracked_nether_bricks",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:mossy_cobblestone_stairs_from_mossy_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_cobblestone",
],
result_id: "minecraft:mossy_cobblestone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cherry_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff_bricks"]),
].into_iter().collect(),
result_id: "minecraft:tuff_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_stairs_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:polished_deepslate_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_chain_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_chain"),
ingredients: vec![
	vec!["minecraft:exposed_copper_chain"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:slime_ball", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:slime_block"],
],
result_id: "minecraft:slime_ball",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_iron_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredients: vec![
	vec!["minecraft:iron_block"],
],
result_id: "minecraft:iron_ingot",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:tuff_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff"]),
].into_iter().collect(),
result_id: "minecraft:tuff_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:birch_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spire_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:spire_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:hopper_minecart", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:hopper"],
	vec!["minecraft:minecart"],
],
result_id: "minecraft:hopper_minecart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:nether_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:end_crystal", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"GGG",
	"GEG",
	"GTG",
],
key: vec![
	("E", vec!["minecraft:ender_eye"]),
	("G", vec!["minecraft:glass"]),
	("T", vec!["minecraft:ghast_tear"]),
].into_iter().collect(),
result_id: "minecraft:end_crystal",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:weathered_copper"]),
].into_iter().collect(),
result_id: "minecraft:weathered_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:acacia_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_acacia_log"]),
].into_iter().collect(),
result_id: "minecraft:acacia_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:oak_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:oak_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:smooth_quartz_slab_from_smooth_quartz_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_quartz",
],
result_id: "minecraft:smooth_quartz_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_slab_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:field_masoned_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:bricks"],
],
result_id: "minecraft:field_masoned_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_cyan_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:cyan_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:cyan_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:yellow_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:yellow_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:diamond_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:diamond_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:diamond_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_slab_from_mossy_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_cobblestone",
],
result_id: "minecraft:mossy_cobblestone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:bolt_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("{k}", vec![
		"minecraft:copper_block",
		"minecraft:waxed_copper_block",
	]),
	("S", vec!["minecraft:bolt_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:bolt_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_stairs_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_cut_copper_stairs"),
ingredients: vec![
	vec!["minecraft:cut_copper_stairs"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_green_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:green_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_chain_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_chain"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_chain"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coarse_dirt", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"DG",
	"GD",
],
key: vec![
	("D", vec!["minecraft:dirt"]),
	("G", vec!["minecraft:gravel"]),
].into_iter().collect(),
result_id: "minecraft:coarse_dirt",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:warped_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_blasting_raw_iron", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:raw_iron",
],
cooking_time: Some(100),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:dye_red_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:red_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_stone_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stone_brick_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_door_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_door"),
ingredients: vec![
	vec!["minecraft:copper_door"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_door",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:andesite", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:diorite"],
	vec!["minecraft:cobblestone"],
],
result_id: "minecraft:andesite",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_brown_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:brown_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:brown_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brush", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"I",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
	("I", vec!["minecraft:stick"]),
	("X", vec!["minecraft:feather"]),
].into_iter().collect(),
result_id: "minecraft:brush",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:brown_dye"]),
].into_iter().collect(),
result_id: "minecraft:brown_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:cyan_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:cyan_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:cyan_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:quartz_from_blasting", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:nether_quartz_ore",
],
cooking_time: Some(100),
result_id: "minecraft:quartz",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:andesite_wall_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:andesite_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:stone_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:stone_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_cut_copper"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:waxed_copper_block"]),
].into_iter().collect(),
result_id: "minecraft:waxed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:brown_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:brown_dye",
],
result_id: "minecraft:brown_shulker_box",
})));
	output.insert("minecraft:birch_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_oxeye_daisy", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:oxeye_daisy"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oxidized_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:oxidized_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:redstone_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:redstone_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:silence_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:silence_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:comparator", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
	"III",
],
key: vec![
	("#", vec!["minecraft:redstone_torch"]),
	("I", vec!["minecraft:stone"]),
	("X", vec!["minecraft:quartz"]),
].into_iter().collect(),
result_id: "minecraft:comparator",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mangrove_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:crimson_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:target", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" R ",
	"RHR",
	" R ",
],
key: vec![
	("H", vec!["minecraft:hay_block"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:target",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_from_waxed_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_copper_block",
],
result_id: "minecraft:waxed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:stripped_warped_hyphae", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_warped_stem"]),
].into_iter().collect(),
result_id: "minecraft:stripped_warped_hyphae",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:purple_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:purple_dye"],
],
result_id: "minecraft:purple_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:magenta_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:magenta_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:minecart", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:minecart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:white_dye",
],
result_id: "minecraft:white_bundle",
})));
	output.insert("minecraft:smooth_sandstone_stairs_from_smooth_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_sandstone",
],
result_id: "minecraft:smooth_sandstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:dark_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:brown_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:brown_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:brown_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:warped_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:warped_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:diamond_chestplate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"XXX",
],
key: vec![
	("X", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:diamond_chestplate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff"]),
].into_iter().collect(),
result_id: "minecraft:tuff_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:fishing_rod", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"  #",
	" #X",
	"# X",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:fishing_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_slab_from_waxed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_cut_copper",
],
result_id: "minecraft:waxed_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dark_oak_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_dark_oak_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mushroom_stew", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:bowl"],
],
result_id: "minecraft:mushroom_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobblestone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:cobblestone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_brown_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:brown_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:brown_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cookie", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#X#",
],
key: vec![
	("#", vec!["minecraft:wheat"]),
	("X", vec!["minecraft:cocoa_beans"]),
].into_iter().collect(),
result_id: "minecraft:cookie",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_bars_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_bars"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_bars"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_bars",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_trapdoor_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_trapdoor"),
ingredients: vec![
	vec!["minecraft:exposed_copper_trapdoor"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sentry_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:cobblestone"]),
	("S", vec!["minecraft:sentry_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:sentry_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:soul_torch", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"S",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("S", vec!["#minecraft:soul_fire_base_blocks"]),
	("{k}", vec![
		"minecraft:coal",
		"minecraft:charcoal",
	]),
].into_iter().collect(),
result_id: "minecraft:soul_torch",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:red_sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:red_sandstone",
		"minecraft:chiseled_red_sandstone",
	]),
].into_iter().collect(),
result_id: "minecraft:red_sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:soul_lantern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X#X",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:soul_torch"]),
	("X", vec!["minecraft:iron_nugget"]),
].into_iter().collect(),
result_id: "minecraft:soul_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_bricks_from_end_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone",
],
result_id: "minecraft:end_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_wall_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_blasting_deepslate_gold_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:deepslate_gold_ore",
],
cooking_time: Some(100),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:firework_star_fade", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::FireworkStarFade,
category: Some(Category::Misc),
}));
	output.insert("minecraft:suspicious_stew_from_lily_of_the_valley", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:lily_of_the_valley"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_blasting_deepslate_copper_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:deepslate_copper_ore",
],
cooking_time: Some(100),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:light_gray_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:light_gray_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_stairs_from_polished_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_diorite",
],
result_id: "minecraft:polished_diorite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_dye_from_torchflower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("orange_dye"),
ingredients: vec![
	vec!["minecraft:torchflower"],
],
result_id: "minecraft:orange_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_grate_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_grate"),
ingredients: vec![
	vec!["minecraft:exposed_copper_grate"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_grate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:granite_wall_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:granite_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_bricks"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:bamboo_mosaic", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:bamboo_slab"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_mosaic",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_purple_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:purple_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:orange_wool"]),
].into_iter().collect(),
result_id: "minecraft:orange_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:warped_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:warped_planks"],
],
result_id: "minecraft:warped_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_open_eyeblossom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:open_eyeblossom"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_tuff", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:tuff_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_tuff",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_stairs_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper_stairs"),
ingredients: vec![
	vec!["minecraft:oxidized_cut_copper_stairs"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sugar_from_sugar_cane", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("sugar"),
ingredients: vec![
	vec!["minecraft:sugar_cane"],
],
result_id: "minecraft:sugar",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_slab_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:tuff_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_lantern_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_lantern"),
ingredients: vec![
	vec!["minecraft:weathered_copper_lantern"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_dye_from_blue_orchid", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_blue_dye"),
ingredients: vec![
	vec!["minecraft:blue_orchid"],
],
result_id: "minecraft:light_blue_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_slab_from_deepslate_tiles_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_tiles",
],
result_id: "minecraft:deepslate_tile_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:light_gray_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:light_gray_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:light_gray_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:red_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:red_dye",
],
result_id: "minecraft:red_shulker_box",
})));
	output.insert("minecraft:shulker_box", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"-",
	"#",
	"-",
],
key: vec![
	("#", vec!["minecraft:chest"]),
	("-", vec!["minecraft:shulker_shell"]),
].into_iter().collect(),
result_id: "minecraft:shulker_box",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:music_disc_5", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
	vec!["minecraft:disc_fragment_5"],
],
result_id: "minecraft:music_disc_5",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:blue_dye",
],
result_id: "minecraft:blue_bundle",
})));
	output.insert("minecraft:melon", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
	vec!["minecraft:melon_slice"],
],
result_id: "minecraft:melon",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_bricks_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_salmon", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:salmon",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_salmon",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:sticky_piston", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"S",
	"P",
],
key: vec![
	("P", vec!["minecraft:piston"]),
	("S", vec!["minecraft:slime_ball"]),
].into_iter().collect(),
result_id: "minecraft:sticky_piston",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:trapped_chest", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: None,
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:tripwire_hook"],
],
result_id: "minecraft:trapped_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:sandstone",
		"minecraft:chiseled_sandstone",
	]),
].into_iter().collect(),
result_id: "minecraft:sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_tuff", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:tuff"]),
].into_iter().collect(),
result_id: "minecraft:polished_tuff",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:brown_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:brown_dye",
],
result_id: "minecraft:brown_bundle",
})));
	output.insert("minecraft:ender_chest", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#E#",
	"###",
],
key: vec![
	("#", vec!["minecraft:obsidian"]),
	("E", vec!["minecraft:ender_eye"]),
].into_iter().collect(),
result_id: "minecraft:ender_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_chain_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_chain"),
ingredients: vec![
	vec!["minecraft:weathered_copper_chain"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_red_sandstone", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:red_sandstone",
],
cooking_time: Some(200),
result_id: "minecraft:smooth_red_sandstone",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:waxed_chiseled_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_chiseled_copper"),
ingredients: vec![
	vec!["minecraft:chiseled_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:orange_dye"]),
].into_iter().collect(),
result_id: "minecraft:orange_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:acacia_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:bricks"]),
].into_iter().collect(),
result_id: "minecraft:brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:bamboo_mosaic_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_mosaic"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_mosaic_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:emerald", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:emerald_block"],
],
result_id: "minecraft:emerald",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:pink_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:pink_dye"]),
].into_iter().collect(),
result_id: "minecraft:pink_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:light_blue_dye",
],
result_id: "minecraft:light_blue_bundle",
})));
	output.insert("minecraft:purpur_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:purpur_block",
		"minecraft:purpur_pillar",
	]),
].into_iter().collect(),
result_id: "minecraft:purpur_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mangrove_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:mangrove_planks"],
],
result_id: "minecraft:mangrove_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:leather_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:leather_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_granite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_granite"]),
].into_iter().collect(),
result_id: "minecraft:polished_granite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:emerald_from_smelting_emerald_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("emerald"),
ingredient: vec![
	"minecraft:emerald_ore",
],
cooking_time: Some(200),
result_id: "minecraft:emerald",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:red_sandstone_stairs_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:red_sandstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:acacia_planks"],
],
result_id: "minecraft:acacia_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:amethyst_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:amethyst_shard"]),
].into_iter().collect(),
result_id: "minecraft:amethyst_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_stairs_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_pink_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:pink_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:pink_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:birch_boat"],
],
result_id: "minecraft:birch_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:cyan_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:cyan_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_stairs_from_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:prismarine",
],
result_id: "minecraft:prismarine_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:orange_dye"],
],
result_id: "minecraft:orange_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_acacia_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_acacia_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_acacia_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:pink_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:pink_dye",
],
result_id: "minecraft:pink_bundle",
})));
	output.insert("minecraft:iron_nugget", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:iron_ingot"],
],
result_id: "minecraft:iron_nugget",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:dried_kelp_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:dried_kelp"]),
].into_iter().collect(),
result_id: "minecraft:dried_kelp_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:andesite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:andesite"]),
].into_iter().collect(),
result_id: "minecraft:andesite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:beetroot_soup", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:beetroot"],
	vec!["minecraft:beetroot"],
	vec!["minecraft:beetroot"],
	vec!["minecraft:beetroot"],
	vec!["minecraft:beetroot"],
	vec!["minecraft:beetroot"],
],
result_id: "minecraft:beetroot_soup",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_stairs_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:tuff_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_lightning_rod_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_lightning_rod"),
ingredients: vec![
	vec!["minecraft:exposed_lightning_rod"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_lightning_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_cod", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:cod",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_cod",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:orange_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:orange_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:orange_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:polished_diorite_stairs_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:polished_diorite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bone_meal", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bonemeal"),
ingredients: vec![
	vec!["minecraft:bone"],
],
result_id: "minecraft:bone_meal",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_red_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:red_tulip"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_scrap_from_blasting", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:ancient_debris",
],
cooking_time: Some(100),
result_id: "minecraft:netherite_scrap",
result_components: None,
experience: Some(2.0),
})));
	output.insert("minecraft:jack_o_lantern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"A",
	"B",
],
key: vec![
	("A", vec!["minecraft:carved_pumpkin"]),
	("B", vec!["minecraft:torch"]),
].into_iter().collect(),
result_id: "minecraft:jack_o_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_nugget", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:copper_ingot"],
],
result_id: "minecraft:copper_nugget",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:pink_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:pink_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:pink_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_wall_from_end_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone",
],
result_id: "minecraft:end_stone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_mutton_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:mutton",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_mutton",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:crimson_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_crimson_stem"]),
].into_iter().collect(),
result_id: "minecraft:crimson_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_yellow_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:yellow_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:yellow_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_trapdoor_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_trapdoor"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_trapdoor"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_quartz_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:quartz_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_quartz_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_wall_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_chestplate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"XXX",
],
key: vec![
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_chestplate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:cobbled_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:polished_deepslate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:mangrove_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:magenta_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:magenta_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:birch_logs"],
],
result_id: "minecraft:birch_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:iron_nugget_from_blasting", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:iron_pickaxe",
	"minecraft:iron_shovel",
	"minecraft:iron_axe",
	"minecraft:iron_hoe",
	"minecraft:iron_sword",
	"minecraft:iron_helmet",
	"minecraft:iron_chestplate",
	"minecraft:iron_leggings",
	"minecraft:iron_boots",
	"minecraft:iron_horse_armor",
	"minecraft:chainmail_helmet",
	"minecraft:chainmail_chestplate",
	"minecraft:chainmail_leggings",
	"minecraft:chainmail_boots",
],
cooking_time: Some(100),
result_id: "minecraft:iron_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:cooked_beef_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:beef",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_beef",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:mossy_stone_bricks_from_vine", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("mossy_stone_bricks"),
ingredients: vec![
	vec!["minecraft:stone_bricks"],
	vec!["minecraft:vine"],
],
result_id: "minecraft:mossy_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_grate_from_waxed_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_copper_block",
],
result_id: "minecraft:waxed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:prismarine_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:prismarine"]),
].into_iter().collect(),
result_id: "minecraft:prismarine_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:waxed_exposed_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:writable_book", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:book"],
	vec!["minecraft:ink_sac"],
	vec!["minecraft:feather"],
],
result_id: "minecraft:writable_book",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:blue_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:blue_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dispenser", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"#R#",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
	("R", vec!["minecraft:redstone"]),
	("X", vec!["minecraft:bow"]),
].into_iter().collect(),
result_id: "minecraft:dispenser",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:green_dye"]),
].into_iter().collect(),
result_id: "minecraft:green_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:diamond_from_smelting_deepslate_diamond_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("diamond"),
ingredient: vec![
	"minecraft:deepslate_diamond_ore",
],
cooking_time: Some(200),
result_id: "minecraft:diamond",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:white_dye_from_lily_of_the_valley", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("white_dye"),
ingredients: vec![
	vec!["minecraft:lily_of_the_valley"],
],
result_id: "minecraft:white_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:purple_wool"]),
].into_iter().collect(),
result_id: "minecraft:purple_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:bamboo_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:birch_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_polished_blackstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_polished_blackstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_white_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:white_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
	],
],
result_id: "minecraft:white_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff_bricks"]),
].into_iter().collect(),
result_id: "minecraft:tuff_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_smelting_raw_gold", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:raw_gold",
],
cooking_time: Some(200),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:popped_chorus_fruit", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:chorus_fruit",
],
cooking_time: Some(200),
result_id: "minecraft:popped_chorus_fruit",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:brick", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:clay_ball",
],
cooking_time: Some(200),
result_id: "minecraft:brick",
result_components: None,
experience: Some(0.3),
})));
	output.insert("minecraft:nether_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:nether_brick"]),
].into_iter().collect(),
result_id: "minecraft:nether_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff"]),
].into_iter().collect(),
result_id: "minecraft:tuff_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_light_gray_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:light_gray_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:light_gray_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_slab_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:magenta_dye_from_allium", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("magenta_dye"),
ingredients: vec![
	vec!["minecraft:allium"],
],
result_id: "minecraft:magenta_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_nugget_from_smelting", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:golden_pickaxe",
	"minecraft:golden_shovel",
	"minecraft:golden_axe",
	"minecraft:golden_hoe",
	"minecraft:golden_sword",
	"minecraft:golden_helmet",
	"minecraft:golden_chestplate",
	"minecraft:golden_leggings",
	"minecraft:golden_boots",
	"minecraft:golden_horse_armor",
],
cooking_time: Some(200),
result_id: "minecraft:gold_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:diorite", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"CQ",
	"QC",
],
key: vec![
	("C", vec!["minecraft:cobblestone"]),
	("Q", vec!["minecraft:quartz"]),
].into_iter().collect(),
result_id: "minecraft:diorite",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:quartz_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:chiseled_quartz_block",
		"minecraft:quartz_block",
		"minecraft:quartz_pillar",
	]),
].into_iter().collect(),
result_id: "minecraft:quartz_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:nether_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:nether_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:oak_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blackstone_stairs_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:blackstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:acacia_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:magenta_dye_from_blue_red_pink", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("magenta_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:red_dye"],
	vec!["minecraft:pink_dye"],
],
result_id: "minecraft:magenta_dye",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:warped_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_cyan_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:cyan_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:cyan_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_polished_blackstone_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:chiseled_polished_blackstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pumpkin_seeds", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:pumpkin"],
],
result_id: "minecraft:pumpkin_seeds",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_bricks"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:sandstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:sandstone",
		"minecraft:chiseled_sandstone",
		"minecraft:cut_sandstone",
	]),
].into_iter().collect(),
result_id: "minecraft:sandstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:light_blue_wool"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:magenta_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:magenta_dye"]),
].into_iter().collect(),
result_id: "minecraft:magenta_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:clay", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:clay_ball"]),
].into_iter().collect(),
result_id: "minecraft:clay",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:ender_eye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:ender_pearl"],
	vec!["minecraft:blaze_powder"],
],
result_id: "minecraft:ender_eye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_brick_slab_from_resin_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:resin_bricks",
],
result_id: "minecraft:resin_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:blue_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:blue_wool"]),
].into_iter().collect(),
result_id: "minecraft:blue_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:stone", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:cobblestone",
],
cooking_time: Some(200),
result_id: "minecraft:stone",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:crimson_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:crimson_planks"],
],
result_id: "minecraft:crimson_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_chicken_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:chicken",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_chicken",
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:waxed_weathered_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_wall_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_tile_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:brown_wool"]),
].into_iter().collect(),
result_id: "minecraft:brown_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:copper_bars", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_bars",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_bulb_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_bulb"),
ingredients: vec![
	vec!["minecraft:exposed_copper_bulb"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_bulb",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:pink_wool"]),
].into_iter().collect(),
result_id: "minecraft:pink_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_lightning_rod_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_lightning_rod"),
ingredients: vec![
	vec!["minecraft:oxidized_lightning_rod"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_lightning_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_dye_from_purple_and_pink", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("magenta_dye"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec!["minecraft:pink_dye"],
],
result_id: "minecraft:magenta_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:calibrated_sculk_sensor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
],
key: vec![
	("#", vec!["minecraft:amethyst_shard"]),
	("X", vec!["minecraft:sculk_sensor"]),
].into_iter().collect(),
result_id: "minecraft:calibrated_sculk_sensor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:jungle_logs"],
],
result_id: "minecraft:jungle_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:chest", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"# #",
	"###",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:sandstone_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:purple_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:purple_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:stripped_mangrove_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_mangrove_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_mangrove_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:purple_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:purple_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:orange_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:orange_dye"]),
].into_iter().collect(),
result_id: "minecraft:orange_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_wall_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:lime_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:lime_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:spruce_logs"],
],
result_id: "minecraft:spruce_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dried_kelp", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:dried_kelp_block"],
],
result_id: "minecraft:dried_kelp",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:light_gray_dye_from_white_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_gray_dye"),
ingredients: vec![
	vec!["minecraft:white_tulip"],
],
result_id: "minecraft:light_gray_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_shovel_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_shovel",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_shovel",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:cyan_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:cyan_dye"]),
].into_iter().collect(),
result_id: "minecraft:cyan_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:yellow_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:yellow_dye"]),
].into_iter().collect(),
result_id: "minecraft:yellow_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:andesite_slab_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:andesite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:blackstone_wall_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:blackstone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_porkchop_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:porkchop",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_porkchop",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_weathered_cut_copper_stairs_from_waxed_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_cut_copper",
],
result_id: "minecraft:waxed_weathered_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_smelting_deepslate_iron_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:deepslate_iron_ore",
],
cooking_time: Some(200),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:shears", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" #",
	"# ",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:shears",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:redstone_from_smelting_deepslate_redstone_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: Some("redstone"),
ingredient: vec![
	"minecraft:deepslate_redstone_ore",
],
cooking_time: Some(200),
result_id: "minecraft:redstone",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:rabbit_stew_from_brown_mushroom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("rabbit_stew"),
ingredients: vec![
	vec!["minecraft:baked_potato"],
	vec!["minecraft:cooked_rabbit"],
	vec!["minecraft:bowl"],
	vec!["minecraft:carrot"],
	vec!["minecraft:brown_mushroom"],
],
result_id: "minecraft:rabbit_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:honeycomb_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:honeycomb"]),
].into_iter().collect(),
result_id: "minecraft:honeycomb_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:black_dye_from_wither_rose", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("black_dye"),
ingredients: vec![
	vec!["minecraft:wither_rose"],
],
result_id: "minecraft:black_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:eye_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:end_stone"]),
	("S", vec!["minecraft:eye_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:eye_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:crimson_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:purpur_slab_from_purpur_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:purpur_block",
],
result_id: "minecraft:purpur_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:chiseled_tuff_bricks_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:chiseled_tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_walls_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_slab_from_stone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone_bricks",
],
result_id: "minecraft:stone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_chest", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
	("X", vec!["minecraft:chest"]),
].into_iter().collect(),
result_id: "minecraft:copper_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:yellow_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:yellow_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:smoker", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
	" # ",
],
key: vec![
	("#", vec!["#minecraft:logs"]),
	("X", vec!["minecraft:furnace"]),
].into_iter().collect(),
result_id: "minecraft:smoker",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_black_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec![
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:black_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:flow_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:breeze_rod"]),
	("S", vec!["minecraft:flow_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:flow_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:bamboo_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:tnt_minecart", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:tnt"],
	vec!["minecraft:minecart"],
],
result_id: "minecraft:tnt_minecart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_sandstone_wall_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:red_sandstone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_stairs_from_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_cut_copper",
],
result_id: "minecraft:exposed_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oxidized_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:oxidized_copper"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dye_light_gray_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:light_gray_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:light_gray_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cherry_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:chiseled_nether_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:nether_brick_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_nether_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_dye_from_dandelion", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("yellow_dye"),
ingredients: vec![
	vec!["minecraft:dandelion"],
],
result_id: "minecraft:yellow_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:heavy_weighted_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:heavy_weighted_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_ingot", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("netherite_ingot"),
ingredients: vec![
	vec!["minecraft:netherite_scrap"],
	vec!["minecraft:netherite_scrap"],
	vec!["minecraft:netherite_scrap"],
	vec!["minecraft:netherite_scrap"],
	vec!["minecraft:gold_ingot"],
	vec!["minecraft:gold_ingot"],
	vec!["minecraft:gold_ingot"],
	vec!["minecraft:gold_ingot"],
],
result_id: "minecraft:netherite_ingot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:white_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_gray_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:gray_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_chest_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_chest"),
ingredients: vec![
	vec!["minecraft:weathered_copper_chest"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_slab_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:polished_tuff_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cherry_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:cherry_planks"],
],
result_id: "minecraft:cherry_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_chiseled_copper_from_waxed_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_copper",
],
result_id: "minecraft:waxed_weathered_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:netherite_sword_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_sword",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_sword",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:dark_prismarine_slab_from_dark_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:dark_prismarine",
],
result_id: "minecraft:dark_prismarine_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:purple_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:purple_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:purple_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_bars_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_bars"),
ingredients: vec![
	vec!["minecraft:weathered_copper_bars"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_bars",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:black_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:black_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:black_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper"),
ingredients: vec![
	vec!["minecraft:weathered_cut_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_cut_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:fire_charge", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:gunpowder"],
	vec!["minecraft:blaze_powder"],
	vec![
		"minecraft:coal",
		"minecraft:charcoal",
	],
],
result_id: "minecraft:fire_charge",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_slab_from_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_cut_copper",
],
result_id: "minecraft:weathered_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:quartz_bricks_from_quartz_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:quartz_block",
],
result_id: "minecraft:quartz_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:firework_rocket", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::FireworkRocket,
category: Some(Category::Misc),
}));
	output.insert("minecraft:chiseled_stone_bricks_from_stone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone_bricks",
],
result_id: "minecraft:chiseled_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:purple_dye",
],
result_id: "minecraft:purple_bundle",
})));
	output.insert("minecraft:polished_granite_stairs_from_polished_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_granite",
],
result_id: "minecraft:polished_granite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_trapdoor_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_trapdoor"),
ingredients: vec![
	vec!["minecraft:weathered_copper_trapdoor"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coast_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:cobblestone"]),
	("S", vec!["minecraft:coast_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:coast_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:warped_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_warped_stem"]),
].into_iter().collect(),
result_id: "minecraft:warped_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_granite_stairs_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:polished_granite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:oak_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:purple_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:purple_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:purple_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:blaze_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:blaze_rod"],
],
result_id: "minecraft:blaze_powder",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_slab_from_waxed_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_copper",
],
result_id: "minecraft:waxed_exposed_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:crimson_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:crimson_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:dye_blue_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:blue_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:barrel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"PSP",
	"P P",
	"PSP",
],
key: vec![
	("P", vec!["#minecraft:planks"]),
	("S", vec!["#minecraft:wooden_slabs"]),
].into_iter().collect(),
result_id: "minecraft:barrel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_stairs_from_deepslate_tiles_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_tiles",
],
result_id: "minecraft:deepslate_tile_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:red_dye"]),
].into_iter().collect(),
result_id: "minecraft:red_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_stairs_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mossy_stone_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dye_green_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:green_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:lime_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_red_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:red_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oak_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:gray_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_wall_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:polished_deepslate_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_stone_slab_from_smooth_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_stone",
],
result_id: "minecraft:smooth_stone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:warped_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_slab_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_tile_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:yellow_dye_from_sunflower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("yellow_dye"),
ingredients: vec![
	vec!["minecraft:sunflower"],
],
result_id: "minecraft:yellow_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cut_sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:sandstone"]),
].into_iter().collect(),
result_id: "minecraft:cut_sandstone",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:gray_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:gray_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:gray_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:red_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:red_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:red_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:honey_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:honey_bottle"]),
].into_iter().collect(),
result_id: "minecraft:honey_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_poppy", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:poppy"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:white_wool"]),
].into_iter().collect(),
result_id: "minecraft:white_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:copper_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:copper_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:copper_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:granite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:granite"]),
].into_iter().collect(),
result_id: "minecraft:granite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:purpur_pillar_from_purpur_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:purpur_block",
],
result_id: "minecraft:purpur_pillar",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:black_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:black_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:black_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:yellow_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:yellow_dye",
],
result_id: "minecraft:yellow_shulker_box",
})));
	output.insert("minecraft:emerald_from_blasting_deepslate_emerald_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("emerald"),
ingredient: vec![
	"minecraft:deepslate_emerald_ore",
],
cooking_time: Some(100),
result_id: "minecraft:emerald",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:warped_fungus_on_a_stick", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# ",
	" X",
],
key: vec![
	("#", vec!["minecraft:fishing_rod"]),
	("X", vec!["minecraft:warped_fungus"]),
].into_iter().collect(),
result_id: "minecraft:warped_fungus_on_a_stick",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_slab_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:polished_deepslate_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_magenta_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:magenta_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:magenta_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_ice", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
	vec!["minecraft:packed_ice"],
],
result_id: "minecraft:blue_ice",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_slab_from_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:nether_bricks",
],
result_id: "minecraft:nether_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:golden_boots", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:golden_boots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tide_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:tide_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:deepslate_brick_wall_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:acacia_log"]),
].into_iter().collect(),
result_id: "minecraft:acacia_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:dye_white_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:white_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
	],
],
result_id: "minecraft:white_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tnt", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"X#X",
	"#X#",
	"X#X",
],
key: vec![
	("{k}", vec![
		"minecraft:sand",
		"minecraft:red_sand",
	]),
	("X", vec!["minecraft:gunpowder"]),
].into_iter().collect(),
result_id: "minecraft:tnt",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cut_sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cut_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:cut_sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_from_waxed_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_copper",
],
result_id: "minecraft:waxed_weathered_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_wall_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_tile_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:mossy_cobblestone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:spyglass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	" X ",
	" X ",
],
key: vec![
	("#", vec!["minecraft:amethyst_shard"]),
	("X", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:spyglass",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:quartz_pillar_from_quartz_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:quartz_block",
],
result_id: "minecraft:quartz_pillar",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:diamond_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_tuff_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:chiseled_tuff",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_resin_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:resin_brick_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_resin_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:bolt_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:bolt_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:gray_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:gray_dye",
],
result_id: "minecraft:gray_shulker_box",
})));
	output.insert("minecraft:end_rod", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"/",
	"#",
],
key: vec![
	("#", vec!["minecraft:popped_chorus_fruit"]),
	("/", vec!["minecraft:blaze_rod"]),
].into_iter().collect(),
result_id: "minecraft:end_rod",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_slab_from_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_cut_copper",
],
result_id: "minecraft:oxidized_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_slab_from_red_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_nether_bricks",
],
result_id: "minecraft:red_nether_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_chestplate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"XXX",
],
key: vec![
	("X", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_chestplate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:black_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("black_dye"),
ingredients: vec![
	vec!["minecraft:ink_sac"],
],
result_id: "minecraft:black_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:light_blue_dye"],
],
result_id: "minecraft:light_blue_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purpur_pillar", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:purpur_slab"]),
].into_iter().collect(),
result_id: "minecraft:purpur_pillar",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:weathered_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:weathered_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:weathered_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_leggings", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:golden_leggings",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:blue_dye"],
],
result_id: "minecraft:blue_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crimson_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_crimson_stem"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:crimson_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:snout_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:blackstone"]),
	("S", vec!["minecraft:snout_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:snout_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_orange_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:orange_tulip"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:compass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
	" # ",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
	("X", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:compass",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_beef_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:beef",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_beef",
result_components: None,
})));
	output.insert("minecraft:bamboo_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:bamboo_planks"],
],
result_id: "minecraft:bamboo_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:brown_dye"],
],
result_id: "minecraft:brown_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_wall_from_red_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_nether_bricks",
],
result_id: "minecraft:red_nether_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:green_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:green_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_light_gray_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:light_gray_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:light_gray_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:pink_dye"],
],
result_id: "minecraft:pink_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_wall_from_tuff_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff_bricks",
],
result_id: "minecraft:tuff_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_bamboo_block"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:spruce_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_spruce_log"]),
].into_iter().collect(),
result_id: "minecraft:spruce_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:chiseled_polished_blackstone_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:chiseled_polished_blackstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_lantern_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_lantern"),
ingredients: vec![
	vec!["minecraft:copper_lantern"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:green_dye",
],
result_id: "minecraft:green_shulker_box",
})));
	output.insert("minecraft:waxed_oxidized_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:waxed_oxidized_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cherry_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_cherry_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:cherry_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_slab_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_tile_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_chest_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_chest"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_chest"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:red_nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:red_nether_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:weathered_chiseled_copper_from_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_copper",
],
result_id: "minecraft:weathered_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:waxed_weathered_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:waxed_oxidized_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cut_sandstone_slab_from_cut_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cut_sandstone",
],
result_id: "minecraft:cut_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:moss_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:moss_block"]),
].into_iter().collect(),
result_id: "minecraft:moss_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:oak_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:wooden_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wooden_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:wooden_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_wart_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
	vec!["minecraft:nether_wart"],
],
result_id: "minecraft:nether_wart_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:glass", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"#minecraft:smelts_to_glass",
],
cooking_time: Some(200),
result_id: "minecraft:glass",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:orange_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:orange_dye",
],
result_id: "minecraft:orange_bundle",
})));
	output.insert("minecraft:book", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:paper"],
	vec!["minecraft:paper"],
	vec!["minecraft:leather"],
],
result_id: "minecraft:book",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_dye_from_black_white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_gray_dye"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec!["minecraft:white_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:light_gray_dye",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:iron_boots", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_boots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:light_blue_dye",
],
result_id: "minecraft:light_blue_shulker_box",
})));
	output.insert("minecraft:respawn_anchor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"OOO",
	"GGG",
	"OOO",
],
key: vec![
	("G", vec!["minecraft:glowstone"]),
	("O", vec!["minecraft:crying_obsidian"]),
].into_iter().collect(),
result_id: "minecraft:respawn_anchor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sea_lantern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SCS",
	"CCC",
	"SCS",
],
key: vec![
	("C", vec!["minecraft:prismarine_crystals"]),
	("S", vec!["minecraft:prismarine_shard"]),
].into_iter().collect(),
result_id: "minecraft:sea_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:tuff_bricks"]),
].into_iter().collect(),
result_id: "minecraft:tuff_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_chest_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_chest"),
ingredients: vec![
	vec!["minecraft:exposed_copper_chest"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:copper_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:copper_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_slab_from_polished_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_andesite",
],
result_id: "minecraft:polished_andesite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:pink_dye_from_pink_petals", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("pink_dye"),
ingredients: vec![
	vec!["minecraft:pink_petals"],
],
result_id: "minecraft:pink_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobbled_deepslate_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:cobbled_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:cobbled_deepslate_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:end_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:end_stone_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:stripped_birch_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_birch_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_birch_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blackstone_slab_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:blackstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:granite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:granite"]),
].into_iter().collect(),
result_id: "minecraft:granite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:iron_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:iron_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:iron_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_mutton", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:mutton",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_mutton",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:gray_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("gray_dye"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:gray_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cooked_chicken_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:chicken",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_chicken",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_weathered_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:waxed_weathered_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:bookshelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"XXX",
	"###",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("X", vec!["minecraft:book"]),
].into_iter().collect(),
result_id: "minecraft:bookshelf",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:emerald_from_blasting_emerald_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("emerald"),
ingredient: vec![
	"minecraft:emerald_ore",
],
cooking_time: Some(100),
result_id: "minecraft:emerald",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:mangrove_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:mangrove_boat"],
],
result_id: "minecraft:mangrove_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:birch_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:dye_light_blue_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:light_blue_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:light_blue_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_stairs_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:slime_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:slime_ball"]),
].into_iter().collect(),
result_id: "minecraft:slime_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:diamond_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:diamond_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lodestone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"SSS",
	"S#S",
	"SSS",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
	("S", vec!["minecraft:chiseled_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:lodestone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cherry_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:oak_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:oak_planks"],
],
result_id: "minecraft:oak_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mangrove_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:orange_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:orange_dye"]),
].into_iter().collect(),
result_id: "minecraft:orange_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:white_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:white_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:white_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:gray_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:gray_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:dye_purple_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:purple_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:warped_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:green_dye", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:cactus",
],
cooking_time: Some(200),
result_id: "minecraft:green_dye",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:granite_stairs_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:granite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:creaking_heart", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" L ",
	" R ",
	" L ",
],
key: vec![
	("L", vec!["minecraft:pale_oak_log"]),
	("R", vec!["minecraft:resin_block"]),
].into_iter().collect(),
result_id: "minecraft:creaking_heart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobblestone_stairs_from_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobblestone",
],
result_id: "minecraft:cobblestone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_blasting_deepslate_iron_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:deepslate_iron_ore",
],
cooking_time: Some(100),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:red_dye_from_rose_bush", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("red_dye"),
ingredients: vec![
	vec!["minecraft:rose_bush"],
],
result_id: "minecraft:red_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dark_prismarine_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_prismarine"]),
].into_iter().collect(),
result_id: "minecraft:dark_prismarine_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_slab_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:chiseled_nether_bricks_from_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:nether_bricks",
],
result_id: "minecraft:chiseled_nether_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:red_dye"]),
].into_iter().collect(),
result_id: "minecraft:red_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:note_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("X", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:note_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_slab_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_smelting_deepslate_copper_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:deepslate_copper_ore",
],
cooking_time: Some(200),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:iron_bars", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_bars",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_stairs_from_end_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone",
],
result_id: "minecraft:end_stone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:turtle_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:turtle_scute"]),
].into_iter().collect(),
result_id: "minecraft:turtle_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:andesite"]),
].into_iter().collect(),
result_id: "minecraft:polished_andesite",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:netherite_hoe_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_hoe",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_hoe",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:shield", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"WoW",
	"WWW",
	" W ",
],
key: vec![
	("W", vec!["#minecraft:wooden_tool_materials"]),
	("o", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:shield",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:yellow_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:yellow_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:diamond_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:diamond_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:diamond_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_chain", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"N",
	"I",
	"N",
],
key: vec![
	("I", vec!["minecraft:iron_ingot"]),
	("N", vec!["minecraft:iron_nugget"]),
].into_iter().collect(),
result_id: "minecraft:iron_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:brown_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:brown_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_white_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:white_tulip"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:scaffolding", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"I~I",
	"I I",
	"I I",
],
key: vec![
	("I", vec!["minecraft:bamboo"]),
	("~", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:scaffolding",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cherry_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_cherry_log"]),
].into_iter().collect(),
result_id: "minecraft:cherry_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:jungle_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_slab_from_waxed_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_copper",
],
result_id: "minecraft:waxed_oxidized_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper"),
ingredients: vec![
	vec!["minecraft:weathered_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lever", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:lever",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:iron_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:iron_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stick", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("sticks"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:stick",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:smooth_sandstone_slab_from_smooth_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_sandstone",
],
result_id: "minecraft:smooth_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:waxed_copper_block"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:waxed_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:copper_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:copper_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:copper_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:purple_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:purple_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_black_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec![
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:black_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_slab_from_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_copper",
],
result_id: "minecraft:oxidized_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:gray_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:gray_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:gray_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bone_meal_from_bone_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bonemeal"),
ingredients: vec![
	vec!["minecraft:bone_block"],
],
result_id: "minecraft:bone_meal",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_stairs_from_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_copper",
],
result_id: "minecraft:weathered_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:arrow", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"Y",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:flint"]),
	("Y", vec!["minecraft:feather"]),
].into_iter().collect(),
result_id: "minecraft:arrow",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:raiser_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:raiser_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:waxed_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_cut_copper_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cut_copper_stairs_from_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:copper_block",
],
result_id: "minecraft:cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cooked_cod_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:cod",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_cod",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_oxidized_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:wooden_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wooden_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:wooden_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:end_stone"]),
].into_iter().collect(),
result_id: "minecraft:end_stone_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:brown_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:brown_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:brown_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_slab_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:polished_andesite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:magenta_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:magenta_dye"]),
].into_iter().collect(),
result_id: "minecraft:magenta_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:dark_oak_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:light_blue_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:light_blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:green_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:green_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:green_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coal_from_blasting_coal_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("coal"),
ingredient: vec![
	"minecraft:coal_ore",
],
cooking_time: Some(100),
result_id: "minecraft:coal",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:red_dye_from_poppy", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("red_dye"),
ingredients: vec![
	vec!["minecraft:poppy"],
],
result_id: "minecraft:red_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_brick_stairs_from_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:prismarine_bricks",
],
result_id: "minecraft:prismarine_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:red_sand"]),
].into_iter().collect(),
result_id: "minecraft:red_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_weathered_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_granite_slab_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:polished_granite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_from_waxed_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_copper",
],
result_id: "minecraft:waxed_exposed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_golem_statue_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_golem_statue"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_golem_statue"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_golem_statue",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stone"]),
].into_iter().collect(),
result_id: "minecraft:stone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:bundle", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"-",
	"#",
],
key: vec![
	("#", vec!["minecraft:leather"]),
	("-", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:bundle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:purple_dye",
],
result_id: "minecraft:purple_shulker_box",
})));
	output.insert("minecraft:mangrove_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:mangrove_log"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:birch_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_slab_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cooked_rabbit_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:rabbit",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_rabbit",
result_components: None,
})));
	output.insert("minecraft:dye_gray_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:gray_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:paper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:sugar_cane"]),
].into_iter().collect(),
result_id: "minecraft:paper",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:polished_granite", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:granite"]),
].into_iter().collect(),
result_id: "minecraft:polished_granite",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_stairs_from_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_cut_copper",
],
result_id: "minecraft:oxidized_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_grate_from_waxed_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_copper",
],
result_id: "minecraft:waxed_exposed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dye_purple_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:purple_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_stairs_from_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_copper",
],
result_id: "minecraft:exposed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:orange_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:orange_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:orange_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:gray_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:gray_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:birch_log"]),
].into_iter().collect(),
result_id: "minecraft:birch_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:mossy_cobblestone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cut_sandstone_slab_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:cut_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:map_cloning", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Mapcloning,
category: Some(Category::Misc),
}));
	output.insert("minecraft:dye_orange_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:orange_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:orange_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_door_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_door"),
ingredients: vec![
	vec!["minecraft:exposed_copper_door"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_door",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_slab_from_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_copper",
],
result_id: "minecraft:exposed_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:granite_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:granite"]),
].into_iter().collect(),
result_id: "minecraft:granite_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_wall_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_slab_from_tuff_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff_bricks",
],
result_id: "minecraft:tuff_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_smelting_nether_gold_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:nether_gold_ore",
],
cooking_time: Some(200),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:dye_lime_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:lime_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:lime_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_red_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:red_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_oxidized_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_stairs_from_waxed_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_copper",
],
result_id: "minecraft:waxed_oxidized_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dark_prismarine", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SSS",
	"SIS",
	"SSS",
],
key: vec![
	("I", vec!["minecraft:black_dye"]),
	("S", vec!["minecraft:prismarine_shard"]),
].into_iter().collect(),
result_id: "minecraft:dark_prismarine",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_slab_from_waxed_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_copper",
],
result_id: "minecraft:waxed_weathered_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:bowl", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	" # ",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:bowl",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_gray_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:light_gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:exposed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:exposed_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:acacia_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_stairs_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper_stairs"),
ingredients: vec![
	vec!["minecraft:weathered_cut_copper_stairs"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_wall_from_end_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone_bricks",
],
result_id: "minecraft:end_stone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:shaper_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:terracotta"]),
	("S", vec!["minecraft:shaper_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:shaper_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:black_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:black_dye"]),
].into_iter().collect(),
result_id: "minecraft:black_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_block_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_block"),
ingredients: vec![
	vec!["minecraft:copper_block"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_prismarine_stairs_from_dark_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:dark_prismarine",
],
result_id: "minecraft:dark_prismarine_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:raw_iron_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:raw_iron"]),
].into_iter().collect(),
result_id: "minecraft:raw_iron_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:pale_oak_boat"],
],
result_id: "minecraft:pale_oak_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_exposed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:eye_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:eye_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:polished_andesite_stairs_from_polished_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_andesite",
],
result_id: "minecraft:polished_andesite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:stone_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:pale_oak_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_quartz_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_quartz"]),
].into_iter().collect(),
result_id: "minecraft:smooth_quartz_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:green_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:green_dye"]),
].into_iter().collect(),
result_id: "minecraft:green_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:copper_nugget_from_blasting", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:copper_pickaxe",
	"minecraft:copper_shovel",
	"minecraft:copper_axe",
	"minecraft:copper_hoe",
	"minecraft:copper_sword",
	"minecraft:copper_helmet",
	"minecraft:copper_chestplate",
	"minecraft:copper_leggings",
	"minecraft:copper_boots",
	"minecraft:copper_horse_armor",
],
cooking_time: Some(100),
result_id: "minecraft:copper_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:weathered_copper_grate_from_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_copper",
],
result_id: "minecraft:weathered_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:crimson_hyphae", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:crimson_stem"]),
].into_iter().collect(),
result_id: "minecraft:crimson_hyphae",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:bamboo_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_stairs_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_allium", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:allium"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_wall_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:polished_deepslate_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:lime_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:lime_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_deepslate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:cobbled_deepslate_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_deepslate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:gold_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_slab_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_cut_copper_slab"),
ingredients: vec![
	vec!["minecraft:cut_copper_slab"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_cut_copper_slab",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_slab_from_polished_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_diorite",
],
result_id: "minecraft:polished_diorite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_yellow_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:yellow_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:yellow_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_lime_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:lime_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:lime_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:orange_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:orange_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_nuggets", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:copper_nugget"]),
].into_iter().collect(),
result_id: "minecraft:copper_ingot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_black_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec![
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:black_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_acacia_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:acacia_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:netherite_upgrade_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:netherrack"]),
	("S", vec!["minecraft:netherite_upgrade_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:netherite_upgrade_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wooden_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wooden_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:wooden_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:green_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:green_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:magenta_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:magenta_dye"]),
].into_iter().collect(),
result_id: "minecraft:magenta_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:red_nether_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"NW",
	"WN",
],
key: vec![
	("N", vec!["minecraft:nether_brick"]),
	("W", vec!["minecraft:nether_wart"]),
].into_iter().collect(),
result_id: "minecraft:red_nether_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_dye_from_open_eyeblossom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("orange_dye"),
ingredients: vec![
	vec!["minecraft:open_eyeblossom"],
],
result_id: "minecraft:orange_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:gray_dye"],
],
result_id: "minecraft:gray_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_magenta_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:magenta_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:magenta_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:cyan_dye",
],
result_id: "minecraft:cyan_bundle",
})));
	output.insert("minecraft:magenta_dye_from_blue_red_white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("magenta_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:red_dye"],
	vec!["minecraft:red_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:magenta_dye",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:spruce_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:red_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:red_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:red_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_lightning_rod_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_lightning_rod"),
ingredients: vec![
	vec!["minecraft:lightning_rod"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_lightning_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smithing_table", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"@@",
	"##",
	"##",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("@", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:smithing_table",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wooden_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wooden_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:wooden_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_from_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_copper",
],
result_id: "minecraft:weathered_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_chiseled_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_chiseled_copper"),
ingredients: vec![
	vec!["minecraft:oxidized_chiseled_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_chiseled_copper_from_waxed_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_cut_copper",
],
result_id: "minecraft:waxed_oxidized_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blast_furnace", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"III",
	"IXI",
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_stone"]),
	("I", vec!["minecraft:iron_ingot"]),
	("X", vec!["minecraft:furnace"]),
].into_iter().collect(),
result_id: "minecraft:blast_furnace",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:stone_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:stone_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_tuff"]),
].into_iter().collect(),
result_id: "minecraft:polished_tuff_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_golem_statue_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_golem_statue"),
ingredients: vec![
	vec!["minecraft:copper_golem_statue"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_golem_statue",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_torchflower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:torchflower"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wooden_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wooden_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:wooden_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cut_copper_from_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:copper_block",
],
result_id: "minecraft:cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cobblestone_wall_from_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobblestone",
],
result_id: "minecraft:cobblestone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: None,
ingredients: vec![
	vec!["minecraft:stone"],
],
result_id: "minecraft:stone_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:light_gray_dye"],
],
result_id: "minecraft:light_gray_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:birch_planks"],
],
result_id: "minecraft:birch_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dried_kelp_from_smelting", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:kelp",
],
cooking_time: Some(200),
result_id: "minecraft:dried_kelp",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:waxed_weathered_copper_door_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_door"),
ingredients: vec![
	vec!["minecraft:weathered_copper_door"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_door",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:diamond_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:diamond_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_gold_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredients: vec![
	vec!["minecraft:gold_block"],
],
result_id: "minecraft:gold_ingot",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:waxed_exposed_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_smelting_raw_iron", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:raw_iron",
],
cooking_time: Some(200),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:polished_deepslate_slab_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:polished_deepslate_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:stone_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:stone_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:stone_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:lime_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:lime_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:mangrove_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:decorated_pot_simple", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"# #",
	" # ",
],
key: vec![
	("#", vec!["minecraft:brick"]),
].into_iter().collect(),
result_id: "minecraft:decorated_pot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:gold_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:golden_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:golden_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:blue_dye",
],
result_id: "minecraft:blue_shulker_box",
})));
	output.insert("minecraft:bamboo_mosaic_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_mosaic"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_mosaic_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cherry_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:lightning_rod", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:lightning_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crafter", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#C#",
	"RDR",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
	("C", vec!["minecraft:crafting_table"]),
	("D", vec!["minecraft:dropper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:crafter",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_stairs_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:polished_tuff_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_chestplate_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_chestplate",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_chestplate",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:creeper_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:creeper_head"],
],
result_id: "minecraft:creeper_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
cooking_time: Some(200),
result_id: "minecraft:deepslate",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:tuff_brick_slab_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:nether_brick", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:netherrack",
],
cooking_time: Some(200),
result_id: "minecraft:nether_brick",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:red_dye_from_beetroot", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("red_dye"),
ingredients: vec![
	vec!["minecraft:beetroot"],
],
result_id: "minecraft:red_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_wall_from_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:nether_bricks",
],
result_id: "minecraft:nether_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:loom", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"@@",
	"##",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("@", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:loom",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lapis_lazuli_from_blasting_deepslate_lapis_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("lapis_lazuli"),
ingredient: vec![
	"minecraft:deepslate_lapis_ore",
],
cooking_time: Some(100),
result_id: "minecraft:lapis_lazuli",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:blackstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:blackstone"]),
].into_iter().collect(),
result_id: "minecraft:blackstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:stone_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:stone_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:resin_brick", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:resin_clump",
],
cooking_time: Some(200),
result_id: "minecraft:resin_brick",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:cherry_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_blasting_copper_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:copper_ore",
],
cooking_time: Some(100),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:cooked_beef", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:beef",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_beef",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_copper_trapdoor_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_trapdoor"),
ingredients: vec![
	vec!["minecraft:copper_trapdoor"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_slab_from_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_copper",
],
result_id: "minecraft:weathered_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:purple_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:purple_dye"]),
].into_iter().collect(),
result_id: "minecraft:purple_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:blue_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:blue_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:blue_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:stone"]),
].into_iter().collect(),
result_id: "minecraft:stone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:tuff_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:polished_tuff"]),
].into_iter().collect(),
result_id: "minecraft:tuff_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:jukebox", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("X", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:jukebox",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_slab_from_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_cut_copper",
],
result_id: "minecraft:exposed_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:lead", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"~~ ",
	"~~ ",
	"  ~",
],
key: vec![
	("~", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:lead",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:bricks"]),
].into_iter().collect(),
result_id: "minecraft:brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:magenta_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:magenta_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:magenta_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magma_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:magma_cream"]),
].into_iter().collect(),
result_id: "minecraft:magma_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_chiseled_copper_from_waxed_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_cut_copper",
],
result_id: "minecraft:waxed_exposed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_from_smelting_diamond_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("diamond"),
ingredient: vec![
	"minecraft:diamond_ore",
],
cooking_time: Some(200),
result_id: "minecraft:diamond",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:cut_red_sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:red_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:cut_red_sandstone",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:mangrove_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:mangrove_logs"],
],
result_id: "minecraft:mangrove_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:brown_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("brown_dye"),
ingredients: vec![
	vec!["minecraft:cocoa_beans"],
],
result_id: "minecraft:brown_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:shaper_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:shaper_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:tuff_bricks_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:acacia_logs"],
],
result_id: "minecraft:acacia_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:oak_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:jungle_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_copper_lantern_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_lantern"),
ingredients: vec![
	vec!["minecraft:exposed_copper_lantern"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:diamond_block"],
],
result_id: "minecraft:diamond",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:composter", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	"# #",
	"###",
],
key: vec![
	("#", vec!["#minecraft:wooden_slabs"]),
].into_iter().collect(),
result_id: "minecraft:composter",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:lime_dye"],
],
result_id: "minecraft:lime_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_grate_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_grate"),
ingredients: vec![
	vec!["minecraft:copper_grate"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_grate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:stone_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:stone_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_dye_from_cactus_flower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("pink_dye"),
ingredients: vec![
	vec!["minecraft:cactus_flower"],
],
result_id: "minecraft:pink_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_stairs_from_end_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone_bricks",
],
result_id: "minecraft:end_stone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:cooked_porkchop_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:porkchop",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_porkchop",
result_components: None,
})));
	output.insert("minecraft:smooth_sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:smooth_sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dark_oak_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:dye_purple_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:purple_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:purple_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:warped_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cartography_table", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"@@",
	"##",
	"##",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("@", vec!["minecraft:paper"]),
].into_iter().collect(),
result_id: "minecraft:cartography_table",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_smelting_raw_copper", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:raw_copper",
],
cooking_time: Some(200),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:grindstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"I-I",
	"# #",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("-", vec!["minecraft:stone_slab"]),
	("I", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:grindstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_sandstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:smooth_sandstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_stairs_from_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_cut_copper",
],
result_id: "minecraft:weathered_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_dye_from_peony", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("pink_dye"),
ingredients: vec![
	vec!["minecraft:peony"],
],
result_id: "minecraft:pink_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:quartz_slab_from_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:quartz_block",
],
result_id: "minecraft:quartz_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:deepslate_tiles", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:deepslate_bricks"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_tiles",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:exposed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:exposed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dye_cyan_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:cyan_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:cyan_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:flint_and_steel", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: None,
ingredients: vec![
	vec!["minecraft:iron_ingot"],
	vec!["minecraft:flint"],
],
result_id: "minecraft:flint_and_steel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mud_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:mud_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mud_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:ladder", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	"###",
	"# #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:ladder",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:black_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:black_dye",
],
result_id: "minecraft:black_shulker_box",
})));
	output.insert("minecraft:quartz_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:quartz_block"]),
].into_iter().collect(),
result_id: "minecraft:quartz_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_chiseled_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_chiseled_copper"),
ingredients: vec![
	vec!["minecraft:exposed_chiseled_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crimson_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cyan_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:cyan_dye"]),
].into_iter().collect(),
result_id: "minecraft:cyan_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:crimson_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:light_blue_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:light_blue_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:light_blue_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:pale_oak_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_bricks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
	vec!["minecraft:prismarine_shard"],
],
result_id: "minecraft:prismarine_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_grate_from_waxed_weathered_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_copper",
],
result_id: "minecraft:waxed_weathered_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cut_copper_slab_from_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:copper_block",
],
result_id: "minecraft:cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:end_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:end_stone_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:orange_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:orange_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:orange_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:cooked_salmon_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:salmon",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_salmon",
result_components: None,
})));
	output.insert("minecraft:painting", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:wool"]),
].into_iter().collect(),
result_id: "minecraft:painting",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oak_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:lime_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:lime_wool"]),
].into_iter().collect(),
result_id: "minecraft:lime_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:golden_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:gold_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:golden_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:warped_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:warped_stems"],
],
result_id: "minecraft:warped_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:pink_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:pink_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:pink_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:resin_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:resin_bricks"]),
].into_iter().collect(),
result_id: "minecraft:resin_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:pumpkin_pie", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:pumpkin"],
	vec!["minecraft:sugar"],
	vec!["#minecraft:eggs"],
],
result_id: "minecraft:pumpkin_pie",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_ingot_from_netherite_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("netherite_ingot"),
ingredients: vec![
	vec!["minecraft:netherite_block"],
],
result_id: "minecraft:netherite_ingot",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:netherite_helmet_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_helmet",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_helmet",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:dripstone_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:pointed_dripstone"]),
].into_iter().collect(),
result_id: "minecraft:dripstone_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chest_minecart", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:minecart"],
],
result_id: "minecraft:chest_minecart",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_yellow_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:yellow_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:yellow_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:pink_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:pink_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:polished_basalt", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:basalt"]),
].into_iter().collect(),
result_id: "minecraft:polished_basalt",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_stairs_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crafting_table", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: Some(false),
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:crafting_table",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_chest_raft", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:bamboo_raft"],
],
result_id: "minecraft:bamboo_chest_raft",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_scrap", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:ancient_debris",
],
cooking_time: Some(200),
result_id: "minecraft:netherite_scrap",
result_components: None,
experience: Some(2.0),
})));
	output.insert("minecraft:oxidized_cut_copper_stairs_from_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_copper",
],
result_id: "minecraft:oxidized_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:netherite_axe_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_axe",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_axe",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:copper_boots", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_boots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:prismarine_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:prismarine"]),
].into_iter().collect(),
result_id: "minecraft:prismarine_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:spruce_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_spruce_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:spruce_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:weathered_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:weathered_cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:black_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:black_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:black_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:repair_item", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Repairitem,
category: Some(Category::Misc),
}));
	output.insert("minecraft:banner_duplicate", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Bannerduplicate,
category: Some(Category::Misc),
}));
	output.insert("minecraft:gray_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:gray_wool"]),
].into_iter().collect(),
result_id: "minecraft:gray_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:emerald_from_smelting_deepslate_emerald_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("emerald"),
ingredient: vec![
	"minecraft:deepslate_emerald_ore",
],
cooking_time: Some(200),
result_id: "minecraft:emerald",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:chiseled_bookshelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"XXX",
	"###",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("X", vec!["#minecraft:wooden_slabs"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_bookshelf",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:polished_deepslate_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mud_brick_slab_from_mud_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mud_bricks",
],
result_id: "minecraft:mud_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dark_oak_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:light_gray_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_bricks_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_lantern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X#X",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:copper_torch"]),
	("X", vec!["minecraft:copper_nugget"]),
].into_iter().collect(),
result_id: "minecraft:copper_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_slab_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:birch_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_black_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec![
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:black_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_blasting_raw_gold", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:raw_gold",
],
cooking_time: Some(100),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:polished_tuff_wall_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:polished_tuff_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spire_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:purpur_block"]),
	("S", vec!["minecraft:spire_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:spire_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_sandstone_slab_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:red_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:polished_diorite", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:diorite"]),
].into_iter().collect(),
result_id: "minecraft:polished_diorite",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:deepslate_bricks_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:light_gray_wool"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:brown_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:brown_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:brown_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:armor_dye", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Armordye,
category: Some(Category::Misc),
}));
	output.insert("minecraft:copper_chain", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"N",
	"I",
	"N",
],
key: vec![
	("I", vec!["minecraft:copper_ingot"]),
	("N", vec!["minecraft:copper_nugget"]),
].into_iter().collect(),
result_id: "minecraft:copper_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_pink_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:pink_tulip"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_wall_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:polished_tuff_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:raw_copper_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:raw_copper"]),
].into_iter().collect(),
result_id: "minecraft:raw_copper_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_rabbit", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:rabbit",
],
cooking_time: Some(200),
result_id: "minecraft:cooked_rabbit",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:deepslate_tiles_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_tiles",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:nether_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_brick_stairs_from_mossy_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_stone_bricks",
],
result_id: "minecraft:mossy_stone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:yellow_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:yellow_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_nuggets", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:iron_nugget"]),
].into_iter().collect(),
result_id: "minecraft:iron_ingot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diorite_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:diorite"]),
].into_iter().collect(),
result_id: "minecraft:diorite_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:enchanting_table", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" B ",
	"D#D",
	"###",
],
key: vec![
	("#", vec!["minecraft:obsidian"]),
	("B", vec!["minecraft:book"]),
	("D", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:enchanting_table",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("blue_dye"),
ingredients: vec![
	vec!["minecraft:lapis_lazuli"],
],
result_id: "minecraft:blue_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
].into_iter().collect(),
result_id: "minecraft:spruce_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sandstone_stairs_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:sandstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_stairs_from_red_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_nether_bricks",
],
result_id: "minecraft:red_nether_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:polished_tuff",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_oak_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:dark_oak_boat"],
],
result_id: "minecraft:dark_oak_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_wool_from_string", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:white_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:rabbit_stew_from_red_mushroom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("rabbit_stew"),
ingredients: vec![
	vec!["minecraft:baked_potato"],
	vec!["minecraft:cooked_rabbit"],
	vec!["minecraft:bowl"],
	vec!["minecraft:carrot"],
	vec!["minecraft:red_mushroom"],
],
result_id: "minecraft:rabbit_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:pale_oak_planks"],
],
result_id: "minecraft:pale_oak_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:orange_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:orange_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:spruce_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:spruce_boat"],
],
result_id: "minecraft:spruce_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:red_dye",
],
result_id: "minecraft:red_bundle",
})));
	output.insert("minecraft:cut_copper_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:cut_copper_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:polished_blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cyan_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:cyan_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:cyan_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:raiser_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:terracotta"]),
	("S", vec!["minecraft:raiser_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:raiser_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:exposed_chiseled_copper_from_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_cut_copper",
],
result_id: "minecraft:exposed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:pink_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:pink_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_tuff"]),
].into_iter().collect(),
result_id: "minecraft:polished_tuff_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_orange_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:orange_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:orange_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:prismarine_shard"]),
].into_iter().collect(),
result_id: "minecraft:prismarine",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:green_wool"]),
].into_iter().collect(),
result_id: "minecraft:green_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:crossbow", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"#&#",
	"~$~",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("$", vec!["minecraft:tripwire_hook"]),
	("&", vec!["minecraft:iron_ingot"]),
	("~", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:crossbow",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_chestplate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"XXX",
],
key: vec![
	("X", vec!["minecraft:gold_ingot"]),
].into_iter().collect(),
result_id: "minecraft:golden_chestplate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_sword", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:iron_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:iron_sword",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:flower_pot", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# #",
	" # ",
],
key: vec![
	("#", vec!["minecraft:brick"]),
].into_iter().collect(),
result_id: "minecraft:flower_pot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:leather", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:rabbit_hide"]),
].into_iter().collect(),
result_id: "minecraft:leather",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tipped_arrow", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Tippedarrow,
category: Some(Category::Misc),
}));
	output.insert("minecraft:smooth_red_sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_red_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:smooth_red_sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:purple_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:purple_dye"]),
].into_iter().collect(),
result_id: "minecraft:purple_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:iron_leggings", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_leggings",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:yellow_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:yellow_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_stairs_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:polished_tuff_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cracked_polished_blackstone_bricks", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:polished_blackstone_bricks",
],
cooking_time: Some(200),
result_id: "minecraft:cracked_polished_blackstone_bricks",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:waxed_weathered_cut_copper_slab_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper_slab"),
ingredients: vec![
	vec!["minecraft:weathered_cut_copper_slab"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_cut_copper_slab",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:gray_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:activator_rail", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"XSX",
	"X#X",
	"XSX",
],
key: vec![
	("#", vec!["minecraft:redstone_torch"]),
	("S", vec!["minecraft:stick"]),
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:activator_rail",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:rib_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:netherrack"]),
	("S", vec!["minecraft:rib_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:rib_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:mangrove_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_slab_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:polished_diorite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cherry_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:cherry_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:cracked_deepslate_tiles", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:deepslate_tiles",
],
cooking_time: Some(200),
result_id: "minecraft:cracked_deepslate_tiles",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:blackstone_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:blackstone"]),
].into_iter().collect(),
result_id: "minecraft:blackstone_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:oak_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:oak_boat"],
],
result_id: "minecraft:oak_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_closed_eyeblossom", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:closed_eyeblossom"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:pink_dye",
],
result_id: "minecraft:pink_shulker_box",
})));
	output.insert("minecraft:gold_ingot_from_smelting_gold_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:gold_ore",
],
cooking_time: Some(200),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:crimson_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:ward_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:cobbled_deepslate"]),
	("S", vec!["minecraft:ward_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:ward_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:chiseled_tuff_bricks_from_tuff_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff_bricks",
],
result_id: "minecraft:chiseled_tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:resin_clump"]),
].into_iter().collect(),
result_id: "minecraft:resin_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:exposed_copper"]),
].into_iter().collect(),
result_id: "minecraft:exposed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cyan_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:cyan_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:cyan_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobblestone_slab_from_cobblestone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobblestone",
],
result_id: "minecraft:cobblestone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_pink_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:pink_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:pink_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:quartz_pillar", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:quartz_block"]),
].into_iter().collect(),
result_id: "minecraft:quartz_pillar",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_stairs_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_tile_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diorite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:diorite"]),
].into_iter().collect(),
result_id: "minecraft:diorite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:firework_rocket_simple", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:gunpowder"],
	vec!["minecraft:paper"],
],
result_id: "minecraft:firework_rocket",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:cooked_cod_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:cod",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_cod",
result_components: None,
})));
	output.insert("minecraft:copper_nugget_from_smelting", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:copper_pickaxe",
	"minecraft:copper_shovel",
	"minecraft:copper_axe",
	"minecraft:copper_hoe",
	"minecraft:copper_sword",
	"minecraft:copper_helmet",
	"minecraft:copper_chestplate",
	"minecraft:copper_leggings",
	"minecraft:copper_boots",
	"minecraft:copper_horse_armor",
],
cooking_time: Some(200),
result_id: "minecraft:copper_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:tuff_brick_stairs_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wayfinder_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:wayfinder_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:light_gray_dye_from_gray_white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_gray_dye"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:light_gray_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:soul_campfire", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" S ",
	"S#S",
	"LLL",
],
key: vec![
	("#", vec!["#minecraft:soul_fire_base_blocks"]),
	("L", vec!["#minecraft:logs"]),
	("S", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:soul_campfire",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:stone_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_stairs_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:polished_andesite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_resin_bricks_from_resin_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:resin_bricks",
],
result_id: "minecraft:chiseled_resin_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:resin_brick"]),
].into_iter().collect(),
result_id: "minecraft:resin_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:diamond_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:diamond_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_pickaxe_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_pickaxe",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_pickaxe",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:magenta_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:magenta_dye",
],
result_id: "minecraft:magenta_bundle",
})));
	output.insert("minecraft:cut_red_sandstone_slab_from_cut_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cut_red_sandstone",
],
result_id: "minecraft:cut_red_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:copper_block"]),
].into_iter().collect(),
result_id: "minecraft:copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:bamboo_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_gray_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:light_gray_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:jungle_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_jungle_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:jungle_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:prismarine_brick_slab_from_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:prismarine_bricks",
],
result_id: "minecraft:prismarine_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_cut_copper_chiseled"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:waxed_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:waxed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:jungle_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_wall_from_deepslate_tiles_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_tiles",
],
result_id: "minecraft:deepslate_tile_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_from_vine", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("mossy_cobblestone"),
ingredients: vec![
	vec!["minecraft:cobblestone"],
	vec!["minecraft:vine"],
],
result_id: "minecraft:mossy_cobblestone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_andesite"]),
].into_iter().collect(),
result_id: "minecraft:polished_andesite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:pink_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:pink_dye"]),
].into_iter().collect(),
result_id: "minecraft:pink_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper_chiseled"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:waxed_oxidized_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_stairs_from_waxed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_cut_copper",
],
result_id: "minecraft:waxed_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lapis_lazuli_from_smelting_deepslate_lapis_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("lapis_lazuli"),
ingredient: vec![
	"minecraft:deepslate_lapis_ore",
],
cooking_time: Some(200),
result_id: "minecraft:lapis_lazuli",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:jungle_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:jungle_boat"],
],
result_id: "minecraft:jungle_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:oxidized_copper"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_nuggets", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:gold_nugget"]),
].into_iter().collect(),
result_id: "minecraft:gold_ingot",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:redstone_from_blasting_redstone_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: Some("redstone"),
ingredient: vec![
	"minecraft:redstone_ore",
],
cooking_time: Some(100),
result_id: "minecraft:redstone",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:dye_blue_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:blue_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:redstone_from_blasting_deepslate_redstone_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: Some("redstone"),
ingredient: vec![
	"minecraft:deepslate_redstone_ore",
],
cooking_time: Some(100),
result_id: "minecraft:redstone",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:melon_seeds", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:melon_slice"],
],
result_id: "minecraft:melon_seeds",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_brick_slab_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:resin_clump", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:resin_block"],
],
result_id: "minecraft:resin_clump",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:glistering_melon_slice", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:gold_nugget"]),
	("X", vec!["minecraft:melon_slice"]),
].into_iter().collect(),
result_id: "minecraft:glistering_melon_slice",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dark_prismarine_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_prismarine"]),
].into_iter().collect(),
result_id: "minecraft:dark_prismarine_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_weathered_cut_copper_chiseled"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:waxed_weathered_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:waxed_weathered_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:vex_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:cobblestone"]),
	("S", vec!["minecraft:vex_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:vex_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:chiseled_copper_from_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cut_copper",
],
result_id: "minecraft:chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_basalt_from_basalt_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:basalt",
],
result_id: "minecraft:polished_basalt",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:raw_iron", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:raw_iron_block"],
],
result_id: "minecraft:raw_iron",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:end_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:end_stone_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_blue_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:blue_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_lightning_rod_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_lightning_rod"),
ingredients: vec![
	vec!["minecraft:weathered_lightning_rod"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_lightning_rod",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:charcoal", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"#minecraft:logs_that_burn",
],
cooking_time: Some(200),
result_id: "minecraft:charcoal",
result_components: None,
experience: Some(0.15),
})));
	output.insert("minecraft:hay_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
	vec!["minecraft:wheat"],
],
result_id: "minecraft:hay_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crimson_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:crimson_planks"]),
].into_iter().collect(),
result_id: "minecraft:crimson_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:weathered_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:weathered_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:weathered_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:lime_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:lime_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:lime_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:dried_ghast", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("dry_ghast"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:ghast_tear"]),
	("X", vec!["minecraft:soul_sand"]),
].into_iter().collect(),
result_id: "minecraft:dried_ghast",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cherry_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:cherry_log"]),
].into_iter().collect(),
result_id: "minecraft:cherry_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:pink_dye_from_pink_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("pink_dye"),
ingredients: vec![
	vec!["minecraft:pink_tulip"],
],
result_id: "minecraft:pink_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:cyan_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:cyan_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:chiseled_copper_from_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:copper_block",
],
result_id: "minecraft:chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:light_blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:white_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:white_dye"]),
].into_iter().collect(),
result_id: "minecraft:white_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:dye_cyan_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:cyan_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:cyan_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredients: vec![
	vec!["minecraft:copper_block"],
],
result_id: "minecraft:copper_ingot",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:copper_block"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:pink_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:pink_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:pink_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:copper_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:copper_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:prismarine"]),
].into_iter().collect(),
result_id: "minecraft:prismarine_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_light_blue_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:light_blue_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:light_blue_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_spruce_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_spruce_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_spruce_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_copper_bulb_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_bulb"),
ingredients: vec![
	vec!["minecraft:weathered_copper_bulb"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_bulb",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:polished_diorite",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_tuff_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:tuff_brick_slab"]),
].into_iter().collect(),
result_id: "minecraft:chiseled_tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:saddle", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" X ",
	"X#X",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:saddle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:snow", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:snow_block"]),
].into_iter().collect(),
result_id: "minecraft:snow",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cake", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"AAA",
	"BEB",
	"CCC",
],
key: vec![
	("A", vec!["minecraft:milk_bucket"]),
	("B", vec!["minecraft:sugar"]),
	("C", vec!["minecraft:wheat"]),
	("E", vec!["#minecraft:eggs"]),
].into_iter().collect(),
result_id: "minecraft:cake",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_magenta_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:magenta_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:magenta_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:lime_dye"]),
].into_iter().collect(),
result_id: "minecraft:lime_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:orange_dye_from_red_yellow", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("orange_dye"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec!["minecraft:yellow_dye"],
],
result_id: "minecraft:orange_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:black_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:black_dye",
],
result_id: "minecraft:black_bundle",
})));
	output.insert("minecraft:yellow_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:yellow_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:yellow_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:green_dye"],
],
result_id: "minecraft:green_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mangrove_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:iron_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:iron_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_moss_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:pale_moss_block"]),
].into_iter().collect(),
result_id: "minecraft:pale_moss_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:exposed_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:exposed_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:exposed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_slab_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_tile_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:birch_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_fence"),
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_fence",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:red_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:red_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:fermented_spider_eye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:spider_eye"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:sugar"],
],
result_id: "minecraft:fermented_spider_eye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:leaf_litter", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"#minecraft:leaves",
],
cooking_time: Some(200),
result_id: "minecraft:leaf_litter",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:dye_brown_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:brown_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:brown_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:jungle_log"]),
].into_iter().collect(),
result_id: "minecraft:jungle_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:wind_charge", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:breeze_rod"],
],
result_id: "minecraft:wind_charge",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:polished_deepslate_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:blue_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:blue_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:blue_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:cooked_mutton_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:mutton",
],
cooking_time: Some(600),
result_id: "minecraft:cooked_mutton",
result_components: None,
})));
	output.insert("minecraft:yellow_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:yellow_dye",
],
result_id: "minecraft:yellow_bundle",
})));
	output.insert("minecraft:waxed_exposed_copper_bars_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_copper_bars"),
ingredients: vec![
	vec!["minecraft:exposed_copper_bars"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_copper_bars",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pink_dye_from_red_white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("pink_dye"),
ingredients: vec![
	vec!["minecraft:red_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:pink_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dark_oak_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:dark_oak_planks"],
],
result_id: "minecraft:dark_oak_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_red_sandstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_red_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:smooth_red_sandstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:mud_brick_wall_from_mud_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mud_bricks",
],
result_id: "minecraft:mud_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
].into_iter().collect(),
result_id: "minecraft:glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:dye_blue_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:blue_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_chain_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_chain"),
ingredients: vec![
	vec!["minecraft:copper_chain"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_chain",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cherry_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:iron_ingot_from_smelting_iron_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("iron_ingot"),
ingredient: vec![
	"minecraft:iron_ore",
],
cooking_time: Some(200),
result_id: "minecraft:iron_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:chiseled_deepslate_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:chiseled_deepslate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:orange_dye",
],
result_id: "minecraft:orange_shulker_box",
})));
	output.insert("minecraft:wild_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:mossy_cobblestone"]),
	("S", vec!["minecraft:wild_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:wild_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:light_gray_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:light_gray_dye",
],
result_id: "minecraft:light_gray_shulker_box",
})));
	output.insert("minecraft:firework_star", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::FireworkStar,
category: Some(Category::Misc),
}));
	output.insert("minecraft:leather_leggings", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:leather_leggings",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_dye_from_lilac", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("magenta_dye"),
ingredients: vec![
	vec!["minecraft:lilac"],
],
result_id: "minecraft:magenta_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:sandstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:sand"]),
].into_iter().collect(),
result_id: "minecraft:sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:wild_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:wild_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:diorite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:diorite"]),
].into_iter().collect(),
result_id: "minecraft:diorite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cyan_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("cyan_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:green_dye"],
],
result_id: "minecraft:cyan_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cut_sandstone_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:cut_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:spruce_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:spruce_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_chest_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_chest"),
ingredients: vec![
	vec!["minecraft:copper_chest"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_chest",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dried_kelp_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:kelp",
],
cooking_time: Some(100),
result_id: "minecraft:dried_kelp",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:raw_copper", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:raw_copper_block"],
],
result_id: "minecraft:raw_copper",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:stick_from_bamboo_item", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("sticks"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:bamboo"]),
].into_iter().collect(),
result_id: "minecraft:stick",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:beacon", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"GGG",
	"GSG",
	"OOO",
],
key: vec![
	("G", vec!["minecraft:glass"]),
	("O", vec!["minecraft:obsidian"]),
	("S", vec!["minecraft:nether_star"]),
].into_iter().collect(),
result_id: "minecraft:beacon",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:orange_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:orange_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:orange_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:brick"]),
].into_iter().collect(),
result_id: "minecraft:bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brewing_stand", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" B ",
	"###",
],
key: vec![
	("#", vec!["#minecraft:stone_crafting_materials"]),
	("B", vec!["minecraft:blaze_rod"]),
].into_iter().collect(),
result_id: "minecraft:brewing_stand",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:magenta_dye"],
],
result_id: "minecraft:magenta_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_granite_slab_from_polished_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_granite",
],
result_id: "minecraft:polished_granite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:lapis_lazuli_from_blasting_lapis_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("lapis_lazuli"),
ingredient: vec![
	"minecraft:lapis_ore",
],
cooking_time: Some(100),
result_id: "minecraft:lapis_lazuli",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:deepslate_tile_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_tiles"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_tile_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:granite_slab_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:granite_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:black_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:black_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:black_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_brown_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:brown_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:brown_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:furnace", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"# #",
	"###",
],
key: vec![
	("#", vec!["#minecraft:stone_crafting_materials"]),
].into_iter().collect(),
result_id: "minecraft:furnace",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:sandstone_slab_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:bordure_indented_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:vine"],
],
result_id: "minecraft:bordure_indented_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_yellow_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:yellow_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:yellow_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cobbled_deepslate_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:cobbled_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:cobbled_deepslate_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mangrove_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:copper_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_brick_slab_from_mossy_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_stone_bricks",
],
result_id: "minecraft:mossy_stone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_slab_from_polished_blackstone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone_bricks",
],
result_id: "minecraft:polished_blackstone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:smooth_stone", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:stone",
],
cooking_time: Some(200),
result_id: "minecraft:smooth_stone",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:polished_deepslate_stairs_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:polished_deepslate_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_dandelion", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:dandelion"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cooked_rabbit_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:rabbit",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_rabbit",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:waxed_chiseled_copper_from_waxed_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_copper_block",
],
result_id: "minecraft:waxed_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:bread", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:wheat"]),
].into_iter().collect(),
result_id: "minecraft:bread",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:emerald_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:emerald"]),
].into_iter().collect(),
result_id: "minecraft:emerald_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_deepslate_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:polished_deepslate_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:white_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:white_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:white_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:snow_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:snowball"]),
].into_iter().collect(),
result_id: "minecraft:snow_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oxidized_chiseled_copper_from_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_copper",
],
result_id: "minecraft:oxidized_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cyan_dye_from_pitcher_plant", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("cyan_dye"),
ingredients: vec![
	vec!["minecraft:pitcher_plant"],
],
result_id: "minecraft:cyan_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:red_nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:red_nether_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:brick_slab_from_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:bricks",
],
result_id: "minecraft:brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_grate_from_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:copper_block",
],
result_id: "minecraft:copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:light_blue_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:light_blue_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:quartz_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:quartz"]),
].into_iter().collect(),
result_id: "minecraft:quartz_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_brick_wall_from_mossy_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mossy_stone_bricks",
],
result_id: "minecraft:mossy_stone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:lime_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:lime_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:copper_ingot"]),
].into_iter().collect(),
result_id: "minecraft:copper_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:exposed_copper_grate_from_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_copper",
],
result_id: "minecraft:exposed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:pale_oak_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_pale_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_orange_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:orange_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:orange_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:raw_gold", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:raw_gold_block"],
],
result_id: "minecraft:raw_gold",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_cornflower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:cornflower"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:light_gray_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:light_gray_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:light_blue_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:quartz_stairs_from_quartz_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:quartz_block",
],
result_id: "minecraft:quartz_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_stairs_from_nether_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:nether_bricks",
],
result_id: "minecraft:nether_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:netherite_leggings_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_leggings",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_leggings",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_stairs_from_waxed_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_copper",
],
result_id: "minecraft:waxed_exposed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:warped_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_warped_stem"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:warped_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_magenta_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:magenta_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:magenta_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_slab_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_smelting_copper_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:copper_ore",
],
cooking_time: Some(200),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:purpur_stairs_from_purpur_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:purpur_block",
],
result_id: "minecraft:purpur_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coal_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:coal"]),
].into_iter().collect(),
result_id: "minecraft:coal_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_lantern_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_lantern"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_lantern"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_lantern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:birch_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_birch_log"]),
].into_iter().collect(),
result_id: "minecraft:birch_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:red_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:red_dye"],
],
result_id: "minecraft:red_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:hopper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"I I",
	"ICI",
	" I ",
],
key: vec![
	("C", vec!["minecraft:chest"]),
	("I", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:hopper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:white_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:white_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:yellow_dye"]),
].into_iter().collect(),
result_id: "minecraft:yellow_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:gray_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:gray_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:gray_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:black_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:black_wool"]),
].into_iter().collect(),
result_id: "minecraft:black_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:iron_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:red_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:red_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:red_harness", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: Some("harness"),
show_notification: None,
pattern: vec![
	"LLL",
	"G#G",
],
key: vec![
	("#", vec!["minecraft:red_wool"]),
	("G", vec!["minecraft:glass"]),
	("L", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:red_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("wooden_sign"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" X ",
],
key: vec![
	("#", vec!["minecraft:pale_oak_planks"]),
	("X", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_sign",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:oxidized_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:oxidized_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:lectern", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"SSS",
	" B ",
	" S ",
],
key: vec![
	("B", vec!["minecraft:bookshelf"]),
	("S", vec!["#minecraft:wooden_slabs"]),
].into_iter().collect(),
result_id: "minecraft:lectern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coal_from_smelting_deepslate_coal_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("coal"),
ingredient: vec![
	"minecraft:deepslate_coal_ore",
],
cooking_time: Some(200),
result_id: "minecraft:coal",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:purpur_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:purpur_block",
		"minecraft:purpur_pillar",
	]),
].into_iter().collect(),
result_id: "minecraft:purpur_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:oxidized_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:oxidized_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:oxidized_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bow", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" #X",
	"# X",
	" #X",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:string"]),
].into_iter().collect(),
result_id: "minecraft:bow",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brick_stairs_from_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:bricks",
],
result_id: "minecraft:brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:lime_dye"]),
].into_iter().collect(),
result_id: "minecraft:lime_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:baked_potato_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:potato",
],
cooking_time: Some(600),
result_id: "minecraft:baked_potato",
result_components: None,
})));
	output.insert("minecraft:blue_dye_from_cornflower", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("blue_dye"),
ingredients: vec![
	vec!["minecraft:cornflower"],
],
result_id: "minecraft:blue_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_blasting_raw_copper", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredient: vec![
	"minecraft:raw_copper",
],
cooking_time: Some(100),
result_id: "minecraft:copper_ingot",
result_components: None,
experience: Some(0.7),
})));
	output.insert("minecraft:mangrove_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_quartz", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:quartz_block",
],
cooking_time: Some(200),
result_id: "minecraft:smooth_quartz",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:redstone", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: None,
ingredients: vec![
	vec!["minecraft:redstone_block"],
],
result_id: "minecraft:redstone",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:vex_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:vex_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:waxed_oxidized_cut_copper_slab_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_cut_copper_slab"),
ingredients: vec![
	vec!["minecraft:oxidized_cut_copper_slab"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_cut_copper_slab",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_chiseled_copper_from_waxed_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_copper",
],
result_id: "minecraft:waxed_exposed_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:yellow_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:yellow_wool"]),
].into_iter().collect(),
result_id: "minecraft:yellow_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:polished_granite_from_granite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:granite",
],
result_id: "minecraft:polished_granite",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_stairs_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:mossy_cobblestone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:light_blue_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:cut_red_sandstone_slab_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:cut_red_sandstone_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:carrot_on_a_stick", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"# ",
	" X",
],
key: vec![
	("#", vec!["minecraft:fishing_rod"]),
	("X", vec!["minecraft:carrot"]),
].into_iter().collect(),
result_id: "minecraft:carrot_on_a_stick",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:green_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:green_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:andesite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:andesite"]),
].into_iter().collect(),
result_id: "minecraft:andesite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:exposed_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:exposed_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:exposed_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_diorite"]),
].into_iter().collect(),
result_id: "minecraft:polished_diorite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:flow_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:flow_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:granite", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:diorite"],
	vec!["minecraft:quartz"],
],
result_id: "minecraft:granite",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:blue_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:quartz", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:nether_quartz_ore",
],
cooking_time: Some(200),
result_id: "minecraft:quartz",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:weathered_chiseled_copper_from_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:weathered_cut_copper",
],
result_id: "minecraft:weathered_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:purple_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:purple_dye"]),
].into_iter().collect(),
result_id: "minecraft:purple_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:iron_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:iron_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:iron_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_bulb_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_bulb"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_bulb"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_bulb",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:skull_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:wither_skeleton_skull"],
],
result_id: "minecraft:skull_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mojang_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:enchanted_golden_apple"],
],
result_id: "minecraft:mojang_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:brown_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:brown_dye"]),
].into_iter().collect(),
result_id: "minecraft:brown_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:pale_oak_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_pale_oak_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:tinted_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" S ",
	"SGS",
	" S ",
],
key: vec![
	("G", vec!["minecraft:glass"]),
	("S", vec!["minecraft:amethyst_shard"]),
].into_iter().collect(),
result_id: "minecraft:tinted_glass",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_dye_from_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("red_dye"),
ingredients: vec![
	vec!["minecraft:red_tulip"],
],
result_id: "minecraft:red_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:copper_pickaxe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	" # ",
	" # ",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:copper_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:copper_pickaxe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:red_wool"]),
].into_iter().collect(),
result_id: "minecraft:red_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:spectral_arrow", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	"#X#",
	" # ",
],
key: vec![
	("#", vec!["minecraft:glowstone_dust"]),
	("X", vec!["minecraft:arrow"]),
].into_iter().collect(),
result_id: "minecraft:spectral_arrow",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:mangrove_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:mangrove_planks"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_stairs_from_waxed_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_cut_copper",
],
result_id: "minecraft:waxed_oxidized_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:torch", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("{k}", vec![
		"minecraft:coal",
		"minecraft:charcoal",
	]),
].into_iter().collect(),
result_id: "minecraft:torch",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:stripped_cherry_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_cherry_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_cherry_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:green_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:green_dye",
],
result_id: "minecraft:green_bundle",
})));
	output.insert("minecraft:netherite_boots_smithing", Recipe::SmithingTransform(Box::new(SmithingTransformData{
template: vec![
	"minecraft:netherite_upgrade_smithing_template",
],
base: vec![
	"minecraft:diamond_boots",
],
addition: vec![
	"#minecraft:netherite_tool_materials",
],
result_id: "minecraft:netherite_boots",
result_count: None,
result_components: None,
})));
	output.insert("minecraft:cooked_salmon_from_smoking", Recipe::Smoking(Box::new(FurnaceData{
category: Some(Category::Food),
group: None,
ingredient: vec![
	"minecraft:salmon",
],
cooking_time: Some(100),
result_id: "minecraft:cooked_salmon",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:birch_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_birch_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:birch_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:black_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:black_dye"],
],
result_id: "minecraft:black_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:cyan_dye",
],
result_id: "minecraft:cyan_shulker_box",
})));
	output.insert("minecraft:silence_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:cobbled_deepslate"]),
	("S", vec!["minecraft:silence_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:silence_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:green_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:green_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:green_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:pale_oak_fence_gate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_fence_gate"),
show_notification: None,
pattern: vec![
	"#W#",
	"#W#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("W", vec!["minecraft:pale_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_fence_gate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:snout_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:snout_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:dye_white_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:white_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
	],
],
result_id: "minecraft:white_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_brick_stairs_from_resin_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:resin_bricks",
],
result_id: "minecraft:resin_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_tiles"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_tile_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:coal_from_smelting_coal_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("coal"),
ingredient: vec![
	"minecraft:coal_ore",
],
cooking_time: Some(200),
result_id: "minecraft:coal",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:mossy_stone_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mossy_stone_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:birch_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:birch_planks"]),
].into_iter().collect(),
result_id: "minecraft:birch_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:smooth_quartz_stairs_from_smooth_quartz_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_quartz",
],
result_id: "minecraft:smooth_quartz_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:red_sandstone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:red_sandstone",
		"minecraft:chiseled_red_sandstone",
		"minecraft:cut_red_sandstone",
	]),
].into_iter().collect(),
result_id: "minecraft:red_sandstone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:leather_horse_armor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:leather_horse_armor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:spruce_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:spruce_log"]),
].into_iter().collect(),
result_id: "minecraft:spruce_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:detector_rail", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X#X",
	"XRX",
],
key: vec![
	("#", vec!["minecraft:stone_pressure_plate"]),
	("R", vec!["minecraft:redstone"]),
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:detector_rail",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:yellow_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:yellow_dye"],
],
result_id: "minecraft:yellow_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:acacia_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:acacia_boat"],
],
result_id: "minecraft:acacia_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_blue_bed", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("bed"),
show_notification: None,
pattern: vec![
	"###",
	"XXX",
],
key: vec![
	("#", vec!["minecraft:light_blue_wool"]),
	("X", vec!["#minecraft:planks"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mossy_cobblestone_from_moss_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("mossy_cobblestone"),
ingredients: vec![
	vec!["minecraft:cobblestone"],
	vec!["minecraft:moss_block"],
],
result_id: "minecraft:mossy_cobblestone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stonecutter", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	" I ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stone"]),
	("I", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:stonecutter",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:cyan_dye"]),
].into_iter().collect(),
result_id: "minecraft:cyan_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_chiseled_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper_chiseled"),
show_notification: None,
pattern: vec![
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:waxed_exposed_cut_copper_slab"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_slab_from_stone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:stone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:gray_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:gray_dye"]),
].into_iter().collect(),
result_id: "minecraft:gray_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:lime_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:lime_dye",
],
result_id: "minecraft:lime_shulker_box",
})));
	output.insert("minecraft:chiseled_tuff_bricks_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:chiseled_tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:polished_andesite",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:quartz_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("{k}", vec![
		"minecraft:chiseled_quartz_block",
		"minecraft:quartz_block",
		"minecraft:quartz_pillar",
	]),
].into_iter().collect(),
result_id: "minecraft:quartz_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:lime_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:lime_dye"]),
].into_iter().collect(),
result_id: "minecraft:lime_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:golden_apple", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:gold_ingot"]),
	("X", vec!["minecraft:apple"]),
].into_iter().collect(),
result_id: "minecraft:golden_apple",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_gray_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:gray_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_trapdoor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_diorite_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_diorite"]),
].into_iter().collect(),
result_id: "minecraft:polished_diorite_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_wall_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_tile_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:white_dye"]),
].into_iter().collect(),
result_id: "minecraft:white_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:magenta_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:magenta_wool"]),
].into_iter().collect(),
result_id: "minecraft:magenta_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_cut_copper_from_waxed_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_copper",
],
result_id: "minecraft:waxed_oxidized_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:daylight_detector", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"GGG",
	"QQQ",
	"WWW",
],
key: vec![
	("G", vec!["minecraft:glass"]),
	("Q", vec!["minecraft:quartz"]),
	("W", vec!["#minecraft:wooden_slabs"]),
].into_iter().collect(),
result_id: "minecraft:daylight_detector",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:pale_oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:pale_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:pale_oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_cut_copper_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:waxed_cut_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:rib_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:rib_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:cut_red_sandstone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cut_red_sandstone"]),
].into_iter().collect(),
result_id: "minecraft:cut_red_sandstone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:light_blue_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:light_blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:light_blue_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:rail", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X#X",
	"X X",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:rail",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:wolf_armor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X  ",
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:armadillo_scute"]),
].into_iter().collect(),
result_id: "minecraft:wolf_armor",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:map_extending", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Mapextending,
category: Some(Category::Misc),
}));
	output.insert("minecraft:diamond_from_blasting_diamond_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("diamond"),
ingredient: vec![
	"minecraft:diamond_ore",
],
cooking_time: Some(100),
result_id: "minecraft:diamond",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:stripped_jungle_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_jungle_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_jungle_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:flower_banner_pattern", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:paper"],
	vec!["minecraft:oxeye_daisy"],
],
result_id: "minecraft:flower_banner_pattern",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("white_dye"),
ingredients: vec![
	vec!["minecraft:bone_meal"],
],
result_id: "minecraft:white_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oak_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:oak_logs"],
],
result_id: "minecraft:oak_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:resin_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:resin_bricks"]),
].into_iter().collect(),
result_id: "minecraft:resin_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:diamond_boots", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:diamond_boots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_slab_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper_slab"),
ingredients: vec![
	vec!["minecraft:exposed_cut_copper_slab"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_cut_copper_slab",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:warped_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_stairs"),
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cracked_stone_bricks", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:stone_bricks",
],
cooking_time: Some(200),
result_id: "minecraft:cracked_stone_bricks",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:waxed_weathered_copper_golem_statue_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_copper_golem_statue"),
ingredients: vec![
	vec!["minecraft:weathered_copper_golem_statue"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_copper_golem_statue",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stone"]),
].into_iter().collect(),
result_id: "minecraft:stone_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:copper_torch", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"C",
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("C", vec!["minecraft:copper_nugget"]),
	("{k}", vec![
		"minecraft:coal",
		"minecraft:charcoal",
	]),
].into_iter().collect(),
result_id: "minecraft:copper_torch",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_azure_bluet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:azure_bluet"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:andesite_stairs_from_andesite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:andesite",
],
result_id: "minecraft:andesite_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mud_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:mud_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mud_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:bamboo_raft", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_raft",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tripwire_hook", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"I",
	"S",
	"#",
],
key: vec![
	("#", vec!["#minecraft:planks"]),
	("I", vec!["minecraft:iron_ingot"]),
	("S", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:tripwire_hook",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:oak_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:oak_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:prismarine_wall_from_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:prismarine",
],
result_id: "minecraft:prismarine_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:leather_boots", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:leather_boots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:redstone_torch", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:redstone_torch",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_crimson_hyphae", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_crimson_stem"]),
].into_iter().collect(),
result_id: "minecraft:stripped_crimson_hyphae",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:weathered_copper_bulb", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	" C ",
	"CBC",
	" R ",
],
key: vec![
	("B", vec!["minecraft:blaze_rod"]),
	("C", vec!["minecraft:weathered_copper"]),
	("R", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:weathered_copper_bulb",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cobbled_deepslate_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:cobbled_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:cobbled_deepslate_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:waxed_exposed_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_exposed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:glowstone", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:glowstone_dust"]),
].into_iter().collect(),
result_id: "minecraft:glowstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:shield_decoration", Recipe::CraftingSpecial(CraftingSpecialData{
special_type: CraftingSpecialType::Shielddecoration,
category: Some(Category::Misc),
}));
	output.insert("minecraft:exposed_chiseled_copper_from_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_copper",
],
result_id: "minecraft:exposed_chiseled_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_stairs_from_waxed_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_copper_block",
],
result_id: "minecraft:waxed_cut_copper_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:weathered_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:weathered_copper"]),
].into_iter().collect(),
result_id: "minecraft:weathered_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:light_blue_dye_from_blue_white_dye", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_blue_dye"),
ingredients: vec![
	vec!["minecraft:blue_dye"],
	vec!["minecraft:white_dye"],
],
result_id: "minecraft:light_blue_dye",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cobblestone_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
].into_iter().collect(),
result_id: "minecraft:cobblestone_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:coal", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:coal_block"],
],
result_id: "minecraft:coal",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_door_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_oxidized_copper_door"),
ingredients: vec![
	vec!["minecraft:oxidized_copper_door"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_oxidized_copper_door",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:bamboo_blocks"],
],
result_id: "minecraft:bamboo_planks",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:red_nether_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:red_nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:red_nether_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cherry_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_cut_copper_slab_from_waxed_weathered_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_weathered_cut_copper",
],
result_id: "minecraft:waxed_weathered_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:netherite_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:netherite_ingot"]),
].into_iter().collect(),
result_id: "minecraft:netherite_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_wall_from_polished_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone",
],
result_id: "minecraft:polished_blackstone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_light_blue_carpet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("carpet_dye"),
ingredients: vec![
	vec!["minecraft:light_blue_dye"],
	vec![
		"minecraft:black_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:cyan_carpet",
		"minecraft:gray_carpet",
		"minecraft:green_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:lime_carpet",
		"minecraft:magenta_carpet",
		"minecraft:orange_carpet",
		"minecraft:pink_carpet",
		"minecraft:purple_carpet",
		"minecraft:red_carpet",
		"minecraft:yellow_carpet",
		"minecraft:white_carpet",
	],
],
result_id: "minecraft:light_blue_carpet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gold_ingot_from_blasting_nether_gold_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("gold_ingot"),
ingredient: vec![
	"minecraft:nether_gold_ore",
],
cooking_time: Some(100),
result_id: "minecraft:gold_ingot",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:cherry_chest_boat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("chest_boat"),
ingredients: vec![
	vec!["minecraft:chest"],
	vec!["minecraft:cherry_boat"],
],
result_id: "minecraft:cherry_chest_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_wall_from_polished_blackstone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone_bricks",
],
result_id: "minecraft:polished_blackstone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:lime_dye",
],
result_id: "minecraft:lime_bundle",
})));
	output.insert("minecraft:white_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:white_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:white_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:anvil", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"III",
	" i ",
	"iii",
],
key: vec![
	("I", vec!["minecraft:iron_block"]),
	("i", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:anvil",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:smooth_stone_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:smooth_stone"]),
].into_iter().collect(),
result_id: "minecraft:smooth_stone_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:warped_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:warped_planks"]),
].into_iter().collect(),
result_id: "minecraft:warped_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:host_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:terracotta"]),
	("S", vec!["minecraft:host_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:host_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:oak_hanging_sign", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("hanging_sign"),
show_notification: None,
pattern: vec![
	"X X",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_oak_log"]),
	("X", vec!["minecraft:iron_chain"]),
].into_iter().collect(),
result_id: "minecraft:oak_hanging_sign",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:light_gray_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:light_gray_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:light_gray_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:jungle_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_jungle_log"]),
].into_iter().collect(),
result_id: "minecraft:jungle_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:chiseled_quartz_block_from_quartz_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:quartz_block",
],
result_id: "minecraft:chiseled_quartz_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_weathered_chiseled_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_weathered_chiseled_copper"),
ingredients: vec![
	vec!["minecraft:weathered_chiseled_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_weathered_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:map", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:paper"]),
	("X", vec!["minecraft:compass"]),
].into_iter().collect(),
result_id: "minecraft:map",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:item_frame", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["minecraft:leather"]),
].into_iter().collect(),
result_id: "minecraft:item_frame",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:light_gray_dye",
],
result_id: "minecraft:light_gray_bundle",
})));
	output.insert("minecraft:waxed_exposed_cut_copper_stairs_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper_stairs"),
ingredients: vec![
	vec!["minecraft:exposed_cut_copper_stairs"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cut_copper", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:copper_block"]),
].into_iter().collect(),
result_id: "minecraft:cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:gold_nugget_from_blasting", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:golden_pickaxe",
	"minecraft:golden_shovel",
	"minecraft:golden_axe",
	"minecraft:golden_hoe",
	"minecraft:golden_sword",
	"minecraft:golden_helmet",
	"minecraft:golden_chestplate",
	"minecraft:golden_leggings",
	"minecraft:golden_boots",
	"minecraft:golden_horse_armor",
],
cooking_time: Some(100),
result_id: "minecraft:gold_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:oxidized_chiseled_copper_from_oxidized_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_cut_copper",
],
result_id: "minecraft:oxidized_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:gray_bundle", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Equipment),
group: Some("bundle_dye"),
input: vec![
	"#minecraft:bundles",
],
material: vec![
	"minecraft:gray_dye",
],
result_id: "minecraft:gray_bundle",
})));
	output.insert("minecraft:sponge", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:wet_sponge",
],
cooking_time: Some(200),
result_id: "minecraft:sponge",
result_components: None,
experience: Some(0.15),
})));
	output.insert("minecraft:orange_dye_from_orange_tulip", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("orange_dye"),
ingredients: vec![
	vec!["minecraft:orange_tulip"],
],
result_id: "minecraft:orange_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_wall_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_copper_bars_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_copper_bars"),
ingredients: vec![
	vec!["minecraft:copper_bars"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_copper_bars",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:cyan_candle", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("dyed_candle"),
ingredients: vec![
	vec!["minecraft:candle"],
	vec!["minecraft:cyan_dye"],
],
result_id: "minecraft:cyan_candle",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_light_gray_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:light_gray_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:pink_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:light_gray_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_white_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:white_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:orange_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
	],
],
result_id: "minecraft:white_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_tuff"]),
].into_iter().collect(),
result_id: "minecraft:polished_tuff_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:jungle_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_door"),
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:conduit", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:nautilus_shell"]),
	("X", vec!["minecraft:heart_of_the_sea"]),
].into_iter().collect(),
result_id: "minecraft:conduit",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stripped_pale_oak_wood", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("bark"),
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:stripped_pale_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:stripped_pale_oak_wood",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_slab_from_waxed_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_cut_copper",
],
result_id: "minecraft:waxed_exposed_cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:muddy_mangrove_roots", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:mud"],
	vec!["minecraft:mangrove_roots"],
],
result_id: "minecraft:muddy_mangrove_roots",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_pink_wool", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("wool"),
ingredients: vec![
	vec!["minecraft:pink_dye"],
	vec![
		"minecraft:black_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:cyan_wool",
		"minecraft:gray_wool",
		"minecraft:green_wool",
		"minecraft:light_blue_wool",
		"minecraft:light_gray_wool",
		"minecraft:lime_wool",
		"minecraft:magenta_wool",
		"minecraft:orange_wool",
		"minecraft:purple_wool",
		"minecraft:red_wool",
		"minecraft:yellow_wool",
		"minecraft:white_wool",
	],
],
result_id: "minecraft:pink_wool",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:coal_from_blasting_deepslate_coal_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("coal"),
ingredient: vec![
	"minecraft:deepslate_coal_ore",
],
cooking_time: Some(100),
result_id: "minecraft:coal",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:deepslate_brick_wall_from_deepslate_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:deepslate_bricks",
],
result_id: "minecraft:deepslate_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"SS",
	"SS",
],
key: vec![
	("S", vec!["minecraft:polished_deepslate"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:iron_door", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:iron_door",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:raw_gold_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:raw_gold"]),
].into_iter().collect(),
result_id: "minecraft:raw_gold_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:andesite_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:andesite"]),
].into_iter().collect(),
result_id: "minecraft:andesite_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_brick_stairs_from_polished_blackstone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_blackstone_bricks",
],
result_id: "minecraft:polished_blackstone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magma_cream", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:blaze_powder"],
	vec!["minecraft:slime_ball"],
],
result_id: "minecraft:magma_cream",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:armor_stand", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"///",
	" / ",
	"/_/",
],
key: vec![
	("/", vec!["minecraft:stick"]),
	("_", vec!["minecraft:smooth_stone_slab"]),
].into_iter().collect(),
result_id: "minecraft:armor_stand",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:iron_nugget_from_smelting", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: None,
ingredient: vec![
	"minecraft:iron_pickaxe",
	"minecraft:iron_shovel",
	"minecraft:iron_axe",
	"minecraft:iron_hoe",
	"minecraft:iron_sword",
	"minecraft:iron_helmet",
	"minecraft:iron_chestplate",
	"minecraft:iron_leggings",
	"minecraft:iron_boots",
	"minecraft:iron_horse_armor",
	"minecraft:chainmail_helmet",
	"minecraft:chainmail_chestplate",
	"minecraft:chainmail_leggings",
	"minecraft:chainmail_boots",
],
cooking_time: Some(200),
result_id: "minecraft:iron_nugget",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:magenta_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:magenta_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:magenta_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:diamond_leggings", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
	"X X",
],
key: vec![
	("X", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:diamond_leggings",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_slab_from_waxed_copper_block_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_copper_block",
],
result_id: "minecraft:waxed_cut_copper_slab",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:exposed_cut_copper_from_exposed_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:exposed_copper",
],
result_id: "minecraft:exposed_cut_copper",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:piston", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"TTT",
	"#X#",
	"#R#",
],
key: vec![
	("#", vec!["minecraft:cobblestone"]),
	("R", vec!["minecraft:redstone"]),
	("T", vec!["#minecraft:planks"]),
	("X", vec!["minecraft:iron_ingot"]),
].into_iter().collect(),
result_id: "minecraft:piston",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:host_armor_trim_smithing_template_smithing_trim", Recipe::SmithingTrim(SmithingTrimData{
template: vec![
	"minecraft:host_armor_trim_smithing_template",
],
base: vec![
	"#minecraft:trimmable_armor",
],
addition: vec![
	"#minecraft:trim_materials",
],
}));
	output.insert("minecraft:cut_copper_slab_from_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cut_copper",
],
result_id: "minecraft:cut_copper_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:dye_gray_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:gray_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:lime_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:gray_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lime_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:lime_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:lime_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:chiseled_sandstone_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:chiseled_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:deepslate_tiles_from_polished_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_deepslate",
],
result_id: "minecraft:deepslate_tiles",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:prismarine_bricks"]),
].into_iter().collect(),
result_id: "minecraft:prismarine_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dark_oak_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:dark_oak_planks"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:acacia_trapdoor", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_trapdoor"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:acacia_planks"]),
].into_iter().collect(),
result_id: "minecraft:acacia_trapdoor",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:golden_hoe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	" #",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:gold_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:golden_hoe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:diamond_from_blasting_deepslate_diamond_ore", Recipe::Blasting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("diamond"),
ingredient: vec![
	"minecraft:deepslate_diamond_ore",
],
cooking_time: Some(100),
result_id: "minecraft:diamond",
result_components: None,
experience: Some(1.0),
})));
	output.insert("minecraft:diamond_helmet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XXX",
	"X X",
],
key: vec![
	("X", vec!["minecraft:diamond"]),
].into_iter().collect(),
result_id: "minecraft:diamond_helmet",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:stone_brick_stairs_from_stone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone_bricks",
],
result_id: "minecraft:stone_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:green_banner", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("banner"),
show_notification: None,
pattern: vec![
	"###",
	"###",
	" | ",
],
key: vec![
	("#", vec!["minecraft:green_wool"]),
	("|", vec!["minecraft:stick"]),
].into_iter().collect(),
result_id: "minecraft:green_banner",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:jungle_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: Some("wooden_pressure_plate"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_bamboo_block"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_grate_from_waxed_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_oxidized_copper",
],
result_id: "minecraft:waxed_oxidized_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:black_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:black_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:black_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:cyan_carpet", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("carpet"),
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:cyan_wool"]),
].into_iter().collect(),
result_id: "minecraft:cyan_carpet",
result_count: Some(3),
result_components: None,
})));
	output.insert("minecraft:dried_kelp_from_campfire_cooking", Recipe::CampfireCooking(Box::new(CampfireCookingData{
ingredient: vec![
	"minecraft:kelp",
],
cooking_time: Some(600),
result_id: "minecraft:dried_kelp",
result_components: None,
})));
	output.insert("minecraft:jungle_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:jungle_planks"]),
].into_iter().collect(),
result_id: "minecraft:jungle_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_shovel", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"X",
	"#",
	"#",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:gold_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:golden_shovel",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:light_gray_dye_from_azure_bluet", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("light_gray_dye"),
ingredients: vec![
	vec!["minecraft:azure_bluet"],
],
result_id: "minecraft:light_gray_dye",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:dye_orange_bed", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("bed_dye"),
ingredients: vec![
	vec!["minecraft:orange_dye"],
	vec![
		"minecraft:black_bed",
		"minecraft:blue_bed",
		"minecraft:brown_bed",
		"minecraft:cyan_bed",
		"minecraft:gray_bed",
		"minecraft:green_bed",
		"minecraft:light_blue_bed",
		"minecraft:light_gray_bed",
		"minecraft:lime_bed",
		"minecraft:magenta_bed",
		"minecraft:pink_bed",
		"minecraft:purple_bed",
		"minecraft:red_bed",
		"minecraft:yellow_bed",
		"minecraft:white_bed",
	],
],
result_id: "minecraft:orange_bed",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:nether_brick_fence", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"W#W",
	"W#W",
],
key: vec![
	("#", vec!["minecraft:nether_brick"]),
	("W", vec!["minecraft:nether_bricks"]),
].into_iter().collect(),
result_id: "minecraft:nether_brick_fence",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:deepslate_tiles_from_cobbled_deepslate_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:cobbled_deepslate",
],
result_id: "minecraft:deepslate_tiles",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_brick_wall_from_resin_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:resin_bricks",
],
result_id: "minecraft:resin_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_oxidized_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:waxed_oxidized_copper"]),
].into_iter().collect(),
result_id: "minecraft:waxed_oxidized_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:repeater", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"#X#",
	"III",
],
key: vec![
	("#", vec!["minecraft:redstone_torch"]),
	("I", vec!["minecraft:stone"]),
	("X", vec!["minecraft:redstone"]),
].into_iter().collect(),
result_id: "minecraft:repeater",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mangrove_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_mangrove_log"]),
].into_iter().collect(),
result_id: "minecraft:mangrove_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:cherry_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:cherry_logs"],
],
result_id: "minecraft:cherry_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:tuff_wall_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:tuff_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tide_armor_trim_smithing_template", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"#S#",
	"#C#",
	"###",
],
key: vec![
	("#", vec!["minecraft:diamond"]),
	("C", vec!["minecraft:prismarine"]),
	("S", vec!["minecraft:tide_armor_trim_smithing_template"]),
].into_iter().collect(),
result_id: "minecraft:tide_armor_trim_smithing_template",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:deepslate_tile_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:deepslate_tiles"]),
].into_iter().collect(),
result_id: "minecraft:deepslate_tile_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:packed_ice", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: None,
ingredients: vec![
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
	vec!["minecraft:ice"],
],
result_id: "minecraft:packed_ice",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_tuff_slab_from_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:tuff",
],
result_id: "minecraft:polished_tuff_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:tuff_bricks_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:tuff_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_cut_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_cut_copper"),
ingredients: vec![
	vec!["minecraft:cut_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_cut_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:clay",
],
cooking_time: Some(200),
result_id: "minecraft:terracotta",
result_components: None,
experience: Some(0.35),
})));
	output.insert("minecraft:polished_blackstone_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mossy_stone_brick_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:mossy_stone_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mossy_stone_brick_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:mace", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	" # ",
	" I ",
],
key: vec![
	("#", vec!["minecraft:heavy_core"]),
	("I", vec!["minecraft:breeze_rod"]),
].into_iter().collect(),
result_id: "minecraft:mace",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:blue_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:blue_dye"]),
].into_iter().collect(),
result_id: "minecraft:blue_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:sandstone_wall_from_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:sandstone",
],
result_id: "minecraft:sandstone_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_shulker_box", Recipe::CraftingTransmute(Box::new(CraftingTransmuteData{
category: Some(Category::Misc),
group: Some("shulker_box_dye"),
input: vec![
	"#minecraft:shulker_boxes",
],
material: vec![
	"minecraft:magenta_dye",
],
result_id: "minecraft:magenta_shulker_box",
})));
	output.insert("minecraft:mud_bricks", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"##",
	"##",
],
key: vec![
	("#", vec!["minecraft:packed_mud"]),
].into_iter().collect(),
result_id: "minecraft:mud_bricks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:copper_ingot_from_waxed_copper_block", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("copper_ingot"),
ingredients: vec![
	vec!["minecraft:waxed_copper_block"],
],
result_id: "minecraft:copper_ingot",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:exposed_copper_grate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	" M ",
	"M M",
	" M ",
],
key: vec![
	("M", vec!["minecraft:exposed_copper"]),
].into_iter().collect(),
result_id: "minecraft:exposed_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:wheat", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: None,
ingredients: vec![
	vec!["minecraft:hay_block"],
],
result_id: "minecraft:wheat",
result_count: Some(9),
result_components: None,
})));
	output.insert("minecraft:mud_brick_stairs_from_mud_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:mud_bricks",
],
result_id: "minecraft:mud_brick_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:lapis_lazuli_from_smelting_lapis_ore", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Misc),
group: Some("lapis_lazuli"),
ingredient: vec![
	"minecraft:lapis_ore",
],
cooking_time: Some(200),
result_id: "minecraft:lapis_lazuli",
result_components: None,
experience: Some(0.2),
})));
	output.insert("minecraft:black_concrete_powder", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("concrete_powder"),
ingredients: vec![
	vec!["minecraft:black_dye"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:sand"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
	vec!["minecraft:gravel"],
],
result_id: "minecraft:black_concrete_powder",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:bone_block", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:bone_meal"]),
].into_iter().collect(),
result_id: "minecraft:bone_block",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:prismarine_brick_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:prismarine_bricks"]),
].into_iter().collect(),
result_id: "minecraft:prismarine_brick_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:cyan_glazed_terracotta", Recipe::Smelting(Box::new(FurnaceData{
category: Some(Category::Blocks),
group: None,
ingredient: vec![
	"minecraft:cyan_terracotta",
],
cooking_time: Some(200),
result_id: "minecraft:cyan_glazed_terracotta",
result_components: None,
experience: Some(0.1),
})));
	output.insert("minecraft:diorite_wall_from_diorite_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:diorite",
],
result_id: "minecraft:diorite_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:crimson_planks", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("planks"),
ingredients: vec![
	vec!["#minecraft:crimson_stems"],
],
result_id: "minecraft:crimson_planks",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:prismarine_slab_from_prismarine_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:prismarine",
],
result_id: "minecraft:prismarine_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:cherry_boat", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("boat"),
show_notification: None,
pattern: vec![
	"# #",
	"###",
],
key: vec![
	("#", vec!["minecraft:cherry_planks"]),
].into_iter().collect(),
result_id: "minecraft:cherry_boat",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_andesite_stairs", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: None,
show_notification: None,
pattern: vec![
	"#  ",
	"## ",
	"###",
],
key: vec![
	("#", vec!["minecraft:polished_andesite"]),
].into_iter().collect(),
result_id: "minecraft:polished_andesite_stairs",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:end_stone_brick_slab_from_end_stone_brick_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:end_stone_bricks",
],
result_id: "minecraft:end_stone_brick_slab",
result_count: Some(2),
result_components: None,
})));
	output.insert("minecraft:smooth_red_sandstone_stairs_from_smooth_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:smooth_red_sandstone",
],
result_id: "minecraft:smooth_red_sandstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_pressure_plate", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Redstone),
group: None,
show_notification: None,
pattern: vec![
	"##",
],
key: vec![
	("#", vec!["minecraft:polished_blackstone"]),
].into_iter().collect(),
result_id: "minecraft:polished_blackstone_pressure_plate",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:bamboo_slab", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("wooden_slab"),
show_notification: None,
pattern: vec![
	"###",
],
key: vec![
	("#", vec!["minecraft:bamboo_planks"]),
].into_iter().collect(),
result_id: "minecraft:bamboo_slab",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:black_terracotta", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_terracotta"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:terracotta"]),
	("X", vec!["minecraft:black_dye"]),
].into_iter().collect(),
result_id: "minecraft:black_terracotta",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:oak_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:oak_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:dye_lime_harness", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Equipment),
group: Some("harness_dye"),
ingredients: vec![
	vec!["minecraft:lime_dye"],
	vec![
		"minecraft:black_harness",
		"minecraft:blue_harness",
		"minecraft:brown_harness",
		"minecraft:cyan_harness",
		"minecraft:gray_harness",
		"minecraft:green_harness",
		"minecraft:light_blue_harness",
		"minecraft:light_gray_harness",
		"minecraft:magenta_harness",
		"minecraft:orange_harness",
		"minecraft:pink_harness",
		"minecraft:purple_harness",
		"minecraft:red_harness",
		"minecraft:yellow_harness",
		"minecraft:white_harness",
	],
],
result_id: "minecraft:lime_harness",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:mud_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:mud_bricks"]),
].into_iter().collect(),
result_id: "minecraft:mud_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:jungle_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:jungle_planks"],
],
result_id: "minecraft:jungle_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:yellow_stained_glass_pane_from_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"#$#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass_pane"]),
	("$", vec!["minecraft:yellow_dye"]),
].into_iter().collect(),
result_id: "minecraft:yellow_stained_glass_pane",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:black_stained_glass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Building),
group: Some("stained_glass"),
show_notification: None,
pattern: vec![
	"###",
	"#X#",
	"###",
],
key: vec![
	("#", vec!["minecraft:glass"]),
	("X", vec!["minecraft:black_dye"]),
].into_iter().collect(),
result_id: "minecraft:black_stained_glass",
result_count: Some(8),
result_components: None,
})));
	output.insert("minecraft:spruce_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: Some("wooden_button"),
ingredients: vec![
	vec!["minecraft:spruce_planks"],
],
result_id: "minecraft:spruce_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:oxidized_copper_grate_from_oxidized_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:oxidized_copper",
],
result_id: "minecraft:oxidized_copper_grate",
result_count: Some(4),
result_components: None,
})));
	output.insert("minecraft:dark_oak_shelf", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("shelf"),
show_notification: None,
pattern: vec![
	"###",
	"   ",
	"###",
],
key: vec![
	("#", vec!["minecraft:stripped_dark_oak_log"]),
].into_iter().collect(),
result_id: "minecraft:dark_oak_shelf",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:stone_brick_wall_from_stone_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone_bricks",
],
result_id: "minecraft:stone_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:chiseled_stone_bricks_stone_from_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:stone",
],
result_id: "minecraft:chiseled_stone_bricks",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:golden_axe", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"XX",
	"X#",
	" #",
],
key: vec![
	("#", vec!["minecraft:stick"]),
	("X", vec!["#minecraft:gold_tool_materials"]),
].into_iter().collect(),
result_id: "minecraft:golden_axe",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:magenta_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:magenta_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:magenta_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:brick_wall_from_bricks_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:bricks",
],
result_id: "minecraft:brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_button", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Redstone),
group: None,
ingredients: vec![
	vec!["minecraft:polished_blackstone"],
],
result_id: "minecraft:polished_blackstone_button",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_from_honeycomb", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Building),
group: Some("waxed_exposed_cut_copper"),
ingredients: vec![
	vec!["minecraft:exposed_cut_copper"],
	vec!["minecraft:honeycomb"],
],
result_id: "minecraft:waxed_exposed_cut_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:resin_brick_wall", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: None,
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:resin_bricks"]),
].into_iter().collect(),
result_id: "minecraft:resin_brick_wall",
result_count: Some(6),
result_components: None,
})));
	output.insert("minecraft:suspicious_stew_from_wither_rose", Recipe::CraftingShapeless(Box::new(CraftingShapelessData{
category: Some(Category::Misc),
group: Some("suspicious_stew"),
ingredients: vec![
	vec!["minecraft:bowl"],
	vec!["minecraft:brown_mushroom"],
	vec!["minecraft:red_mushroom"],
	vec!["minecraft:wither_rose"],
],
result_id: "minecraft:suspicious_stew",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:polished_blackstone_stairs_from_blackstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:blackstone",
],
result_id: "minecraft:polished_blackstone_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_exposed_cut_copper_stairs_from_waxed_exposed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_exposed_cut_copper",
],
result_id: "minecraft:waxed_exposed_cut_copper_stairs",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:white_stained_glass_pane", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Misc),
group: Some("stained_glass_pane"),
show_notification: None,
pattern: vec![
	"###",
	"###",
],
key: vec![
	("#", vec!["minecraft:white_stained_glass"]),
].into_iter().collect(),
result_id: "minecraft:white_stained_glass_pane",
result_count: Some(16),
result_components: None,
})));
	output.insert("minecraft:chiseled_red_sandstone_from_red_sandstone_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:red_sandstone",
],
result_id: "minecraft:chiseled_red_sandstone",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:waxed_chiseled_copper_from_waxed_cut_copper_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:waxed_cut_copper",
],
result_id: "minecraft:waxed_chiseled_copper",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:tuff_brick_wall_from_polished_tuff_stonecutting", Recipe::StoneCutting(Box::new(StoneCuttingData{
ingredient: vec![
	"minecraft:polished_tuff",
],
result_id: "minecraft:tuff_brick_wall",
result_count: Some(1),
result_components: None,
})));
	output.insert("minecraft:recovery_compass", Recipe::CraftingShaped(Box::new(CraftingShapedData{
category: Some(Category::Equipment),
group: None,
show_notification: None,
pattern: vec![
	"SSS",
	"SCS",
	"SSS",
],
key: vec![
	("C", vec!["minecraft:compass"]),
	("S", vec!["minecraft:echo_shard"]),
].into_iter().collect(),
result_id: "minecraft:recovery_compass",
result_count: Some(1),
result_components: None,
})));
	return output;
}