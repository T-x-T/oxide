#![allow(clippy::needless_return)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ItemRarity {
	Common,
	Uncommon,
	Rare,
	Epic,
}

#[derive(Debug, Clone)]
pub struct Item {
	pub max_stack_size: u8,
	pub rarity: ItemRarity,
	pub repair_cost: u8,
	pub id: i32,
	pub tool_rules: Vec<ToolRule>,
	pub nutrition: Option<u8>,
	pub saturation: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ToolRule {
	pub blocks: Vec<&'static str>,
	pub correct_for_drops: bool,
	pub speed: Option<f32>,
}

pub fn get_item_name_by_id(id: i32) -> &'static str {
  return match id {
		871 => "minecraft:acacia_boat",
		755 => "minecraft:acacia_button",
		872 => "minecraft:acacia_chest_boat",
		784 => "minecraft:acacia_door",
		348 => "minecraft:acacia_fence",
		825 => "minecraft:acacia_fence_gate",
		1004 => "minecraft:acacia_hanging_sign",
		186 => "minecraft:acacia_leaves",
		138 => "minecraft:acacia_log",
		40 => "minecraft:acacia_planks",
		771 => "minecraft:acacia_pressure_plate",
		53 => "minecraft:acacia_sapling",
		305 => "minecraft:acacia_shelf",
		992 => "minecraft:acacia_sign",
		274 => "minecraft:acacia_slab",
		445 => "minecraft:acacia_stairs",
		805 => "minecraft:acacia_trapdoor",
		175 => "minecraft:acacia_wood",
		836 => "minecraft:activator_rail",
		0 => "minecraft:air",
		1131 => "minecraft:allay_spawn_egg",
		234 => "minecraft:allium",
		88 => "minecraft:amethyst_block",
		1401 => "minecraft:amethyst_cluster",
		902 => "minecraft:amethyst_shard",
		82 => "minecraft:ancient_debris",
		6 => "minecraft:andesite",
		708 => "minecraft:andesite_slab",
		691 => "minecraft:andesite_stairs",
		466 => "minecraft:andesite_wall",
		1428 => "minecraft:angler_pottery_sherd",
		478 => "minecraft:anvil",
		893 => "minecraft:apple",
		1429 => "minecraft:archer_pottery_sherd",
		889 => "minecraft:armadillo_scute",
		1130 => "minecraft:armadillo_spawn_egg",
		1250 => "minecraft:armor_stand",
		1430 => "minecraft:arms_up_pottery_sherd",
		895 => "minecraft:arrow",
		1023 => "minecraft:axolotl_bucket",
		1132 => "minecraft:axolotl_spawn_egg",
		205 => "minecraft:azalea",
		191 => "minecraft:azalea_leaves",
		235 => "minecraft:azure_bluet",
		1225 => "minecraft:baked_potato",
		269 => "minecraft:bamboo",
		147 => "minecraft:bamboo_block",
		760 => "minecraft:bamboo_button",
		882 => "minecraft:bamboo_chest_raft",
		789 => "minecraft:bamboo_door",
		353 => "minecraft:bamboo_fence",
		830 => "minecraft:bamboo_fence_gate",
		1009 => "minecraft:bamboo_hanging_sign",
		48 => "minecraft:bamboo_mosaic",
		280 => "minecraft:bamboo_mosaic_slab",
		451 => "minecraft:bamboo_mosaic_stairs",
		45 => "minecraft:bamboo_planks",
		776 => "minecraft:bamboo_pressure_plate",
		881 => "minecraft:bamboo_raft",
		306 => "minecraft:bamboo_shelf",
		997 => "minecraft:bamboo_sign",
		279 => "minecraft:bamboo_slab",
		450 => "minecraft:bamboo_stairs",
		810 => "minecraft:bamboo_trapdoor",
		1337 => "minecraft:barrel",
		502 => "minecraft:barrier",
		362 => "minecraft:basalt",
		1133 => "minecraft:bat_spawn_egg",
		455 => "minecraft:beacon",
		58 => "minecraft:bedrock",
		1362 => "minecraft:bee_nest",
		1134 => "minecraft:bee_spawn_egg",
		1110 => "minecraft:beef",
		1363 => "minecraft:beehive",
		1282 => "minecraft:beetroot",
		1283 => "minecraft:beetroot_seeds",
		1284 => "minecraft:beetroot_soup",
		1345 => "minecraft:bell",
		267 => "minecraft:big_dripleaf",
		867 => "minecraft:birch_boat",
		753 => "minecraft:birch_button",
		868 => "minecraft:birch_chest_boat",
		782 => "minecraft:birch_door",
		346 => "minecraft:birch_fence",
		823 => "minecraft:birch_fence_gate",
		1002 => "minecraft:birch_hanging_sign",
		184 => "minecraft:birch_leaves",
		136 => "minecraft:birch_log",
		38 => "minecraft:birch_planks",
		769 => "minecraft:birch_pressure_plate",
		51 => "minecraft:birch_sapling",
		307 => "minecraft:birch_shelf",
		990 => "minecraft:birch_sign",
		272 => "minecraft:birch_slab",
		443 => "minecraft:birch_stairs",
		803 => "minecraft:birch_trapdoor",
		173 => "minecraft:birch_wood",
		1276 => "minecraft:black_banner",
		1101 => "minecraft:black_bed",
		1052 => "minecraft:black_bundle",
		1397 => "minecraft:black_candle",
		520 => "minecraft:black_carpet",
		629 => "minecraft:black_concrete",
		645 => "minecraft:black_concrete_powder",
		1081 => "minecraft:black_dye",
		613 => "minecraft:black_glazed_terracotta",
		853 => "minecraft:black_harness",
		597 => "minecraft:black_shulker_box",
		545 => "minecraft:black_stained_glass",
		561 => "minecraft:black_stained_glass_pane",
		501 => "minecraft:black_terracotta",
		228 => "minecraft:black_wool",
		1368 => "minecraft:blackstone",
		1369 => "minecraft:blackstone_slab",
		1370 => "minecraft:blackstone_stairs",
		471 => "minecraft:blackstone_wall",
		1431 => "minecraft:blade_pottery_sherd",
		1339 => "minecraft:blast_furnace",
		1124 => "minecraft:blaze_powder",
		1116 => "minecraft:blaze_rod",
		1135 => "minecraft:blaze_spawn_egg",
		1272 => "minecraft:blue_banner",
		1097 => "minecraft:blue_bed",
		1048 => "minecraft:blue_bundle",
		1393 => "minecraft:blue_candle",
		516 => "minecraft:blue_carpet",
		625 => "minecraft:blue_concrete",
		641 => "minecraft:blue_concrete_powder",
		1077 => "minecraft:blue_dye",
		1032 => "minecraft:blue_egg",
		609 => "minecraft:blue_glazed_terracotta",
		849 => "minecraft:blue_harness",
		679 => "minecraft:blue_ice",
		233 => "minecraft:blue_orchid",
		593 => "minecraft:blue_shulker_box",
		541 => "minecraft:blue_stained_glass",
		557 => "minecraft:blue_stained_glass_pane",
		497 => "minecraft:blue_terracotta",
		224 => "minecraft:blue_wool",
		1136 => "minecraft:bogged_spawn_egg",
		1427 => "minecraft:bolt_armor_trim_smithing_template",
		1083 => "minecraft:bone",
		579 => "minecraft:bone_block",
		1082 => "minecraft:bone_meal",
		1029 => "minecraft:book",
		317 => "minecraft:bookshelf",
		1334 => "minecraft:bordure_indented_banner_pattern",
		894 => "minecraft:bow",
		892 => "minecraft:bowl",
		660 => "minecraft:brain_coral",
		655 => "minecraft:brain_coral_block",
		670 => "minecraft:brain_coral_fan",
		953 => "minecraft:bread",
		1218 => "minecraft:breeze_rod",
		1137 => "minecraft:breeze_spawn_egg",
		1432 => "minecraft:brewer_pottery_sherd",
		1126 => "minecraft:brewing_stand",
		1025 => "minecraft:brick",
		289 => "minecraft:brick_slab",
		419 => "minecraft:brick_stairs",
		458 => "minecraft:brick_wall",
		304 => "minecraft:bricks",
		1273 => "minecraft:brown_banner",
		1098 => "minecraft:brown_bed",
		1049 => "minecraft:brown_bundle",
		1394 => "minecraft:brown_candle",
		517 => "minecraft:brown_carpet",
		626 => "minecraft:brown_concrete",
		642 => "minecraft:brown_concrete_powder",
		1078 => "minecraft:brown_dye",
		1033 => "minecraft:brown_egg",
		610 => "minecraft:brown_glazed_terracotta",
		850 => "minecraft:brown_harness",
		247 => "minecraft:brown_mushroom",
		387 => "minecraft:brown_mushroom_block",
		594 => "minecraft:brown_shulker_box",
		542 => "minecraft:brown_stained_glass",
		558 => "minecraft:brown_stained_glass_pane",
		498 => "minecraft:brown_terracotta",
		225 => "minecraft:brown_wool",
		1408 => "minecraft:brush",
		661 => "minecraft:bubble_coral",
		656 => "minecraft:bubble_coral_block",
		671 => "minecraft:bubble_coral_fan",
		1012 => "minecraft:bucket",
		89 => "minecraft:budding_amethyst",
		1036 => "minecraft:bundle",
		1433 => "minecraft:burn_pottery_sherd",
		204 => "minecraft:bush",
		340 => "minecraft:cactus",
		341 => "minecraft:cactus_flower",
		1085 => "minecraft:cake",
		11 => "minecraft:calcite",
		743 => "minecraft:calibrated_sculk_sensor",
		1139 => "minecraft:camel_spawn_egg",
		1358 => "minecraft:campfire",
		1381 => "minecraft:candle",
		1223 => "minecraft:carrot",
		859 => "minecraft:carrot_on_a_stick",
		1340 => "minecraft:cartography_table",
		357 => "minecraft:carved_pumpkin",
		1138 => "minecraft:cat_spawn_egg",
		1127 => "minecraft:cauldron",
		1140 => "minecraft:cave_spider_spawn_egg",
		574 => "minecraft:chain_command_block",
		965 => "minecraft:chainmail_boots",
		963 => "minecraft:chainmail_chestplate",
		962 => "minecraft:chainmail_helmet",
		964 => "minecraft:chainmail_leggings",
		897 => "minecraft:charcoal",
		873 => "minecraft:cherry_boat",
		756 => "minecraft:cherry_button",
		874 => "minecraft:cherry_chest_boat",
		785 => "minecraft:cherry_door",
		349 => "minecraft:cherry_fence",
		826 => "minecraft:cherry_fence_gate",
		1005 => "minecraft:cherry_hanging_sign",
		187 => "minecraft:cherry_leaves",
		139 => "minecraft:cherry_log",
		41 => "minecraft:cherry_planks",
		772 => "minecraft:cherry_pressure_plate",
		54 => "minecraft:cherry_sapling",
		308 => "minecraft:cherry_shelf",
		993 => "minecraft:cherry_sign",
		275 => "minecraft:cherry_slab",
		446 => "minecraft:cherry_stairs",
		806 => "minecraft:cherry_trapdoor",
		176 => "minecraft:cherry_wood",
		331 => "minecraft:chest",
		855 => "minecraft:chest_minecart",
		1112 => "minecraft:chicken",
		1141 => "minecraft:chicken_spawn_egg",
		479 => "minecraft:chipped_anvil",
		318 => "minecraft:chiseled_bookshelf",
		98 => "minecraft:chiseled_copper",
		385 => "minecraft:chiseled_deepslate",
		426 => "minecraft:chiseled_nether_bricks",
		1375 => "minecraft:chiseled_polished_blackstone",
		481 => "minecraft:chiseled_quartz_block",
		570 => "minecraft:chiseled_red_sandstone",
		418 => "minecraft:chiseled_resin_bricks",
		199 => "minecraft:chiseled_sandstone",
		378 => "minecraft:chiseled_stone_bricks",
		16 => "minecraft:chiseled_tuff",
		25 => "minecraft:chiseled_tuff_bricks",
		325 => "minecraft:chorus_flower",
		1278 => "minecraft:chorus_fruit",
		324 => "minecraft:chorus_plant",
		342 => "minecraft:clay",
		1026 => "minecraft:clay_ball",
		1054 => "minecraft:clock",
		231 => "minecraft:closed_eyeblossom",
		896 => "minecraft:coal",
		83 => "minecraft:coal_block",
		64 => "minecraft:coal_ore",
		29 => "minecraft:coarse_dirt",
		1412 => "minecraft:coast_armor_trim_smithing_template",
		9 => "minecraft:cobbled_deepslate",
		712 => "minecraft:cobbled_deepslate_slab",
		695 => "minecraft:cobbled_deepslate_stairs",
		474 => "minecraft:cobbled_deepslate_wall",
		35 => "minecraft:cobblestone",
		288 => "minecraft:cobblestone_slab",
		336 => "minecraft:cobblestone_stairs",
		456 => "minecraft:cobblestone_wall",
		201 => "minecraft:cobweb",
		1065 => "minecraft:cocoa_beans",
		1057 => "minecraft:cod",
		1021 => "minecraft:cod_bucket",
		1142 => "minecraft:cod_spawn_egg",
		454 => "minecraft:command_block",
		1258 => "minecraft:command_block_minecart",
		721 => "minecraft:comparator",
		1034 => "minecraft:compass",
		1336 => "minecraft:composter",
		680 => "minecraft:conduit",
		1111 => "minecraft:cooked_beef",
		1113 => "minecraft:cooked_chicken",
		1061 => "minecraft:cooked_cod",
		1260 => "minecraft:cooked_mutton",
		984 => "minecraft:cooked_porkchop",
		1246 => "minecraft:cooked_rabbit",
		1062 => "minecraft:cooked_salmon",
		1102 => "minecraft:cookie",
		919 => "minecraft:copper_axe",
		391 => "minecraft:copper_bars",
		91 => "minecraft:copper_block",
		961 => "minecraft:copper_boots",
		1459 => "minecraft:copper_bulb",
		400 => "minecraft:copper_chain",
		1467 => "minecraft:copper_chest",
		959 => "minecraft:copper_chestplate",
		792 => "minecraft:copper_door",
		1143 => "minecraft:copper_golem_spawn_egg",
		1475 => "minecraft:copper_golem_statue",
		1451 => "minecraft:copper_grate",
		958 => "minecraft:copper_helmet",
		920 => "minecraft:copper_hoe",
		1251 => "minecraft:copper_horse_armor",
		906 => "minecraft:copper_ingot",
		1348 => "minecraft:copper_lantern",
		960 => "minecraft:copper_leggings",
		1294 => "minecraft:copper_nugget",
		68 => "minecraft:copper_ore",
		918 => "minecraft:copper_pickaxe",
		917 => "minecraft:copper_shovel",
		916 => "minecraft:copper_sword",
		366 => "minecraft:copper_torch",
		813 => "minecraft:copper_trapdoor",
		241 => "minecraft:cornflower",
		1144 => "minecraft:cow_spawn_egg",
		382 => "minecraft:cracked_deepslate_bricks",
		384 => "minecraft:cracked_deepslate_tiles",
		425 => "minecraft:cracked_nether_bricks",
		1379 => "minecraft:cracked_polished_blackstone_bricks",
		377 => "minecraft:cracked_stone_bricks",
		1103 => "minecraft:crafter",
		332 => "minecraft:crafting_table",
		330 => "minecraft:creaking_heart",
		1208 => "minecraft:creaking_spawn_egg",
		1326 => "minecraft:creeper_banner_pattern",
		1233 => "minecraft:creeper_head",
		1145 => "minecraft:creeper_spawn_egg",
		761 => "minecraft:crimson_button",
		790 => "minecraft:crimson_door",
		354 => "minecraft:crimson_fence",
		831 => "minecraft:crimson_fence_gate",
		249 => "minecraft:crimson_fungus",
		1010 => "minecraft:crimson_hanging_sign",
		180 => "minecraft:crimson_hyphae",
		33 => "minecraft:crimson_nylium",
		46 => "minecraft:crimson_planks",
		777 => "minecraft:crimson_pressure_plate",
		251 => "minecraft:crimson_roots",
		309 => "minecraft:crimson_shelf",
		998 => "minecraft:crimson_sign",
		281 => "minecraft:crimson_slab",
		452 => "minecraft:crimson_stairs",
		145 => "minecraft:crimson_stem",
		811 => "minecraft:crimson_trapdoor",
		1322 => "minecraft:crossbow",
		1367 => "minecraft:crying_obsidian",
		102 => "minecraft:cut_copper",
		110 => "minecraft:cut_copper_slab",
		106 => "minecraft:cut_copper_stairs",
		571 => "minecraft:cut_red_sandstone",
		295 => "minecraft:cut_red_sandstone_slab",
		200 => "minecraft:cut_sandstone",
		286 => "minecraft:cut_sandstone_slab",
		1270 => "minecraft:cyan_banner",
		1095 => "minecraft:cyan_bed",
		1046 => "minecraft:cyan_bundle",
		1391 => "minecraft:cyan_candle",
		514 => "minecraft:cyan_carpet",
		623 => "minecraft:cyan_concrete",
		639 => "minecraft:cyan_concrete_powder",
		1075 => "minecraft:cyan_dye",
		607 => "minecraft:cyan_glazed_terracotta",
		847 => "minecraft:cyan_harness",
		591 => "minecraft:cyan_shulker_box",
		539 => "minecraft:cyan_stained_glass",
		555 => "minecraft:cyan_stained_glass_pane",
		495 => "minecraft:cyan_terracotta",
		222 => "minecraft:cyan_wool",
		480 => "minecraft:damaged_anvil",
		229 => "minecraft:dandelion",
		1434 => "minecraft:danger_pottery_sherd",
		875 => "minecraft:dark_oak_boat",
		757 => "minecraft:dark_oak_button",
		876 => "minecraft:dark_oak_chest_boat",
		786 => "minecraft:dark_oak_door",
		350 => "minecraft:dark_oak_fence",
		827 => "minecraft:dark_oak_fence_gate",
		1006 => "minecraft:dark_oak_hanging_sign",
		188 => "minecraft:dark_oak_leaves",
		141 => "minecraft:dark_oak_log",
		42 => "minecraft:dark_oak_planks",
		773 => "minecraft:dark_oak_pressure_plate",
		55 => "minecraft:dark_oak_sapling",
		310 => "minecraft:dark_oak_shelf",
		994 => "minecraft:dark_oak_sign",
		276 => "minecraft:dark_oak_slab",
		447 => "minecraft:dark_oak_stairs",
		807 => "minecraft:dark_oak_trapdoor",
		178 => "minecraft:dark_oak_wood",
		564 => "minecraft:dark_prismarine",
		299 => "minecraft:dark_prismarine_slab",
		567 => "minecraft:dark_prismarine_stairs",
		741 => "minecraft:daylight_detector",
		664 => "minecraft:dead_brain_coral",
		650 => "minecraft:dead_brain_coral_block",
		675 => "minecraft:dead_brain_coral_fan",
		665 => "minecraft:dead_bubble_coral",
		651 => "minecraft:dead_bubble_coral_block",
		676 => "minecraft:dead_bubble_coral_fan",
		207 => "minecraft:dead_bush",
		666 => "minecraft:dead_fire_coral",
		652 => "minecraft:dead_fire_coral_block",
		677 => "minecraft:dead_fire_coral_fan",
		667 => "minecraft:dead_horn_coral",
		653 => "minecraft:dead_horn_coral_block",
		678 => "minecraft:dead_horn_coral_fan",
		668 => "minecraft:dead_tube_coral",
		649 => "minecraft:dead_tube_coral_block",
		674 => "minecraft:dead_tube_coral_fan",
		1296 => "minecraft:debug_stick",
		319 => "minecraft:decorated_pot",
		8 => "minecraft:deepslate",
		714 => "minecraft:deepslate_brick_slab",
		697 => "minecraft:deepslate_brick_stairs",
		476 => "minecraft:deepslate_brick_wall",
		381 => "minecraft:deepslate_bricks",
		65 => "minecraft:deepslate_coal_ore",
		69 => "minecraft:deepslate_copper_ore",
		79 => "minecraft:deepslate_diamond_ore",
		75 => "minecraft:deepslate_emerald_ore",
		71 => "minecraft:deepslate_gold_ore",
		67 => "minecraft:deepslate_iron_ore",
		77 => "minecraft:deepslate_lapis_ore",
		73 => "minecraft:deepslate_redstone_ore",
		715 => "minecraft:deepslate_tile_slab",
		698 => "minecraft:deepslate_tile_stairs",
		477 => "minecraft:deepslate_tile_wall",
		383 => "minecraft:deepslate_tiles",
		834 => "minecraft:detector_rail",
		898 => "minecraft:diamond",
		939 => "minecraft:diamond_axe",
		93 => "minecraft:diamond_block",
		973 => "minecraft:diamond_boots",
		971 => "minecraft:diamond_chestplate",
		970 => "minecraft:diamond_helmet",
		940 => "minecraft:diamond_hoe",
		1254 => "minecraft:diamond_horse_armor",
		972 => "minecraft:diamond_leggings",
		78 => "minecraft:diamond_ore",
		938 => "minecraft:diamond_pickaxe",
		937 => "minecraft:diamond_shovel",
		936 => "minecraft:diamond_sword",
		4 => "minecraft:diorite",
		711 => "minecraft:diorite_slab",
		694 => "minecraft:diorite_stairs",
		470 => "minecraft:diorite_wall",
		28 => "minecraft:dirt",
		523 => "minecraft:dirt_path",
		1318 => "minecraft:disc_fragment_5",
		728 => "minecraft:dispenser",
		1146 => "minecraft:dolphin_spawn_egg",
		1147 => "minecraft:donkey_spawn_egg",
		1285 => "minecraft:dragon_breath",
		437 => "minecraft:dragon_egg",
		1234 => "minecraft:dragon_head",
		648 => "minecraft:dried_ghast",
		1107 => "minecraft:dried_kelp",
		1027 => "minecraft:dried_kelp_block",
		26 => "minecraft:dripstone_block",
		729 => "minecraft:dropper",
		1148 => "minecraft:drowned_spawn_egg",
		1411 => "minecraft:dune_armor_trim_smithing_template",
		1407 => "minecraft:echo_shard",
		1031 => "minecraft:egg",
		1149 => "minecraft:elder_guardian_spawn_egg",
		862 => "minecraft:elytra",
		899 => "minecraft:emerald",
		440 => "minecraft:emerald_block",
		74 => "minecraft:emerald_ore",
		1240 => "minecraft:enchanted_book",
		987 => "minecraft:enchanted_golden_apple",
		433 => "minecraft:enchanting_table",
		1277 => "minecraft:end_crystal",
		434 => "minecraft:end_portal_frame",
		323 => "minecraft:end_rod",
		435 => "minecraft:end_stone",
		704 => "minecraft:end_stone_brick_slab",
		686 => "minecraft:end_stone_brick_stairs",
		469 => "minecraft:end_stone_brick_wall",
		436 => "minecraft:end_stone_bricks",
		439 => "minecraft:ender_chest",
		1150 => "minecraft:ender_dragon_spawn_egg",
		1128 => "minecraft:ender_eye",
		1115 => "minecraft:ender_pearl",
		1151 => "minecraft:enderman_spawn_egg",
		1152 => "minecraft:endermite_spawn_egg",
		1153 => "minecraft:evoker_spawn_egg",
		1213 => "minecraft:experience_bottle",
		1435 => "minecraft:explorer_pottery_sherd",
		99 => "minecraft:exposed_chiseled_copper",
		95 => "minecraft:exposed_copper",
		392 => "minecraft:exposed_copper_bars",
		1460 => "minecraft:exposed_copper_bulb",
		401 => "minecraft:exposed_copper_chain",
		1468 => "minecraft:exposed_copper_chest",
		793 => "minecraft:exposed_copper_door",
		1476 => "minecraft:exposed_copper_golem_statue",
		1452 => "minecraft:exposed_copper_grate",
		1349 => "minecraft:exposed_copper_lantern",
		814 => "minecraft:exposed_copper_trapdoor",
		103 => "minecraft:exposed_cut_copper",
		111 => "minecraft:exposed_cut_copper_slab",
		107 => "minecraft:exposed_cut_copper_stairs",
		734 => "minecraft:exposed_lightning_rod",
		1415 => "minecraft:eye_armor_trim_smithing_template",
		333 => "minecraft:farmland",
		949 => "minecraft:feather",
		1123 => "minecraft:fermented_spider_eye",
		203 => "minecraft:fern",
		1333 => "minecraft:field_masoned_banner_pattern",
		1104 => "minecraft:filled_map",
		1214 => "minecraft:fire_charge",
		662 => "minecraft:fire_coral",
		657 => "minecraft:fire_coral_block",
		672 => "minecraft:fire_coral_fan",
		208 => "minecraft:firefly_bush",
		1238 => "minecraft:firework_rocket",
		1239 => "minecraft:firework_star",
		1053 => "minecraft:fishing_rod",
		1341 => "minecraft:fletching_table",
		982 => "minecraft:flint",
		891 => "minecraft:flint_and_steel",
		1426 => "minecraft:flow_armor_trim_smithing_template",
		1331 => "minecraft:flow_banner_pattern",
		1436 => "minecraft:flow_pottery_sherd",
		1325 => "minecraft:flower_banner_pattern",
		1222 => "minecraft:flower_pot",
		206 => "minecraft:flowering_azalea",
		192 => "minecraft:flowering_azalea_leaves",
		1154 => "minecraft:fox_spawn_egg",
		1437 => "minecraft:friend_pottery_sherd",
		1155 => "minecraft:frog_spawn_egg",
		1406 => "minecraft:frogspawn",
		334 => "minecraft:furnace",
		856 => "minecraft:furnace_minecart",
		1156 => "minecraft:ghast_spawn_egg",
		1117 => "minecraft:ghast_tear",
		1371 => "minecraft:gilded_blackstone",
		195 => "minecraft:glass",
		1120 => "minecraft:glass_bottle",
		408 => "minecraft:glass_pane",
		1129 => "minecraft:glistering_melon_slice",
		1329 => "minecraft:globe_banner_pattern",
		1357 => "minecraft:glow_berries",
		1064 => "minecraft:glow_ink_sac",
		1221 => "minecraft:glow_item_frame",
		411 => "minecraft:glow_lichen",
		1158 => "minecraft:glow_squid_spawn_egg",
		367 => "minecraft:glowstone",
		1056 => "minecraft:glowstone_dust",
		1335 => "minecraft:goat_horn",
		1159 => "minecraft:goat_spawn_egg",
		92 => "minecraft:gold_block",
		908 => "minecraft:gold_ingot",
		1118 => "minecraft:gold_nugget",
		70 => "minecraft:gold_ore",
		986 => "minecraft:golden_apple",
		929 => "minecraft:golden_axe",
		977 => "minecraft:golden_boots",
		1228 => "minecraft:golden_carrot",
		975 => "minecraft:golden_chestplate",
		974 => "minecraft:golden_helmet",
		930 => "minecraft:golden_hoe",
		1253 => "minecraft:golden_horse_armor",
		976 => "minecraft:golden_leggings",
		928 => "minecraft:golden_pickaxe",
		927 => "minecraft:golden_shovel",
		926 => "minecraft:golden_sword",
		2 => "minecraft:granite",
		707 => "minecraft:granite_slab",
		690 => "minecraft:granite_stairs",
		462 => "minecraft:granite_wall",
		27 => "minecraft:grass_block",
		63 => "minecraft:gravel",
		1268 => "minecraft:gray_banner",
		1093 => "minecraft:gray_bed",
		1044 => "minecraft:gray_bundle",
		1389 => "minecraft:gray_candle",
		512 => "minecraft:gray_carpet",
		621 => "minecraft:gray_concrete",
		637 => "minecraft:gray_concrete_powder",
		1073 => "minecraft:gray_dye",
		605 => "minecraft:gray_glazed_terracotta",
		845 => "minecraft:gray_harness",
		589 => "minecraft:gray_shulker_box",
		537 => "minecraft:gray_stained_glass",
		553 => "minecraft:gray_stained_glass_pane",
		493 => "minecraft:gray_terracotta",
		220 => "minecraft:gray_wool",
		1274 => "minecraft:green_banner",
		1099 => "minecraft:green_bed",
		1050 => "minecraft:green_bundle",
		1395 => "minecraft:green_candle",
		518 => "minecraft:green_carpet",
		627 => "minecraft:green_concrete",
		643 => "minecraft:green_concrete_powder",
		1079 => "minecraft:green_dye",
		611 => "minecraft:green_glazed_terracotta",
		851 => "minecraft:green_harness",
		595 => "minecraft:green_shulker_box",
		543 => "minecraft:green_stained_glass",
		559 => "minecraft:green_stained_glass_pane",
		499 => "minecraft:green_terracotta",
		226 => "minecraft:green_wool",
		1342 => "minecraft:grindstone",
		1160 => "minecraft:guardian_spawn_egg",
		950 => "minecraft:gunpowder",
		1332 => "minecraft:guster_banner_pattern",
		1438 => "minecraft:guster_pottery_sherd",
		266 => "minecraft:hanging_roots",
		1157 => "minecraft:happy_ghast_spawn_egg",
		504 => "minecraft:hay_block",
		1321 => "minecraft:heart_of_the_sea",
		1439 => "minecraft:heart_pottery_sherd",
		1440 => "minecraft:heartbreak_pottery_sherd",
		87 => "minecraft:heavy_core",
		766 => "minecraft:heavy_weighted_pressure_plate",
		1161 => "minecraft:hoglin_spawn_egg",
		725 => "minecraft:honey_block",
		1364 => "minecraft:honey_bottle",
		1361 => "minecraft:honeycomb",
		1365 => "minecraft:honeycomb_block",
		727 => "minecraft:hopper",
		858 => "minecraft:hopper_minecart",
		663 => "minecraft:horn_coral",
		658 => "minecraft:horn_coral_block",
		673 => "minecraft:horn_coral_fan",
		1162 => "minecraft:horse_spawn_egg",
		1425 => "minecraft:host_armor_trim_smithing_template",
		1441 => "minecraft:howl_pottery_sherd",
		1163 => "minecraft:husk_spawn_egg",
		338 => "minecraft:ice",
		373 => "minecraft:infested_chiseled_stone_bricks",
		369 => "minecraft:infested_cobblestone",
		372 => "minecraft:infested_cracked_stone_bricks",
		374 => "minecraft:infested_deepslate",
		371 => "minecraft:infested_mossy_stone_bricks",
		368 => "minecraft:infested_stone",
		370 => "minecraft:infested_stone_bricks",
		1063 => "minecraft:ink_sac",
		934 => "minecraft:iron_axe",
		390 => "minecraft:iron_bars",
		90 => "minecraft:iron_block",
		969 => "minecraft:iron_boots",
		399 => "minecraft:iron_chain",
		967 => "minecraft:iron_chestplate",
		779 => "minecraft:iron_door",
		1164 => "minecraft:iron_golem_spawn_egg",
		966 => "minecraft:iron_helmet",
		935 => "minecraft:iron_hoe",
		1252 => "minecraft:iron_horse_armor",
		904 => "minecraft:iron_ingot",
		968 => "minecraft:iron_leggings",
		1293 => "minecraft:iron_nugget",
		66 => "minecraft:iron_ore",
		933 => "minecraft:iron_pickaxe",
		932 => "minecraft:iron_shovel",
		931 => "minecraft:iron_sword",
		800 => "minecraft:iron_trapdoor",
		1220 => "minecraft:item_frame",
		358 => "minecraft:jack_o_lantern",
		884 => "minecraft:jigsaw",
		343 => "minecraft:jukebox",
		869 => "minecraft:jungle_boat",
		754 => "minecraft:jungle_button",
		870 => "minecraft:jungle_chest_boat",
		783 => "minecraft:jungle_door",
		347 => "minecraft:jungle_fence",
		824 => "minecraft:jungle_fence_gate",
		1003 => "minecraft:jungle_hanging_sign",
		185 => "minecraft:jungle_leaves",
		137 => "minecraft:jungle_log",
		39 => "minecraft:jungle_planks",
		770 => "minecraft:jungle_pressure_plate",
		52 => "minecraft:jungle_sapling",
		311 => "minecraft:jungle_shelf",
		991 => "minecraft:jungle_sign",
		273 => "minecraft:jungle_slab",
		444 => "minecraft:jungle_stairs",
		804 => "minecraft:jungle_trapdoor",
		174 => "minecraft:jungle_wood",
		257 => "minecraft:kelp",
		1295 => "minecraft:knowledge_book",
		335 => "minecraft:ladder",
		1346 => "minecraft:lantern",
		197 => "minecraft:lapis_block",
		900 => "minecraft:lapis_lazuli",
		76 => "minecraft:lapis_ore",
		1400 => "minecraft:large_amethyst_bud",
		529 => "minecraft:large_fern",
		1014 => "minecraft:lava_bucket",
		1256 => "minecraft:lead",
		260 => "minecraft:leaf_litter",
		1017 => "minecraft:leather",
		957 => "minecraft:leather_boots",
		955 => "minecraft:leather_chestplate",
		954 => "minecraft:leather_helmet",
		1255 => "minecraft:leather_horse_armor",
		956 => "minecraft:leather_leggings",
		730 => "minecraft:lectern",
		732 => "minecraft:lever",
		503 => "minecraft:light",
		1264 => "minecraft:light_blue_banner",
		1089 => "minecraft:light_blue_bed",
		1040 => "minecraft:light_blue_bundle",
		1385 => "minecraft:light_blue_candle",
		508 => "minecraft:light_blue_carpet",
		617 => "minecraft:light_blue_concrete",
		633 => "minecraft:light_blue_concrete_powder",
		1069 => "minecraft:light_blue_dye",
		601 => "minecraft:light_blue_glazed_terracotta",
		841 => "minecraft:light_blue_harness",
		585 => "minecraft:light_blue_shulker_box",
		533 => "minecraft:light_blue_stained_glass",
		549 => "minecraft:light_blue_stained_glass_pane",
		489 => "minecraft:light_blue_terracotta",
		216 => "minecraft:light_blue_wool",
		1269 => "minecraft:light_gray_banner",
		1094 => "minecraft:light_gray_bed",
		1045 => "minecraft:light_gray_bundle",
		1390 => "minecraft:light_gray_candle",
		513 => "minecraft:light_gray_carpet",
		622 => "minecraft:light_gray_concrete",
		638 => "minecraft:light_gray_concrete_powder",
		1074 => "minecraft:light_gray_dye",
		606 => "minecraft:light_gray_glazed_terracotta",
		846 => "minecraft:light_gray_harness",
		590 => "minecraft:light_gray_shulker_box",
		538 => "minecraft:light_gray_stained_glass",
		554 => "minecraft:light_gray_stained_glass_pane",
		494 => "minecraft:light_gray_terracotta",
		221 => "minecraft:light_gray_wool",
		765 => "minecraft:light_weighted_pressure_plate",
		733 => "minecraft:lightning_rod",
		525 => "minecraft:lilac",
		242 => "minecraft:lily_of_the_valley",
		423 => "minecraft:lily_pad",
		1266 => "minecraft:lime_banner",
		1091 => "minecraft:lime_bed",
		1042 => "minecraft:lime_bundle",
		1387 => "minecraft:lime_candle",
		510 => "minecraft:lime_carpet",
		619 => "minecraft:lime_concrete",
		635 => "minecraft:lime_concrete_powder",
		1071 => "minecraft:lime_dye",
		603 => "minecraft:lime_glazed_terracotta",
		843 => "minecraft:lime_harness",
		587 => "minecraft:lime_shulker_box",
		535 => "minecraft:lime_stained_glass",
		551 => "minecraft:lime_stained_glass_pane",
		491 => "minecraft:lime_terracotta",
		218 => "minecraft:lime_wool",
		1289 => "minecraft:lingering_potion",
		1165 => "minecraft:llama_spawn_egg",
		1366 => "minecraft:lodestone",
		1324 => "minecraft:loom",
		1219 => "minecraft:mace",
		1263 => "minecraft:magenta_banner",
		1088 => "minecraft:magenta_bed",
		1039 => "minecraft:magenta_bundle",
		1384 => "minecraft:magenta_candle",
		507 => "minecraft:magenta_carpet",
		616 => "minecraft:magenta_concrete",
		632 => "minecraft:magenta_concrete_powder",
		1068 => "minecraft:magenta_dye",
		600 => "minecraft:magenta_glazed_terracotta",
		840 => "minecraft:magenta_harness",
		584 => "minecraft:magenta_shulker_box",
		532 => "minecraft:magenta_stained_glass",
		548 => "minecraft:magenta_stained_glass_pane",
		488 => "minecraft:magenta_terracotta",
		215 => "minecraft:magenta_wool",
		575 => "minecraft:magma_block",
		1125 => "minecraft:magma_cream",
		1166 => "minecraft:magma_cube_spawn_egg",
		879 => "minecraft:mangrove_boat",
		759 => "minecraft:mangrove_button",
		880 => "minecraft:mangrove_chest_boat",
		788 => "minecraft:mangrove_door",
		352 => "minecraft:mangrove_fence",
		829 => "minecraft:mangrove_fence_gate",
		1008 => "minecraft:mangrove_hanging_sign",
		190 => "minecraft:mangrove_leaves",
		142 => "minecraft:mangrove_log",
		44 => "minecraft:mangrove_planks",
		775 => "minecraft:mangrove_pressure_plate",
		57 => "minecraft:mangrove_propagule",
		143 => "minecraft:mangrove_roots",
		312 => "minecraft:mangrove_shelf",
		996 => "minecraft:mangrove_sign",
		278 => "minecraft:mangrove_slab",
		449 => "minecraft:mangrove_stairs",
		809 => "minecraft:mangrove_trapdoor",
		179 => "minecraft:mangrove_wood",
		1227 => "minecraft:map",
		1399 => "minecraft:medium_amethyst_bud",
		409 => "minecraft:melon",
		1109 => "minecraft:melon_seeds",
		1106 => "minecraft:melon_slice",
		1018 => "minecraft:milk_bucket",
		854 => "minecraft:minecart",
		1442 => "minecraft:miner_pottery_sherd",
		1328 => "minecraft:mojang_banner_pattern",
		1167 => "minecraft:mooshroom_spawn_egg",
		262 => "minecraft:moss_block",
		261 => "minecraft:moss_carpet",
		320 => "minecraft:mossy_cobblestone",
		703 => "minecraft:mossy_cobblestone_slab",
		685 => "minecraft:mossy_cobblestone_stairs",
		457 => "minecraft:mossy_cobblestone_wall",
		701 => "minecraft:mossy_stone_brick_slab",
		683 => "minecraft:mossy_stone_brick_stairs",
		461 => "minecraft:mossy_stone_brick_wall",
		376 => "minecraft:mossy_stone_bricks",
		1443 => "minecraft:mourner_pottery_sherd",
		32 => "minecraft:mud",
		291 => "minecraft:mud_brick_slab",
		421 => "minecraft:mud_brick_stairs",
		464 => "minecraft:mud_brick_wall",
		380 => "minecraft:mud_bricks",
		144 => "minecraft:muddy_mangrove_roots",
		1168 => "minecraft:mule_spawn_egg",
		389 => "minecraft:mushroom_stem",
		947 => "minecraft:mushroom_stew",
		1310 => "minecraft:music_disc_11",
		1297 => "minecraft:music_disc_13",
		1314 => "minecraft:music_disc_5",
		1299 => "minecraft:music_disc_blocks",
		1298 => "minecraft:music_disc_cat",
		1300 => "minecraft:music_disc_chirp",
		1301 => "minecraft:music_disc_creator",
		1302 => "minecraft:music_disc_creator_music_box",
		1303 => "minecraft:music_disc_far",
		1304 => "minecraft:music_disc_lava_chicken",
		1305 => "minecraft:music_disc_mall",
		1306 => "minecraft:music_disc_mellohi",
		1312 => "minecraft:music_disc_otherside",
		1315 => "minecraft:music_disc_pigstep",
		1316 => "minecraft:music_disc_precipice",
		1313 => "minecraft:music_disc_relic",
		1307 => "minecraft:music_disc_stal",
		1308 => "minecraft:music_disc_strad",
		1317 => "minecraft:music_disc_tears",
		1311 => "minecraft:music_disc_wait",
		1309 => "minecraft:music_disc_ward",
		1259 => "minecraft:mutton",
		422 => "minecraft:mycelium",
		1257 => "minecraft:name_tag",
		1320 => "minecraft:nautilus_shell",
		1241 => "minecraft:nether_brick",
		427 => "minecraft:nether_brick_fence",
		292 => "minecraft:nether_brick_slab",
		428 => "minecraft:nether_brick_stairs",
		465 => "minecraft:nether_brick_wall",
		424 => "minecraft:nether_bricks",
		80 => "minecraft:nether_gold_ore",
		81 => "minecraft:nether_quartz_ore",
		253 => "minecraft:nether_sprouts",
		1236 => "minecraft:nether_star",
		1119 => "minecraft:nether_wart",
		576 => "minecraft:nether_wart_block",
		944 => "minecraft:netherite_axe",
		94 => "minecraft:netherite_block",
		981 => "minecraft:netherite_boots",
		979 => "minecraft:netherite_chestplate",
		978 => "minecraft:netherite_helmet",
		945 => "minecraft:netherite_hoe",
		909 => "minecraft:netherite_ingot",
		980 => "minecraft:netherite_leggings",
		943 => "minecraft:netherite_pickaxe",
		910 => "minecraft:netherite_scrap",
		942 => "minecraft:netherite_shovel",
		941 => "minecraft:netherite_sword",
		1409 => "minecraft:netherite_upgrade_smithing_template",
		359 => "minecraft:netherrack",
		748 => "minecraft:note_block",
		863 => "minecraft:oak_boat",
		751 => "minecraft:oak_button",
		864 => "minecraft:oak_chest_boat",
		780 => "minecraft:oak_door",
		344 => "minecraft:oak_fence",
		821 => "minecraft:oak_fence_gate",
		1000 => "minecraft:oak_hanging_sign",
		182 => "minecraft:oak_leaves",
		134 => "minecraft:oak_log",
		36 => "minecraft:oak_planks",
		767 => "minecraft:oak_pressure_plate",
		49 => "minecraft:oak_sapling",
		313 => "minecraft:oak_shelf",
		988 => "minecraft:oak_sign",
		270 => "minecraft:oak_slab",
		441 => "minecraft:oak_stairs",
		801 => "minecraft:oak_trapdoor",
		171 => "minecraft:oak_wood",
		726 => "minecraft:observer",
		321 => "minecraft:obsidian",
		1169 => "minecraft:ocelot_spawn_egg",
		1403 => "minecraft:ochre_froglight",
		1487 => "minecraft:ominous_bottle",
		1485 => "minecraft:ominous_trial_key",
		230 => "minecraft:open_eyeblossom",
		1262 => "minecraft:orange_banner",
		1087 => "minecraft:orange_bed",
		1038 => "minecraft:orange_bundle",
		1383 => "minecraft:orange_candle",
		506 => "minecraft:orange_carpet",
		615 => "minecraft:orange_concrete",
		631 => "minecraft:orange_concrete_powder",
		1067 => "minecraft:orange_dye",
		599 => "minecraft:orange_glazed_terracotta",
		839 => "minecraft:orange_harness",
		583 => "minecraft:orange_shulker_box",
		531 => "minecraft:orange_stained_glass",
		547 => "minecraft:orange_stained_glass_pane",
		487 => "minecraft:orange_terracotta",
		237 => "minecraft:orange_tulip",
		214 => "minecraft:orange_wool",
		240 => "minecraft:oxeye_daisy",
		101 => "minecraft:oxidized_chiseled_copper",
		97 => "minecraft:oxidized_copper",
		394 => "minecraft:oxidized_copper_bars",
		1462 => "minecraft:oxidized_copper_bulb",
		403 => "minecraft:oxidized_copper_chain",
		1470 => "minecraft:oxidized_copper_chest",
		795 => "minecraft:oxidized_copper_door",
		1478 => "minecraft:oxidized_copper_golem_statue",
		1454 => "minecraft:oxidized_copper_grate",
		1351 => "minecraft:oxidized_copper_lantern",
		816 => "minecraft:oxidized_copper_trapdoor",
		105 => "minecraft:oxidized_cut_copper",
		113 => "minecraft:oxidized_cut_copper_slab",
		109 => "minecraft:oxidized_cut_copper_stairs",
		736 => "minecraft:oxidized_lightning_rod",
		522 => "minecraft:packed_ice",
		379 => "minecraft:packed_mud",
		985 => "minecraft:painting",
		264 => "minecraft:pale_hanging_moss",
		265 => "minecraft:pale_moss_block",
		263 => "minecraft:pale_moss_carpet",
		877 => "minecraft:pale_oak_boat",
		758 => "minecraft:pale_oak_button",
		878 => "minecraft:pale_oak_chest_boat",
		787 => "minecraft:pale_oak_door",
		351 => "minecraft:pale_oak_fence",
		828 => "minecraft:pale_oak_fence_gate",
		1007 => "minecraft:pale_oak_hanging_sign",
		189 => "minecraft:pale_oak_leaves",
		140 => "minecraft:pale_oak_log",
		43 => "minecraft:pale_oak_planks",
		774 => "minecraft:pale_oak_pressure_plate",
		56 => "minecraft:pale_oak_sapling",
		314 => "minecraft:pale_oak_shelf",
		995 => "minecraft:pale_oak_sign",
		277 => "minecraft:pale_oak_slab",
		448 => "minecraft:pale_oak_stairs",
		808 => "minecraft:pale_oak_trapdoor",
		177 => "minecraft:pale_oak_wood",
		1170 => "minecraft:panda_spawn_egg",
		1028 => "minecraft:paper",
		1171 => "minecraft:parrot_spawn_egg",
		1405 => "minecraft:pearlescent_froglight",
		527 => "minecraft:peony",
		287 => "minecraft:petrified_oak_slab",
		861 => "minecraft:phantom_membrane",
		1172 => "minecraft:phantom_spawn_egg",
		1173 => "minecraft:pig_spawn_egg",
		1330 => "minecraft:piglin_banner_pattern",
		1175 => "minecraft:piglin_brute_spawn_egg",
		1235 => "minecraft:piglin_head",
		1174 => "minecraft:piglin_spawn_egg",
		1176 => "minecraft:pillager_spawn_egg",
		1267 => "minecraft:pink_banner",
		1092 => "minecraft:pink_bed",
		1043 => "minecraft:pink_bundle",
		1388 => "minecraft:pink_candle",
		511 => "minecraft:pink_carpet",
		620 => "minecraft:pink_concrete",
		636 => "minecraft:pink_concrete_powder",
		1072 => "minecraft:pink_dye",
		604 => "minecraft:pink_glazed_terracotta",
		844 => "minecraft:pink_harness",
		258 => "minecraft:pink_petals",
		588 => "minecraft:pink_shulker_box",
		536 => "minecraft:pink_stained_glass",
		552 => "minecraft:pink_stained_glass_pane",
		492 => "minecraft:pink_terracotta",
		239 => "minecraft:pink_tulip",
		219 => "minecraft:pink_wool",
		722 => "minecraft:piston",
		245 => "minecraft:pitcher_plant",
		1281 => "minecraft:pitcher_pod",
		1231 => "minecraft:player_head",
		1444 => "minecraft:plenty_pottery_sherd",
		30 => "minecraft:podzol",
		1402 => "minecraft:pointed_dripstone",
		1226 => "minecraft:poisonous_potato",
		1177 => "minecraft:polar_bear_spawn_egg",
		7 => "minecraft:polished_andesite",
		710 => "minecraft:polished_andesite_slab",
		693 => "minecraft:polished_andesite_stairs",
		363 => "minecraft:polished_basalt",
		1372 => "minecraft:polished_blackstone",
		1377 => "minecraft:polished_blackstone_brick_slab",
		1378 => "minecraft:polished_blackstone_brick_stairs",
		473 => "minecraft:polished_blackstone_brick_wall",
		1376 => "minecraft:polished_blackstone_bricks",
		750 => "minecraft:polished_blackstone_button",
		764 => "minecraft:polished_blackstone_pressure_plate",
		1373 => "minecraft:polished_blackstone_slab",
		1374 => "minecraft:polished_blackstone_stairs",
		472 => "minecraft:polished_blackstone_wall",
		10 => "minecraft:polished_deepslate",
		713 => "minecraft:polished_deepslate_slab",
		696 => "minecraft:polished_deepslate_stairs",
		475 => "minecraft:polished_deepslate_wall",
		5 => "minecraft:polished_diorite",
		702 => "minecraft:polished_diorite_slab",
		684 => "minecraft:polished_diorite_stairs",
		3 => "minecraft:polished_granite",
		699 => "minecraft:polished_granite_slab",
		681 => "minecraft:polished_granite_stairs",
		17 => "minecraft:polished_tuff",
		18 => "minecraft:polished_tuff_slab",
		19 => "minecraft:polished_tuff_stairs",
		20 => "minecraft:polished_tuff_wall",
		1279 => "minecraft:popped_chorus_fruit",
		232 => "minecraft:poppy",
		983 => "minecraft:porkchop",
		1224 => "minecraft:potato",
		1121 => "minecraft:potion",
		1015 => "minecraft:powder_snow_bucket",
		833 => "minecraft:powered_rail",
		562 => "minecraft:prismarine",
		298 => "minecraft:prismarine_brick_slab",
		566 => "minecraft:prismarine_brick_stairs",
		563 => "minecraft:prismarine_bricks",
		1244 => "minecraft:prismarine_crystals",
		1243 => "minecraft:prismarine_shard",
		297 => "minecraft:prismarine_slab",
		565 => "minecraft:prismarine_stairs",
		459 => "minecraft:prismarine_wall",
		1445 => "minecraft:prize_pottery_sherd",
		1060 => "minecraft:pufferfish",
		1019 => "minecraft:pufferfish_bucket",
		1178 => "minecraft:pufferfish_spawn_egg",
		356 => "minecraft:pumpkin",
		1237 => "minecraft:pumpkin_pie",
		1108 => "minecraft:pumpkin_seeds",
		1271 => "minecraft:purple_banner",
		1096 => "minecraft:purple_bed",
		1047 => "minecraft:purple_bundle",
		1392 => "minecraft:purple_candle",
		515 => "minecraft:purple_carpet",
		624 => "minecraft:purple_concrete",
		640 => "minecraft:purple_concrete_powder",
		1076 => "minecraft:purple_dye",
		608 => "minecraft:purple_glazed_terracotta",
		848 => "minecraft:purple_harness",
		592 => "minecraft:purple_shulker_box",
		540 => "minecraft:purple_stained_glass",
		556 => "minecraft:purple_stained_glass_pane",
		496 => "minecraft:purple_terracotta",
		223 => "minecraft:purple_wool",
		326 => "minecraft:purpur_block",
		327 => "minecraft:purpur_pillar",
		296 => "minecraft:purpur_slab",
		328 => "minecraft:purpur_stairs",
		901 => "minecraft:quartz",
		482 => "minecraft:quartz_block",
		483 => "minecraft:quartz_bricks",
		484 => "minecraft:quartz_pillar",
		293 => "minecraft:quartz_slab",
		485 => "minecraft:quartz_stairs",
		1245 => "minecraft:rabbit",
		1248 => "minecraft:rabbit_foot",
		1249 => "minecraft:rabbit_hide",
		1179 => "minecraft:rabbit_spawn_egg",
		1247 => "minecraft:rabbit_stew",
		835 => "minecraft:rail",
		1424 => "minecraft:raiser_armor_trim_smithing_template",
		1180 => "minecraft:ravager_spawn_egg",
		905 => "minecraft:raw_copper",
		85 => "minecraft:raw_copper_block",
		907 => "minecraft:raw_gold",
		86 => "minecraft:raw_gold_block",
		903 => "minecraft:raw_iron",
		84 => "minecraft:raw_iron_block",
		1035 => "minecraft:recovery_compass",
		1275 => "minecraft:red_banner",
		1100 => "minecraft:red_bed",
		1051 => "minecraft:red_bundle",
		1396 => "minecraft:red_candle",
		519 => "minecraft:red_carpet",
		628 => "minecraft:red_concrete",
		644 => "minecraft:red_concrete_powder",
		1080 => "minecraft:red_dye",
		612 => "minecraft:red_glazed_terracotta",
		852 => "minecraft:red_harness",
		248 => "minecraft:red_mushroom",
		388 => "minecraft:red_mushroom_block",
		709 => "minecraft:red_nether_brick_slab",
		692 => "minecraft:red_nether_brick_stairs",
		467 => "minecraft:red_nether_brick_wall",
		578 => "minecraft:red_nether_bricks",
		62 => "minecraft:red_sand",
		569 => "minecraft:red_sandstone",
		294 => "minecraft:red_sandstone_slab",
		572 => "minecraft:red_sandstone_stairs",
		460 => "minecraft:red_sandstone_wall",
		596 => "minecraft:red_shulker_box",
		544 => "minecraft:red_stained_glass",
		560 => "minecraft:red_stained_glass_pane",
		500 => "minecraft:red_terracotta",
		236 => "minecraft:red_tulip",
		227 => "minecraft:red_wool",
		717 => "minecraft:redstone",
		719 => "minecraft:redstone_block",
		747 => "minecraft:redstone_lamp",
		72 => "minecraft:redstone_ore",
		718 => "minecraft:redstone_torch",
		386 => "minecraft:reinforced_deepslate",
		720 => "minecraft:repeater",
		573 => "minecraft:repeating_command_block",
		413 => "minecraft:resin_block",
		1242 => "minecraft:resin_brick",
		416 => "minecraft:resin_brick_slab",
		415 => "minecraft:resin_brick_stairs",
		417 => "minecraft:resin_brick_wall",
		414 => "minecraft:resin_bricks",
		412 => "minecraft:resin_clump",
		1380 => "minecraft:respawn_anchor",
		1419 => "minecraft:rib_armor_trim_smithing_template",
		31 => "minecraft:rooted_dirt",
		526 => "minecraft:rose_bush",
		1114 => "minecraft:rotten_flesh",
		837 => "minecraft:saddle",
		1058 => "minecraft:salmon",
		1020 => "minecraft:salmon_bucket",
		1181 => "minecraft:salmon_spawn_egg",
		59 => "minecraft:sand",
		198 => "minecraft:sandstone",
		285 => "minecraft:sandstone_slab",
		438 => "minecraft:sandstone_stairs",
		468 => "minecraft:sandstone_wall",
		716 => "minecraft:scaffolding",
		1446 => "minecraft:scrape_pottery_sherd",
		429 => "minecraft:sculk",
		431 => "minecraft:sculk_catalyst",
		742 => "minecraft:sculk_sensor",
		432 => "minecraft:sculk_shrieker",
		430 => "minecraft:sculk_vein",
		568 => "minecraft:sea_lantern",
		212 => "minecraft:sea_pickle",
		211 => "minecraft:seagrass",
		1410 => "minecraft:sentry_armor_trim_smithing_template",
		1422 => "minecraft:shaper_armor_trim_smithing_template",
		1447 => "minecraft:sheaf_pottery_sherd",
		1105 => "minecraft:shears",
		1182 => "minecraft:sheep_spawn_egg",
		1448 => "minecraft:shelter_pottery_sherd",
		1290 => "minecraft:shield",
		209 => "minecraft:short_dry_grass",
		202 => "minecraft:short_grass",
		1360 => "minecraft:shroomlight",
		581 => "minecraft:shulker_box",
		1292 => "minecraft:shulker_shell",
		1183 => "minecraft:shulker_spawn_egg",
		1423 => "minecraft:silence_armor_trim_smithing_template",
		1184 => "minecraft:silverfish_spawn_egg",
		1186 => "minecraft:skeleton_horse_spawn_egg",
		1229 => "minecraft:skeleton_skull",
		1185 => "minecraft:skeleton_spawn_egg",
		1327 => "minecraft:skull_banner_pattern",
		1449 => "minecraft:skull_pottery_sherd",
		1030 => "minecraft:slime_ball",
		724 => "minecraft:slime_block",
		1187 => "minecraft:slime_spawn_egg",
		1398 => "minecraft:small_amethyst_bud",
		268 => "minecraft:small_dripleaf",
		1343 => "minecraft:smithing_table",
		1338 => "minecraft:smoker",
		364 => "minecraft:smooth_basalt",
		300 => "minecraft:smooth_quartz",
		706 => "minecraft:smooth_quartz_slab",
		689 => "minecraft:smooth_quartz_stairs",
		301 => "minecraft:smooth_red_sandstone",
		700 => "minecraft:smooth_red_sandstone_slab",
		682 => "minecraft:smooth_red_sandstone_stairs",
		302 => "minecraft:smooth_sandstone",
		705 => "minecraft:smooth_sandstone_slab",
		688 => "minecraft:smooth_sandstone_stairs",
		303 => "minecraft:smooth_stone",
		284 => "minecraft:smooth_stone_slab",
		647 => "minecraft:sniffer_egg",
		1188 => "minecraft:sniffer_spawn_egg",
		1450 => "minecraft:snort_pottery_sherd",
		1418 => "minecraft:snout_armor_trim_smithing_template",
		337 => "minecraft:snow",
		339 => "minecraft:snow_block",
		1189 => "minecraft:snow_golem_spawn_egg",
		1016 => "minecraft:snowball",
		1359 => "minecraft:soul_campfire",
		1347 => "minecraft:soul_lantern",
		360 => "minecraft:soul_sand",
		361 => "minecraft:soul_soil",
		365 => "minecraft:soul_torch",
		329 => "minecraft:spawner",
		1287 => "minecraft:spectral_arrow",
		1122 => "minecraft:spider_eye",
		1190 => "minecraft:spider_spawn_egg",
		1420 => "minecraft:spire_armor_trim_smithing_template",
		1286 => "minecraft:splash_potion",
		193 => "minecraft:sponge",
		246 => "minecraft:spore_blossom",
		865 => "minecraft:spruce_boat",
		752 => "minecraft:spruce_button",
		866 => "minecraft:spruce_chest_boat",
		781 => "minecraft:spruce_door",
		345 => "minecraft:spruce_fence",
		822 => "minecraft:spruce_fence_gate",
		1001 => "minecraft:spruce_hanging_sign",
		183 => "minecraft:spruce_leaves",
		135 => "minecraft:spruce_log",
		37 => "minecraft:spruce_planks",
		768 => "minecraft:spruce_pressure_plate",
		50 => "minecraft:spruce_sapling",
		315 => "minecraft:spruce_shelf",
		989 => "minecraft:spruce_sign",
		271 => "minecraft:spruce_slab",
		442 => "minecraft:spruce_stairs",
		802 => "minecraft:spruce_trapdoor",
		172 => "minecraft:spruce_wood",
		1055 => "minecraft:spyglass",
		1191 => "minecraft:squid_spawn_egg",
		946 => "minecraft:stick",
		723 => "minecraft:sticky_piston",
		1 => "minecraft:stone",
		924 => "minecraft:stone_axe",
		290 => "minecraft:stone_brick_slab",
		420 => "minecraft:stone_brick_stairs",
		463 => "minecraft:stone_brick_wall",
		375 => "minecraft:stone_bricks",
		749 => "minecraft:stone_button",
		925 => "minecraft:stone_hoe",
		923 => "minecraft:stone_pickaxe",
		763 => "minecraft:stone_pressure_plate",
		922 => "minecraft:stone_shovel",
		283 => "minecraft:stone_slab",
		687 => "minecraft:stone_stairs",
		921 => "minecraft:stone_sword",
		1344 => "minecraft:stonecutter",
		1192 => "minecraft:stray_spawn_egg",
		1193 => "minecraft:strider_spawn_egg",
		948 => "minecraft:string",
		152 => "minecraft:stripped_acacia_log",
		163 => "minecraft:stripped_acacia_wood",
		170 => "minecraft:stripped_bamboo_block",
		150 => "minecraft:stripped_birch_log",
		161 => "minecraft:stripped_birch_wood",
		153 => "minecraft:stripped_cherry_log",
		164 => "minecraft:stripped_cherry_wood",
		168 => "minecraft:stripped_crimson_hyphae",
		157 => "minecraft:stripped_crimson_stem",
		154 => "minecraft:stripped_dark_oak_log",
		165 => "minecraft:stripped_dark_oak_wood",
		151 => "minecraft:stripped_jungle_log",
		162 => "minecraft:stripped_jungle_wood",
		156 => "minecraft:stripped_mangrove_log",
		167 => "minecraft:stripped_mangrove_wood",
		148 => "minecraft:stripped_oak_log",
		159 => "minecraft:stripped_oak_wood",
		155 => "minecraft:stripped_pale_oak_log",
		166 => "minecraft:stripped_pale_oak_wood",
		149 => "minecraft:stripped_spruce_log",
		160 => "minecraft:stripped_spruce_wood",
		169 => "minecraft:stripped_warped_hyphae",
		158 => "minecraft:stripped_warped_stem",
		883 => "minecraft:structure_block",
		580 => "minecraft:structure_void",
		1084 => "minecraft:sugar",
		256 => "minecraft:sugar_cane",
		524 => "minecraft:sunflower",
		61 => "minecraft:suspicious_gravel",
		60 => "minecraft:suspicious_sand",
		1323 => "minecraft:suspicious_stew",
		1356 => "minecraft:sweet_berries",
		1024 => "minecraft:tadpole_bucket",
		1194 => "minecraft:tadpole_spawn_egg",
		210 => "minecraft:tall_dry_grass",
		528 => "minecraft:tall_grass",
		731 => "minecraft:target",
		521 => "minecraft:terracotta",
		885 => "minecraft:test_block",
		886 => "minecraft:test_instance_block",
		1417 => "minecraft:tide_armor_trim_smithing_template",
		196 => "minecraft:tinted_glass",
		1288 => "minecraft:tipped_arrow",
		746 => "minecraft:tnt",
		857 => "minecraft:tnt_minecart",
		322 => "minecraft:torch",
		244 => "minecraft:torchflower",
		1280 => "minecraft:torchflower_seeds",
		1291 => "minecraft:totem_of_undying",
		1195 => "minecraft:trader_llama_spawn_egg",
		745 => "minecraft:trapped_chest",
		1484 => "minecraft:trial_key",
		1483 => "minecraft:trial_spawner",
		1319 => "minecraft:trident",
		744 => "minecraft:tripwire_hook",
		1059 => "minecraft:tropical_fish",
		1022 => "minecraft:tropical_fish_bucket",
		1196 => "minecraft:tropical_fish_spawn_egg",
		659 => "minecraft:tube_coral",
		654 => "minecraft:tube_coral_block",
		669 => "minecraft:tube_coral_fan",
		12 => "minecraft:tuff",
		22 => "minecraft:tuff_brick_slab",
		23 => "minecraft:tuff_brick_stairs",
		24 => "minecraft:tuff_brick_wall",
		21 => "minecraft:tuff_bricks",
		13 => "minecraft:tuff_slab",
		14 => "minecraft:tuff_stairs",
		15 => "minecraft:tuff_wall",
		646 => "minecraft:turtle_egg",
		887 => "minecraft:turtle_helmet",
		888 => "minecraft:turtle_scute",
		1197 => "minecraft:turtle_spawn_egg",
		255 => "minecraft:twisting_vines",
		1486 => "minecraft:vault",
		1404 => "minecraft:verdant_froglight",
		1416 => "minecraft:vex_armor_trim_smithing_template",
		1198 => "minecraft:vex_spawn_egg",
		1199 => "minecraft:villager_spawn_egg",
		1200 => "minecraft:vindicator_spawn_egg",
		410 => "minecraft:vine",
		1201 => "minecraft:wandering_trader_spawn_egg",
		1414 => "minecraft:ward_armor_trim_smithing_template",
		1202 => "minecraft:warden_spawn_egg",
		762 => "minecraft:warped_button",
		791 => "minecraft:warped_door",
		355 => "minecraft:warped_fence",
		832 => "minecraft:warped_fence_gate",
		250 => "minecraft:warped_fungus",
		860 => "minecraft:warped_fungus_on_a_stick",
		1011 => "minecraft:warped_hanging_sign",
		181 => "minecraft:warped_hyphae",
		34 => "minecraft:warped_nylium",
		47 => "minecraft:warped_planks",
		778 => "minecraft:warped_pressure_plate",
		252 => "minecraft:warped_roots",
		316 => "minecraft:warped_shelf",
		999 => "minecraft:warped_sign",
		282 => "minecraft:warped_slab",
		453 => "minecraft:warped_stairs",
		146 => "minecraft:warped_stem",
		812 => "minecraft:warped_trapdoor",
		577 => "minecraft:warped_wart_block",
		1013 => "minecraft:water_bucket",
		118 => "minecraft:waxed_chiseled_copper",
		395 => "minecraft:waxed_copper_bars",
		114 => "minecraft:waxed_copper_block",
		1463 => "minecraft:waxed_copper_bulb",
		404 => "minecraft:waxed_copper_chain",
		1471 => "minecraft:waxed_copper_chest",
		796 => "minecraft:waxed_copper_door",
		1479 => "minecraft:waxed_copper_golem_statue",
		1455 => "minecraft:waxed_copper_grate",
		1352 => "minecraft:waxed_copper_lantern",
		817 => "minecraft:waxed_copper_trapdoor",
		122 => "minecraft:waxed_cut_copper",
		130 => "minecraft:waxed_cut_copper_slab",
		126 => "minecraft:waxed_cut_copper_stairs",
		119 => "minecraft:waxed_exposed_chiseled_copper",
		115 => "minecraft:waxed_exposed_copper",
		396 => "minecraft:waxed_exposed_copper_bars",
		1464 => "minecraft:waxed_exposed_copper_bulb",
		405 => "minecraft:waxed_exposed_copper_chain",
		1472 => "minecraft:waxed_exposed_copper_chest",
		797 => "minecraft:waxed_exposed_copper_door",
		1480 => "minecraft:waxed_exposed_copper_golem_statue",
		1456 => "minecraft:waxed_exposed_copper_grate",
		1353 => "minecraft:waxed_exposed_copper_lantern",
		818 => "minecraft:waxed_exposed_copper_trapdoor",
		123 => "minecraft:waxed_exposed_cut_copper",
		131 => "minecraft:waxed_exposed_cut_copper_slab",
		127 => "minecraft:waxed_exposed_cut_copper_stairs",
		738 => "minecraft:waxed_exposed_lightning_rod",
		737 => "minecraft:waxed_lightning_rod",
		121 => "minecraft:waxed_oxidized_chiseled_copper",
		117 => "minecraft:waxed_oxidized_copper",
		398 => "minecraft:waxed_oxidized_copper_bars",
		1466 => "minecraft:waxed_oxidized_copper_bulb",
		407 => "minecraft:waxed_oxidized_copper_chain",
		1474 => "minecraft:waxed_oxidized_copper_chest",
		799 => "minecraft:waxed_oxidized_copper_door",
		1482 => "minecraft:waxed_oxidized_copper_golem_statue",
		1458 => "minecraft:waxed_oxidized_copper_grate",
		1355 => "minecraft:waxed_oxidized_copper_lantern",
		820 => "minecraft:waxed_oxidized_copper_trapdoor",
		125 => "minecraft:waxed_oxidized_cut_copper",
		133 => "minecraft:waxed_oxidized_cut_copper_slab",
		129 => "minecraft:waxed_oxidized_cut_copper_stairs",
		740 => "minecraft:waxed_oxidized_lightning_rod",
		120 => "minecraft:waxed_weathered_chiseled_copper",
		116 => "minecraft:waxed_weathered_copper",
		397 => "minecraft:waxed_weathered_copper_bars",
		1465 => "minecraft:waxed_weathered_copper_bulb",
		406 => "minecraft:waxed_weathered_copper_chain",
		1473 => "minecraft:waxed_weathered_copper_chest",
		798 => "minecraft:waxed_weathered_copper_door",
		1481 => "minecraft:waxed_weathered_copper_golem_statue",
		1457 => "minecraft:waxed_weathered_copper_grate",
		1354 => "minecraft:waxed_weathered_copper_lantern",
		819 => "minecraft:waxed_weathered_copper_trapdoor",
		124 => "minecraft:waxed_weathered_cut_copper",
		132 => "minecraft:waxed_weathered_cut_copper_slab",
		128 => "minecraft:waxed_weathered_cut_copper_stairs",
		739 => "minecraft:waxed_weathered_lightning_rod",
		1421 => "minecraft:wayfinder_armor_trim_smithing_template",
		100 => "minecraft:weathered_chiseled_copper",
		96 => "minecraft:weathered_copper",
		393 => "minecraft:weathered_copper_bars",
		1461 => "minecraft:weathered_copper_bulb",
		402 => "minecraft:weathered_copper_chain",
		1469 => "minecraft:weathered_copper_chest",
		794 => "minecraft:weathered_copper_door",
		1477 => "minecraft:weathered_copper_golem_statue",
		1453 => "minecraft:weathered_copper_grate",
		1350 => "minecraft:weathered_copper_lantern",
		815 => "minecraft:weathered_copper_trapdoor",
		104 => "minecraft:weathered_cut_copper",
		112 => "minecraft:weathered_cut_copper_slab",
		108 => "minecraft:weathered_cut_copper_stairs",
		735 => "minecraft:weathered_lightning_rod",
		254 => "minecraft:weeping_vines",
		194 => "minecraft:wet_sponge",
		952 => "minecraft:wheat",
		951 => "minecraft:wheat_seeds",
		1261 => "minecraft:white_banner",
		1086 => "minecraft:white_bed",
		1037 => "minecraft:white_bundle",
		1382 => "minecraft:white_candle",
		505 => "minecraft:white_carpet",
		614 => "minecraft:white_concrete",
		630 => "minecraft:white_concrete_powder",
		1066 => "minecraft:white_dye",
		598 => "minecraft:white_glazed_terracotta",
		838 => "minecraft:white_harness",
		582 => "minecraft:white_shulker_box",
		530 => "minecraft:white_stained_glass",
		546 => "minecraft:white_stained_glass_pane",
		486 => "minecraft:white_terracotta",
		238 => "minecraft:white_tulip",
		213 => "minecraft:white_wool",
		1413 => "minecraft:wild_armor_trim_smithing_template",
		259 => "minecraft:wildflowers",
		1215 => "minecraft:wind_charge",
		1203 => "minecraft:witch_spawn_egg",
		243 => "minecraft:wither_rose",
		1230 => "minecraft:wither_skeleton_skull",
		1205 => "minecraft:wither_skeleton_spawn_egg",
		1204 => "minecraft:wither_spawn_egg",
		890 => "minecraft:wolf_armor",
		1206 => "minecraft:wolf_spawn_egg",
		914 => "minecraft:wooden_axe",
		915 => "minecraft:wooden_hoe",
		913 => "minecraft:wooden_pickaxe",
		912 => "minecraft:wooden_shovel",
		911 => "minecraft:wooden_sword",
		1216 => "minecraft:writable_book",
		1217 => "minecraft:written_book",
		1265 => "minecraft:yellow_banner",
		1090 => "minecraft:yellow_bed",
		1041 => "minecraft:yellow_bundle",
		1386 => "minecraft:yellow_candle",
		509 => "minecraft:yellow_carpet",
		618 => "minecraft:yellow_concrete",
		634 => "minecraft:yellow_concrete_powder",
		1070 => "minecraft:yellow_dye",
		602 => "minecraft:yellow_glazed_terracotta",
		842 => "minecraft:yellow_harness",
		586 => "minecraft:yellow_shulker_box",
		534 => "minecraft:yellow_stained_glass",
		550 => "minecraft:yellow_stained_glass_pane",
		490 => "minecraft:yellow_terracotta",
		217 => "minecraft:yellow_wool",
		1207 => "minecraft:zoglin_spawn_egg",
		1232 => "minecraft:zombie_head",
		1210 => "minecraft:zombie_horse_spawn_egg",
		1209 => "minecraft:zombie_spawn_egg",
		1211 => "minecraft:zombie_villager_spawn_egg",
		1212 => "minecraft:zombified_piglin_spawn_egg",
    x => panic!("no idea what item id {x} is"),
	};
}
pub fn get_item_id_by_name(name: &str) -> i32 {
  return match name {
		"minecraft:acacia_boat" => 871,
		"minecraft:acacia_button" => 755,
		"minecraft:acacia_chest_boat" => 872,
		"minecraft:acacia_door" => 784,
		"minecraft:acacia_fence" => 348,
		"minecraft:acacia_fence_gate" => 825,
		"minecraft:acacia_hanging_sign" => 1004,
		"minecraft:acacia_leaves" => 186,
		"minecraft:acacia_log" => 138,
		"minecraft:acacia_planks" => 40,
		"minecraft:acacia_pressure_plate" => 771,
		"minecraft:acacia_sapling" => 53,
		"minecraft:acacia_shelf" => 305,
		"minecraft:acacia_sign" => 992,
		"minecraft:acacia_slab" => 274,
		"minecraft:acacia_stairs" => 445,
		"minecraft:acacia_trapdoor" => 805,
		"minecraft:acacia_wood" => 175,
		"minecraft:activator_rail" => 836,
		"minecraft:air" => 0,
		"minecraft:allay_spawn_egg" => 1131,
		"minecraft:allium" => 234,
		"minecraft:amethyst_block" => 88,
		"minecraft:amethyst_cluster" => 1401,
		"minecraft:amethyst_shard" => 902,
		"minecraft:ancient_debris" => 82,
		"minecraft:andesite" => 6,
		"minecraft:andesite_slab" => 708,
		"minecraft:andesite_stairs" => 691,
		"minecraft:andesite_wall" => 466,
		"minecraft:angler_pottery_sherd" => 1428,
		"minecraft:anvil" => 478,
		"minecraft:apple" => 893,
		"minecraft:archer_pottery_sherd" => 1429,
		"minecraft:armadillo_scute" => 889,
		"minecraft:armadillo_spawn_egg" => 1130,
		"minecraft:armor_stand" => 1250,
		"minecraft:arms_up_pottery_sherd" => 1430,
		"minecraft:arrow" => 895,
		"minecraft:axolotl_bucket" => 1023,
		"minecraft:axolotl_spawn_egg" => 1132,
		"minecraft:azalea" => 205,
		"minecraft:azalea_leaves" => 191,
		"minecraft:azure_bluet" => 235,
		"minecraft:baked_potato" => 1225,
		"minecraft:bamboo" => 269,
		"minecraft:bamboo_block" => 147,
		"minecraft:bamboo_button" => 760,
		"minecraft:bamboo_chest_raft" => 882,
		"minecraft:bamboo_door" => 789,
		"minecraft:bamboo_fence" => 353,
		"minecraft:bamboo_fence_gate" => 830,
		"minecraft:bamboo_hanging_sign" => 1009,
		"minecraft:bamboo_mosaic" => 48,
		"minecraft:bamboo_mosaic_slab" => 280,
		"minecraft:bamboo_mosaic_stairs" => 451,
		"minecraft:bamboo_planks" => 45,
		"minecraft:bamboo_pressure_plate" => 776,
		"minecraft:bamboo_raft" => 881,
		"minecraft:bamboo_shelf" => 306,
		"minecraft:bamboo_sign" => 997,
		"minecraft:bamboo_slab" => 279,
		"minecraft:bamboo_stairs" => 450,
		"minecraft:bamboo_trapdoor" => 810,
		"minecraft:barrel" => 1337,
		"minecraft:barrier" => 502,
		"minecraft:basalt" => 362,
		"minecraft:bat_spawn_egg" => 1133,
		"minecraft:beacon" => 455,
		"minecraft:bedrock" => 58,
		"minecraft:bee_nest" => 1362,
		"minecraft:bee_spawn_egg" => 1134,
		"minecraft:beef" => 1110,
		"minecraft:beehive" => 1363,
		"minecraft:beetroot" => 1282,
		"minecraft:beetroot_seeds" => 1283,
		"minecraft:beetroot_soup" => 1284,
		"minecraft:bell" => 1345,
		"minecraft:big_dripleaf" => 267,
		"minecraft:birch_boat" => 867,
		"minecraft:birch_button" => 753,
		"minecraft:birch_chest_boat" => 868,
		"minecraft:birch_door" => 782,
		"minecraft:birch_fence" => 346,
		"minecraft:birch_fence_gate" => 823,
		"minecraft:birch_hanging_sign" => 1002,
		"minecraft:birch_leaves" => 184,
		"minecraft:birch_log" => 136,
		"minecraft:birch_planks" => 38,
		"minecraft:birch_pressure_plate" => 769,
		"minecraft:birch_sapling" => 51,
		"minecraft:birch_shelf" => 307,
		"minecraft:birch_sign" => 990,
		"minecraft:birch_slab" => 272,
		"minecraft:birch_stairs" => 443,
		"minecraft:birch_trapdoor" => 803,
		"minecraft:birch_wood" => 173,
		"minecraft:black_banner" => 1276,
		"minecraft:black_bed" => 1101,
		"minecraft:black_bundle" => 1052,
		"minecraft:black_candle" => 1397,
		"minecraft:black_carpet" => 520,
		"minecraft:black_concrete" => 629,
		"minecraft:black_concrete_powder" => 645,
		"minecraft:black_dye" => 1081,
		"minecraft:black_glazed_terracotta" => 613,
		"minecraft:black_harness" => 853,
		"minecraft:black_shulker_box" => 597,
		"minecraft:black_stained_glass" => 545,
		"minecraft:black_stained_glass_pane" => 561,
		"minecraft:black_terracotta" => 501,
		"minecraft:black_wool" => 228,
		"minecraft:blackstone" => 1368,
		"minecraft:blackstone_slab" => 1369,
		"minecraft:blackstone_stairs" => 1370,
		"minecraft:blackstone_wall" => 471,
		"minecraft:blade_pottery_sherd" => 1431,
		"minecraft:blast_furnace" => 1339,
		"minecraft:blaze_powder" => 1124,
		"minecraft:blaze_rod" => 1116,
		"minecraft:blaze_spawn_egg" => 1135,
		"minecraft:blue_banner" => 1272,
		"minecraft:blue_bed" => 1097,
		"minecraft:blue_bundle" => 1048,
		"minecraft:blue_candle" => 1393,
		"minecraft:blue_carpet" => 516,
		"minecraft:blue_concrete" => 625,
		"minecraft:blue_concrete_powder" => 641,
		"minecraft:blue_dye" => 1077,
		"minecraft:blue_egg" => 1032,
		"minecraft:blue_glazed_terracotta" => 609,
		"minecraft:blue_harness" => 849,
		"minecraft:blue_ice" => 679,
		"minecraft:blue_orchid" => 233,
		"minecraft:blue_shulker_box" => 593,
		"minecraft:blue_stained_glass" => 541,
		"minecraft:blue_stained_glass_pane" => 557,
		"minecraft:blue_terracotta" => 497,
		"minecraft:blue_wool" => 224,
		"minecraft:bogged_spawn_egg" => 1136,
		"minecraft:bolt_armor_trim_smithing_template" => 1427,
		"minecraft:bone" => 1083,
		"minecraft:bone_block" => 579,
		"minecraft:bone_meal" => 1082,
		"minecraft:book" => 1029,
		"minecraft:bookshelf" => 317,
		"minecraft:bordure_indented_banner_pattern" => 1334,
		"minecraft:bow" => 894,
		"minecraft:bowl" => 892,
		"minecraft:brain_coral" => 660,
		"minecraft:brain_coral_block" => 655,
		"minecraft:brain_coral_fan" => 670,
		"minecraft:bread" => 953,
		"minecraft:breeze_rod" => 1218,
		"minecraft:breeze_spawn_egg" => 1137,
		"minecraft:brewer_pottery_sherd" => 1432,
		"minecraft:brewing_stand" => 1126,
		"minecraft:brick" => 1025,
		"minecraft:brick_slab" => 289,
		"minecraft:brick_stairs" => 419,
		"minecraft:brick_wall" => 458,
		"minecraft:bricks" => 304,
		"minecraft:brown_banner" => 1273,
		"minecraft:brown_bed" => 1098,
		"minecraft:brown_bundle" => 1049,
		"minecraft:brown_candle" => 1394,
		"minecraft:brown_carpet" => 517,
		"minecraft:brown_concrete" => 626,
		"minecraft:brown_concrete_powder" => 642,
		"minecraft:brown_dye" => 1078,
		"minecraft:brown_egg" => 1033,
		"minecraft:brown_glazed_terracotta" => 610,
		"minecraft:brown_harness" => 850,
		"minecraft:brown_mushroom" => 247,
		"minecraft:brown_mushroom_block" => 387,
		"minecraft:brown_shulker_box" => 594,
		"minecraft:brown_stained_glass" => 542,
		"minecraft:brown_stained_glass_pane" => 558,
		"minecraft:brown_terracotta" => 498,
		"minecraft:brown_wool" => 225,
		"minecraft:brush" => 1408,
		"minecraft:bubble_coral" => 661,
		"minecraft:bubble_coral_block" => 656,
		"minecraft:bubble_coral_fan" => 671,
		"minecraft:bucket" => 1012,
		"minecraft:budding_amethyst" => 89,
		"minecraft:bundle" => 1036,
		"minecraft:burn_pottery_sherd" => 1433,
		"minecraft:bush" => 204,
		"minecraft:cactus" => 340,
		"minecraft:cactus_flower" => 341,
		"minecraft:cake" => 1085,
		"minecraft:calcite" => 11,
		"minecraft:calibrated_sculk_sensor" => 743,
		"minecraft:camel_spawn_egg" => 1139,
		"minecraft:campfire" => 1358,
		"minecraft:candle" => 1381,
		"minecraft:carrot" => 1223,
		"minecraft:carrot_on_a_stick" => 859,
		"minecraft:cartography_table" => 1340,
		"minecraft:carved_pumpkin" => 357,
		"minecraft:cat_spawn_egg" => 1138,
		"minecraft:cauldron" => 1127,
		"minecraft:cave_spider_spawn_egg" => 1140,
		"minecraft:chain_command_block" => 574,
		"minecraft:chainmail_boots" => 965,
		"minecraft:chainmail_chestplate" => 963,
		"minecraft:chainmail_helmet" => 962,
		"minecraft:chainmail_leggings" => 964,
		"minecraft:charcoal" => 897,
		"minecraft:cherry_boat" => 873,
		"minecraft:cherry_button" => 756,
		"minecraft:cherry_chest_boat" => 874,
		"minecraft:cherry_door" => 785,
		"minecraft:cherry_fence" => 349,
		"minecraft:cherry_fence_gate" => 826,
		"minecraft:cherry_hanging_sign" => 1005,
		"minecraft:cherry_leaves" => 187,
		"minecraft:cherry_log" => 139,
		"minecraft:cherry_planks" => 41,
		"minecraft:cherry_pressure_plate" => 772,
		"minecraft:cherry_sapling" => 54,
		"minecraft:cherry_shelf" => 308,
		"minecraft:cherry_sign" => 993,
		"minecraft:cherry_slab" => 275,
		"minecraft:cherry_stairs" => 446,
		"minecraft:cherry_trapdoor" => 806,
		"minecraft:cherry_wood" => 176,
		"minecraft:chest" => 331,
		"minecraft:chest_minecart" => 855,
		"minecraft:chicken" => 1112,
		"minecraft:chicken_spawn_egg" => 1141,
		"minecraft:chipped_anvil" => 479,
		"minecraft:chiseled_bookshelf" => 318,
		"minecraft:chiseled_copper" => 98,
		"minecraft:chiseled_deepslate" => 385,
		"minecraft:chiseled_nether_bricks" => 426,
		"minecraft:chiseled_polished_blackstone" => 1375,
		"minecraft:chiseled_quartz_block" => 481,
		"minecraft:chiseled_red_sandstone" => 570,
		"minecraft:chiseled_resin_bricks" => 418,
		"minecraft:chiseled_sandstone" => 199,
		"minecraft:chiseled_stone_bricks" => 378,
		"minecraft:chiseled_tuff" => 16,
		"minecraft:chiseled_tuff_bricks" => 25,
		"minecraft:chorus_flower" => 325,
		"minecraft:chorus_fruit" => 1278,
		"minecraft:chorus_plant" => 324,
		"minecraft:clay" => 342,
		"minecraft:clay_ball" => 1026,
		"minecraft:clock" => 1054,
		"minecraft:closed_eyeblossom" => 231,
		"minecraft:coal" => 896,
		"minecraft:coal_block" => 83,
		"minecraft:coal_ore" => 64,
		"minecraft:coarse_dirt" => 29,
		"minecraft:coast_armor_trim_smithing_template" => 1412,
		"minecraft:cobbled_deepslate" => 9,
		"minecraft:cobbled_deepslate_slab" => 712,
		"minecraft:cobbled_deepslate_stairs" => 695,
		"minecraft:cobbled_deepslate_wall" => 474,
		"minecraft:cobblestone" => 35,
		"minecraft:cobblestone_slab" => 288,
		"minecraft:cobblestone_stairs" => 336,
		"minecraft:cobblestone_wall" => 456,
		"minecraft:cobweb" => 201,
		"minecraft:cocoa_beans" => 1065,
		"minecraft:cod" => 1057,
		"minecraft:cod_bucket" => 1021,
		"minecraft:cod_spawn_egg" => 1142,
		"minecraft:command_block" => 454,
		"minecraft:command_block_minecart" => 1258,
		"minecraft:comparator" => 721,
		"minecraft:compass" => 1034,
		"minecraft:composter" => 1336,
		"minecraft:conduit" => 680,
		"minecraft:cooked_beef" => 1111,
		"minecraft:cooked_chicken" => 1113,
		"minecraft:cooked_cod" => 1061,
		"minecraft:cooked_mutton" => 1260,
		"minecraft:cooked_porkchop" => 984,
		"minecraft:cooked_rabbit" => 1246,
		"minecraft:cooked_salmon" => 1062,
		"minecraft:cookie" => 1102,
		"minecraft:copper_axe" => 919,
		"minecraft:copper_bars" => 391,
		"minecraft:copper_block" => 91,
		"minecraft:copper_boots" => 961,
		"minecraft:copper_bulb" => 1459,
		"minecraft:copper_chain" => 400,
		"minecraft:copper_chest" => 1467,
		"minecraft:copper_chestplate" => 959,
		"minecraft:copper_door" => 792,
		"minecraft:copper_golem_spawn_egg" => 1143,
		"minecraft:copper_golem_statue" => 1475,
		"minecraft:copper_grate" => 1451,
		"minecraft:copper_helmet" => 958,
		"minecraft:copper_hoe" => 920,
		"minecraft:copper_horse_armor" => 1251,
		"minecraft:copper_ingot" => 906,
		"minecraft:copper_lantern" => 1348,
		"minecraft:copper_leggings" => 960,
		"minecraft:copper_nugget" => 1294,
		"minecraft:copper_ore" => 68,
		"minecraft:copper_pickaxe" => 918,
		"minecraft:copper_shovel" => 917,
		"minecraft:copper_sword" => 916,
		"minecraft:copper_torch" => 366,
		"minecraft:copper_trapdoor" => 813,
		"minecraft:cornflower" => 241,
		"minecraft:cow_spawn_egg" => 1144,
		"minecraft:cracked_deepslate_bricks" => 382,
		"minecraft:cracked_deepslate_tiles" => 384,
		"minecraft:cracked_nether_bricks" => 425,
		"minecraft:cracked_polished_blackstone_bricks" => 1379,
		"minecraft:cracked_stone_bricks" => 377,
		"minecraft:crafter" => 1103,
		"minecraft:crafting_table" => 332,
		"minecraft:creaking_heart" => 330,
		"minecraft:creaking_spawn_egg" => 1208,
		"minecraft:creeper_banner_pattern" => 1326,
		"minecraft:creeper_head" => 1233,
		"minecraft:creeper_spawn_egg" => 1145,
		"minecraft:crimson_button" => 761,
		"minecraft:crimson_door" => 790,
		"minecraft:crimson_fence" => 354,
		"minecraft:crimson_fence_gate" => 831,
		"minecraft:crimson_fungus" => 249,
		"minecraft:crimson_hanging_sign" => 1010,
		"minecraft:crimson_hyphae" => 180,
		"minecraft:crimson_nylium" => 33,
		"minecraft:crimson_planks" => 46,
		"minecraft:crimson_pressure_plate" => 777,
		"minecraft:crimson_roots" => 251,
		"minecraft:crimson_shelf" => 309,
		"minecraft:crimson_sign" => 998,
		"minecraft:crimson_slab" => 281,
		"minecraft:crimson_stairs" => 452,
		"minecraft:crimson_stem" => 145,
		"minecraft:crimson_trapdoor" => 811,
		"minecraft:crossbow" => 1322,
		"minecraft:crying_obsidian" => 1367,
		"minecraft:cut_copper" => 102,
		"minecraft:cut_copper_slab" => 110,
		"minecraft:cut_copper_stairs" => 106,
		"minecraft:cut_red_sandstone" => 571,
		"minecraft:cut_red_sandstone_slab" => 295,
		"minecraft:cut_sandstone" => 200,
		"minecraft:cut_sandstone_slab" => 286,
		"minecraft:cyan_banner" => 1270,
		"minecraft:cyan_bed" => 1095,
		"minecraft:cyan_bundle" => 1046,
		"minecraft:cyan_candle" => 1391,
		"minecraft:cyan_carpet" => 514,
		"minecraft:cyan_concrete" => 623,
		"minecraft:cyan_concrete_powder" => 639,
		"minecraft:cyan_dye" => 1075,
		"minecraft:cyan_glazed_terracotta" => 607,
		"minecraft:cyan_harness" => 847,
		"minecraft:cyan_shulker_box" => 591,
		"minecraft:cyan_stained_glass" => 539,
		"minecraft:cyan_stained_glass_pane" => 555,
		"minecraft:cyan_terracotta" => 495,
		"minecraft:cyan_wool" => 222,
		"minecraft:damaged_anvil" => 480,
		"minecraft:dandelion" => 229,
		"minecraft:danger_pottery_sherd" => 1434,
		"minecraft:dark_oak_boat" => 875,
		"minecraft:dark_oak_button" => 757,
		"minecraft:dark_oak_chest_boat" => 876,
		"minecraft:dark_oak_door" => 786,
		"minecraft:dark_oak_fence" => 350,
		"minecraft:dark_oak_fence_gate" => 827,
		"minecraft:dark_oak_hanging_sign" => 1006,
		"minecraft:dark_oak_leaves" => 188,
		"minecraft:dark_oak_log" => 141,
		"minecraft:dark_oak_planks" => 42,
		"minecraft:dark_oak_pressure_plate" => 773,
		"minecraft:dark_oak_sapling" => 55,
		"minecraft:dark_oak_shelf" => 310,
		"minecraft:dark_oak_sign" => 994,
		"minecraft:dark_oak_slab" => 276,
		"minecraft:dark_oak_stairs" => 447,
		"minecraft:dark_oak_trapdoor" => 807,
		"minecraft:dark_oak_wood" => 178,
		"minecraft:dark_prismarine" => 564,
		"minecraft:dark_prismarine_slab" => 299,
		"minecraft:dark_prismarine_stairs" => 567,
		"minecraft:daylight_detector" => 741,
		"minecraft:dead_brain_coral" => 664,
		"minecraft:dead_brain_coral_block" => 650,
		"minecraft:dead_brain_coral_fan" => 675,
		"minecraft:dead_bubble_coral" => 665,
		"minecraft:dead_bubble_coral_block" => 651,
		"minecraft:dead_bubble_coral_fan" => 676,
		"minecraft:dead_bush" => 207,
		"minecraft:dead_fire_coral" => 666,
		"minecraft:dead_fire_coral_block" => 652,
		"minecraft:dead_fire_coral_fan" => 677,
		"minecraft:dead_horn_coral" => 667,
		"minecraft:dead_horn_coral_block" => 653,
		"minecraft:dead_horn_coral_fan" => 678,
		"minecraft:dead_tube_coral" => 668,
		"minecraft:dead_tube_coral_block" => 649,
		"minecraft:dead_tube_coral_fan" => 674,
		"minecraft:debug_stick" => 1296,
		"minecraft:decorated_pot" => 319,
		"minecraft:deepslate" => 8,
		"minecraft:deepslate_brick_slab" => 714,
		"minecraft:deepslate_brick_stairs" => 697,
		"minecraft:deepslate_brick_wall" => 476,
		"minecraft:deepslate_bricks" => 381,
		"minecraft:deepslate_coal_ore" => 65,
		"minecraft:deepslate_copper_ore" => 69,
		"minecraft:deepslate_diamond_ore" => 79,
		"minecraft:deepslate_emerald_ore" => 75,
		"minecraft:deepslate_gold_ore" => 71,
		"minecraft:deepslate_iron_ore" => 67,
		"minecraft:deepslate_lapis_ore" => 77,
		"minecraft:deepslate_redstone_ore" => 73,
		"minecraft:deepslate_tile_slab" => 715,
		"minecraft:deepslate_tile_stairs" => 698,
		"minecraft:deepslate_tile_wall" => 477,
		"minecraft:deepslate_tiles" => 383,
		"minecraft:detector_rail" => 834,
		"minecraft:diamond" => 898,
		"minecraft:diamond_axe" => 939,
		"minecraft:diamond_block" => 93,
		"minecraft:diamond_boots" => 973,
		"minecraft:diamond_chestplate" => 971,
		"minecraft:diamond_helmet" => 970,
		"minecraft:diamond_hoe" => 940,
		"minecraft:diamond_horse_armor" => 1254,
		"minecraft:diamond_leggings" => 972,
		"minecraft:diamond_ore" => 78,
		"minecraft:diamond_pickaxe" => 938,
		"minecraft:diamond_shovel" => 937,
		"minecraft:diamond_sword" => 936,
		"minecraft:diorite" => 4,
		"minecraft:diorite_slab" => 711,
		"minecraft:diorite_stairs" => 694,
		"minecraft:diorite_wall" => 470,
		"minecraft:dirt" => 28,
		"minecraft:dirt_path" => 523,
		"minecraft:disc_fragment_5" => 1318,
		"minecraft:dispenser" => 728,
		"minecraft:dolphin_spawn_egg" => 1146,
		"minecraft:donkey_spawn_egg" => 1147,
		"minecraft:dragon_breath" => 1285,
		"minecraft:dragon_egg" => 437,
		"minecraft:dragon_head" => 1234,
		"minecraft:dried_ghast" => 648,
		"minecraft:dried_kelp" => 1107,
		"minecraft:dried_kelp_block" => 1027,
		"minecraft:dripstone_block" => 26,
		"minecraft:dropper" => 729,
		"minecraft:drowned_spawn_egg" => 1148,
		"minecraft:dune_armor_trim_smithing_template" => 1411,
		"minecraft:echo_shard" => 1407,
		"minecraft:egg" => 1031,
		"minecraft:elder_guardian_spawn_egg" => 1149,
		"minecraft:elytra" => 862,
		"minecraft:emerald" => 899,
		"minecraft:emerald_block" => 440,
		"minecraft:emerald_ore" => 74,
		"minecraft:enchanted_book" => 1240,
		"minecraft:enchanted_golden_apple" => 987,
		"minecraft:enchanting_table" => 433,
		"minecraft:end_crystal" => 1277,
		"minecraft:end_portal_frame" => 434,
		"minecraft:end_rod" => 323,
		"minecraft:end_stone" => 435,
		"minecraft:end_stone_brick_slab" => 704,
		"minecraft:end_stone_brick_stairs" => 686,
		"minecraft:end_stone_brick_wall" => 469,
		"minecraft:end_stone_bricks" => 436,
		"minecraft:ender_chest" => 439,
		"minecraft:ender_dragon_spawn_egg" => 1150,
		"minecraft:ender_eye" => 1128,
		"minecraft:ender_pearl" => 1115,
		"minecraft:enderman_spawn_egg" => 1151,
		"minecraft:endermite_spawn_egg" => 1152,
		"minecraft:evoker_spawn_egg" => 1153,
		"minecraft:experience_bottle" => 1213,
		"minecraft:explorer_pottery_sherd" => 1435,
		"minecraft:exposed_chiseled_copper" => 99,
		"minecraft:exposed_copper" => 95,
		"minecraft:exposed_copper_bars" => 392,
		"minecraft:exposed_copper_bulb" => 1460,
		"minecraft:exposed_copper_chain" => 401,
		"minecraft:exposed_copper_chest" => 1468,
		"minecraft:exposed_copper_door" => 793,
		"minecraft:exposed_copper_golem_statue" => 1476,
		"minecraft:exposed_copper_grate" => 1452,
		"minecraft:exposed_copper_lantern" => 1349,
		"minecraft:exposed_copper_trapdoor" => 814,
		"minecraft:exposed_cut_copper" => 103,
		"minecraft:exposed_cut_copper_slab" => 111,
		"minecraft:exposed_cut_copper_stairs" => 107,
		"minecraft:exposed_lightning_rod" => 734,
		"minecraft:eye_armor_trim_smithing_template" => 1415,
		"minecraft:farmland" => 333,
		"minecraft:feather" => 949,
		"minecraft:fermented_spider_eye" => 1123,
		"minecraft:fern" => 203,
		"minecraft:field_masoned_banner_pattern" => 1333,
		"minecraft:filled_map" => 1104,
		"minecraft:fire_charge" => 1214,
		"minecraft:fire_coral" => 662,
		"minecraft:fire_coral_block" => 657,
		"minecraft:fire_coral_fan" => 672,
		"minecraft:firefly_bush" => 208,
		"minecraft:firework_rocket" => 1238,
		"minecraft:firework_star" => 1239,
		"minecraft:fishing_rod" => 1053,
		"minecraft:fletching_table" => 1341,
		"minecraft:flint" => 982,
		"minecraft:flint_and_steel" => 891,
		"minecraft:flow_armor_trim_smithing_template" => 1426,
		"minecraft:flow_banner_pattern" => 1331,
		"minecraft:flow_pottery_sherd" => 1436,
		"minecraft:flower_banner_pattern" => 1325,
		"minecraft:flower_pot" => 1222,
		"minecraft:flowering_azalea" => 206,
		"minecraft:flowering_azalea_leaves" => 192,
		"minecraft:fox_spawn_egg" => 1154,
		"minecraft:friend_pottery_sherd" => 1437,
		"minecraft:frog_spawn_egg" => 1155,
		"minecraft:frogspawn" => 1406,
		"minecraft:furnace" => 334,
		"minecraft:furnace_minecart" => 856,
		"minecraft:ghast_spawn_egg" => 1156,
		"minecraft:ghast_tear" => 1117,
		"minecraft:gilded_blackstone" => 1371,
		"minecraft:glass" => 195,
		"minecraft:glass_bottle" => 1120,
		"minecraft:glass_pane" => 408,
		"minecraft:glistering_melon_slice" => 1129,
		"minecraft:globe_banner_pattern" => 1329,
		"minecraft:glow_berries" => 1357,
		"minecraft:glow_ink_sac" => 1064,
		"minecraft:glow_item_frame" => 1221,
		"minecraft:glow_lichen" => 411,
		"minecraft:glow_squid_spawn_egg" => 1158,
		"minecraft:glowstone" => 367,
		"minecraft:glowstone_dust" => 1056,
		"minecraft:goat_horn" => 1335,
		"minecraft:goat_spawn_egg" => 1159,
		"minecraft:gold_block" => 92,
		"minecraft:gold_ingot" => 908,
		"minecraft:gold_nugget" => 1118,
		"minecraft:gold_ore" => 70,
		"minecraft:golden_apple" => 986,
		"minecraft:golden_axe" => 929,
		"minecraft:golden_boots" => 977,
		"minecraft:golden_carrot" => 1228,
		"minecraft:golden_chestplate" => 975,
		"minecraft:golden_helmet" => 974,
		"minecraft:golden_hoe" => 930,
		"minecraft:golden_horse_armor" => 1253,
		"minecraft:golden_leggings" => 976,
		"minecraft:golden_pickaxe" => 928,
		"minecraft:golden_shovel" => 927,
		"minecraft:golden_sword" => 926,
		"minecraft:granite" => 2,
		"minecraft:granite_slab" => 707,
		"minecraft:granite_stairs" => 690,
		"minecraft:granite_wall" => 462,
		"minecraft:grass_block" => 27,
		"minecraft:gravel" => 63,
		"minecraft:gray_banner" => 1268,
		"minecraft:gray_bed" => 1093,
		"minecraft:gray_bundle" => 1044,
		"minecraft:gray_candle" => 1389,
		"minecraft:gray_carpet" => 512,
		"minecraft:gray_concrete" => 621,
		"minecraft:gray_concrete_powder" => 637,
		"minecraft:gray_dye" => 1073,
		"minecraft:gray_glazed_terracotta" => 605,
		"minecraft:gray_harness" => 845,
		"minecraft:gray_shulker_box" => 589,
		"minecraft:gray_stained_glass" => 537,
		"minecraft:gray_stained_glass_pane" => 553,
		"minecraft:gray_terracotta" => 493,
		"minecraft:gray_wool" => 220,
		"minecraft:green_banner" => 1274,
		"minecraft:green_bed" => 1099,
		"minecraft:green_bundle" => 1050,
		"minecraft:green_candle" => 1395,
		"minecraft:green_carpet" => 518,
		"minecraft:green_concrete" => 627,
		"minecraft:green_concrete_powder" => 643,
		"minecraft:green_dye" => 1079,
		"minecraft:green_glazed_terracotta" => 611,
		"minecraft:green_harness" => 851,
		"minecraft:green_shulker_box" => 595,
		"minecraft:green_stained_glass" => 543,
		"minecraft:green_stained_glass_pane" => 559,
		"minecraft:green_terracotta" => 499,
		"minecraft:green_wool" => 226,
		"minecraft:grindstone" => 1342,
		"minecraft:guardian_spawn_egg" => 1160,
		"minecraft:gunpowder" => 950,
		"minecraft:guster_banner_pattern" => 1332,
		"minecraft:guster_pottery_sherd" => 1438,
		"minecraft:hanging_roots" => 266,
		"minecraft:happy_ghast_spawn_egg" => 1157,
		"minecraft:hay_block" => 504,
		"minecraft:heart_of_the_sea" => 1321,
		"minecraft:heart_pottery_sherd" => 1439,
		"minecraft:heartbreak_pottery_sherd" => 1440,
		"minecraft:heavy_core" => 87,
		"minecraft:heavy_weighted_pressure_plate" => 766,
		"minecraft:hoglin_spawn_egg" => 1161,
		"minecraft:honey_block" => 725,
		"minecraft:honey_bottle" => 1364,
		"minecraft:honeycomb" => 1361,
		"minecraft:honeycomb_block" => 1365,
		"minecraft:hopper" => 727,
		"minecraft:hopper_minecart" => 858,
		"minecraft:horn_coral" => 663,
		"minecraft:horn_coral_block" => 658,
		"minecraft:horn_coral_fan" => 673,
		"minecraft:horse_spawn_egg" => 1162,
		"minecraft:host_armor_trim_smithing_template" => 1425,
		"minecraft:howl_pottery_sherd" => 1441,
		"minecraft:husk_spawn_egg" => 1163,
		"minecraft:ice" => 338,
		"minecraft:infested_chiseled_stone_bricks" => 373,
		"minecraft:infested_cobblestone" => 369,
		"minecraft:infested_cracked_stone_bricks" => 372,
		"minecraft:infested_deepslate" => 374,
		"minecraft:infested_mossy_stone_bricks" => 371,
		"minecraft:infested_stone" => 368,
		"minecraft:infested_stone_bricks" => 370,
		"minecraft:ink_sac" => 1063,
		"minecraft:iron_axe" => 934,
		"minecraft:iron_bars" => 390,
		"minecraft:iron_block" => 90,
		"minecraft:iron_boots" => 969,
		"minecraft:iron_chain" => 399,
		"minecraft:iron_chestplate" => 967,
		"minecraft:iron_door" => 779,
		"minecraft:iron_golem_spawn_egg" => 1164,
		"minecraft:iron_helmet" => 966,
		"minecraft:iron_hoe" => 935,
		"minecraft:iron_horse_armor" => 1252,
		"minecraft:iron_ingot" => 904,
		"minecraft:iron_leggings" => 968,
		"minecraft:iron_nugget" => 1293,
		"minecraft:iron_ore" => 66,
		"minecraft:iron_pickaxe" => 933,
		"minecraft:iron_shovel" => 932,
		"minecraft:iron_sword" => 931,
		"minecraft:iron_trapdoor" => 800,
		"minecraft:item_frame" => 1220,
		"minecraft:jack_o_lantern" => 358,
		"minecraft:jigsaw" => 884,
		"minecraft:jukebox" => 343,
		"minecraft:jungle_boat" => 869,
		"minecraft:jungle_button" => 754,
		"minecraft:jungle_chest_boat" => 870,
		"minecraft:jungle_door" => 783,
		"minecraft:jungle_fence" => 347,
		"minecraft:jungle_fence_gate" => 824,
		"minecraft:jungle_hanging_sign" => 1003,
		"minecraft:jungle_leaves" => 185,
		"minecraft:jungle_log" => 137,
		"minecraft:jungle_planks" => 39,
		"minecraft:jungle_pressure_plate" => 770,
		"minecraft:jungle_sapling" => 52,
		"minecraft:jungle_shelf" => 311,
		"minecraft:jungle_sign" => 991,
		"minecraft:jungle_slab" => 273,
		"minecraft:jungle_stairs" => 444,
		"minecraft:jungle_trapdoor" => 804,
		"minecraft:jungle_wood" => 174,
		"minecraft:kelp" => 257,
		"minecraft:knowledge_book" => 1295,
		"minecraft:ladder" => 335,
		"minecraft:lantern" => 1346,
		"minecraft:lapis_block" => 197,
		"minecraft:lapis_lazuli" => 900,
		"minecraft:lapis_ore" => 76,
		"minecraft:large_amethyst_bud" => 1400,
		"minecraft:large_fern" => 529,
		"minecraft:lava_bucket" => 1014,
		"minecraft:lead" => 1256,
		"minecraft:leaf_litter" => 260,
		"minecraft:leather" => 1017,
		"minecraft:leather_boots" => 957,
		"minecraft:leather_chestplate" => 955,
		"minecraft:leather_helmet" => 954,
		"minecraft:leather_horse_armor" => 1255,
		"minecraft:leather_leggings" => 956,
		"minecraft:lectern" => 730,
		"minecraft:lever" => 732,
		"minecraft:light" => 503,
		"minecraft:light_blue_banner" => 1264,
		"minecraft:light_blue_bed" => 1089,
		"minecraft:light_blue_bundle" => 1040,
		"minecraft:light_blue_candle" => 1385,
		"minecraft:light_blue_carpet" => 508,
		"minecraft:light_blue_concrete" => 617,
		"minecraft:light_blue_concrete_powder" => 633,
		"minecraft:light_blue_dye" => 1069,
		"minecraft:light_blue_glazed_terracotta" => 601,
		"minecraft:light_blue_harness" => 841,
		"minecraft:light_blue_shulker_box" => 585,
		"minecraft:light_blue_stained_glass" => 533,
		"minecraft:light_blue_stained_glass_pane" => 549,
		"minecraft:light_blue_terracotta" => 489,
		"minecraft:light_blue_wool" => 216,
		"minecraft:light_gray_banner" => 1269,
		"minecraft:light_gray_bed" => 1094,
		"minecraft:light_gray_bundle" => 1045,
		"minecraft:light_gray_candle" => 1390,
		"minecraft:light_gray_carpet" => 513,
		"minecraft:light_gray_concrete" => 622,
		"minecraft:light_gray_concrete_powder" => 638,
		"minecraft:light_gray_dye" => 1074,
		"minecraft:light_gray_glazed_terracotta" => 606,
		"minecraft:light_gray_harness" => 846,
		"minecraft:light_gray_shulker_box" => 590,
		"minecraft:light_gray_stained_glass" => 538,
		"minecraft:light_gray_stained_glass_pane" => 554,
		"minecraft:light_gray_terracotta" => 494,
		"minecraft:light_gray_wool" => 221,
		"minecraft:light_weighted_pressure_plate" => 765,
		"minecraft:lightning_rod" => 733,
		"minecraft:lilac" => 525,
		"minecraft:lily_of_the_valley" => 242,
		"minecraft:lily_pad" => 423,
		"minecraft:lime_banner" => 1266,
		"minecraft:lime_bed" => 1091,
		"minecraft:lime_bundle" => 1042,
		"minecraft:lime_candle" => 1387,
		"minecraft:lime_carpet" => 510,
		"minecraft:lime_concrete" => 619,
		"minecraft:lime_concrete_powder" => 635,
		"minecraft:lime_dye" => 1071,
		"minecraft:lime_glazed_terracotta" => 603,
		"minecraft:lime_harness" => 843,
		"minecraft:lime_shulker_box" => 587,
		"minecraft:lime_stained_glass" => 535,
		"minecraft:lime_stained_glass_pane" => 551,
		"minecraft:lime_terracotta" => 491,
		"minecraft:lime_wool" => 218,
		"minecraft:lingering_potion" => 1289,
		"minecraft:llama_spawn_egg" => 1165,
		"minecraft:lodestone" => 1366,
		"minecraft:loom" => 1324,
		"minecraft:mace" => 1219,
		"minecraft:magenta_banner" => 1263,
		"minecraft:magenta_bed" => 1088,
		"minecraft:magenta_bundle" => 1039,
		"minecraft:magenta_candle" => 1384,
		"minecraft:magenta_carpet" => 507,
		"minecraft:magenta_concrete" => 616,
		"minecraft:magenta_concrete_powder" => 632,
		"minecraft:magenta_dye" => 1068,
		"minecraft:magenta_glazed_terracotta" => 600,
		"minecraft:magenta_harness" => 840,
		"minecraft:magenta_shulker_box" => 584,
		"minecraft:magenta_stained_glass" => 532,
		"minecraft:magenta_stained_glass_pane" => 548,
		"minecraft:magenta_terracotta" => 488,
		"minecraft:magenta_wool" => 215,
		"minecraft:magma_block" => 575,
		"minecraft:magma_cream" => 1125,
		"minecraft:magma_cube_spawn_egg" => 1166,
		"minecraft:mangrove_boat" => 879,
		"minecraft:mangrove_button" => 759,
		"minecraft:mangrove_chest_boat" => 880,
		"minecraft:mangrove_door" => 788,
		"minecraft:mangrove_fence" => 352,
		"minecraft:mangrove_fence_gate" => 829,
		"minecraft:mangrove_hanging_sign" => 1008,
		"minecraft:mangrove_leaves" => 190,
		"minecraft:mangrove_log" => 142,
		"minecraft:mangrove_planks" => 44,
		"minecraft:mangrove_pressure_plate" => 775,
		"minecraft:mangrove_propagule" => 57,
		"minecraft:mangrove_roots" => 143,
		"minecraft:mangrove_shelf" => 312,
		"minecraft:mangrove_sign" => 996,
		"minecraft:mangrove_slab" => 278,
		"minecraft:mangrove_stairs" => 449,
		"minecraft:mangrove_trapdoor" => 809,
		"minecraft:mangrove_wood" => 179,
		"minecraft:map" => 1227,
		"minecraft:medium_amethyst_bud" => 1399,
		"minecraft:melon" => 409,
		"minecraft:melon_seeds" => 1109,
		"minecraft:melon_slice" => 1106,
		"minecraft:milk_bucket" => 1018,
		"minecraft:minecart" => 854,
		"minecraft:miner_pottery_sherd" => 1442,
		"minecraft:mojang_banner_pattern" => 1328,
		"minecraft:mooshroom_spawn_egg" => 1167,
		"minecraft:moss_block" => 262,
		"minecraft:moss_carpet" => 261,
		"minecraft:mossy_cobblestone" => 320,
		"minecraft:mossy_cobblestone_slab" => 703,
		"minecraft:mossy_cobblestone_stairs" => 685,
		"minecraft:mossy_cobblestone_wall" => 457,
		"minecraft:mossy_stone_brick_slab" => 701,
		"minecraft:mossy_stone_brick_stairs" => 683,
		"minecraft:mossy_stone_brick_wall" => 461,
		"minecraft:mossy_stone_bricks" => 376,
		"minecraft:mourner_pottery_sherd" => 1443,
		"minecraft:mud" => 32,
		"minecraft:mud_brick_slab" => 291,
		"minecraft:mud_brick_stairs" => 421,
		"minecraft:mud_brick_wall" => 464,
		"minecraft:mud_bricks" => 380,
		"minecraft:muddy_mangrove_roots" => 144,
		"minecraft:mule_spawn_egg" => 1168,
		"minecraft:mushroom_stem" => 389,
		"minecraft:mushroom_stew" => 947,
		"minecraft:music_disc_11" => 1310,
		"minecraft:music_disc_13" => 1297,
		"minecraft:music_disc_5" => 1314,
		"minecraft:music_disc_blocks" => 1299,
		"minecraft:music_disc_cat" => 1298,
		"minecraft:music_disc_chirp" => 1300,
		"minecraft:music_disc_creator" => 1301,
		"minecraft:music_disc_creator_music_box" => 1302,
		"minecraft:music_disc_far" => 1303,
		"minecraft:music_disc_lava_chicken" => 1304,
		"minecraft:music_disc_mall" => 1305,
		"minecraft:music_disc_mellohi" => 1306,
		"minecraft:music_disc_otherside" => 1312,
		"minecraft:music_disc_pigstep" => 1315,
		"minecraft:music_disc_precipice" => 1316,
		"minecraft:music_disc_relic" => 1313,
		"minecraft:music_disc_stal" => 1307,
		"minecraft:music_disc_strad" => 1308,
		"minecraft:music_disc_tears" => 1317,
		"minecraft:music_disc_wait" => 1311,
		"minecraft:music_disc_ward" => 1309,
		"minecraft:mutton" => 1259,
		"minecraft:mycelium" => 422,
		"minecraft:name_tag" => 1257,
		"minecraft:nautilus_shell" => 1320,
		"minecraft:nether_brick" => 1241,
		"minecraft:nether_brick_fence" => 427,
		"minecraft:nether_brick_slab" => 292,
		"minecraft:nether_brick_stairs" => 428,
		"minecraft:nether_brick_wall" => 465,
		"minecraft:nether_bricks" => 424,
		"minecraft:nether_gold_ore" => 80,
		"minecraft:nether_quartz_ore" => 81,
		"minecraft:nether_sprouts" => 253,
		"minecraft:nether_star" => 1236,
		"minecraft:nether_wart" => 1119,
		"minecraft:nether_wart_block" => 576,
		"minecraft:netherite_axe" => 944,
		"minecraft:netherite_block" => 94,
		"minecraft:netherite_boots" => 981,
		"minecraft:netherite_chestplate" => 979,
		"minecraft:netherite_helmet" => 978,
		"minecraft:netherite_hoe" => 945,
		"minecraft:netherite_ingot" => 909,
		"minecraft:netherite_leggings" => 980,
		"minecraft:netherite_pickaxe" => 943,
		"minecraft:netherite_scrap" => 910,
		"minecraft:netherite_shovel" => 942,
		"minecraft:netherite_sword" => 941,
		"minecraft:netherite_upgrade_smithing_template" => 1409,
		"minecraft:netherrack" => 359,
		"minecraft:note_block" => 748,
		"minecraft:oak_boat" => 863,
		"minecraft:oak_button" => 751,
		"minecraft:oak_chest_boat" => 864,
		"minecraft:oak_door" => 780,
		"minecraft:oak_fence" => 344,
		"minecraft:oak_fence_gate" => 821,
		"minecraft:oak_hanging_sign" => 1000,
		"minecraft:oak_leaves" => 182,
		"minecraft:oak_log" => 134,
		"minecraft:oak_planks" => 36,
		"minecraft:oak_pressure_plate" => 767,
		"minecraft:oak_sapling" => 49,
		"minecraft:oak_shelf" => 313,
		"minecraft:oak_sign" => 988,
		"minecraft:oak_slab" => 270,
		"minecraft:oak_stairs" => 441,
		"minecraft:oak_trapdoor" => 801,
		"minecraft:oak_wood" => 171,
		"minecraft:observer" => 726,
		"minecraft:obsidian" => 321,
		"minecraft:ocelot_spawn_egg" => 1169,
		"minecraft:ochre_froglight" => 1403,
		"minecraft:ominous_bottle" => 1487,
		"minecraft:ominous_trial_key" => 1485,
		"minecraft:open_eyeblossom" => 230,
		"minecraft:orange_banner" => 1262,
		"minecraft:orange_bed" => 1087,
		"minecraft:orange_bundle" => 1038,
		"minecraft:orange_candle" => 1383,
		"minecraft:orange_carpet" => 506,
		"minecraft:orange_concrete" => 615,
		"minecraft:orange_concrete_powder" => 631,
		"minecraft:orange_dye" => 1067,
		"minecraft:orange_glazed_terracotta" => 599,
		"minecraft:orange_harness" => 839,
		"minecraft:orange_shulker_box" => 583,
		"minecraft:orange_stained_glass" => 531,
		"minecraft:orange_stained_glass_pane" => 547,
		"minecraft:orange_terracotta" => 487,
		"minecraft:orange_tulip" => 237,
		"minecraft:orange_wool" => 214,
		"minecraft:oxeye_daisy" => 240,
		"minecraft:oxidized_chiseled_copper" => 101,
		"minecraft:oxidized_copper" => 97,
		"minecraft:oxidized_copper_bars" => 394,
		"minecraft:oxidized_copper_bulb" => 1462,
		"minecraft:oxidized_copper_chain" => 403,
		"minecraft:oxidized_copper_chest" => 1470,
		"minecraft:oxidized_copper_door" => 795,
		"minecraft:oxidized_copper_golem_statue" => 1478,
		"minecraft:oxidized_copper_grate" => 1454,
		"minecraft:oxidized_copper_lantern" => 1351,
		"minecraft:oxidized_copper_trapdoor" => 816,
		"minecraft:oxidized_cut_copper" => 105,
		"minecraft:oxidized_cut_copper_slab" => 113,
		"minecraft:oxidized_cut_copper_stairs" => 109,
		"minecraft:oxidized_lightning_rod" => 736,
		"minecraft:packed_ice" => 522,
		"minecraft:packed_mud" => 379,
		"minecraft:painting" => 985,
		"minecraft:pale_hanging_moss" => 264,
		"minecraft:pale_moss_block" => 265,
		"minecraft:pale_moss_carpet" => 263,
		"minecraft:pale_oak_boat" => 877,
		"minecraft:pale_oak_button" => 758,
		"minecraft:pale_oak_chest_boat" => 878,
		"minecraft:pale_oak_door" => 787,
		"minecraft:pale_oak_fence" => 351,
		"minecraft:pale_oak_fence_gate" => 828,
		"minecraft:pale_oak_hanging_sign" => 1007,
		"minecraft:pale_oak_leaves" => 189,
		"minecraft:pale_oak_log" => 140,
		"minecraft:pale_oak_planks" => 43,
		"minecraft:pale_oak_pressure_plate" => 774,
		"minecraft:pale_oak_sapling" => 56,
		"minecraft:pale_oak_shelf" => 314,
		"minecraft:pale_oak_sign" => 995,
		"minecraft:pale_oak_slab" => 277,
		"minecraft:pale_oak_stairs" => 448,
		"minecraft:pale_oak_trapdoor" => 808,
		"minecraft:pale_oak_wood" => 177,
		"minecraft:panda_spawn_egg" => 1170,
		"minecraft:paper" => 1028,
		"minecraft:parrot_spawn_egg" => 1171,
		"minecraft:pearlescent_froglight" => 1405,
		"minecraft:peony" => 527,
		"minecraft:petrified_oak_slab" => 287,
		"minecraft:phantom_membrane" => 861,
		"minecraft:phantom_spawn_egg" => 1172,
		"minecraft:pig_spawn_egg" => 1173,
		"minecraft:piglin_banner_pattern" => 1330,
		"minecraft:piglin_brute_spawn_egg" => 1175,
		"minecraft:piglin_head" => 1235,
		"minecraft:piglin_spawn_egg" => 1174,
		"minecraft:pillager_spawn_egg" => 1176,
		"minecraft:pink_banner" => 1267,
		"minecraft:pink_bed" => 1092,
		"minecraft:pink_bundle" => 1043,
		"minecraft:pink_candle" => 1388,
		"minecraft:pink_carpet" => 511,
		"minecraft:pink_concrete" => 620,
		"minecraft:pink_concrete_powder" => 636,
		"minecraft:pink_dye" => 1072,
		"minecraft:pink_glazed_terracotta" => 604,
		"minecraft:pink_harness" => 844,
		"minecraft:pink_petals" => 258,
		"minecraft:pink_shulker_box" => 588,
		"minecraft:pink_stained_glass" => 536,
		"minecraft:pink_stained_glass_pane" => 552,
		"minecraft:pink_terracotta" => 492,
		"minecraft:pink_tulip" => 239,
		"minecraft:pink_wool" => 219,
		"minecraft:piston" => 722,
		"minecraft:pitcher_plant" => 245,
		"minecraft:pitcher_pod" => 1281,
		"minecraft:player_head" => 1231,
		"minecraft:plenty_pottery_sherd" => 1444,
		"minecraft:podzol" => 30,
		"minecraft:pointed_dripstone" => 1402,
		"minecraft:poisonous_potato" => 1226,
		"minecraft:polar_bear_spawn_egg" => 1177,
		"minecraft:polished_andesite" => 7,
		"minecraft:polished_andesite_slab" => 710,
		"minecraft:polished_andesite_stairs" => 693,
		"minecraft:polished_basalt" => 363,
		"minecraft:polished_blackstone" => 1372,
		"minecraft:polished_blackstone_brick_slab" => 1377,
		"minecraft:polished_blackstone_brick_stairs" => 1378,
		"minecraft:polished_blackstone_brick_wall" => 473,
		"minecraft:polished_blackstone_bricks" => 1376,
		"minecraft:polished_blackstone_button" => 750,
		"minecraft:polished_blackstone_pressure_plate" => 764,
		"minecraft:polished_blackstone_slab" => 1373,
		"minecraft:polished_blackstone_stairs" => 1374,
		"minecraft:polished_blackstone_wall" => 472,
		"minecraft:polished_deepslate" => 10,
		"minecraft:polished_deepslate_slab" => 713,
		"minecraft:polished_deepslate_stairs" => 696,
		"minecraft:polished_deepslate_wall" => 475,
		"minecraft:polished_diorite" => 5,
		"minecraft:polished_diorite_slab" => 702,
		"minecraft:polished_diorite_stairs" => 684,
		"minecraft:polished_granite" => 3,
		"minecraft:polished_granite_slab" => 699,
		"minecraft:polished_granite_stairs" => 681,
		"minecraft:polished_tuff" => 17,
		"minecraft:polished_tuff_slab" => 18,
		"minecraft:polished_tuff_stairs" => 19,
		"minecraft:polished_tuff_wall" => 20,
		"minecraft:popped_chorus_fruit" => 1279,
		"minecraft:poppy" => 232,
		"minecraft:porkchop" => 983,
		"minecraft:potato" => 1224,
		"minecraft:potion" => 1121,
		"minecraft:powder_snow_bucket" => 1015,
		"minecraft:powered_rail" => 833,
		"minecraft:prismarine" => 562,
		"minecraft:prismarine_brick_slab" => 298,
		"minecraft:prismarine_brick_stairs" => 566,
		"minecraft:prismarine_bricks" => 563,
		"minecraft:prismarine_crystals" => 1244,
		"minecraft:prismarine_shard" => 1243,
		"minecraft:prismarine_slab" => 297,
		"minecraft:prismarine_stairs" => 565,
		"minecraft:prismarine_wall" => 459,
		"minecraft:prize_pottery_sherd" => 1445,
		"minecraft:pufferfish" => 1060,
		"minecraft:pufferfish_bucket" => 1019,
		"minecraft:pufferfish_spawn_egg" => 1178,
		"minecraft:pumpkin" => 356,
		"minecraft:pumpkin_pie" => 1237,
		"minecraft:pumpkin_seeds" => 1108,
		"minecraft:purple_banner" => 1271,
		"minecraft:purple_bed" => 1096,
		"minecraft:purple_bundle" => 1047,
		"minecraft:purple_candle" => 1392,
		"minecraft:purple_carpet" => 515,
		"minecraft:purple_concrete" => 624,
		"minecraft:purple_concrete_powder" => 640,
		"minecraft:purple_dye" => 1076,
		"minecraft:purple_glazed_terracotta" => 608,
		"minecraft:purple_harness" => 848,
		"minecraft:purple_shulker_box" => 592,
		"minecraft:purple_stained_glass" => 540,
		"minecraft:purple_stained_glass_pane" => 556,
		"minecraft:purple_terracotta" => 496,
		"minecraft:purple_wool" => 223,
		"minecraft:purpur_block" => 326,
		"minecraft:purpur_pillar" => 327,
		"minecraft:purpur_slab" => 296,
		"minecraft:purpur_stairs" => 328,
		"minecraft:quartz" => 901,
		"minecraft:quartz_block" => 482,
		"minecraft:quartz_bricks" => 483,
		"minecraft:quartz_pillar" => 484,
		"minecraft:quartz_slab" => 293,
		"minecraft:quartz_stairs" => 485,
		"minecraft:rabbit" => 1245,
		"minecraft:rabbit_foot" => 1248,
		"minecraft:rabbit_hide" => 1249,
		"minecraft:rabbit_spawn_egg" => 1179,
		"minecraft:rabbit_stew" => 1247,
		"minecraft:rail" => 835,
		"minecraft:raiser_armor_trim_smithing_template" => 1424,
		"minecraft:ravager_spawn_egg" => 1180,
		"minecraft:raw_copper" => 905,
		"minecraft:raw_copper_block" => 85,
		"minecraft:raw_gold" => 907,
		"minecraft:raw_gold_block" => 86,
		"minecraft:raw_iron" => 903,
		"minecraft:raw_iron_block" => 84,
		"minecraft:recovery_compass" => 1035,
		"minecraft:red_banner" => 1275,
		"minecraft:red_bed" => 1100,
		"minecraft:red_bundle" => 1051,
		"minecraft:red_candle" => 1396,
		"minecraft:red_carpet" => 519,
		"minecraft:red_concrete" => 628,
		"minecraft:red_concrete_powder" => 644,
		"minecraft:red_dye" => 1080,
		"minecraft:red_glazed_terracotta" => 612,
		"minecraft:red_harness" => 852,
		"minecraft:red_mushroom" => 248,
		"minecraft:red_mushroom_block" => 388,
		"minecraft:red_nether_brick_slab" => 709,
		"minecraft:red_nether_brick_stairs" => 692,
		"minecraft:red_nether_brick_wall" => 467,
		"minecraft:red_nether_bricks" => 578,
		"minecraft:red_sand" => 62,
		"minecraft:red_sandstone" => 569,
		"minecraft:red_sandstone_slab" => 294,
		"minecraft:red_sandstone_stairs" => 572,
		"minecraft:red_sandstone_wall" => 460,
		"minecraft:red_shulker_box" => 596,
		"minecraft:red_stained_glass" => 544,
		"minecraft:red_stained_glass_pane" => 560,
		"minecraft:red_terracotta" => 500,
		"minecraft:red_tulip" => 236,
		"minecraft:red_wool" => 227,
		"minecraft:redstone" => 717,
		"minecraft:redstone_block" => 719,
		"minecraft:redstone_lamp" => 747,
		"minecraft:redstone_ore" => 72,
		"minecraft:redstone_torch" => 718,
		"minecraft:reinforced_deepslate" => 386,
		"minecraft:repeater" => 720,
		"minecraft:repeating_command_block" => 573,
		"minecraft:resin_block" => 413,
		"minecraft:resin_brick" => 1242,
		"minecraft:resin_brick_slab" => 416,
		"minecraft:resin_brick_stairs" => 415,
		"minecraft:resin_brick_wall" => 417,
		"minecraft:resin_bricks" => 414,
		"minecraft:resin_clump" => 412,
		"minecraft:respawn_anchor" => 1380,
		"minecraft:rib_armor_trim_smithing_template" => 1419,
		"minecraft:rooted_dirt" => 31,
		"minecraft:rose_bush" => 526,
		"minecraft:rotten_flesh" => 1114,
		"minecraft:saddle" => 837,
		"minecraft:salmon" => 1058,
		"minecraft:salmon_bucket" => 1020,
		"minecraft:salmon_spawn_egg" => 1181,
		"minecraft:sand" => 59,
		"minecraft:sandstone" => 198,
		"minecraft:sandstone_slab" => 285,
		"minecraft:sandstone_stairs" => 438,
		"minecraft:sandstone_wall" => 468,
		"minecraft:scaffolding" => 716,
		"minecraft:scrape_pottery_sherd" => 1446,
		"minecraft:sculk" => 429,
		"minecraft:sculk_catalyst" => 431,
		"minecraft:sculk_sensor" => 742,
		"minecraft:sculk_shrieker" => 432,
		"minecraft:sculk_vein" => 430,
		"minecraft:sea_lantern" => 568,
		"minecraft:sea_pickle" => 212,
		"minecraft:seagrass" => 211,
		"minecraft:sentry_armor_trim_smithing_template" => 1410,
		"minecraft:shaper_armor_trim_smithing_template" => 1422,
		"minecraft:sheaf_pottery_sherd" => 1447,
		"minecraft:shears" => 1105,
		"minecraft:sheep_spawn_egg" => 1182,
		"minecraft:shelter_pottery_sherd" => 1448,
		"minecraft:shield" => 1290,
		"minecraft:short_dry_grass" => 209,
		"minecraft:short_grass" => 202,
		"minecraft:shroomlight" => 1360,
		"minecraft:shulker_box" => 581,
		"minecraft:shulker_shell" => 1292,
		"minecraft:shulker_spawn_egg" => 1183,
		"minecraft:silence_armor_trim_smithing_template" => 1423,
		"minecraft:silverfish_spawn_egg" => 1184,
		"minecraft:skeleton_horse_spawn_egg" => 1186,
		"minecraft:skeleton_skull" => 1229,
		"minecraft:skeleton_spawn_egg" => 1185,
		"minecraft:skull_banner_pattern" => 1327,
		"minecraft:skull_pottery_sherd" => 1449,
		"minecraft:slime_ball" => 1030,
		"minecraft:slime_block" => 724,
		"minecraft:slime_spawn_egg" => 1187,
		"minecraft:small_amethyst_bud" => 1398,
		"minecraft:small_dripleaf" => 268,
		"minecraft:smithing_table" => 1343,
		"minecraft:smoker" => 1338,
		"minecraft:smooth_basalt" => 364,
		"minecraft:smooth_quartz" => 300,
		"minecraft:smooth_quartz_slab" => 706,
		"minecraft:smooth_quartz_stairs" => 689,
		"minecraft:smooth_red_sandstone" => 301,
		"minecraft:smooth_red_sandstone_slab" => 700,
		"minecraft:smooth_red_sandstone_stairs" => 682,
		"minecraft:smooth_sandstone" => 302,
		"minecraft:smooth_sandstone_slab" => 705,
		"minecraft:smooth_sandstone_stairs" => 688,
		"minecraft:smooth_stone" => 303,
		"minecraft:smooth_stone_slab" => 284,
		"minecraft:sniffer_egg" => 647,
		"minecraft:sniffer_spawn_egg" => 1188,
		"minecraft:snort_pottery_sherd" => 1450,
		"minecraft:snout_armor_trim_smithing_template" => 1418,
		"minecraft:snow" => 337,
		"minecraft:snow_block" => 339,
		"minecraft:snow_golem_spawn_egg" => 1189,
		"minecraft:snowball" => 1016,
		"minecraft:soul_campfire" => 1359,
		"minecraft:soul_lantern" => 1347,
		"minecraft:soul_sand" => 360,
		"minecraft:soul_soil" => 361,
		"minecraft:soul_torch" => 365,
		"minecraft:spawner" => 329,
		"minecraft:spectral_arrow" => 1287,
		"minecraft:spider_eye" => 1122,
		"minecraft:spider_spawn_egg" => 1190,
		"minecraft:spire_armor_trim_smithing_template" => 1420,
		"minecraft:splash_potion" => 1286,
		"minecraft:sponge" => 193,
		"minecraft:spore_blossom" => 246,
		"minecraft:spruce_boat" => 865,
		"minecraft:spruce_button" => 752,
		"minecraft:spruce_chest_boat" => 866,
		"minecraft:spruce_door" => 781,
		"minecraft:spruce_fence" => 345,
		"minecraft:spruce_fence_gate" => 822,
		"minecraft:spruce_hanging_sign" => 1001,
		"minecraft:spruce_leaves" => 183,
		"minecraft:spruce_log" => 135,
		"minecraft:spruce_planks" => 37,
		"minecraft:spruce_pressure_plate" => 768,
		"minecraft:spruce_sapling" => 50,
		"minecraft:spruce_shelf" => 315,
		"minecraft:spruce_sign" => 989,
		"minecraft:spruce_slab" => 271,
		"minecraft:spruce_stairs" => 442,
		"minecraft:spruce_trapdoor" => 802,
		"minecraft:spruce_wood" => 172,
		"minecraft:spyglass" => 1055,
		"minecraft:squid_spawn_egg" => 1191,
		"minecraft:stick" => 946,
		"minecraft:sticky_piston" => 723,
		"minecraft:stone" => 1,
		"minecraft:stone_axe" => 924,
		"minecraft:stone_brick_slab" => 290,
		"minecraft:stone_brick_stairs" => 420,
		"minecraft:stone_brick_wall" => 463,
		"minecraft:stone_bricks" => 375,
		"minecraft:stone_button" => 749,
		"minecraft:stone_hoe" => 925,
		"minecraft:stone_pickaxe" => 923,
		"minecraft:stone_pressure_plate" => 763,
		"minecraft:stone_shovel" => 922,
		"minecraft:stone_slab" => 283,
		"minecraft:stone_stairs" => 687,
		"minecraft:stone_sword" => 921,
		"minecraft:stonecutter" => 1344,
		"minecraft:stray_spawn_egg" => 1192,
		"minecraft:strider_spawn_egg" => 1193,
		"minecraft:string" => 948,
		"minecraft:stripped_acacia_log" => 152,
		"minecraft:stripped_acacia_wood" => 163,
		"minecraft:stripped_bamboo_block" => 170,
		"minecraft:stripped_birch_log" => 150,
		"minecraft:stripped_birch_wood" => 161,
		"minecraft:stripped_cherry_log" => 153,
		"minecraft:stripped_cherry_wood" => 164,
		"minecraft:stripped_crimson_hyphae" => 168,
		"minecraft:stripped_crimson_stem" => 157,
		"minecraft:stripped_dark_oak_log" => 154,
		"minecraft:stripped_dark_oak_wood" => 165,
		"minecraft:stripped_jungle_log" => 151,
		"minecraft:stripped_jungle_wood" => 162,
		"minecraft:stripped_mangrove_log" => 156,
		"minecraft:stripped_mangrove_wood" => 167,
		"minecraft:stripped_oak_log" => 148,
		"minecraft:stripped_oak_wood" => 159,
		"minecraft:stripped_pale_oak_log" => 155,
		"minecraft:stripped_pale_oak_wood" => 166,
		"minecraft:stripped_spruce_log" => 149,
		"minecraft:stripped_spruce_wood" => 160,
		"minecraft:stripped_warped_hyphae" => 169,
		"minecraft:stripped_warped_stem" => 158,
		"minecraft:structure_block" => 883,
		"minecraft:structure_void" => 580,
		"minecraft:sugar" => 1084,
		"minecraft:sugar_cane" => 256,
		"minecraft:sunflower" => 524,
		"minecraft:suspicious_gravel" => 61,
		"minecraft:suspicious_sand" => 60,
		"minecraft:suspicious_stew" => 1323,
		"minecraft:sweet_berries" => 1356,
		"minecraft:tadpole_bucket" => 1024,
		"minecraft:tadpole_spawn_egg" => 1194,
		"minecraft:tall_dry_grass" => 210,
		"minecraft:tall_grass" => 528,
		"minecraft:target" => 731,
		"minecraft:terracotta" => 521,
		"minecraft:test_block" => 885,
		"minecraft:test_instance_block" => 886,
		"minecraft:tide_armor_trim_smithing_template" => 1417,
		"minecraft:tinted_glass" => 196,
		"minecraft:tipped_arrow" => 1288,
		"minecraft:tnt" => 746,
		"minecraft:tnt_minecart" => 857,
		"minecraft:torch" => 322,
		"minecraft:torchflower" => 244,
		"minecraft:torchflower_seeds" => 1280,
		"minecraft:totem_of_undying" => 1291,
		"minecraft:trader_llama_spawn_egg" => 1195,
		"minecraft:trapped_chest" => 745,
		"minecraft:trial_key" => 1484,
		"minecraft:trial_spawner" => 1483,
		"minecraft:trident" => 1319,
		"minecraft:tripwire_hook" => 744,
		"minecraft:tropical_fish" => 1059,
		"minecraft:tropical_fish_bucket" => 1022,
		"minecraft:tropical_fish_spawn_egg" => 1196,
		"minecraft:tube_coral" => 659,
		"minecraft:tube_coral_block" => 654,
		"minecraft:tube_coral_fan" => 669,
		"minecraft:tuff" => 12,
		"minecraft:tuff_brick_slab" => 22,
		"minecraft:tuff_brick_stairs" => 23,
		"minecraft:tuff_brick_wall" => 24,
		"minecraft:tuff_bricks" => 21,
		"minecraft:tuff_slab" => 13,
		"minecraft:tuff_stairs" => 14,
		"minecraft:tuff_wall" => 15,
		"minecraft:turtle_egg" => 646,
		"minecraft:turtle_helmet" => 887,
		"minecraft:turtle_scute" => 888,
		"minecraft:turtle_spawn_egg" => 1197,
		"minecraft:twisting_vines" => 255,
		"minecraft:vault" => 1486,
		"minecraft:verdant_froglight" => 1404,
		"minecraft:vex_armor_trim_smithing_template" => 1416,
		"minecraft:vex_spawn_egg" => 1198,
		"minecraft:villager_spawn_egg" => 1199,
		"minecraft:vindicator_spawn_egg" => 1200,
		"minecraft:vine" => 410,
		"minecraft:wandering_trader_spawn_egg" => 1201,
		"minecraft:ward_armor_trim_smithing_template" => 1414,
		"minecraft:warden_spawn_egg" => 1202,
		"minecraft:warped_button" => 762,
		"minecraft:warped_door" => 791,
		"minecraft:warped_fence" => 355,
		"minecraft:warped_fence_gate" => 832,
		"minecraft:warped_fungus" => 250,
		"minecraft:warped_fungus_on_a_stick" => 860,
		"minecraft:warped_hanging_sign" => 1011,
		"minecraft:warped_hyphae" => 181,
		"minecraft:warped_nylium" => 34,
		"minecraft:warped_planks" => 47,
		"minecraft:warped_pressure_plate" => 778,
		"minecraft:warped_roots" => 252,
		"minecraft:warped_shelf" => 316,
		"minecraft:warped_sign" => 999,
		"minecraft:warped_slab" => 282,
		"minecraft:warped_stairs" => 453,
		"minecraft:warped_stem" => 146,
		"minecraft:warped_trapdoor" => 812,
		"minecraft:warped_wart_block" => 577,
		"minecraft:water_bucket" => 1013,
		"minecraft:waxed_chiseled_copper" => 118,
		"minecraft:waxed_copper_bars" => 395,
		"minecraft:waxed_copper_block" => 114,
		"minecraft:waxed_copper_bulb" => 1463,
		"minecraft:waxed_copper_chain" => 404,
		"minecraft:waxed_copper_chest" => 1471,
		"minecraft:waxed_copper_door" => 796,
		"minecraft:waxed_copper_golem_statue" => 1479,
		"minecraft:waxed_copper_grate" => 1455,
		"minecraft:waxed_copper_lantern" => 1352,
		"minecraft:waxed_copper_trapdoor" => 817,
		"minecraft:waxed_cut_copper" => 122,
		"minecraft:waxed_cut_copper_slab" => 130,
		"minecraft:waxed_cut_copper_stairs" => 126,
		"minecraft:waxed_exposed_chiseled_copper" => 119,
		"minecraft:waxed_exposed_copper" => 115,
		"minecraft:waxed_exposed_copper_bars" => 396,
		"minecraft:waxed_exposed_copper_bulb" => 1464,
		"minecraft:waxed_exposed_copper_chain" => 405,
		"minecraft:waxed_exposed_copper_chest" => 1472,
		"minecraft:waxed_exposed_copper_door" => 797,
		"minecraft:waxed_exposed_copper_golem_statue" => 1480,
		"minecraft:waxed_exposed_copper_grate" => 1456,
		"minecraft:waxed_exposed_copper_lantern" => 1353,
		"minecraft:waxed_exposed_copper_trapdoor" => 818,
		"minecraft:waxed_exposed_cut_copper" => 123,
		"minecraft:waxed_exposed_cut_copper_slab" => 131,
		"minecraft:waxed_exposed_cut_copper_stairs" => 127,
		"minecraft:waxed_exposed_lightning_rod" => 738,
		"minecraft:waxed_lightning_rod" => 737,
		"minecraft:waxed_oxidized_chiseled_copper" => 121,
		"minecraft:waxed_oxidized_copper" => 117,
		"minecraft:waxed_oxidized_copper_bars" => 398,
		"minecraft:waxed_oxidized_copper_bulb" => 1466,
		"minecraft:waxed_oxidized_copper_chain" => 407,
		"minecraft:waxed_oxidized_copper_chest" => 1474,
		"minecraft:waxed_oxidized_copper_door" => 799,
		"minecraft:waxed_oxidized_copper_golem_statue" => 1482,
		"minecraft:waxed_oxidized_copper_grate" => 1458,
		"minecraft:waxed_oxidized_copper_lantern" => 1355,
		"minecraft:waxed_oxidized_copper_trapdoor" => 820,
		"minecraft:waxed_oxidized_cut_copper" => 125,
		"minecraft:waxed_oxidized_cut_copper_slab" => 133,
		"minecraft:waxed_oxidized_cut_copper_stairs" => 129,
		"minecraft:waxed_oxidized_lightning_rod" => 740,
		"minecraft:waxed_weathered_chiseled_copper" => 120,
		"minecraft:waxed_weathered_copper" => 116,
		"minecraft:waxed_weathered_copper_bars" => 397,
		"minecraft:waxed_weathered_copper_bulb" => 1465,
		"minecraft:waxed_weathered_copper_chain" => 406,
		"minecraft:waxed_weathered_copper_chest" => 1473,
		"minecraft:waxed_weathered_copper_door" => 798,
		"minecraft:waxed_weathered_copper_golem_statue" => 1481,
		"minecraft:waxed_weathered_copper_grate" => 1457,
		"minecraft:waxed_weathered_copper_lantern" => 1354,
		"minecraft:waxed_weathered_copper_trapdoor" => 819,
		"minecraft:waxed_weathered_cut_copper" => 124,
		"minecraft:waxed_weathered_cut_copper_slab" => 132,
		"minecraft:waxed_weathered_cut_copper_stairs" => 128,
		"minecraft:waxed_weathered_lightning_rod" => 739,
		"minecraft:wayfinder_armor_trim_smithing_template" => 1421,
		"minecraft:weathered_chiseled_copper" => 100,
		"minecraft:weathered_copper" => 96,
		"minecraft:weathered_copper_bars" => 393,
		"minecraft:weathered_copper_bulb" => 1461,
		"minecraft:weathered_copper_chain" => 402,
		"minecraft:weathered_copper_chest" => 1469,
		"minecraft:weathered_copper_door" => 794,
		"minecraft:weathered_copper_golem_statue" => 1477,
		"minecraft:weathered_copper_grate" => 1453,
		"minecraft:weathered_copper_lantern" => 1350,
		"minecraft:weathered_copper_trapdoor" => 815,
		"minecraft:weathered_cut_copper" => 104,
		"minecraft:weathered_cut_copper_slab" => 112,
		"minecraft:weathered_cut_copper_stairs" => 108,
		"minecraft:weathered_lightning_rod" => 735,
		"minecraft:weeping_vines" => 254,
		"minecraft:wet_sponge" => 194,
		"minecraft:wheat" => 952,
		"minecraft:wheat_seeds" => 951,
		"minecraft:white_banner" => 1261,
		"minecraft:white_bed" => 1086,
		"minecraft:white_bundle" => 1037,
		"minecraft:white_candle" => 1382,
		"minecraft:white_carpet" => 505,
		"minecraft:white_concrete" => 614,
		"minecraft:white_concrete_powder" => 630,
		"minecraft:white_dye" => 1066,
		"minecraft:white_glazed_terracotta" => 598,
		"minecraft:white_harness" => 838,
		"minecraft:white_shulker_box" => 582,
		"minecraft:white_stained_glass" => 530,
		"minecraft:white_stained_glass_pane" => 546,
		"minecraft:white_terracotta" => 486,
		"minecraft:white_tulip" => 238,
		"minecraft:white_wool" => 213,
		"minecraft:wild_armor_trim_smithing_template" => 1413,
		"minecraft:wildflowers" => 259,
		"minecraft:wind_charge" => 1215,
		"minecraft:witch_spawn_egg" => 1203,
		"minecraft:wither_rose" => 243,
		"minecraft:wither_skeleton_skull" => 1230,
		"minecraft:wither_skeleton_spawn_egg" => 1205,
		"minecraft:wither_spawn_egg" => 1204,
		"minecraft:wolf_armor" => 890,
		"minecraft:wolf_spawn_egg" => 1206,
		"minecraft:wooden_axe" => 914,
		"minecraft:wooden_hoe" => 915,
		"minecraft:wooden_pickaxe" => 913,
		"minecraft:wooden_shovel" => 912,
		"minecraft:wooden_sword" => 911,
		"minecraft:writable_book" => 1216,
		"minecraft:written_book" => 1217,
		"minecraft:yellow_banner" => 1265,
		"minecraft:yellow_bed" => 1090,
		"minecraft:yellow_bundle" => 1041,
		"minecraft:yellow_candle" => 1386,
		"minecraft:yellow_carpet" => 509,
		"minecraft:yellow_concrete" => 618,
		"minecraft:yellow_concrete_powder" => 634,
		"minecraft:yellow_dye" => 1070,
		"minecraft:yellow_glazed_terracotta" => 602,
		"minecraft:yellow_harness" => 842,
		"minecraft:yellow_shulker_box" => 586,
		"minecraft:yellow_stained_glass" => 534,
		"minecraft:yellow_stained_glass_pane" => 550,
		"minecraft:yellow_terracotta" => 490,
		"minecraft:yellow_wool" => 217,
		"minecraft:zoglin_spawn_egg" => 1207,
		"minecraft:zombie_head" => 1232,
		"minecraft:zombie_horse_spawn_egg" => 1210,
		"minecraft:zombie_spawn_egg" => 1209,
		"minecraft:zombie_villager_spawn_egg" => 1211,
		"minecraft:zombified_piglin_spawn_egg" => 1212,
    x => panic!("no idea what item name {x} is"),
	};
}
pub fn get_items() -> HashMap<&'static str, Item> {
	let mut items = HashMap::new();
	items.insert("minecraft:acacia_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 871, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 755, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 872, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 784, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 348, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 825, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1004, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 186, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 138, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 40, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 771, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 53, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 305, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 992, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 274, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 445, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 805, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 175, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:activator_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 836, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:air", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 0, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:allay_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1131, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:allium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 234, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:amethyst_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 88, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:amethyst_cluster", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1401, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:amethyst_shard", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 902, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ancient_debris", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 82, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 6, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 708, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 691, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 466, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:angler_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1428, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 478, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:apple", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 893, nutrition: Some(4), saturation: Some(2.40), tool_rules: vec![] });
	items.insert("minecraft:archer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1429, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armadillo_scute", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 889, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armadillo_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1130, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armor_stand", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1250, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:arms_up_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1430, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 895, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:axolotl_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1023, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:axolotl_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1132, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:azalea", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 205, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:azalea_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 191, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:azure_bluet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 235, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:baked_potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1225, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:bamboo", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 269, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 147, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 760, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_chest_raft", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 882, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 789, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 353, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 830, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1009, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 48, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 280, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 451, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 45, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 776, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_raft", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 881, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 306, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 997, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 279, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 450, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 810, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:barrel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1337, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:barrier", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 502, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 362, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1133, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beacon", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 455, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bedrock", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 58, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bee_nest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1362, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bee_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1134, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beef", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1110, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:beehive", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1363, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beetroot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1282, nutrition: Some(1), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:beetroot_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1283, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beetroot_soup", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1284, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:bell", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1345, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:big_dripleaf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 267, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 867, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 753, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 868, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 782, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 346, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 823, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1002, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 184, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 136, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 38, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 769, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 51, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 307, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 990, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 272, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 443, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 803, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 173, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1276, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1101, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1052, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1397, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 520, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 629, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 645, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1081, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 613, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 853, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 597, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 545, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 561, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 501, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 228, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1368, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1369, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1370, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 471, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blade_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1431, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blast_furnace", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1339, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blaze_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1124, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blaze_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1116, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blaze_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1135, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1272, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1097, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1048, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1393, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 516, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 625, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 641, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1077, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1032, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 609, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 849, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 679, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_orchid", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 233, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 593, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 541, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 557, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 497, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 224, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bogged_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1136, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bolt_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1427, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1083, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 579, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone_meal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1082, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:book", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1029, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bookshelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 317, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bordure_indented_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1334, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bow", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 894, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bowl", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 892, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 660, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 655, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 670, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bread", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 953, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:breeze_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1218, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:breeze_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1137, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brewer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1432, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brewing_stand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1126, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1025, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 289, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 419, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 458, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 304, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1273, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1098, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1049, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1394, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 517, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 626, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 642, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1078, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1033, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 610, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 850, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_mushroom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 247, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_mushroom_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 387, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 594, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 542, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 558, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 498, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 225, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brush", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1408, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 661, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 656, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 671, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bucket", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1012, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:budding_amethyst", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 89, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1036, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:burn_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1433, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 204, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cactus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 340, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cactus_flower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 341, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cake", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1085, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:calcite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 11, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:calibrated_sculk_sensor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 743, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:camel_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1139, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:campfire", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1358, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1381, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:carrot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1223, nutrition: Some(3), saturation: Some(3.60), tool_rules: vec![] });
	items.insert("minecraft:carrot_on_a_stick", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 859, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cartography_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1340, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:carved_pumpkin", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 357, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1138, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cauldron", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1127, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cave_spider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1140, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chain_command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 574, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_boots", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 965, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 963, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 962, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 964, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:charcoal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 897, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 873, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 756, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 874, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 785, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 349, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 826, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1005, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 187, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 139, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 41, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 772, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 54, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 308, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 993, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 275, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 446, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 806, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 176, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 331, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chest_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 855, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chicken", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1112, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:chicken_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1141, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chipped_anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 479, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_bookshelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 318, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 98, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 385, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 426, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_polished_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1375, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_quartz_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 481, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 570, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_resin_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 418, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 199, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 378, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 16, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_tuff_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 25, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chorus_flower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 325, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chorus_fruit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1278, nutrition: Some(4), saturation: Some(2.40), tool_rules: vec![] });
	items.insert("minecraft:chorus_plant", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 324, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clay", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 342, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clay_ball", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1026, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clock", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1054, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:closed_eyeblossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 231, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 896, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 83, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 64, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coarse_dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 29, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coast_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1412, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 9, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 712, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 695, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 474, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 35, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 288, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 336, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 456, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobweb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 201, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cocoa_beans", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1065, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1057, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:cod_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1021, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cod_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1142, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 454, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:command_block_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1258, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:comparator", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 721, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:compass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1034, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:composter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1336, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:conduit", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 680, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_beef", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1111, nutrition: Some(8), saturation: Some(12.80), tool_rules: vec![] });
	items.insert("minecraft:cooked_chicken", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1113, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:cooked_cod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1061, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:cooked_mutton", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1260, nutrition: Some(6), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:cooked_porkchop", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 984, nutrition: Some(8), saturation: Some(12.80), tool_rules: vec![] });
	items.insert("minecraft:cooked_rabbit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1246, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:cooked_salmon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1062, nutrition: Some(6), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:cookie", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1102, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:copper_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 919, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 391, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 91, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 961, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1459, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 400, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1467, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 959, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 792, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1143, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1475, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1451, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 958, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 920, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:copper_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1251, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 906, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1348, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 960, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1294, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 68, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 918, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:copper_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 917, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:copper_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 916, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:copper_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 366, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 813, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cornflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 241, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cow_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1144, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_deepslate_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 382, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_deepslate_tiles", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 384, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 425, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_polished_blackstone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1379, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 377, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crafter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1103, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crafting_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 332, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creaking_heart", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 330, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creaking_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1208, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1326, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1233, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1145, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 761, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 790, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 354, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 831, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fungus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 249, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1010, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 180, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_nylium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 33, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 46, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 777, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 251, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 309, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 998, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 281, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 452, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 145, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 811, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crossbow", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1322, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crying_obsidian", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1367, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 102, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 110, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 106, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 571, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 295, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 200, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 286, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1270, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1095, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1046, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1391, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 514, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 623, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 639, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1075, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 607, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 847, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 591, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 539, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 555, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 495, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 222, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:damaged_anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 480, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dandelion", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 229, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:danger_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1434, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 875, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 757, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 876, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 786, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 350, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 827, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1006, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 188, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 141, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 42, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 773, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 55, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 310, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 994, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 276, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 447, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 807, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 178, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 564, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 299, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 567, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:daylight_detector", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 741, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 664, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 650, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 675, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bubble_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 665, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bubble_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 651, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bubble_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 676, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 207, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 666, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 652, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 677, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 667, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 653, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 678, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 668, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 649, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 674, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:debug_stick", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1296, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:decorated_pot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 319, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 8, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 714, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 697, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 476, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 381, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_coal_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 65, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_copper_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 69, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_diamond_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 79, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_emerald_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 75, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 71, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_iron_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 67, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_lapis_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 77, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_redstone_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 73, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 715, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 698, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 477, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tiles", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 383, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:detector_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 834, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 898, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 939, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:diamond_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 93, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 973, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 971, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 970, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 940, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:diamond_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1254, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 972, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 78, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 938, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:diamond_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 937, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:diamond_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 936, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:diorite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 4, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 711, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 694, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 470, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 28, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dirt_path", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 523, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:disc_fragment_5", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1318, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dispenser", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 728, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dolphin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1146, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:donkey_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1147, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_breath", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1285, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_egg", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 437, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_head", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 1234, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dried_ghast", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 648, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dried_kelp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1107, nutrition: Some(1), saturation: Some(0.60), tool_rules: vec![] });
	items.insert("minecraft:dried_kelp_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1027, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dripstone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 26, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dropper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 729, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:drowned_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1148, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dune_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1411, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:echo_shard", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1407, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1031, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:elder_guardian_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1149, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:elytra", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 862, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:emerald", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 899, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:emerald_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 440, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:emerald_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 74, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enchanted_book", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1240, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enchanted_golden_apple", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 987, nutrition: Some(4), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:enchanting_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 433, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_crystal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1277, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_portal_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 434, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 323, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 435, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 704, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 686, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 469, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 436, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 439, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_dragon_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1150, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1128, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_pearl", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1115, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enderman_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1151, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:endermite_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1152, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:evoker_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1153, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:experience_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1213, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:explorer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1435, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 99, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 95, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 392, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1460, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 401, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1468, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 793, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1476, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1452, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1349, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 814, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 103, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 111, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 107, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 734, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:eye_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1415, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:farmland", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 333, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:feather", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 949, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fermented_spider_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1123, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 203, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:field_masoned_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1333, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:filled_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1104, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_charge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1214, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 662, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 657, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 672, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firefly_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 208, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firework_rocket", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1238, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firework_star", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1239, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fishing_rod", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1053, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fletching_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1341, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flint", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 982, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flint_and_steel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 891, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1426, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1331, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1436, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flower_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1325, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flower_pot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1222, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flowering_azalea", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 206, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flowering_azalea_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 192, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fox_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1154, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:friend_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1437, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:frog_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1155, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:frogspawn", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1406, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:furnace", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 334, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:furnace_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 856, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ghast_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1156, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ghast_tear", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1117, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gilded_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1371, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 195, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1120, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 408, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glistering_melon_slice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1129, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:globe_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1329, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_berries", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1357, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:glow_ink_sac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1064, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_item_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1221, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_lichen", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 411, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_squid_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1158, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glowstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 367, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glowstone_dust", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1056, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:goat_horn", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1335, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:goat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1159, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 92, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 908, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1118, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 70, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_apple", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 986, nutrition: Some(4), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:golden_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 929, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:golden_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 977, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_carrot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1228, nutrition: Some(6), saturation: Some(14.40), tool_rules: vec![] });
	items.insert("minecraft:golden_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 975, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 974, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 930, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:golden_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1253, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 976, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 928, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:golden_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 927, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:golden_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 926, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:granite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 2, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 707, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 690, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 462, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:grass_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 27, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gravel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 63, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1268, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1093, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1044, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1389, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 512, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 621, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 637, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1073, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 605, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 845, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 589, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 537, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 553, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 493, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 220, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1274, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1099, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1050, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1395, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 518, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 627, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 643, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1079, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 611, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 851, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 595, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 543, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 559, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 499, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 226, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:grindstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1342, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guardian_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1160, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gunpowder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 950, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guster_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1332, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guster_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1438, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hanging_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 266, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:happy_ghast_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1157, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hay_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 504, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heart_of_the_sea", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1321, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heart_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1439, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heartbreak_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1440, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heavy_core", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 87, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heavy_weighted_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 766, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hoglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1161, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honey_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 725, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honey_bottle", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1364, nutrition: Some(6), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:honeycomb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1361, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honeycomb_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1365, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hopper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 727, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hopper_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 858, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 663, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 658, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 673, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1162, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:host_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1425, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:howl_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1441, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:husk_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1163, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 338, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_chiseled_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 373, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 369, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_cracked_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 372, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 374, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_mossy_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 371, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 368, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 370, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ink_sac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1063, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 934, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:iron_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 390, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 90, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 969, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 399, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 967, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 779, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1164, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 966, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 935, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:iron_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1252, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 904, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 968, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1293, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 66, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 933, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:iron_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 932, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:iron_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 931, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:iron_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 800, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:item_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1220, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jack_o_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 358, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jigsaw", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 884, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jukebox", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 343, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 869, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 754, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 870, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 783, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 347, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 824, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1003, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 185, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 137, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 39, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 770, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 52, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 311, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 991, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 273, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 444, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 804, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 174, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:kelp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 257, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:knowledge_book", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1295, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ladder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 335, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1346, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 197, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_lazuli", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 900, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 76, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:large_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1400, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:large_fern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 529, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lava_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1014, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lead", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1256, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leaf_litter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 260, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1017, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 957, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 955, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 954, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1255, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 956, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lectern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 730, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lever", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 732, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 503, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1264, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1089, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1040, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1385, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 508, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 617, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 633, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1069, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 601, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 841, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 585, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 533, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 549, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 489, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 216, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1269, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1094, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1045, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1390, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 513, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 622, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 638, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1074, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 606, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 846, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 590, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 538, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 554, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 494, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 221, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_weighted_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 765, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 733, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lilac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 525, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lily_of_the_valley", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 242, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lily_pad", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 423, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1266, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1091, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1042, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1387, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 510, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 619, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 635, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1071, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 603, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 843, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 587, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 535, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 551, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 491, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 218, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lingering_potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1289, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:llama_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1165, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lodestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1366, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:loom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1324, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mace", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1219, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1263, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1088, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1039, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1384, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 507, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 616, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 632, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1068, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 600, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 840, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 584, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 532, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 548, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 488, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 215, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 575, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_cream", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1125, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_cube_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1166, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 879, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 759, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 880, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 788, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 352, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 829, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1008, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 190, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 142, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 44, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 775, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_propagule", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 57, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 143, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 312, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 996, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 278, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 449, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 809, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 179, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1227, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:medium_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1399, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 409, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1109, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon_slice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1106, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:milk_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1018, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 854, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:miner_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1442, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mojang_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1328, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mooshroom_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1167, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:moss_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 262, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:moss_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 261, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 320, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 703, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 685, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 457, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 701, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 683, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 461, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 376, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mourner_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1443, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 32, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 291, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 421, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 464, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 380, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:muddy_mangrove_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 144, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mule_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1168, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mushroom_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 389, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mushroom_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 947, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:music_disc_11", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1310, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_13", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1297, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_5", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1314, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_blocks", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1299, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_cat", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1298, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_chirp", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1300, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_creator", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1301, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_creator_music_box", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1302, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_far", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1303, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_lava_chicken", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1304, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_mall", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1305, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_mellohi", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1306, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_otherside", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1312, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_pigstep", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1315, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_precipice", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1316, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_relic", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1313, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_stal", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1307, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_strad", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1308, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_tears", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1317, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_wait", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1311, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_ward", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1309, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mutton", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1259, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:mycelium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 422, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:name_tag", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1257, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nautilus_shell", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1320, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1241, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 427, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 292, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 428, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 465, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 424, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 80, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_quartz_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 81, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_sprouts", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 253, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_star", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1236, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_wart", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1119, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_wart_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 576, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 944, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:netherite_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 94, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 981, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 979, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 978, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 945, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:netherite_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 909, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 980, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 943, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:netherite_scrap", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 910, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 942, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:netherite_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 941, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:netherite_upgrade_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1409, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherrack", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 359, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:note_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 748, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 863, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 751, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 864, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 780, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 344, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 821, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1000, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 182, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 134, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 36, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 767, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 49, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 313, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 988, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 270, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 441, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 801, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 171, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:observer", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 726, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:obsidian", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 321, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ocelot_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1169, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ochre_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1403, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ominous_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1487, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ominous_trial_key", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1485, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:open_eyeblossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 230, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1262, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1087, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1038, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1383, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 506, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 615, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 631, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1067, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 599, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 839, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 583, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 531, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 547, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 487, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 237, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 214, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxeye_daisy", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 240, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 101, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 97, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 394, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1462, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 403, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1470, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 795, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1478, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1454, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1351, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 816, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 105, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 113, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 109, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 736, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:packed_ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 522, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:packed_mud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 379, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:painting", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 985, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_hanging_moss", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 264, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_moss_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 265, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_moss_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 263, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 877, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 758, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 878, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 787, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 351, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 828, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1007, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 189, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 140, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 43, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 774, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 56, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 314, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 995, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 277, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 448, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 808, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 177, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:panda_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1170, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:paper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1028, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:parrot_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1171, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pearlescent_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1405, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:peony", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 527, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:petrified_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 287, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:phantom_membrane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 861, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:phantom_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1172, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pig_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1173, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1330, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_brute_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1175, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1235, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1174, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pillager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1176, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1267, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1092, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1043, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1388, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 511, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 620, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 636, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1072, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 604, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 844, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_petals", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 258, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 588, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 536, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 552, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 492, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 239, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 219, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piston", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 722, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pitcher_plant", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 245, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pitcher_pod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1281, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:player_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1231, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:plenty_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1444, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:podzol", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 30, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pointed_dripstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1402, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poisonous_potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1226, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:polar_bear_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1177, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 7, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 710, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 693, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 363, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1372, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1377, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1378, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 473, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1376, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 750, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 764, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1373, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1374, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 472, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 10, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 713, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 696, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 475, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_diorite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 5, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_diorite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 702, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_diorite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 684, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 3, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 699, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 681, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 17, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 18, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 19, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 20, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:popped_chorus_fruit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1279, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poppy", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 232, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:porkchop", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 983, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1224, nutrition: Some(1), saturation: Some(0.60), tool_rules: vec![] });
	items.insert("minecraft:potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1121, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:powder_snow_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1015, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:powered_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 833, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 562, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 298, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 566, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 563, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_crystals", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1244, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_shard", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1243, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 297, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 565, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 459, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prize_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1445, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pufferfish", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1060, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:pufferfish_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1019, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pufferfish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1178, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pumpkin", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 356, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pumpkin_pie", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1237, nutrition: Some(8), saturation: Some(4.80), tool_rules: vec![] });
	items.insert("minecraft:pumpkin_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1108, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1271, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1096, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1047, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1392, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 515, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 624, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 640, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1076, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 608, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 848, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 592, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 540, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 556, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 496, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 223, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 326, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_pillar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 327, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 296, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 328, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 901, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 482, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 483, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_pillar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 484, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 293, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 485, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1245, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:rabbit_foot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1248, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_hide", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1249, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1179, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1247, nutrition: Some(10), saturation: Some(12.00), tool_rules: vec![] });
	items.insert("minecraft:rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 835, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raiser_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1424, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ravager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1180, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 905, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 85, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_gold", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 907, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_gold_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 86, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_iron", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 903, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_iron_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 84, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:recovery_compass", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1035, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1275, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1100, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1051, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1396, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 519, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 628, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 644, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1080, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 612, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 852, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_mushroom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 248, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_mushroom_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 388, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 709, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 692, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 467, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 578, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 62, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 569, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 294, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 572, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 460, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 596, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 544, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 560, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 500, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 236, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 227, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 717, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 719, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_lamp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 747, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 72, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 718, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:reinforced_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 386, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:repeater", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 720, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:repeating_command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 573, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 413, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1242, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 416, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 415, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 417, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 414, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_clump", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 412, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:respawn_anchor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1380, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rib_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1419, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rooted_dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 31, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rose_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 526, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rotten_flesh", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1114, nutrition: Some(4), saturation: Some(0.80), tool_rules: vec![] });
	items.insert("minecraft:saddle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 837, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:salmon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1058, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:salmon_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1020, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:salmon_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1181, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 59, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 198, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 285, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 438, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 468, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:scaffolding", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 716, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:scrape_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1446, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 429, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_catalyst", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 431, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_sensor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 742, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_shrieker", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 432, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_vein", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 430, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sea_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 568, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sea_pickle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 212, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:seagrass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 211, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sentry_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1410, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shaper_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1422, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sheaf_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1447, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shears", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1105, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:leaves"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:wool"], correct_for_drops: true, speed: Some(5.0)},ToolRule {blocks: vec!["minecraft:vine", "minecraft:glow_lichen"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:sheep_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1182, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shelter_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1448, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shield", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1290, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:short_dry_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 209, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:short_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 202, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shroomlight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1360, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 581, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_shell", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1292, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1183, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:silence_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 1423, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:silverfish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1184, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1186, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_skull", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1229, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1185, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skull_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1327, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skull_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1449, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_ball", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1030, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 724, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1187, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:small_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1398, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:small_dripleaf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 268, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smithing_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1343, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smoker", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1338, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 364, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 300, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 706, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 689, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 301, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 700, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 682, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 302, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 705, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 688, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 303, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_stone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 284, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sniffer_egg", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 647, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sniffer_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1188, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snort_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1450, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snout_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1418, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 337, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snow_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 339, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snow_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1189, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snowball", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1016, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_campfire", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1359, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1347, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 360, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_soil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 361, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 365, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spawner", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 329, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spectral_arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1287, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spider_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1122, nutrition: Some(2), saturation: Some(3.20), tool_rules: vec![] });
	items.insert("minecraft:spider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1190, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spire_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1420, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:splash_potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1286, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sponge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 193, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spore_blossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 246, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 865, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 752, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 866, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 781, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 345, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 822, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1001, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 183, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 135, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 37, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 768, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 50, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 315, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 989, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 271, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 442, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 802, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 172, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spyglass", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1055, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:squid_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1191, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 946, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sticky_piston", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 723, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 924, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 290, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 420, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 463, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 375, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 749, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 925, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:stone_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 923, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:stone_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 763, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 922, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:stone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 283, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 687, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 921, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:stonecutter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1344, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stray_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1192, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:strider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1193, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:string", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 948, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_acacia_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 152, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_acacia_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 163, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_bamboo_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 170, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_birch_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 150, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_birch_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 161, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_cherry_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 153, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_cherry_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 164, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_crimson_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 168, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_crimson_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 157, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_dark_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 154, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_dark_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 165, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_jungle_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 151, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_jungle_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 162, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_mangrove_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 156, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_mangrove_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 167, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 148, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 159, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_pale_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 155, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_pale_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 166, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_spruce_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 149, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_spruce_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 160, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_warped_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 169, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_warped_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 158, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:structure_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 883, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:structure_void", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 580, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sugar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1084, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sugar_cane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 256, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sunflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 524, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_gravel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 61, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 60, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1323, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:sweet_berries", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1356, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:tadpole_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1024, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tadpole_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1194, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tall_dry_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 210, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tall_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 528, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:target", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 731, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 521, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:test_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 885, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:test_instance_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 886, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tide_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1417, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tinted_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 196, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tipped_arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1288, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tnt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 746, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tnt_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 857, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 322, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:torchflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 244, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:torchflower_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1280, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:totem_of_undying", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1291, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trader_llama_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1195, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trapped_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 745, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trial_key", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1484, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trial_spawner", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1483, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trident", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1319, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tripwire_hook", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 744, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tropical_fish", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1059, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:tropical_fish_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1022, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tropical_fish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1196, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 659, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 654, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 669, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 12, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 22, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 23, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 24, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 21, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 13, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 14, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 15, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 646, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 887, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_scute", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 888, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1197, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:twisting_vines", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 255, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vault", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1486, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:verdant_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1404, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vex_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1416, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vex_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1198, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:villager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1199, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vindicator_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1200, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 410, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wandering_trader_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1201, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ward_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1414, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warden_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1202, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 762, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 791, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 355, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 832, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fungus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 250, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fungus_on_a_stick", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 860, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1011, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 181, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_nylium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 34, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 47, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 778, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 252, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 316, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 999, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 282, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 453, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 146, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 812, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_wart_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 577, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:water_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1013, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 118, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 395, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 114, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1463, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 404, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1471, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 796, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1479, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1455, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1352, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 817, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 122, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 130, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 126, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 119, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 115, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 396, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1464, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 405, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1472, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 797, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1480, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1456, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1353, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 818, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 123, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 131, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 127, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 738, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 737, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 121, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 117, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 398, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1466, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 407, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1474, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 799, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1482, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1458, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1355, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 820, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 125, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 133, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 129, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 740, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 120, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 116, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 397, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1465, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 406, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1473, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 798, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1481, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1457, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1354, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 819, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 124, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 132, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 128, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 739, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wayfinder_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1421, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 100, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 96, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 393, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1461, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 402, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1469, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 794, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1477, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1453, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1350, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 815, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 104, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 112, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 108, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 735, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weeping_vines", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 254, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wet_sponge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 194, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wheat", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 952, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wheat_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 951, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1261, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1086, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1037, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1382, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 505, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 614, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 630, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1066, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 598, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 838, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 582, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 530, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 546, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 486, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 238, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 213, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wild_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1413, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wildflowers", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 259, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wind_charge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1215, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:witch_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1203, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_rose", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 243, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_skeleton_skull", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1230, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_skeleton_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1205, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1204, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wolf_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 890, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wolf_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1206, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 914, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:wooden_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 915, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:wooden_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 913, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:wooden_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 912, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:wooden_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 911, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:writable_book", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1216, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:written_book", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1217, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1265, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1090, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1041, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1386, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 509, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 618, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 634, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1070, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 602, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 842, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 586, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 534, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 550, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 490, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 217, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zoglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1207, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1232, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1210, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1209, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_villager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1211, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombified_piglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1212, nutrition: None, saturation: None, tool_rules: vec![] });

	return items;
}
