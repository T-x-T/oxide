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

pub fn get_item_name_by_id(id: i32) -> Option<&'static str> {
  return match id {
		871 => Some("minecraft:acacia_boat"),
		755 => Some("minecraft:acacia_button"),
		872 => Some("minecraft:acacia_chest_boat"),
		784 => Some("minecraft:acacia_door"),
		348 => Some("minecraft:acacia_fence"),
		825 => Some("minecraft:acacia_fence_gate"),
		1004 => Some("minecraft:acacia_hanging_sign"),
		186 => Some("minecraft:acacia_leaves"),
		138 => Some("minecraft:acacia_log"),
		40 => Some("minecraft:acacia_planks"),
		771 => Some("minecraft:acacia_pressure_plate"),
		53 => Some("minecraft:acacia_sapling"),
		305 => Some("minecraft:acacia_shelf"),
		992 => Some("minecraft:acacia_sign"),
		274 => Some("minecraft:acacia_slab"),
		445 => Some("minecraft:acacia_stairs"),
		805 => Some("minecraft:acacia_trapdoor"),
		175 => Some("minecraft:acacia_wood"),
		836 => Some("minecraft:activator_rail"),
		0 => Some("minecraft:air"),
		1131 => Some("minecraft:allay_spawn_egg"),
		234 => Some("minecraft:allium"),
		88 => Some("minecraft:amethyst_block"),
		1401 => Some("minecraft:amethyst_cluster"),
		902 => Some("minecraft:amethyst_shard"),
		82 => Some("minecraft:ancient_debris"),
		6 => Some("minecraft:andesite"),
		708 => Some("minecraft:andesite_slab"),
		691 => Some("minecraft:andesite_stairs"),
		466 => Some("minecraft:andesite_wall"),
		1428 => Some("minecraft:angler_pottery_sherd"),
		478 => Some("minecraft:anvil"),
		893 => Some("minecraft:apple"),
		1429 => Some("minecraft:archer_pottery_sherd"),
		889 => Some("minecraft:armadillo_scute"),
		1130 => Some("minecraft:armadillo_spawn_egg"),
		1250 => Some("minecraft:armor_stand"),
		1430 => Some("minecraft:arms_up_pottery_sherd"),
		895 => Some("minecraft:arrow"),
		1023 => Some("minecraft:axolotl_bucket"),
		1132 => Some("minecraft:axolotl_spawn_egg"),
		205 => Some("minecraft:azalea"),
		191 => Some("minecraft:azalea_leaves"),
		235 => Some("minecraft:azure_bluet"),
		1225 => Some("minecraft:baked_potato"),
		269 => Some("minecraft:bamboo"),
		147 => Some("minecraft:bamboo_block"),
		760 => Some("minecraft:bamboo_button"),
		882 => Some("minecraft:bamboo_chest_raft"),
		789 => Some("minecraft:bamboo_door"),
		353 => Some("minecraft:bamboo_fence"),
		830 => Some("minecraft:bamboo_fence_gate"),
		1009 => Some("minecraft:bamboo_hanging_sign"),
		48 => Some("minecraft:bamboo_mosaic"),
		280 => Some("minecraft:bamboo_mosaic_slab"),
		451 => Some("minecraft:bamboo_mosaic_stairs"),
		45 => Some("minecraft:bamboo_planks"),
		776 => Some("minecraft:bamboo_pressure_plate"),
		881 => Some("minecraft:bamboo_raft"),
		306 => Some("minecraft:bamboo_shelf"),
		997 => Some("minecraft:bamboo_sign"),
		279 => Some("minecraft:bamboo_slab"),
		450 => Some("minecraft:bamboo_stairs"),
		810 => Some("minecraft:bamboo_trapdoor"),
		1337 => Some("minecraft:barrel"),
		502 => Some("minecraft:barrier"),
		362 => Some("minecraft:basalt"),
		1133 => Some("minecraft:bat_spawn_egg"),
		455 => Some("minecraft:beacon"),
		58 => Some("minecraft:bedrock"),
		1362 => Some("minecraft:bee_nest"),
		1134 => Some("minecraft:bee_spawn_egg"),
		1110 => Some("minecraft:beef"),
		1363 => Some("minecraft:beehive"),
		1282 => Some("minecraft:beetroot"),
		1283 => Some("minecraft:beetroot_seeds"),
		1284 => Some("minecraft:beetroot_soup"),
		1345 => Some("minecraft:bell"),
		267 => Some("minecraft:big_dripleaf"),
		867 => Some("minecraft:birch_boat"),
		753 => Some("minecraft:birch_button"),
		868 => Some("minecraft:birch_chest_boat"),
		782 => Some("minecraft:birch_door"),
		346 => Some("minecraft:birch_fence"),
		823 => Some("minecraft:birch_fence_gate"),
		1002 => Some("minecraft:birch_hanging_sign"),
		184 => Some("minecraft:birch_leaves"),
		136 => Some("minecraft:birch_log"),
		38 => Some("minecraft:birch_planks"),
		769 => Some("minecraft:birch_pressure_plate"),
		51 => Some("minecraft:birch_sapling"),
		307 => Some("minecraft:birch_shelf"),
		990 => Some("minecraft:birch_sign"),
		272 => Some("minecraft:birch_slab"),
		443 => Some("minecraft:birch_stairs"),
		803 => Some("minecraft:birch_trapdoor"),
		173 => Some("minecraft:birch_wood"),
		1276 => Some("minecraft:black_banner"),
		1101 => Some("minecraft:black_bed"),
		1052 => Some("minecraft:black_bundle"),
		1397 => Some("minecraft:black_candle"),
		520 => Some("minecraft:black_carpet"),
		629 => Some("minecraft:black_concrete"),
		645 => Some("minecraft:black_concrete_powder"),
		1081 => Some("minecraft:black_dye"),
		613 => Some("minecraft:black_glazed_terracotta"),
		853 => Some("minecraft:black_harness"),
		597 => Some("minecraft:black_shulker_box"),
		545 => Some("minecraft:black_stained_glass"),
		561 => Some("minecraft:black_stained_glass_pane"),
		501 => Some("minecraft:black_terracotta"),
		228 => Some("minecraft:black_wool"),
		1368 => Some("minecraft:blackstone"),
		1369 => Some("minecraft:blackstone_slab"),
		1370 => Some("minecraft:blackstone_stairs"),
		471 => Some("minecraft:blackstone_wall"),
		1431 => Some("minecraft:blade_pottery_sherd"),
		1339 => Some("minecraft:blast_furnace"),
		1124 => Some("minecraft:blaze_powder"),
		1116 => Some("minecraft:blaze_rod"),
		1135 => Some("minecraft:blaze_spawn_egg"),
		1272 => Some("minecraft:blue_banner"),
		1097 => Some("minecraft:blue_bed"),
		1048 => Some("minecraft:blue_bundle"),
		1393 => Some("minecraft:blue_candle"),
		516 => Some("minecraft:blue_carpet"),
		625 => Some("minecraft:blue_concrete"),
		641 => Some("minecraft:blue_concrete_powder"),
		1077 => Some("minecraft:blue_dye"),
		1032 => Some("minecraft:blue_egg"),
		609 => Some("minecraft:blue_glazed_terracotta"),
		849 => Some("minecraft:blue_harness"),
		679 => Some("minecraft:blue_ice"),
		233 => Some("minecraft:blue_orchid"),
		593 => Some("minecraft:blue_shulker_box"),
		541 => Some("minecraft:blue_stained_glass"),
		557 => Some("minecraft:blue_stained_glass_pane"),
		497 => Some("minecraft:blue_terracotta"),
		224 => Some("minecraft:blue_wool"),
		1136 => Some("minecraft:bogged_spawn_egg"),
		1427 => Some("minecraft:bolt_armor_trim_smithing_template"),
		1083 => Some("minecraft:bone"),
		579 => Some("minecraft:bone_block"),
		1082 => Some("minecraft:bone_meal"),
		1029 => Some("minecraft:book"),
		317 => Some("minecraft:bookshelf"),
		1334 => Some("minecraft:bordure_indented_banner_pattern"),
		894 => Some("minecraft:bow"),
		892 => Some("minecraft:bowl"),
		660 => Some("minecraft:brain_coral"),
		655 => Some("minecraft:brain_coral_block"),
		670 => Some("minecraft:brain_coral_fan"),
		953 => Some("minecraft:bread"),
		1218 => Some("minecraft:breeze_rod"),
		1137 => Some("minecraft:breeze_spawn_egg"),
		1432 => Some("minecraft:brewer_pottery_sherd"),
		1126 => Some("minecraft:brewing_stand"),
		1025 => Some("minecraft:brick"),
		289 => Some("minecraft:brick_slab"),
		419 => Some("minecraft:brick_stairs"),
		458 => Some("minecraft:brick_wall"),
		304 => Some("minecraft:bricks"),
		1273 => Some("minecraft:brown_banner"),
		1098 => Some("minecraft:brown_bed"),
		1049 => Some("minecraft:brown_bundle"),
		1394 => Some("minecraft:brown_candle"),
		517 => Some("minecraft:brown_carpet"),
		626 => Some("minecraft:brown_concrete"),
		642 => Some("minecraft:brown_concrete_powder"),
		1078 => Some("minecraft:brown_dye"),
		1033 => Some("minecraft:brown_egg"),
		610 => Some("minecraft:brown_glazed_terracotta"),
		850 => Some("minecraft:brown_harness"),
		247 => Some("minecraft:brown_mushroom"),
		387 => Some("minecraft:brown_mushroom_block"),
		594 => Some("minecraft:brown_shulker_box"),
		542 => Some("minecraft:brown_stained_glass"),
		558 => Some("minecraft:brown_stained_glass_pane"),
		498 => Some("minecraft:brown_terracotta"),
		225 => Some("minecraft:brown_wool"),
		1408 => Some("minecraft:brush"),
		661 => Some("minecraft:bubble_coral"),
		656 => Some("minecraft:bubble_coral_block"),
		671 => Some("minecraft:bubble_coral_fan"),
		1012 => Some("minecraft:bucket"),
		89 => Some("minecraft:budding_amethyst"),
		1036 => Some("minecraft:bundle"),
		1433 => Some("minecraft:burn_pottery_sherd"),
		204 => Some("minecraft:bush"),
		340 => Some("minecraft:cactus"),
		341 => Some("minecraft:cactus_flower"),
		1085 => Some("minecraft:cake"),
		11 => Some("minecraft:calcite"),
		743 => Some("minecraft:calibrated_sculk_sensor"),
		1139 => Some("minecraft:camel_spawn_egg"),
		1358 => Some("minecraft:campfire"),
		1381 => Some("minecraft:candle"),
		1223 => Some("minecraft:carrot"),
		859 => Some("minecraft:carrot_on_a_stick"),
		1340 => Some("minecraft:cartography_table"),
		357 => Some("minecraft:carved_pumpkin"),
		1138 => Some("minecraft:cat_spawn_egg"),
		1127 => Some("minecraft:cauldron"),
		1140 => Some("minecraft:cave_spider_spawn_egg"),
		574 => Some("minecraft:chain_command_block"),
		965 => Some("minecraft:chainmail_boots"),
		963 => Some("minecraft:chainmail_chestplate"),
		962 => Some("minecraft:chainmail_helmet"),
		964 => Some("minecraft:chainmail_leggings"),
		897 => Some("minecraft:charcoal"),
		873 => Some("minecraft:cherry_boat"),
		756 => Some("minecraft:cherry_button"),
		874 => Some("minecraft:cherry_chest_boat"),
		785 => Some("minecraft:cherry_door"),
		349 => Some("minecraft:cherry_fence"),
		826 => Some("minecraft:cherry_fence_gate"),
		1005 => Some("minecraft:cherry_hanging_sign"),
		187 => Some("minecraft:cherry_leaves"),
		139 => Some("minecraft:cherry_log"),
		41 => Some("minecraft:cherry_planks"),
		772 => Some("minecraft:cherry_pressure_plate"),
		54 => Some("minecraft:cherry_sapling"),
		308 => Some("minecraft:cherry_shelf"),
		993 => Some("minecraft:cherry_sign"),
		275 => Some("minecraft:cherry_slab"),
		446 => Some("minecraft:cherry_stairs"),
		806 => Some("minecraft:cherry_trapdoor"),
		176 => Some("minecraft:cherry_wood"),
		331 => Some("minecraft:chest"),
		855 => Some("minecraft:chest_minecart"),
		1112 => Some("minecraft:chicken"),
		1141 => Some("minecraft:chicken_spawn_egg"),
		479 => Some("minecraft:chipped_anvil"),
		318 => Some("minecraft:chiseled_bookshelf"),
		98 => Some("minecraft:chiseled_copper"),
		385 => Some("minecraft:chiseled_deepslate"),
		426 => Some("minecraft:chiseled_nether_bricks"),
		1375 => Some("minecraft:chiseled_polished_blackstone"),
		481 => Some("minecraft:chiseled_quartz_block"),
		570 => Some("minecraft:chiseled_red_sandstone"),
		418 => Some("minecraft:chiseled_resin_bricks"),
		199 => Some("minecraft:chiseled_sandstone"),
		378 => Some("minecraft:chiseled_stone_bricks"),
		16 => Some("minecraft:chiseled_tuff"),
		25 => Some("minecraft:chiseled_tuff_bricks"),
		325 => Some("minecraft:chorus_flower"),
		1278 => Some("minecraft:chorus_fruit"),
		324 => Some("minecraft:chorus_plant"),
		342 => Some("minecraft:clay"),
		1026 => Some("minecraft:clay_ball"),
		1054 => Some("minecraft:clock"),
		231 => Some("minecraft:closed_eyeblossom"),
		896 => Some("minecraft:coal"),
		83 => Some("minecraft:coal_block"),
		64 => Some("minecraft:coal_ore"),
		29 => Some("minecraft:coarse_dirt"),
		1412 => Some("minecraft:coast_armor_trim_smithing_template"),
		9 => Some("minecraft:cobbled_deepslate"),
		712 => Some("minecraft:cobbled_deepslate_slab"),
		695 => Some("minecraft:cobbled_deepslate_stairs"),
		474 => Some("minecraft:cobbled_deepslate_wall"),
		35 => Some("minecraft:cobblestone"),
		288 => Some("minecraft:cobblestone_slab"),
		336 => Some("minecraft:cobblestone_stairs"),
		456 => Some("minecraft:cobblestone_wall"),
		201 => Some("minecraft:cobweb"),
		1065 => Some("minecraft:cocoa_beans"),
		1057 => Some("minecraft:cod"),
		1021 => Some("minecraft:cod_bucket"),
		1142 => Some("minecraft:cod_spawn_egg"),
		454 => Some("minecraft:command_block"),
		1258 => Some("minecraft:command_block_minecart"),
		721 => Some("minecraft:comparator"),
		1034 => Some("minecraft:compass"),
		1336 => Some("minecraft:composter"),
		680 => Some("minecraft:conduit"),
		1111 => Some("minecraft:cooked_beef"),
		1113 => Some("minecraft:cooked_chicken"),
		1061 => Some("minecraft:cooked_cod"),
		1260 => Some("minecraft:cooked_mutton"),
		984 => Some("minecraft:cooked_porkchop"),
		1246 => Some("minecraft:cooked_rabbit"),
		1062 => Some("minecraft:cooked_salmon"),
		1102 => Some("minecraft:cookie"),
		919 => Some("minecraft:copper_axe"),
		391 => Some("minecraft:copper_bars"),
		91 => Some("minecraft:copper_block"),
		961 => Some("minecraft:copper_boots"),
		1459 => Some("minecraft:copper_bulb"),
		400 => Some("minecraft:copper_chain"),
		1467 => Some("minecraft:copper_chest"),
		959 => Some("minecraft:copper_chestplate"),
		792 => Some("minecraft:copper_door"),
		1143 => Some("minecraft:copper_golem_spawn_egg"),
		1475 => Some("minecraft:copper_golem_statue"),
		1451 => Some("minecraft:copper_grate"),
		958 => Some("minecraft:copper_helmet"),
		920 => Some("minecraft:copper_hoe"),
		1251 => Some("minecraft:copper_horse_armor"),
		906 => Some("minecraft:copper_ingot"),
		1348 => Some("minecraft:copper_lantern"),
		960 => Some("minecraft:copper_leggings"),
		1294 => Some("minecraft:copper_nugget"),
		68 => Some("minecraft:copper_ore"),
		918 => Some("minecraft:copper_pickaxe"),
		917 => Some("minecraft:copper_shovel"),
		916 => Some("minecraft:copper_sword"),
		366 => Some("minecraft:copper_torch"),
		813 => Some("minecraft:copper_trapdoor"),
		241 => Some("minecraft:cornflower"),
		1144 => Some("minecraft:cow_spawn_egg"),
		382 => Some("minecraft:cracked_deepslate_bricks"),
		384 => Some("minecraft:cracked_deepslate_tiles"),
		425 => Some("minecraft:cracked_nether_bricks"),
		1379 => Some("minecraft:cracked_polished_blackstone_bricks"),
		377 => Some("minecraft:cracked_stone_bricks"),
		1103 => Some("minecraft:crafter"),
		332 => Some("minecraft:crafting_table"),
		330 => Some("minecraft:creaking_heart"),
		1208 => Some("minecraft:creaking_spawn_egg"),
		1326 => Some("minecraft:creeper_banner_pattern"),
		1233 => Some("minecraft:creeper_head"),
		1145 => Some("minecraft:creeper_spawn_egg"),
		761 => Some("minecraft:crimson_button"),
		790 => Some("minecraft:crimson_door"),
		354 => Some("minecraft:crimson_fence"),
		831 => Some("minecraft:crimson_fence_gate"),
		249 => Some("minecraft:crimson_fungus"),
		1010 => Some("minecraft:crimson_hanging_sign"),
		180 => Some("minecraft:crimson_hyphae"),
		33 => Some("minecraft:crimson_nylium"),
		46 => Some("minecraft:crimson_planks"),
		777 => Some("minecraft:crimson_pressure_plate"),
		251 => Some("minecraft:crimson_roots"),
		309 => Some("minecraft:crimson_shelf"),
		998 => Some("minecraft:crimson_sign"),
		281 => Some("minecraft:crimson_slab"),
		452 => Some("minecraft:crimson_stairs"),
		145 => Some("minecraft:crimson_stem"),
		811 => Some("minecraft:crimson_trapdoor"),
		1322 => Some("minecraft:crossbow"),
		1367 => Some("minecraft:crying_obsidian"),
		102 => Some("minecraft:cut_copper"),
		110 => Some("minecraft:cut_copper_slab"),
		106 => Some("minecraft:cut_copper_stairs"),
		571 => Some("minecraft:cut_red_sandstone"),
		295 => Some("minecraft:cut_red_sandstone_slab"),
		200 => Some("minecraft:cut_sandstone"),
		286 => Some("minecraft:cut_sandstone_slab"),
		1270 => Some("minecraft:cyan_banner"),
		1095 => Some("minecraft:cyan_bed"),
		1046 => Some("minecraft:cyan_bundle"),
		1391 => Some("minecraft:cyan_candle"),
		514 => Some("minecraft:cyan_carpet"),
		623 => Some("minecraft:cyan_concrete"),
		639 => Some("minecraft:cyan_concrete_powder"),
		1075 => Some("minecraft:cyan_dye"),
		607 => Some("minecraft:cyan_glazed_terracotta"),
		847 => Some("minecraft:cyan_harness"),
		591 => Some("minecraft:cyan_shulker_box"),
		539 => Some("minecraft:cyan_stained_glass"),
		555 => Some("minecraft:cyan_stained_glass_pane"),
		495 => Some("minecraft:cyan_terracotta"),
		222 => Some("minecraft:cyan_wool"),
		480 => Some("minecraft:damaged_anvil"),
		229 => Some("minecraft:dandelion"),
		1434 => Some("minecraft:danger_pottery_sherd"),
		875 => Some("minecraft:dark_oak_boat"),
		757 => Some("minecraft:dark_oak_button"),
		876 => Some("minecraft:dark_oak_chest_boat"),
		786 => Some("minecraft:dark_oak_door"),
		350 => Some("minecraft:dark_oak_fence"),
		827 => Some("minecraft:dark_oak_fence_gate"),
		1006 => Some("minecraft:dark_oak_hanging_sign"),
		188 => Some("minecraft:dark_oak_leaves"),
		141 => Some("minecraft:dark_oak_log"),
		42 => Some("minecraft:dark_oak_planks"),
		773 => Some("minecraft:dark_oak_pressure_plate"),
		55 => Some("minecraft:dark_oak_sapling"),
		310 => Some("minecraft:dark_oak_shelf"),
		994 => Some("minecraft:dark_oak_sign"),
		276 => Some("minecraft:dark_oak_slab"),
		447 => Some("minecraft:dark_oak_stairs"),
		807 => Some("minecraft:dark_oak_trapdoor"),
		178 => Some("minecraft:dark_oak_wood"),
		564 => Some("minecraft:dark_prismarine"),
		299 => Some("minecraft:dark_prismarine_slab"),
		567 => Some("minecraft:dark_prismarine_stairs"),
		741 => Some("minecraft:daylight_detector"),
		664 => Some("minecraft:dead_brain_coral"),
		650 => Some("minecraft:dead_brain_coral_block"),
		675 => Some("minecraft:dead_brain_coral_fan"),
		665 => Some("minecraft:dead_bubble_coral"),
		651 => Some("minecraft:dead_bubble_coral_block"),
		676 => Some("minecraft:dead_bubble_coral_fan"),
		207 => Some("minecraft:dead_bush"),
		666 => Some("minecraft:dead_fire_coral"),
		652 => Some("minecraft:dead_fire_coral_block"),
		677 => Some("minecraft:dead_fire_coral_fan"),
		667 => Some("minecraft:dead_horn_coral"),
		653 => Some("minecraft:dead_horn_coral_block"),
		678 => Some("minecraft:dead_horn_coral_fan"),
		668 => Some("minecraft:dead_tube_coral"),
		649 => Some("minecraft:dead_tube_coral_block"),
		674 => Some("minecraft:dead_tube_coral_fan"),
		1296 => Some("minecraft:debug_stick"),
		319 => Some("minecraft:decorated_pot"),
		8 => Some("minecraft:deepslate"),
		714 => Some("minecraft:deepslate_brick_slab"),
		697 => Some("minecraft:deepslate_brick_stairs"),
		476 => Some("minecraft:deepslate_brick_wall"),
		381 => Some("minecraft:deepslate_bricks"),
		65 => Some("minecraft:deepslate_coal_ore"),
		69 => Some("minecraft:deepslate_copper_ore"),
		79 => Some("minecraft:deepslate_diamond_ore"),
		75 => Some("minecraft:deepslate_emerald_ore"),
		71 => Some("minecraft:deepslate_gold_ore"),
		67 => Some("minecraft:deepslate_iron_ore"),
		77 => Some("minecraft:deepslate_lapis_ore"),
		73 => Some("minecraft:deepslate_redstone_ore"),
		715 => Some("minecraft:deepslate_tile_slab"),
		698 => Some("minecraft:deepslate_tile_stairs"),
		477 => Some("minecraft:deepslate_tile_wall"),
		383 => Some("minecraft:deepslate_tiles"),
		834 => Some("minecraft:detector_rail"),
		898 => Some("minecraft:diamond"),
		939 => Some("minecraft:diamond_axe"),
		93 => Some("minecraft:diamond_block"),
		973 => Some("minecraft:diamond_boots"),
		971 => Some("minecraft:diamond_chestplate"),
		970 => Some("minecraft:diamond_helmet"),
		940 => Some("minecraft:diamond_hoe"),
		1254 => Some("minecraft:diamond_horse_armor"),
		972 => Some("minecraft:diamond_leggings"),
		78 => Some("minecraft:diamond_ore"),
		938 => Some("minecraft:diamond_pickaxe"),
		937 => Some("minecraft:diamond_shovel"),
		936 => Some("minecraft:diamond_sword"),
		4 => Some("minecraft:diorite"),
		711 => Some("minecraft:diorite_slab"),
		694 => Some("minecraft:diorite_stairs"),
		470 => Some("minecraft:diorite_wall"),
		28 => Some("minecraft:dirt"),
		523 => Some("minecraft:dirt_path"),
		1318 => Some("minecraft:disc_fragment_5"),
		728 => Some("minecraft:dispenser"),
		1146 => Some("minecraft:dolphin_spawn_egg"),
		1147 => Some("minecraft:donkey_spawn_egg"),
		1285 => Some("minecraft:dragon_breath"),
		437 => Some("minecraft:dragon_egg"),
		1234 => Some("minecraft:dragon_head"),
		648 => Some("minecraft:dried_ghast"),
		1107 => Some("minecraft:dried_kelp"),
		1027 => Some("minecraft:dried_kelp_block"),
		26 => Some("minecraft:dripstone_block"),
		729 => Some("minecraft:dropper"),
		1148 => Some("minecraft:drowned_spawn_egg"),
		1411 => Some("minecraft:dune_armor_trim_smithing_template"),
		1407 => Some("minecraft:echo_shard"),
		1031 => Some("minecraft:egg"),
		1149 => Some("minecraft:elder_guardian_spawn_egg"),
		862 => Some("minecraft:elytra"),
		899 => Some("minecraft:emerald"),
		440 => Some("minecraft:emerald_block"),
		74 => Some("minecraft:emerald_ore"),
		1240 => Some("minecraft:enchanted_book"),
		987 => Some("minecraft:enchanted_golden_apple"),
		433 => Some("minecraft:enchanting_table"),
		1277 => Some("minecraft:end_crystal"),
		434 => Some("minecraft:end_portal_frame"),
		323 => Some("minecraft:end_rod"),
		435 => Some("minecraft:end_stone"),
		704 => Some("minecraft:end_stone_brick_slab"),
		686 => Some("minecraft:end_stone_brick_stairs"),
		469 => Some("minecraft:end_stone_brick_wall"),
		436 => Some("minecraft:end_stone_bricks"),
		439 => Some("minecraft:ender_chest"),
		1150 => Some("minecraft:ender_dragon_spawn_egg"),
		1128 => Some("minecraft:ender_eye"),
		1115 => Some("minecraft:ender_pearl"),
		1151 => Some("minecraft:enderman_spawn_egg"),
		1152 => Some("minecraft:endermite_spawn_egg"),
		1153 => Some("minecraft:evoker_spawn_egg"),
		1213 => Some("minecraft:experience_bottle"),
		1435 => Some("minecraft:explorer_pottery_sherd"),
		99 => Some("minecraft:exposed_chiseled_copper"),
		95 => Some("minecraft:exposed_copper"),
		392 => Some("minecraft:exposed_copper_bars"),
		1460 => Some("minecraft:exposed_copper_bulb"),
		401 => Some("minecraft:exposed_copper_chain"),
		1468 => Some("minecraft:exposed_copper_chest"),
		793 => Some("minecraft:exposed_copper_door"),
		1476 => Some("minecraft:exposed_copper_golem_statue"),
		1452 => Some("minecraft:exposed_copper_grate"),
		1349 => Some("minecraft:exposed_copper_lantern"),
		814 => Some("minecraft:exposed_copper_trapdoor"),
		103 => Some("minecraft:exposed_cut_copper"),
		111 => Some("minecraft:exposed_cut_copper_slab"),
		107 => Some("minecraft:exposed_cut_copper_stairs"),
		734 => Some("minecraft:exposed_lightning_rod"),
		1415 => Some("minecraft:eye_armor_trim_smithing_template"),
		333 => Some("minecraft:farmland"),
		949 => Some("minecraft:feather"),
		1123 => Some("minecraft:fermented_spider_eye"),
		203 => Some("minecraft:fern"),
		1333 => Some("minecraft:field_masoned_banner_pattern"),
		1104 => Some("minecraft:filled_map"),
		1214 => Some("minecraft:fire_charge"),
		662 => Some("minecraft:fire_coral"),
		657 => Some("minecraft:fire_coral_block"),
		672 => Some("minecraft:fire_coral_fan"),
		208 => Some("minecraft:firefly_bush"),
		1238 => Some("minecraft:firework_rocket"),
		1239 => Some("minecraft:firework_star"),
		1053 => Some("minecraft:fishing_rod"),
		1341 => Some("minecraft:fletching_table"),
		982 => Some("minecraft:flint"),
		891 => Some("minecraft:flint_and_steel"),
		1426 => Some("minecraft:flow_armor_trim_smithing_template"),
		1331 => Some("minecraft:flow_banner_pattern"),
		1436 => Some("minecraft:flow_pottery_sherd"),
		1325 => Some("minecraft:flower_banner_pattern"),
		1222 => Some("minecraft:flower_pot"),
		206 => Some("minecraft:flowering_azalea"),
		192 => Some("minecraft:flowering_azalea_leaves"),
		1154 => Some("minecraft:fox_spawn_egg"),
		1437 => Some("minecraft:friend_pottery_sherd"),
		1155 => Some("minecraft:frog_spawn_egg"),
		1406 => Some("minecraft:frogspawn"),
		334 => Some("minecraft:furnace"),
		856 => Some("minecraft:furnace_minecart"),
		1156 => Some("minecraft:ghast_spawn_egg"),
		1117 => Some("minecraft:ghast_tear"),
		1371 => Some("minecraft:gilded_blackstone"),
		195 => Some("minecraft:glass"),
		1120 => Some("minecraft:glass_bottle"),
		408 => Some("minecraft:glass_pane"),
		1129 => Some("minecraft:glistering_melon_slice"),
		1329 => Some("minecraft:globe_banner_pattern"),
		1357 => Some("minecraft:glow_berries"),
		1064 => Some("minecraft:glow_ink_sac"),
		1221 => Some("minecraft:glow_item_frame"),
		411 => Some("minecraft:glow_lichen"),
		1158 => Some("minecraft:glow_squid_spawn_egg"),
		367 => Some("minecraft:glowstone"),
		1056 => Some("minecraft:glowstone_dust"),
		1335 => Some("minecraft:goat_horn"),
		1159 => Some("minecraft:goat_spawn_egg"),
		92 => Some("minecraft:gold_block"),
		908 => Some("minecraft:gold_ingot"),
		1118 => Some("minecraft:gold_nugget"),
		70 => Some("minecraft:gold_ore"),
		986 => Some("minecraft:golden_apple"),
		929 => Some("minecraft:golden_axe"),
		977 => Some("minecraft:golden_boots"),
		1228 => Some("minecraft:golden_carrot"),
		975 => Some("minecraft:golden_chestplate"),
		974 => Some("minecraft:golden_helmet"),
		930 => Some("minecraft:golden_hoe"),
		1253 => Some("minecraft:golden_horse_armor"),
		976 => Some("minecraft:golden_leggings"),
		928 => Some("minecraft:golden_pickaxe"),
		927 => Some("minecraft:golden_shovel"),
		926 => Some("minecraft:golden_sword"),
		2 => Some("minecraft:granite"),
		707 => Some("minecraft:granite_slab"),
		690 => Some("minecraft:granite_stairs"),
		462 => Some("minecraft:granite_wall"),
		27 => Some("minecraft:grass_block"),
		63 => Some("minecraft:gravel"),
		1268 => Some("minecraft:gray_banner"),
		1093 => Some("minecraft:gray_bed"),
		1044 => Some("minecraft:gray_bundle"),
		1389 => Some("minecraft:gray_candle"),
		512 => Some("minecraft:gray_carpet"),
		621 => Some("minecraft:gray_concrete"),
		637 => Some("minecraft:gray_concrete_powder"),
		1073 => Some("minecraft:gray_dye"),
		605 => Some("minecraft:gray_glazed_terracotta"),
		845 => Some("minecraft:gray_harness"),
		589 => Some("minecraft:gray_shulker_box"),
		537 => Some("minecraft:gray_stained_glass"),
		553 => Some("minecraft:gray_stained_glass_pane"),
		493 => Some("minecraft:gray_terracotta"),
		220 => Some("minecraft:gray_wool"),
		1274 => Some("minecraft:green_banner"),
		1099 => Some("minecraft:green_bed"),
		1050 => Some("minecraft:green_bundle"),
		1395 => Some("minecraft:green_candle"),
		518 => Some("minecraft:green_carpet"),
		627 => Some("minecraft:green_concrete"),
		643 => Some("minecraft:green_concrete_powder"),
		1079 => Some("minecraft:green_dye"),
		611 => Some("minecraft:green_glazed_terracotta"),
		851 => Some("minecraft:green_harness"),
		595 => Some("minecraft:green_shulker_box"),
		543 => Some("minecraft:green_stained_glass"),
		559 => Some("minecraft:green_stained_glass_pane"),
		499 => Some("minecraft:green_terracotta"),
		226 => Some("minecraft:green_wool"),
		1342 => Some("minecraft:grindstone"),
		1160 => Some("minecraft:guardian_spawn_egg"),
		950 => Some("minecraft:gunpowder"),
		1332 => Some("minecraft:guster_banner_pattern"),
		1438 => Some("minecraft:guster_pottery_sherd"),
		266 => Some("minecraft:hanging_roots"),
		1157 => Some("minecraft:happy_ghast_spawn_egg"),
		504 => Some("minecraft:hay_block"),
		1321 => Some("minecraft:heart_of_the_sea"),
		1439 => Some("minecraft:heart_pottery_sherd"),
		1440 => Some("minecraft:heartbreak_pottery_sherd"),
		87 => Some("minecraft:heavy_core"),
		766 => Some("minecraft:heavy_weighted_pressure_plate"),
		1161 => Some("minecraft:hoglin_spawn_egg"),
		725 => Some("minecraft:honey_block"),
		1364 => Some("minecraft:honey_bottle"),
		1361 => Some("minecraft:honeycomb"),
		1365 => Some("minecraft:honeycomb_block"),
		727 => Some("minecraft:hopper"),
		858 => Some("minecraft:hopper_minecart"),
		663 => Some("minecraft:horn_coral"),
		658 => Some("minecraft:horn_coral_block"),
		673 => Some("minecraft:horn_coral_fan"),
		1162 => Some("minecraft:horse_spawn_egg"),
		1425 => Some("minecraft:host_armor_trim_smithing_template"),
		1441 => Some("minecraft:howl_pottery_sherd"),
		1163 => Some("minecraft:husk_spawn_egg"),
		338 => Some("minecraft:ice"),
		373 => Some("minecraft:infested_chiseled_stone_bricks"),
		369 => Some("minecraft:infested_cobblestone"),
		372 => Some("minecraft:infested_cracked_stone_bricks"),
		374 => Some("minecraft:infested_deepslate"),
		371 => Some("minecraft:infested_mossy_stone_bricks"),
		368 => Some("minecraft:infested_stone"),
		370 => Some("minecraft:infested_stone_bricks"),
		1063 => Some("minecraft:ink_sac"),
		934 => Some("minecraft:iron_axe"),
		390 => Some("minecraft:iron_bars"),
		90 => Some("minecraft:iron_block"),
		969 => Some("minecraft:iron_boots"),
		399 => Some("minecraft:iron_chain"),
		967 => Some("minecraft:iron_chestplate"),
		779 => Some("minecraft:iron_door"),
		1164 => Some("minecraft:iron_golem_spawn_egg"),
		966 => Some("minecraft:iron_helmet"),
		935 => Some("minecraft:iron_hoe"),
		1252 => Some("minecraft:iron_horse_armor"),
		904 => Some("minecraft:iron_ingot"),
		968 => Some("minecraft:iron_leggings"),
		1293 => Some("minecraft:iron_nugget"),
		66 => Some("minecraft:iron_ore"),
		933 => Some("minecraft:iron_pickaxe"),
		932 => Some("minecraft:iron_shovel"),
		931 => Some("minecraft:iron_sword"),
		800 => Some("minecraft:iron_trapdoor"),
		1220 => Some("minecraft:item_frame"),
		358 => Some("minecraft:jack_o_lantern"),
		884 => Some("minecraft:jigsaw"),
		343 => Some("minecraft:jukebox"),
		869 => Some("minecraft:jungle_boat"),
		754 => Some("minecraft:jungle_button"),
		870 => Some("minecraft:jungle_chest_boat"),
		783 => Some("minecraft:jungle_door"),
		347 => Some("minecraft:jungle_fence"),
		824 => Some("minecraft:jungle_fence_gate"),
		1003 => Some("minecraft:jungle_hanging_sign"),
		185 => Some("minecraft:jungle_leaves"),
		137 => Some("minecraft:jungle_log"),
		39 => Some("minecraft:jungle_planks"),
		770 => Some("minecraft:jungle_pressure_plate"),
		52 => Some("minecraft:jungle_sapling"),
		311 => Some("minecraft:jungle_shelf"),
		991 => Some("minecraft:jungle_sign"),
		273 => Some("minecraft:jungle_slab"),
		444 => Some("minecraft:jungle_stairs"),
		804 => Some("minecraft:jungle_trapdoor"),
		174 => Some("minecraft:jungle_wood"),
		257 => Some("minecraft:kelp"),
		1295 => Some("minecraft:knowledge_book"),
		335 => Some("minecraft:ladder"),
		1346 => Some("minecraft:lantern"),
		197 => Some("minecraft:lapis_block"),
		900 => Some("minecraft:lapis_lazuli"),
		76 => Some("minecraft:lapis_ore"),
		1400 => Some("minecraft:large_amethyst_bud"),
		529 => Some("minecraft:large_fern"),
		1014 => Some("minecraft:lava_bucket"),
		1256 => Some("minecraft:lead"),
		260 => Some("minecraft:leaf_litter"),
		1017 => Some("minecraft:leather"),
		957 => Some("minecraft:leather_boots"),
		955 => Some("minecraft:leather_chestplate"),
		954 => Some("minecraft:leather_helmet"),
		1255 => Some("minecraft:leather_horse_armor"),
		956 => Some("minecraft:leather_leggings"),
		730 => Some("minecraft:lectern"),
		732 => Some("minecraft:lever"),
		503 => Some("minecraft:light"),
		1264 => Some("minecraft:light_blue_banner"),
		1089 => Some("minecraft:light_blue_bed"),
		1040 => Some("minecraft:light_blue_bundle"),
		1385 => Some("minecraft:light_blue_candle"),
		508 => Some("minecraft:light_blue_carpet"),
		617 => Some("minecraft:light_blue_concrete"),
		633 => Some("minecraft:light_blue_concrete_powder"),
		1069 => Some("minecraft:light_blue_dye"),
		601 => Some("minecraft:light_blue_glazed_terracotta"),
		841 => Some("minecraft:light_blue_harness"),
		585 => Some("minecraft:light_blue_shulker_box"),
		533 => Some("minecraft:light_blue_stained_glass"),
		549 => Some("minecraft:light_blue_stained_glass_pane"),
		489 => Some("minecraft:light_blue_terracotta"),
		216 => Some("minecraft:light_blue_wool"),
		1269 => Some("minecraft:light_gray_banner"),
		1094 => Some("minecraft:light_gray_bed"),
		1045 => Some("minecraft:light_gray_bundle"),
		1390 => Some("minecraft:light_gray_candle"),
		513 => Some("minecraft:light_gray_carpet"),
		622 => Some("minecraft:light_gray_concrete"),
		638 => Some("minecraft:light_gray_concrete_powder"),
		1074 => Some("minecraft:light_gray_dye"),
		606 => Some("minecraft:light_gray_glazed_terracotta"),
		846 => Some("minecraft:light_gray_harness"),
		590 => Some("minecraft:light_gray_shulker_box"),
		538 => Some("minecraft:light_gray_stained_glass"),
		554 => Some("minecraft:light_gray_stained_glass_pane"),
		494 => Some("minecraft:light_gray_terracotta"),
		221 => Some("minecraft:light_gray_wool"),
		765 => Some("minecraft:light_weighted_pressure_plate"),
		733 => Some("minecraft:lightning_rod"),
		525 => Some("minecraft:lilac"),
		242 => Some("minecraft:lily_of_the_valley"),
		423 => Some("minecraft:lily_pad"),
		1266 => Some("minecraft:lime_banner"),
		1091 => Some("minecraft:lime_bed"),
		1042 => Some("minecraft:lime_bundle"),
		1387 => Some("minecraft:lime_candle"),
		510 => Some("minecraft:lime_carpet"),
		619 => Some("minecraft:lime_concrete"),
		635 => Some("minecraft:lime_concrete_powder"),
		1071 => Some("minecraft:lime_dye"),
		603 => Some("minecraft:lime_glazed_terracotta"),
		843 => Some("minecraft:lime_harness"),
		587 => Some("minecraft:lime_shulker_box"),
		535 => Some("minecraft:lime_stained_glass"),
		551 => Some("minecraft:lime_stained_glass_pane"),
		491 => Some("minecraft:lime_terracotta"),
		218 => Some("minecraft:lime_wool"),
		1289 => Some("minecraft:lingering_potion"),
		1165 => Some("minecraft:llama_spawn_egg"),
		1366 => Some("minecraft:lodestone"),
		1324 => Some("minecraft:loom"),
		1219 => Some("minecraft:mace"),
		1263 => Some("minecraft:magenta_banner"),
		1088 => Some("minecraft:magenta_bed"),
		1039 => Some("minecraft:magenta_bundle"),
		1384 => Some("minecraft:magenta_candle"),
		507 => Some("minecraft:magenta_carpet"),
		616 => Some("minecraft:magenta_concrete"),
		632 => Some("minecraft:magenta_concrete_powder"),
		1068 => Some("minecraft:magenta_dye"),
		600 => Some("minecraft:magenta_glazed_terracotta"),
		840 => Some("minecraft:magenta_harness"),
		584 => Some("minecraft:magenta_shulker_box"),
		532 => Some("minecraft:magenta_stained_glass"),
		548 => Some("minecraft:magenta_stained_glass_pane"),
		488 => Some("minecraft:magenta_terracotta"),
		215 => Some("minecraft:magenta_wool"),
		575 => Some("minecraft:magma_block"),
		1125 => Some("minecraft:magma_cream"),
		1166 => Some("minecraft:magma_cube_spawn_egg"),
		879 => Some("minecraft:mangrove_boat"),
		759 => Some("minecraft:mangrove_button"),
		880 => Some("minecraft:mangrove_chest_boat"),
		788 => Some("minecraft:mangrove_door"),
		352 => Some("minecraft:mangrove_fence"),
		829 => Some("minecraft:mangrove_fence_gate"),
		1008 => Some("minecraft:mangrove_hanging_sign"),
		190 => Some("minecraft:mangrove_leaves"),
		142 => Some("minecraft:mangrove_log"),
		44 => Some("minecraft:mangrove_planks"),
		775 => Some("minecraft:mangrove_pressure_plate"),
		57 => Some("minecraft:mangrove_propagule"),
		143 => Some("minecraft:mangrove_roots"),
		312 => Some("minecraft:mangrove_shelf"),
		996 => Some("minecraft:mangrove_sign"),
		278 => Some("minecraft:mangrove_slab"),
		449 => Some("minecraft:mangrove_stairs"),
		809 => Some("minecraft:mangrove_trapdoor"),
		179 => Some("minecraft:mangrove_wood"),
		1227 => Some("minecraft:map"),
		1399 => Some("minecraft:medium_amethyst_bud"),
		409 => Some("minecraft:melon"),
		1109 => Some("minecraft:melon_seeds"),
		1106 => Some("minecraft:melon_slice"),
		1018 => Some("minecraft:milk_bucket"),
		854 => Some("minecraft:minecart"),
		1442 => Some("minecraft:miner_pottery_sherd"),
		1328 => Some("minecraft:mojang_banner_pattern"),
		1167 => Some("minecraft:mooshroom_spawn_egg"),
		262 => Some("minecraft:moss_block"),
		261 => Some("minecraft:moss_carpet"),
		320 => Some("minecraft:mossy_cobblestone"),
		703 => Some("minecraft:mossy_cobblestone_slab"),
		685 => Some("minecraft:mossy_cobblestone_stairs"),
		457 => Some("minecraft:mossy_cobblestone_wall"),
		701 => Some("minecraft:mossy_stone_brick_slab"),
		683 => Some("minecraft:mossy_stone_brick_stairs"),
		461 => Some("minecraft:mossy_stone_brick_wall"),
		376 => Some("minecraft:mossy_stone_bricks"),
		1443 => Some("minecraft:mourner_pottery_sherd"),
		32 => Some("minecraft:mud"),
		291 => Some("minecraft:mud_brick_slab"),
		421 => Some("minecraft:mud_brick_stairs"),
		464 => Some("minecraft:mud_brick_wall"),
		380 => Some("minecraft:mud_bricks"),
		144 => Some("minecraft:muddy_mangrove_roots"),
		1168 => Some("minecraft:mule_spawn_egg"),
		389 => Some("minecraft:mushroom_stem"),
		947 => Some("minecraft:mushroom_stew"),
		1310 => Some("minecraft:music_disc_11"),
		1297 => Some("minecraft:music_disc_13"),
		1314 => Some("minecraft:music_disc_5"),
		1299 => Some("minecraft:music_disc_blocks"),
		1298 => Some("minecraft:music_disc_cat"),
		1300 => Some("minecraft:music_disc_chirp"),
		1301 => Some("minecraft:music_disc_creator"),
		1302 => Some("minecraft:music_disc_creator_music_box"),
		1303 => Some("minecraft:music_disc_far"),
		1304 => Some("minecraft:music_disc_lava_chicken"),
		1305 => Some("minecraft:music_disc_mall"),
		1306 => Some("minecraft:music_disc_mellohi"),
		1312 => Some("minecraft:music_disc_otherside"),
		1315 => Some("minecraft:music_disc_pigstep"),
		1316 => Some("minecraft:music_disc_precipice"),
		1313 => Some("minecraft:music_disc_relic"),
		1307 => Some("minecraft:music_disc_stal"),
		1308 => Some("minecraft:music_disc_strad"),
		1317 => Some("minecraft:music_disc_tears"),
		1311 => Some("minecraft:music_disc_wait"),
		1309 => Some("minecraft:music_disc_ward"),
		1259 => Some("minecraft:mutton"),
		422 => Some("minecraft:mycelium"),
		1257 => Some("minecraft:name_tag"),
		1320 => Some("minecraft:nautilus_shell"),
		1241 => Some("minecraft:nether_brick"),
		427 => Some("minecraft:nether_brick_fence"),
		292 => Some("minecraft:nether_brick_slab"),
		428 => Some("minecraft:nether_brick_stairs"),
		465 => Some("minecraft:nether_brick_wall"),
		424 => Some("minecraft:nether_bricks"),
		80 => Some("minecraft:nether_gold_ore"),
		81 => Some("minecraft:nether_quartz_ore"),
		253 => Some("minecraft:nether_sprouts"),
		1236 => Some("minecraft:nether_star"),
		1119 => Some("minecraft:nether_wart"),
		576 => Some("minecraft:nether_wart_block"),
		944 => Some("minecraft:netherite_axe"),
		94 => Some("minecraft:netherite_block"),
		981 => Some("minecraft:netherite_boots"),
		979 => Some("minecraft:netherite_chestplate"),
		978 => Some("minecraft:netherite_helmet"),
		945 => Some("minecraft:netherite_hoe"),
		909 => Some("minecraft:netherite_ingot"),
		980 => Some("minecraft:netherite_leggings"),
		943 => Some("minecraft:netherite_pickaxe"),
		910 => Some("minecraft:netherite_scrap"),
		942 => Some("minecraft:netherite_shovel"),
		941 => Some("minecraft:netherite_sword"),
		1409 => Some("minecraft:netherite_upgrade_smithing_template"),
		359 => Some("minecraft:netherrack"),
		748 => Some("minecraft:note_block"),
		863 => Some("minecraft:oak_boat"),
		751 => Some("minecraft:oak_button"),
		864 => Some("minecraft:oak_chest_boat"),
		780 => Some("minecraft:oak_door"),
		344 => Some("minecraft:oak_fence"),
		821 => Some("minecraft:oak_fence_gate"),
		1000 => Some("minecraft:oak_hanging_sign"),
		182 => Some("minecraft:oak_leaves"),
		134 => Some("minecraft:oak_log"),
		36 => Some("minecraft:oak_planks"),
		767 => Some("minecraft:oak_pressure_plate"),
		49 => Some("minecraft:oak_sapling"),
		313 => Some("minecraft:oak_shelf"),
		988 => Some("minecraft:oak_sign"),
		270 => Some("minecraft:oak_slab"),
		441 => Some("minecraft:oak_stairs"),
		801 => Some("minecraft:oak_trapdoor"),
		171 => Some("minecraft:oak_wood"),
		726 => Some("minecraft:observer"),
		321 => Some("minecraft:obsidian"),
		1169 => Some("minecraft:ocelot_spawn_egg"),
		1403 => Some("minecraft:ochre_froglight"),
		1487 => Some("minecraft:ominous_bottle"),
		1485 => Some("minecraft:ominous_trial_key"),
		230 => Some("minecraft:open_eyeblossom"),
		1262 => Some("minecraft:orange_banner"),
		1087 => Some("minecraft:orange_bed"),
		1038 => Some("minecraft:orange_bundle"),
		1383 => Some("minecraft:orange_candle"),
		506 => Some("minecraft:orange_carpet"),
		615 => Some("minecraft:orange_concrete"),
		631 => Some("minecraft:orange_concrete_powder"),
		1067 => Some("minecraft:orange_dye"),
		599 => Some("minecraft:orange_glazed_terracotta"),
		839 => Some("minecraft:orange_harness"),
		583 => Some("minecraft:orange_shulker_box"),
		531 => Some("minecraft:orange_stained_glass"),
		547 => Some("minecraft:orange_stained_glass_pane"),
		487 => Some("minecraft:orange_terracotta"),
		237 => Some("minecraft:orange_tulip"),
		214 => Some("minecraft:orange_wool"),
		240 => Some("minecraft:oxeye_daisy"),
		101 => Some("minecraft:oxidized_chiseled_copper"),
		97 => Some("minecraft:oxidized_copper"),
		394 => Some("minecraft:oxidized_copper_bars"),
		1462 => Some("minecraft:oxidized_copper_bulb"),
		403 => Some("minecraft:oxidized_copper_chain"),
		1470 => Some("minecraft:oxidized_copper_chest"),
		795 => Some("minecraft:oxidized_copper_door"),
		1478 => Some("minecraft:oxidized_copper_golem_statue"),
		1454 => Some("minecraft:oxidized_copper_grate"),
		1351 => Some("minecraft:oxidized_copper_lantern"),
		816 => Some("minecraft:oxidized_copper_trapdoor"),
		105 => Some("minecraft:oxidized_cut_copper"),
		113 => Some("minecraft:oxidized_cut_copper_slab"),
		109 => Some("minecraft:oxidized_cut_copper_stairs"),
		736 => Some("minecraft:oxidized_lightning_rod"),
		522 => Some("minecraft:packed_ice"),
		379 => Some("minecraft:packed_mud"),
		985 => Some("minecraft:painting"),
		264 => Some("minecraft:pale_hanging_moss"),
		265 => Some("minecraft:pale_moss_block"),
		263 => Some("minecraft:pale_moss_carpet"),
		877 => Some("minecraft:pale_oak_boat"),
		758 => Some("minecraft:pale_oak_button"),
		878 => Some("minecraft:pale_oak_chest_boat"),
		787 => Some("minecraft:pale_oak_door"),
		351 => Some("minecraft:pale_oak_fence"),
		828 => Some("minecraft:pale_oak_fence_gate"),
		1007 => Some("minecraft:pale_oak_hanging_sign"),
		189 => Some("minecraft:pale_oak_leaves"),
		140 => Some("minecraft:pale_oak_log"),
		43 => Some("minecraft:pale_oak_planks"),
		774 => Some("minecraft:pale_oak_pressure_plate"),
		56 => Some("minecraft:pale_oak_sapling"),
		314 => Some("minecraft:pale_oak_shelf"),
		995 => Some("minecraft:pale_oak_sign"),
		277 => Some("minecraft:pale_oak_slab"),
		448 => Some("minecraft:pale_oak_stairs"),
		808 => Some("minecraft:pale_oak_trapdoor"),
		177 => Some("minecraft:pale_oak_wood"),
		1170 => Some("minecraft:panda_spawn_egg"),
		1028 => Some("minecraft:paper"),
		1171 => Some("minecraft:parrot_spawn_egg"),
		1405 => Some("minecraft:pearlescent_froglight"),
		527 => Some("minecraft:peony"),
		287 => Some("minecraft:petrified_oak_slab"),
		861 => Some("minecraft:phantom_membrane"),
		1172 => Some("minecraft:phantom_spawn_egg"),
		1173 => Some("minecraft:pig_spawn_egg"),
		1330 => Some("minecraft:piglin_banner_pattern"),
		1175 => Some("minecraft:piglin_brute_spawn_egg"),
		1235 => Some("minecraft:piglin_head"),
		1174 => Some("minecraft:piglin_spawn_egg"),
		1176 => Some("minecraft:pillager_spawn_egg"),
		1267 => Some("minecraft:pink_banner"),
		1092 => Some("minecraft:pink_bed"),
		1043 => Some("minecraft:pink_bundle"),
		1388 => Some("minecraft:pink_candle"),
		511 => Some("minecraft:pink_carpet"),
		620 => Some("minecraft:pink_concrete"),
		636 => Some("minecraft:pink_concrete_powder"),
		1072 => Some("minecraft:pink_dye"),
		604 => Some("minecraft:pink_glazed_terracotta"),
		844 => Some("minecraft:pink_harness"),
		258 => Some("minecraft:pink_petals"),
		588 => Some("minecraft:pink_shulker_box"),
		536 => Some("minecraft:pink_stained_glass"),
		552 => Some("minecraft:pink_stained_glass_pane"),
		492 => Some("minecraft:pink_terracotta"),
		239 => Some("minecraft:pink_tulip"),
		219 => Some("minecraft:pink_wool"),
		722 => Some("minecraft:piston"),
		245 => Some("minecraft:pitcher_plant"),
		1281 => Some("minecraft:pitcher_pod"),
		1231 => Some("minecraft:player_head"),
		1444 => Some("minecraft:plenty_pottery_sherd"),
		30 => Some("minecraft:podzol"),
		1402 => Some("minecraft:pointed_dripstone"),
		1226 => Some("minecraft:poisonous_potato"),
		1177 => Some("minecraft:polar_bear_spawn_egg"),
		7 => Some("minecraft:polished_andesite"),
		710 => Some("minecraft:polished_andesite_slab"),
		693 => Some("minecraft:polished_andesite_stairs"),
		363 => Some("minecraft:polished_basalt"),
		1372 => Some("minecraft:polished_blackstone"),
		1377 => Some("minecraft:polished_blackstone_brick_slab"),
		1378 => Some("minecraft:polished_blackstone_brick_stairs"),
		473 => Some("minecraft:polished_blackstone_brick_wall"),
		1376 => Some("minecraft:polished_blackstone_bricks"),
		750 => Some("minecraft:polished_blackstone_button"),
		764 => Some("minecraft:polished_blackstone_pressure_plate"),
		1373 => Some("minecraft:polished_blackstone_slab"),
		1374 => Some("minecraft:polished_blackstone_stairs"),
		472 => Some("minecraft:polished_blackstone_wall"),
		10 => Some("minecraft:polished_deepslate"),
		713 => Some("minecraft:polished_deepslate_slab"),
		696 => Some("minecraft:polished_deepslate_stairs"),
		475 => Some("minecraft:polished_deepslate_wall"),
		5 => Some("minecraft:polished_diorite"),
		702 => Some("minecraft:polished_diorite_slab"),
		684 => Some("minecraft:polished_diorite_stairs"),
		3 => Some("minecraft:polished_granite"),
		699 => Some("minecraft:polished_granite_slab"),
		681 => Some("minecraft:polished_granite_stairs"),
		17 => Some("minecraft:polished_tuff"),
		18 => Some("minecraft:polished_tuff_slab"),
		19 => Some("minecraft:polished_tuff_stairs"),
		20 => Some("minecraft:polished_tuff_wall"),
		1279 => Some("minecraft:popped_chorus_fruit"),
		232 => Some("minecraft:poppy"),
		983 => Some("minecraft:porkchop"),
		1224 => Some("minecraft:potato"),
		1121 => Some("minecraft:potion"),
		1015 => Some("minecraft:powder_snow_bucket"),
		833 => Some("minecraft:powered_rail"),
		562 => Some("minecraft:prismarine"),
		298 => Some("minecraft:prismarine_brick_slab"),
		566 => Some("minecraft:prismarine_brick_stairs"),
		563 => Some("minecraft:prismarine_bricks"),
		1244 => Some("minecraft:prismarine_crystals"),
		1243 => Some("minecraft:prismarine_shard"),
		297 => Some("minecraft:prismarine_slab"),
		565 => Some("minecraft:prismarine_stairs"),
		459 => Some("minecraft:prismarine_wall"),
		1445 => Some("minecraft:prize_pottery_sherd"),
		1060 => Some("minecraft:pufferfish"),
		1019 => Some("minecraft:pufferfish_bucket"),
		1178 => Some("minecraft:pufferfish_spawn_egg"),
		356 => Some("minecraft:pumpkin"),
		1237 => Some("minecraft:pumpkin_pie"),
		1108 => Some("minecraft:pumpkin_seeds"),
		1271 => Some("minecraft:purple_banner"),
		1096 => Some("minecraft:purple_bed"),
		1047 => Some("minecraft:purple_bundle"),
		1392 => Some("minecraft:purple_candle"),
		515 => Some("minecraft:purple_carpet"),
		624 => Some("minecraft:purple_concrete"),
		640 => Some("minecraft:purple_concrete_powder"),
		1076 => Some("minecraft:purple_dye"),
		608 => Some("minecraft:purple_glazed_terracotta"),
		848 => Some("minecraft:purple_harness"),
		592 => Some("minecraft:purple_shulker_box"),
		540 => Some("minecraft:purple_stained_glass"),
		556 => Some("minecraft:purple_stained_glass_pane"),
		496 => Some("minecraft:purple_terracotta"),
		223 => Some("minecraft:purple_wool"),
		326 => Some("minecraft:purpur_block"),
		327 => Some("minecraft:purpur_pillar"),
		296 => Some("minecraft:purpur_slab"),
		328 => Some("minecraft:purpur_stairs"),
		901 => Some("minecraft:quartz"),
		482 => Some("minecraft:quartz_block"),
		483 => Some("minecraft:quartz_bricks"),
		484 => Some("minecraft:quartz_pillar"),
		293 => Some("minecraft:quartz_slab"),
		485 => Some("minecraft:quartz_stairs"),
		1245 => Some("minecraft:rabbit"),
		1248 => Some("minecraft:rabbit_foot"),
		1249 => Some("minecraft:rabbit_hide"),
		1179 => Some("minecraft:rabbit_spawn_egg"),
		1247 => Some("minecraft:rabbit_stew"),
		835 => Some("minecraft:rail"),
		1424 => Some("minecraft:raiser_armor_trim_smithing_template"),
		1180 => Some("minecraft:ravager_spawn_egg"),
		905 => Some("minecraft:raw_copper"),
		85 => Some("minecraft:raw_copper_block"),
		907 => Some("minecraft:raw_gold"),
		86 => Some("minecraft:raw_gold_block"),
		903 => Some("minecraft:raw_iron"),
		84 => Some("minecraft:raw_iron_block"),
		1035 => Some("minecraft:recovery_compass"),
		1275 => Some("minecraft:red_banner"),
		1100 => Some("minecraft:red_bed"),
		1051 => Some("minecraft:red_bundle"),
		1396 => Some("minecraft:red_candle"),
		519 => Some("minecraft:red_carpet"),
		628 => Some("minecraft:red_concrete"),
		644 => Some("minecraft:red_concrete_powder"),
		1080 => Some("minecraft:red_dye"),
		612 => Some("minecraft:red_glazed_terracotta"),
		852 => Some("minecraft:red_harness"),
		248 => Some("minecraft:red_mushroom"),
		388 => Some("minecraft:red_mushroom_block"),
		709 => Some("minecraft:red_nether_brick_slab"),
		692 => Some("minecraft:red_nether_brick_stairs"),
		467 => Some("minecraft:red_nether_brick_wall"),
		578 => Some("minecraft:red_nether_bricks"),
		62 => Some("minecraft:red_sand"),
		569 => Some("minecraft:red_sandstone"),
		294 => Some("minecraft:red_sandstone_slab"),
		572 => Some("minecraft:red_sandstone_stairs"),
		460 => Some("minecraft:red_sandstone_wall"),
		596 => Some("minecraft:red_shulker_box"),
		544 => Some("minecraft:red_stained_glass"),
		560 => Some("minecraft:red_stained_glass_pane"),
		500 => Some("minecraft:red_terracotta"),
		236 => Some("minecraft:red_tulip"),
		227 => Some("minecraft:red_wool"),
		717 => Some("minecraft:redstone"),
		719 => Some("minecraft:redstone_block"),
		747 => Some("minecraft:redstone_lamp"),
		72 => Some("minecraft:redstone_ore"),
		718 => Some("minecraft:redstone_torch"),
		386 => Some("minecraft:reinforced_deepslate"),
		720 => Some("minecraft:repeater"),
		573 => Some("minecraft:repeating_command_block"),
		413 => Some("minecraft:resin_block"),
		1242 => Some("minecraft:resin_brick"),
		416 => Some("minecraft:resin_brick_slab"),
		415 => Some("minecraft:resin_brick_stairs"),
		417 => Some("minecraft:resin_brick_wall"),
		414 => Some("minecraft:resin_bricks"),
		412 => Some("minecraft:resin_clump"),
		1380 => Some("minecraft:respawn_anchor"),
		1419 => Some("minecraft:rib_armor_trim_smithing_template"),
		31 => Some("minecraft:rooted_dirt"),
		526 => Some("minecraft:rose_bush"),
		1114 => Some("minecraft:rotten_flesh"),
		837 => Some("minecraft:saddle"),
		1058 => Some("minecraft:salmon"),
		1020 => Some("minecraft:salmon_bucket"),
		1181 => Some("minecraft:salmon_spawn_egg"),
		59 => Some("minecraft:sand"),
		198 => Some("minecraft:sandstone"),
		285 => Some("minecraft:sandstone_slab"),
		438 => Some("minecraft:sandstone_stairs"),
		468 => Some("minecraft:sandstone_wall"),
		716 => Some("minecraft:scaffolding"),
		1446 => Some("minecraft:scrape_pottery_sherd"),
		429 => Some("minecraft:sculk"),
		431 => Some("minecraft:sculk_catalyst"),
		742 => Some("minecraft:sculk_sensor"),
		432 => Some("minecraft:sculk_shrieker"),
		430 => Some("minecraft:sculk_vein"),
		568 => Some("minecraft:sea_lantern"),
		212 => Some("minecraft:sea_pickle"),
		211 => Some("minecraft:seagrass"),
		1410 => Some("minecraft:sentry_armor_trim_smithing_template"),
		1422 => Some("minecraft:shaper_armor_trim_smithing_template"),
		1447 => Some("minecraft:sheaf_pottery_sherd"),
		1105 => Some("minecraft:shears"),
		1182 => Some("minecraft:sheep_spawn_egg"),
		1448 => Some("minecraft:shelter_pottery_sherd"),
		1290 => Some("minecraft:shield"),
		209 => Some("minecraft:short_dry_grass"),
		202 => Some("minecraft:short_grass"),
		1360 => Some("minecraft:shroomlight"),
		581 => Some("minecraft:shulker_box"),
		1292 => Some("minecraft:shulker_shell"),
		1183 => Some("minecraft:shulker_spawn_egg"),
		1423 => Some("minecraft:silence_armor_trim_smithing_template"),
		1184 => Some("minecraft:silverfish_spawn_egg"),
		1186 => Some("minecraft:skeleton_horse_spawn_egg"),
		1229 => Some("minecraft:skeleton_skull"),
		1185 => Some("minecraft:skeleton_spawn_egg"),
		1327 => Some("minecraft:skull_banner_pattern"),
		1449 => Some("minecraft:skull_pottery_sherd"),
		1030 => Some("minecraft:slime_ball"),
		724 => Some("minecraft:slime_block"),
		1187 => Some("minecraft:slime_spawn_egg"),
		1398 => Some("minecraft:small_amethyst_bud"),
		268 => Some("minecraft:small_dripleaf"),
		1343 => Some("minecraft:smithing_table"),
		1338 => Some("minecraft:smoker"),
		364 => Some("minecraft:smooth_basalt"),
		300 => Some("minecraft:smooth_quartz"),
		706 => Some("minecraft:smooth_quartz_slab"),
		689 => Some("minecraft:smooth_quartz_stairs"),
		301 => Some("minecraft:smooth_red_sandstone"),
		700 => Some("minecraft:smooth_red_sandstone_slab"),
		682 => Some("minecraft:smooth_red_sandstone_stairs"),
		302 => Some("minecraft:smooth_sandstone"),
		705 => Some("minecraft:smooth_sandstone_slab"),
		688 => Some("minecraft:smooth_sandstone_stairs"),
		303 => Some("minecraft:smooth_stone"),
		284 => Some("minecraft:smooth_stone_slab"),
		647 => Some("minecraft:sniffer_egg"),
		1188 => Some("minecraft:sniffer_spawn_egg"),
		1450 => Some("minecraft:snort_pottery_sherd"),
		1418 => Some("minecraft:snout_armor_trim_smithing_template"),
		337 => Some("minecraft:snow"),
		339 => Some("minecraft:snow_block"),
		1189 => Some("minecraft:snow_golem_spawn_egg"),
		1016 => Some("minecraft:snowball"),
		1359 => Some("minecraft:soul_campfire"),
		1347 => Some("minecraft:soul_lantern"),
		360 => Some("minecraft:soul_sand"),
		361 => Some("minecraft:soul_soil"),
		365 => Some("minecraft:soul_torch"),
		329 => Some("minecraft:spawner"),
		1287 => Some("minecraft:spectral_arrow"),
		1122 => Some("minecraft:spider_eye"),
		1190 => Some("minecraft:spider_spawn_egg"),
		1420 => Some("minecraft:spire_armor_trim_smithing_template"),
		1286 => Some("minecraft:splash_potion"),
		193 => Some("minecraft:sponge"),
		246 => Some("minecraft:spore_blossom"),
		865 => Some("minecraft:spruce_boat"),
		752 => Some("minecraft:spruce_button"),
		866 => Some("minecraft:spruce_chest_boat"),
		781 => Some("minecraft:spruce_door"),
		345 => Some("minecraft:spruce_fence"),
		822 => Some("minecraft:spruce_fence_gate"),
		1001 => Some("minecraft:spruce_hanging_sign"),
		183 => Some("minecraft:spruce_leaves"),
		135 => Some("minecraft:spruce_log"),
		37 => Some("minecraft:spruce_planks"),
		768 => Some("minecraft:spruce_pressure_plate"),
		50 => Some("minecraft:spruce_sapling"),
		315 => Some("minecraft:spruce_shelf"),
		989 => Some("minecraft:spruce_sign"),
		271 => Some("minecraft:spruce_slab"),
		442 => Some("minecraft:spruce_stairs"),
		802 => Some("minecraft:spruce_trapdoor"),
		172 => Some("minecraft:spruce_wood"),
		1055 => Some("minecraft:spyglass"),
		1191 => Some("minecraft:squid_spawn_egg"),
		946 => Some("minecraft:stick"),
		723 => Some("minecraft:sticky_piston"),
		1 => Some("minecraft:stone"),
		924 => Some("minecraft:stone_axe"),
		290 => Some("minecraft:stone_brick_slab"),
		420 => Some("minecraft:stone_brick_stairs"),
		463 => Some("minecraft:stone_brick_wall"),
		375 => Some("minecraft:stone_bricks"),
		749 => Some("minecraft:stone_button"),
		925 => Some("minecraft:stone_hoe"),
		923 => Some("minecraft:stone_pickaxe"),
		763 => Some("minecraft:stone_pressure_plate"),
		922 => Some("minecraft:stone_shovel"),
		283 => Some("minecraft:stone_slab"),
		687 => Some("minecraft:stone_stairs"),
		921 => Some("minecraft:stone_sword"),
		1344 => Some("minecraft:stonecutter"),
		1192 => Some("minecraft:stray_spawn_egg"),
		1193 => Some("minecraft:strider_spawn_egg"),
		948 => Some("minecraft:string"),
		152 => Some("minecraft:stripped_acacia_log"),
		163 => Some("minecraft:stripped_acacia_wood"),
		170 => Some("minecraft:stripped_bamboo_block"),
		150 => Some("minecraft:stripped_birch_log"),
		161 => Some("minecraft:stripped_birch_wood"),
		153 => Some("minecraft:stripped_cherry_log"),
		164 => Some("minecraft:stripped_cherry_wood"),
		168 => Some("minecraft:stripped_crimson_hyphae"),
		157 => Some("minecraft:stripped_crimson_stem"),
		154 => Some("minecraft:stripped_dark_oak_log"),
		165 => Some("minecraft:stripped_dark_oak_wood"),
		151 => Some("minecraft:stripped_jungle_log"),
		162 => Some("minecraft:stripped_jungle_wood"),
		156 => Some("minecraft:stripped_mangrove_log"),
		167 => Some("minecraft:stripped_mangrove_wood"),
		148 => Some("minecraft:stripped_oak_log"),
		159 => Some("minecraft:stripped_oak_wood"),
		155 => Some("minecraft:stripped_pale_oak_log"),
		166 => Some("minecraft:stripped_pale_oak_wood"),
		149 => Some("minecraft:stripped_spruce_log"),
		160 => Some("minecraft:stripped_spruce_wood"),
		169 => Some("minecraft:stripped_warped_hyphae"),
		158 => Some("minecraft:stripped_warped_stem"),
		883 => Some("minecraft:structure_block"),
		580 => Some("minecraft:structure_void"),
		1084 => Some("minecraft:sugar"),
		256 => Some("minecraft:sugar_cane"),
		524 => Some("minecraft:sunflower"),
		61 => Some("minecraft:suspicious_gravel"),
		60 => Some("minecraft:suspicious_sand"),
		1323 => Some("minecraft:suspicious_stew"),
		1356 => Some("minecraft:sweet_berries"),
		1024 => Some("minecraft:tadpole_bucket"),
		1194 => Some("minecraft:tadpole_spawn_egg"),
		210 => Some("minecraft:tall_dry_grass"),
		528 => Some("minecraft:tall_grass"),
		731 => Some("minecraft:target"),
		521 => Some("minecraft:terracotta"),
		885 => Some("minecraft:test_block"),
		886 => Some("minecraft:test_instance_block"),
		1417 => Some("minecraft:tide_armor_trim_smithing_template"),
		196 => Some("minecraft:tinted_glass"),
		1288 => Some("minecraft:tipped_arrow"),
		746 => Some("minecraft:tnt"),
		857 => Some("minecraft:tnt_minecart"),
		322 => Some("minecraft:torch"),
		244 => Some("minecraft:torchflower"),
		1280 => Some("minecraft:torchflower_seeds"),
		1291 => Some("minecraft:totem_of_undying"),
		1195 => Some("minecraft:trader_llama_spawn_egg"),
		745 => Some("minecraft:trapped_chest"),
		1484 => Some("minecraft:trial_key"),
		1483 => Some("minecraft:trial_spawner"),
		1319 => Some("minecraft:trident"),
		744 => Some("minecraft:tripwire_hook"),
		1059 => Some("minecraft:tropical_fish"),
		1022 => Some("minecraft:tropical_fish_bucket"),
		1196 => Some("minecraft:tropical_fish_spawn_egg"),
		659 => Some("minecraft:tube_coral"),
		654 => Some("minecraft:tube_coral_block"),
		669 => Some("minecraft:tube_coral_fan"),
		12 => Some("minecraft:tuff"),
		22 => Some("minecraft:tuff_brick_slab"),
		23 => Some("minecraft:tuff_brick_stairs"),
		24 => Some("minecraft:tuff_brick_wall"),
		21 => Some("minecraft:tuff_bricks"),
		13 => Some("minecraft:tuff_slab"),
		14 => Some("minecraft:tuff_stairs"),
		15 => Some("minecraft:tuff_wall"),
		646 => Some("minecraft:turtle_egg"),
		887 => Some("minecraft:turtle_helmet"),
		888 => Some("minecraft:turtle_scute"),
		1197 => Some("minecraft:turtle_spawn_egg"),
		255 => Some("minecraft:twisting_vines"),
		1486 => Some("minecraft:vault"),
		1404 => Some("minecraft:verdant_froglight"),
		1416 => Some("minecraft:vex_armor_trim_smithing_template"),
		1198 => Some("minecraft:vex_spawn_egg"),
		1199 => Some("minecraft:villager_spawn_egg"),
		1200 => Some("minecraft:vindicator_spawn_egg"),
		410 => Some("minecraft:vine"),
		1201 => Some("minecraft:wandering_trader_spawn_egg"),
		1414 => Some("minecraft:ward_armor_trim_smithing_template"),
		1202 => Some("minecraft:warden_spawn_egg"),
		762 => Some("minecraft:warped_button"),
		791 => Some("minecraft:warped_door"),
		355 => Some("minecraft:warped_fence"),
		832 => Some("minecraft:warped_fence_gate"),
		250 => Some("minecraft:warped_fungus"),
		860 => Some("minecraft:warped_fungus_on_a_stick"),
		1011 => Some("minecraft:warped_hanging_sign"),
		181 => Some("minecraft:warped_hyphae"),
		34 => Some("minecraft:warped_nylium"),
		47 => Some("minecraft:warped_planks"),
		778 => Some("minecraft:warped_pressure_plate"),
		252 => Some("minecraft:warped_roots"),
		316 => Some("minecraft:warped_shelf"),
		999 => Some("minecraft:warped_sign"),
		282 => Some("minecraft:warped_slab"),
		453 => Some("minecraft:warped_stairs"),
		146 => Some("minecraft:warped_stem"),
		812 => Some("minecraft:warped_trapdoor"),
		577 => Some("minecraft:warped_wart_block"),
		1013 => Some("minecraft:water_bucket"),
		118 => Some("minecraft:waxed_chiseled_copper"),
		395 => Some("minecraft:waxed_copper_bars"),
		114 => Some("minecraft:waxed_copper_block"),
		1463 => Some("minecraft:waxed_copper_bulb"),
		404 => Some("minecraft:waxed_copper_chain"),
		1471 => Some("minecraft:waxed_copper_chest"),
		796 => Some("minecraft:waxed_copper_door"),
		1479 => Some("minecraft:waxed_copper_golem_statue"),
		1455 => Some("minecraft:waxed_copper_grate"),
		1352 => Some("minecraft:waxed_copper_lantern"),
		817 => Some("minecraft:waxed_copper_trapdoor"),
		122 => Some("minecraft:waxed_cut_copper"),
		130 => Some("minecraft:waxed_cut_copper_slab"),
		126 => Some("minecraft:waxed_cut_copper_stairs"),
		119 => Some("minecraft:waxed_exposed_chiseled_copper"),
		115 => Some("minecraft:waxed_exposed_copper"),
		396 => Some("minecraft:waxed_exposed_copper_bars"),
		1464 => Some("minecraft:waxed_exposed_copper_bulb"),
		405 => Some("minecraft:waxed_exposed_copper_chain"),
		1472 => Some("minecraft:waxed_exposed_copper_chest"),
		797 => Some("minecraft:waxed_exposed_copper_door"),
		1480 => Some("minecraft:waxed_exposed_copper_golem_statue"),
		1456 => Some("minecraft:waxed_exposed_copper_grate"),
		1353 => Some("minecraft:waxed_exposed_copper_lantern"),
		818 => Some("minecraft:waxed_exposed_copper_trapdoor"),
		123 => Some("minecraft:waxed_exposed_cut_copper"),
		131 => Some("minecraft:waxed_exposed_cut_copper_slab"),
		127 => Some("minecraft:waxed_exposed_cut_copper_stairs"),
		738 => Some("minecraft:waxed_exposed_lightning_rod"),
		737 => Some("minecraft:waxed_lightning_rod"),
		121 => Some("minecraft:waxed_oxidized_chiseled_copper"),
		117 => Some("minecraft:waxed_oxidized_copper"),
		398 => Some("minecraft:waxed_oxidized_copper_bars"),
		1466 => Some("minecraft:waxed_oxidized_copper_bulb"),
		407 => Some("minecraft:waxed_oxidized_copper_chain"),
		1474 => Some("minecraft:waxed_oxidized_copper_chest"),
		799 => Some("minecraft:waxed_oxidized_copper_door"),
		1482 => Some("minecraft:waxed_oxidized_copper_golem_statue"),
		1458 => Some("minecraft:waxed_oxidized_copper_grate"),
		1355 => Some("minecraft:waxed_oxidized_copper_lantern"),
		820 => Some("minecraft:waxed_oxidized_copper_trapdoor"),
		125 => Some("minecraft:waxed_oxidized_cut_copper"),
		133 => Some("minecraft:waxed_oxidized_cut_copper_slab"),
		129 => Some("minecraft:waxed_oxidized_cut_copper_stairs"),
		740 => Some("minecraft:waxed_oxidized_lightning_rod"),
		120 => Some("minecraft:waxed_weathered_chiseled_copper"),
		116 => Some("minecraft:waxed_weathered_copper"),
		397 => Some("minecraft:waxed_weathered_copper_bars"),
		1465 => Some("minecraft:waxed_weathered_copper_bulb"),
		406 => Some("minecraft:waxed_weathered_copper_chain"),
		1473 => Some("minecraft:waxed_weathered_copper_chest"),
		798 => Some("minecraft:waxed_weathered_copper_door"),
		1481 => Some("minecraft:waxed_weathered_copper_golem_statue"),
		1457 => Some("minecraft:waxed_weathered_copper_grate"),
		1354 => Some("minecraft:waxed_weathered_copper_lantern"),
		819 => Some("minecraft:waxed_weathered_copper_trapdoor"),
		124 => Some("minecraft:waxed_weathered_cut_copper"),
		132 => Some("minecraft:waxed_weathered_cut_copper_slab"),
		128 => Some("minecraft:waxed_weathered_cut_copper_stairs"),
		739 => Some("minecraft:waxed_weathered_lightning_rod"),
		1421 => Some("minecraft:wayfinder_armor_trim_smithing_template"),
		100 => Some("minecraft:weathered_chiseled_copper"),
		96 => Some("minecraft:weathered_copper"),
		393 => Some("minecraft:weathered_copper_bars"),
		1461 => Some("minecraft:weathered_copper_bulb"),
		402 => Some("minecraft:weathered_copper_chain"),
		1469 => Some("minecraft:weathered_copper_chest"),
		794 => Some("minecraft:weathered_copper_door"),
		1477 => Some("minecraft:weathered_copper_golem_statue"),
		1453 => Some("minecraft:weathered_copper_grate"),
		1350 => Some("minecraft:weathered_copper_lantern"),
		815 => Some("minecraft:weathered_copper_trapdoor"),
		104 => Some("minecraft:weathered_cut_copper"),
		112 => Some("minecraft:weathered_cut_copper_slab"),
		108 => Some("minecraft:weathered_cut_copper_stairs"),
		735 => Some("minecraft:weathered_lightning_rod"),
		254 => Some("minecraft:weeping_vines"),
		194 => Some("minecraft:wet_sponge"),
		952 => Some("minecraft:wheat"),
		951 => Some("minecraft:wheat_seeds"),
		1261 => Some("minecraft:white_banner"),
		1086 => Some("minecraft:white_bed"),
		1037 => Some("minecraft:white_bundle"),
		1382 => Some("minecraft:white_candle"),
		505 => Some("minecraft:white_carpet"),
		614 => Some("minecraft:white_concrete"),
		630 => Some("minecraft:white_concrete_powder"),
		1066 => Some("minecraft:white_dye"),
		598 => Some("minecraft:white_glazed_terracotta"),
		838 => Some("minecraft:white_harness"),
		582 => Some("minecraft:white_shulker_box"),
		530 => Some("minecraft:white_stained_glass"),
		546 => Some("minecraft:white_stained_glass_pane"),
		486 => Some("minecraft:white_terracotta"),
		238 => Some("minecraft:white_tulip"),
		213 => Some("minecraft:white_wool"),
		1413 => Some("minecraft:wild_armor_trim_smithing_template"),
		259 => Some("minecraft:wildflowers"),
		1215 => Some("minecraft:wind_charge"),
		1203 => Some("minecraft:witch_spawn_egg"),
		243 => Some("minecraft:wither_rose"),
		1230 => Some("minecraft:wither_skeleton_skull"),
		1205 => Some("minecraft:wither_skeleton_spawn_egg"),
		1204 => Some("minecraft:wither_spawn_egg"),
		890 => Some("minecraft:wolf_armor"),
		1206 => Some("minecraft:wolf_spawn_egg"),
		914 => Some("minecraft:wooden_axe"),
		915 => Some("minecraft:wooden_hoe"),
		913 => Some("minecraft:wooden_pickaxe"),
		912 => Some("minecraft:wooden_shovel"),
		911 => Some("minecraft:wooden_sword"),
		1216 => Some("minecraft:writable_book"),
		1217 => Some("minecraft:written_book"),
		1265 => Some("minecraft:yellow_banner"),
		1090 => Some("minecraft:yellow_bed"),
		1041 => Some("minecraft:yellow_bundle"),
		1386 => Some("minecraft:yellow_candle"),
		509 => Some("minecraft:yellow_carpet"),
		618 => Some("minecraft:yellow_concrete"),
		634 => Some("minecraft:yellow_concrete_powder"),
		1070 => Some("minecraft:yellow_dye"),
		602 => Some("minecraft:yellow_glazed_terracotta"),
		842 => Some("minecraft:yellow_harness"),
		586 => Some("minecraft:yellow_shulker_box"),
		534 => Some("minecraft:yellow_stained_glass"),
		550 => Some("minecraft:yellow_stained_glass_pane"),
		490 => Some("minecraft:yellow_terracotta"),
		217 => Some("minecraft:yellow_wool"),
		1207 => Some("minecraft:zoglin_spawn_egg"),
		1232 => Some("minecraft:zombie_head"),
		1210 => Some("minecraft:zombie_horse_spawn_egg"),
		1209 => Some("minecraft:zombie_spawn_egg"),
		1211 => Some("minecraft:zombie_villager_spawn_egg"),
		1212 => Some("minecraft:zombified_piglin_spawn_egg"),
    _ => None,
	};
}
pub fn get_item_id_by_name(name: &str) -> Option<i32> {
  return match name {
		"minecraft:acacia_boat" => Some(871),
		"minecraft:acacia_button" => Some(755),
		"minecraft:acacia_chest_boat" => Some(872),
		"minecraft:acacia_door" => Some(784),
		"minecraft:acacia_fence" => Some(348),
		"minecraft:acacia_fence_gate" => Some(825),
		"minecraft:acacia_hanging_sign" => Some(1004),
		"minecraft:acacia_leaves" => Some(186),
		"minecraft:acacia_log" => Some(138),
		"minecraft:acacia_planks" => Some(40),
		"minecraft:acacia_pressure_plate" => Some(771),
		"minecraft:acacia_sapling" => Some(53),
		"minecraft:acacia_shelf" => Some(305),
		"minecraft:acacia_sign" => Some(992),
		"minecraft:acacia_slab" => Some(274),
		"minecraft:acacia_stairs" => Some(445),
		"minecraft:acacia_trapdoor" => Some(805),
		"minecraft:acacia_wood" => Some(175),
		"minecraft:activator_rail" => Some(836),
		"minecraft:air" => Some(0),
		"minecraft:allay_spawn_egg" => Some(1131),
		"minecraft:allium" => Some(234),
		"minecraft:amethyst_block" => Some(88),
		"minecraft:amethyst_cluster" => Some(1401),
		"minecraft:amethyst_shard" => Some(902),
		"minecraft:ancient_debris" => Some(82),
		"minecraft:andesite" => Some(6),
		"minecraft:andesite_slab" => Some(708),
		"minecraft:andesite_stairs" => Some(691),
		"minecraft:andesite_wall" => Some(466),
		"minecraft:angler_pottery_sherd" => Some(1428),
		"minecraft:anvil" => Some(478),
		"minecraft:apple" => Some(893),
		"minecraft:archer_pottery_sherd" => Some(1429),
		"minecraft:armadillo_scute" => Some(889),
		"minecraft:armadillo_spawn_egg" => Some(1130),
		"minecraft:armor_stand" => Some(1250),
		"minecraft:arms_up_pottery_sherd" => Some(1430),
		"minecraft:arrow" => Some(895),
		"minecraft:axolotl_bucket" => Some(1023),
		"minecraft:axolotl_spawn_egg" => Some(1132),
		"minecraft:azalea" => Some(205),
		"minecraft:azalea_leaves" => Some(191),
		"minecraft:azure_bluet" => Some(235),
		"minecraft:baked_potato" => Some(1225),
		"minecraft:bamboo" => Some(269),
		"minecraft:bamboo_block" => Some(147),
		"minecraft:bamboo_button" => Some(760),
		"minecraft:bamboo_chest_raft" => Some(882),
		"minecraft:bamboo_door" => Some(789),
		"minecraft:bamboo_fence" => Some(353),
		"minecraft:bamboo_fence_gate" => Some(830),
		"minecraft:bamboo_hanging_sign" => Some(1009),
		"minecraft:bamboo_mosaic" => Some(48),
		"minecraft:bamboo_mosaic_slab" => Some(280),
		"minecraft:bamboo_mosaic_stairs" => Some(451),
		"minecraft:bamboo_planks" => Some(45),
		"minecraft:bamboo_pressure_plate" => Some(776),
		"minecraft:bamboo_raft" => Some(881),
		"minecraft:bamboo_shelf" => Some(306),
		"minecraft:bamboo_sign" => Some(997),
		"minecraft:bamboo_slab" => Some(279),
		"minecraft:bamboo_stairs" => Some(450),
		"minecraft:bamboo_trapdoor" => Some(810),
		"minecraft:barrel" => Some(1337),
		"minecraft:barrier" => Some(502),
		"minecraft:basalt" => Some(362),
		"minecraft:bat_spawn_egg" => Some(1133),
		"minecraft:beacon" => Some(455),
		"minecraft:bedrock" => Some(58),
		"minecraft:bee_nest" => Some(1362),
		"minecraft:bee_spawn_egg" => Some(1134),
		"minecraft:beef" => Some(1110),
		"minecraft:beehive" => Some(1363),
		"minecraft:beetroot" => Some(1282),
		"minecraft:beetroot_seeds" => Some(1283),
		"minecraft:beetroot_soup" => Some(1284),
		"minecraft:bell" => Some(1345),
		"minecraft:big_dripleaf" => Some(267),
		"minecraft:birch_boat" => Some(867),
		"minecraft:birch_button" => Some(753),
		"minecraft:birch_chest_boat" => Some(868),
		"minecraft:birch_door" => Some(782),
		"minecraft:birch_fence" => Some(346),
		"minecraft:birch_fence_gate" => Some(823),
		"minecraft:birch_hanging_sign" => Some(1002),
		"minecraft:birch_leaves" => Some(184),
		"minecraft:birch_log" => Some(136),
		"minecraft:birch_planks" => Some(38),
		"minecraft:birch_pressure_plate" => Some(769),
		"minecraft:birch_sapling" => Some(51),
		"minecraft:birch_shelf" => Some(307),
		"minecraft:birch_sign" => Some(990),
		"minecraft:birch_slab" => Some(272),
		"minecraft:birch_stairs" => Some(443),
		"minecraft:birch_trapdoor" => Some(803),
		"minecraft:birch_wood" => Some(173),
		"minecraft:black_banner" => Some(1276),
		"minecraft:black_bed" => Some(1101),
		"minecraft:black_bundle" => Some(1052),
		"minecraft:black_candle" => Some(1397),
		"minecraft:black_carpet" => Some(520),
		"minecraft:black_concrete" => Some(629),
		"minecraft:black_concrete_powder" => Some(645),
		"minecraft:black_dye" => Some(1081),
		"minecraft:black_glazed_terracotta" => Some(613),
		"minecraft:black_harness" => Some(853),
		"minecraft:black_shulker_box" => Some(597),
		"minecraft:black_stained_glass" => Some(545),
		"minecraft:black_stained_glass_pane" => Some(561),
		"minecraft:black_terracotta" => Some(501),
		"minecraft:black_wool" => Some(228),
		"minecraft:blackstone" => Some(1368),
		"minecraft:blackstone_slab" => Some(1369),
		"minecraft:blackstone_stairs" => Some(1370),
		"minecraft:blackstone_wall" => Some(471),
		"minecraft:blade_pottery_sherd" => Some(1431),
		"minecraft:blast_furnace" => Some(1339),
		"minecraft:blaze_powder" => Some(1124),
		"minecraft:blaze_rod" => Some(1116),
		"minecraft:blaze_spawn_egg" => Some(1135),
		"minecraft:blue_banner" => Some(1272),
		"minecraft:blue_bed" => Some(1097),
		"minecraft:blue_bundle" => Some(1048),
		"minecraft:blue_candle" => Some(1393),
		"minecraft:blue_carpet" => Some(516),
		"minecraft:blue_concrete" => Some(625),
		"minecraft:blue_concrete_powder" => Some(641),
		"minecraft:blue_dye" => Some(1077),
		"minecraft:blue_egg" => Some(1032),
		"minecraft:blue_glazed_terracotta" => Some(609),
		"minecraft:blue_harness" => Some(849),
		"minecraft:blue_ice" => Some(679),
		"minecraft:blue_orchid" => Some(233),
		"minecraft:blue_shulker_box" => Some(593),
		"minecraft:blue_stained_glass" => Some(541),
		"minecraft:blue_stained_glass_pane" => Some(557),
		"minecraft:blue_terracotta" => Some(497),
		"minecraft:blue_wool" => Some(224),
		"minecraft:bogged_spawn_egg" => Some(1136),
		"minecraft:bolt_armor_trim_smithing_template" => Some(1427),
		"minecraft:bone" => Some(1083),
		"minecraft:bone_block" => Some(579),
		"minecraft:bone_meal" => Some(1082),
		"minecraft:book" => Some(1029),
		"minecraft:bookshelf" => Some(317),
		"minecraft:bordure_indented_banner_pattern" => Some(1334),
		"minecraft:bow" => Some(894),
		"minecraft:bowl" => Some(892),
		"minecraft:brain_coral" => Some(660),
		"minecraft:brain_coral_block" => Some(655),
		"minecraft:brain_coral_fan" => Some(670),
		"minecraft:bread" => Some(953),
		"minecraft:breeze_rod" => Some(1218),
		"minecraft:breeze_spawn_egg" => Some(1137),
		"minecraft:brewer_pottery_sherd" => Some(1432),
		"minecraft:brewing_stand" => Some(1126),
		"minecraft:brick" => Some(1025),
		"minecraft:brick_slab" => Some(289),
		"minecraft:brick_stairs" => Some(419),
		"minecraft:brick_wall" => Some(458),
		"minecraft:bricks" => Some(304),
		"minecraft:brown_banner" => Some(1273),
		"minecraft:brown_bed" => Some(1098),
		"minecraft:brown_bundle" => Some(1049),
		"minecraft:brown_candle" => Some(1394),
		"minecraft:brown_carpet" => Some(517),
		"minecraft:brown_concrete" => Some(626),
		"minecraft:brown_concrete_powder" => Some(642),
		"minecraft:brown_dye" => Some(1078),
		"minecraft:brown_egg" => Some(1033),
		"minecraft:brown_glazed_terracotta" => Some(610),
		"minecraft:brown_harness" => Some(850),
		"minecraft:brown_mushroom" => Some(247),
		"minecraft:brown_mushroom_block" => Some(387),
		"minecraft:brown_shulker_box" => Some(594),
		"minecraft:brown_stained_glass" => Some(542),
		"minecraft:brown_stained_glass_pane" => Some(558),
		"minecraft:brown_terracotta" => Some(498),
		"minecraft:brown_wool" => Some(225),
		"minecraft:brush" => Some(1408),
		"minecraft:bubble_coral" => Some(661),
		"minecraft:bubble_coral_block" => Some(656),
		"minecraft:bubble_coral_fan" => Some(671),
		"minecraft:bucket" => Some(1012),
		"minecraft:budding_amethyst" => Some(89),
		"minecraft:bundle" => Some(1036),
		"minecraft:burn_pottery_sherd" => Some(1433),
		"minecraft:bush" => Some(204),
		"minecraft:cactus" => Some(340),
		"minecraft:cactus_flower" => Some(341),
		"minecraft:cake" => Some(1085),
		"minecraft:calcite" => Some(11),
		"minecraft:calibrated_sculk_sensor" => Some(743),
		"minecraft:camel_spawn_egg" => Some(1139),
		"minecraft:campfire" => Some(1358),
		"minecraft:candle" => Some(1381),
		"minecraft:carrot" => Some(1223),
		"minecraft:carrot_on_a_stick" => Some(859),
		"minecraft:cartography_table" => Some(1340),
		"minecraft:carved_pumpkin" => Some(357),
		"minecraft:cat_spawn_egg" => Some(1138),
		"minecraft:cauldron" => Some(1127),
		"minecraft:cave_spider_spawn_egg" => Some(1140),
		"minecraft:chain_command_block" => Some(574),
		"minecraft:chainmail_boots" => Some(965),
		"minecraft:chainmail_chestplate" => Some(963),
		"minecraft:chainmail_helmet" => Some(962),
		"minecraft:chainmail_leggings" => Some(964),
		"minecraft:charcoal" => Some(897),
		"minecraft:cherry_boat" => Some(873),
		"minecraft:cherry_button" => Some(756),
		"minecraft:cherry_chest_boat" => Some(874),
		"minecraft:cherry_door" => Some(785),
		"minecraft:cherry_fence" => Some(349),
		"minecraft:cherry_fence_gate" => Some(826),
		"minecraft:cherry_hanging_sign" => Some(1005),
		"minecraft:cherry_leaves" => Some(187),
		"minecraft:cherry_log" => Some(139),
		"minecraft:cherry_planks" => Some(41),
		"minecraft:cherry_pressure_plate" => Some(772),
		"minecraft:cherry_sapling" => Some(54),
		"minecraft:cherry_shelf" => Some(308),
		"minecraft:cherry_sign" => Some(993),
		"minecraft:cherry_slab" => Some(275),
		"minecraft:cherry_stairs" => Some(446),
		"minecraft:cherry_trapdoor" => Some(806),
		"minecraft:cherry_wood" => Some(176),
		"minecraft:chest" => Some(331),
		"minecraft:chest_minecart" => Some(855),
		"minecraft:chicken" => Some(1112),
		"minecraft:chicken_spawn_egg" => Some(1141),
		"minecraft:chipped_anvil" => Some(479),
		"minecraft:chiseled_bookshelf" => Some(318),
		"minecraft:chiseled_copper" => Some(98),
		"minecraft:chiseled_deepslate" => Some(385),
		"minecraft:chiseled_nether_bricks" => Some(426),
		"minecraft:chiseled_polished_blackstone" => Some(1375),
		"minecraft:chiseled_quartz_block" => Some(481),
		"minecraft:chiseled_red_sandstone" => Some(570),
		"minecraft:chiseled_resin_bricks" => Some(418),
		"minecraft:chiseled_sandstone" => Some(199),
		"minecraft:chiseled_stone_bricks" => Some(378),
		"minecraft:chiseled_tuff" => Some(16),
		"minecraft:chiseled_tuff_bricks" => Some(25),
		"minecraft:chorus_flower" => Some(325),
		"minecraft:chorus_fruit" => Some(1278),
		"minecraft:chorus_plant" => Some(324),
		"minecraft:clay" => Some(342),
		"minecraft:clay_ball" => Some(1026),
		"minecraft:clock" => Some(1054),
		"minecraft:closed_eyeblossom" => Some(231),
		"minecraft:coal" => Some(896),
		"minecraft:coal_block" => Some(83),
		"minecraft:coal_ore" => Some(64),
		"minecraft:coarse_dirt" => Some(29),
		"minecraft:coast_armor_trim_smithing_template" => Some(1412),
		"minecraft:cobbled_deepslate" => Some(9),
		"minecraft:cobbled_deepslate_slab" => Some(712),
		"minecraft:cobbled_deepslate_stairs" => Some(695),
		"minecraft:cobbled_deepslate_wall" => Some(474),
		"minecraft:cobblestone" => Some(35),
		"minecraft:cobblestone_slab" => Some(288),
		"minecraft:cobblestone_stairs" => Some(336),
		"minecraft:cobblestone_wall" => Some(456),
		"minecraft:cobweb" => Some(201),
		"minecraft:cocoa_beans" => Some(1065),
		"minecraft:cod" => Some(1057),
		"minecraft:cod_bucket" => Some(1021),
		"minecraft:cod_spawn_egg" => Some(1142),
		"minecraft:command_block" => Some(454),
		"minecraft:command_block_minecart" => Some(1258),
		"minecraft:comparator" => Some(721),
		"minecraft:compass" => Some(1034),
		"minecraft:composter" => Some(1336),
		"minecraft:conduit" => Some(680),
		"minecraft:cooked_beef" => Some(1111),
		"minecraft:cooked_chicken" => Some(1113),
		"minecraft:cooked_cod" => Some(1061),
		"minecraft:cooked_mutton" => Some(1260),
		"minecraft:cooked_porkchop" => Some(984),
		"minecraft:cooked_rabbit" => Some(1246),
		"minecraft:cooked_salmon" => Some(1062),
		"minecraft:cookie" => Some(1102),
		"minecraft:copper_axe" => Some(919),
		"minecraft:copper_bars" => Some(391),
		"minecraft:copper_block" => Some(91),
		"minecraft:copper_boots" => Some(961),
		"minecraft:copper_bulb" => Some(1459),
		"minecraft:copper_chain" => Some(400),
		"minecraft:copper_chest" => Some(1467),
		"minecraft:copper_chestplate" => Some(959),
		"minecraft:copper_door" => Some(792),
		"minecraft:copper_golem_spawn_egg" => Some(1143),
		"minecraft:copper_golem_statue" => Some(1475),
		"minecraft:copper_grate" => Some(1451),
		"minecraft:copper_helmet" => Some(958),
		"minecraft:copper_hoe" => Some(920),
		"minecraft:copper_horse_armor" => Some(1251),
		"minecraft:copper_ingot" => Some(906),
		"minecraft:copper_lantern" => Some(1348),
		"minecraft:copper_leggings" => Some(960),
		"minecraft:copper_nugget" => Some(1294),
		"minecraft:copper_ore" => Some(68),
		"minecraft:copper_pickaxe" => Some(918),
		"minecraft:copper_shovel" => Some(917),
		"minecraft:copper_sword" => Some(916),
		"minecraft:copper_torch" => Some(366),
		"minecraft:copper_trapdoor" => Some(813),
		"minecraft:cornflower" => Some(241),
		"minecraft:cow_spawn_egg" => Some(1144),
		"minecraft:cracked_deepslate_bricks" => Some(382),
		"minecraft:cracked_deepslate_tiles" => Some(384),
		"minecraft:cracked_nether_bricks" => Some(425),
		"minecraft:cracked_polished_blackstone_bricks" => Some(1379),
		"minecraft:cracked_stone_bricks" => Some(377),
		"minecraft:crafter" => Some(1103),
		"minecraft:crafting_table" => Some(332),
		"minecraft:creaking_heart" => Some(330),
		"minecraft:creaking_spawn_egg" => Some(1208),
		"minecraft:creeper_banner_pattern" => Some(1326),
		"minecraft:creeper_head" => Some(1233),
		"minecraft:creeper_spawn_egg" => Some(1145),
		"minecraft:crimson_button" => Some(761),
		"minecraft:crimson_door" => Some(790),
		"minecraft:crimson_fence" => Some(354),
		"minecraft:crimson_fence_gate" => Some(831),
		"minecraft:crimson_fungus" => Some(249),
		"minecraft:crimson_hanging_sign" => Some(1010),
		"minecraft:crimson_hyphae" => Some(180),
		"minecraft:crimson_nylium" => Some(33),
		"minecraft:crimson_planks" => Some(46),
		"minecraft:crimson_pressure_plate" => Some(777),
		"minecraft:crimson_roots" => Some(251),
		"minecraft:crimson_shelf" => Some(309),
		"minecraft:crimson_sign" => Some(998),
		"minecraft:crimson_slab" => Some(281),
		"minecraft:crimson_stairs" => Some(452),
		"minecraft:crimson_stem" => Some(145),
		"minecraft:crimson_trapdoor" => Some(811),
		"minecraft:crossbow" => Some(1322),
		"minecraft:crying_obsidian" => Some(1367),
		"minecraft:cut_copper" => Some(102),
		"minecraft:cut_copper_slab" => Some(110),
		"minecraft:cut_copper_stairs" => Some(106),
		"minecraft:cut_red_sandstone" => Some(571),
		"minecraft:cut_red_sandstone_slab" => Some(295),
		"minecraft:cut_sandstone" => Some(200),
		"minecraft:cut_sandstone_slab" => Some(286),
		"minecraft:cyan_banner" => Some(1270),
		"minecraft:cyan_bed" => Some(1095),
		"minecraft:cyan_bundle" => Some(1046),
		"minecraft:cyan_candle" => Some(1391),
		"minecraft:cyan_carpet" => Some(514),
		"minecraft:cyan_concrete" => Some(623),
		"minecraft:cyan_concrete_powder" => Some(639),
		"minecraft:cyan_dye" => Some(1075),
		"minecraft:cyan_glazed_terracotta" => Some(607),
		"minecraft:cyan_harness" => Some(847),
		"minecraft:cyan_shulker_box" => Some(591),
		"minecraft:cyan_stained_glass" => Some(539),
		"minecraft:cyan_stained_glass_pane" => Some(555),
		"minecraft:cyan_terracotta" => Some(495),
		"minecraft:cyan_wool" => Some(222),
		"minecraft:damaged_anvil" => Some(480),
		"minecraft:dandelion" => Some(229),
		"minecraft:danger_pottery_sherd" => Some(1434),
		"minecraft:dark_oak_boat" => Some(875),
		"minecraft:dark_oak_button" => Some(757),
		"minecraft:dark_oak_chest_boat" => Some(876),
		"minecraft:dark_oak_door" => Some(786),
		"minecraft:dark_oak_fence" => Some(350),
		"minecraft:dark_oak_fence_gate" => Some(827),
		"minecraft:dark_oak_hanging_sign" => Some(1006),
		"minecraft:dark_oak_leaves" => Some(188),
		"minecraft:dark_oak_log" => Some(141),
		"minecraft:dark_oak_planks" => Some(42),
		"minecraft:dark_oak_pressure_plate" => Some(773),
		"minecraft:dark_oak_sapling" => Some(55),
		"minecraft:dark_oak_shelf" => Some(310),
		"minecraft:dark_oak_sign" => Some(994),
		"minecraft:dark_oak_slab" => Some(276),
		"minecraft:dark_oak_stairs" => Some(447),
		"minecraft:dark_oak_trapdoor" => Some(807),
		"minecraft:dark_oak_wood" => Some(178),
		"minecraft:dark_prismarine" => Some(564),
		"minecraft:dark_prismarine_slab" => Some(299),
		"minecraft:dark_prismarine_stairs" => Some(567),
		"minecraft:daylight_detector" => Some(741),
		"minecraft:dead_brain_coral" => Some(664),
		"minecraft:dead_brain_coral_block" => Some(650),
		"minecraft:dead_brain_coral_fan" => Some(675),
		"minecraft:dead_bubble_coral" => Some(665),
		"minecraft:dead_bubble_coral_block" => Some(651),
		"minecraft:dead_bubble_coral_fan" => Some(676),
		"minecraft:dead_bush" => Some(207),
		"minecraft:dead_fire_coral" => Some(666),
		"minecraft:dead_fire_coral_block" => Some(652),
		"minecraft:dead_fire_coral_fan" => Some(677),
		"minecraft:dead_horn_coral" => Some(667),
		"minecraft:dead_horn_coral_block" => Some(653),
		"minecraft:dead_horn_coral_fan" => Some(678),
		"minecraft:dead_tube_coral" => Some(668),
		"minecraft:dead_tube_coral_block" => Some(649),
		"minecraft:dead_tube_coral_fan" => Some(674),
		"minecraft:debug_stick" => Some(1296),
		"minecraft:decorated_pot" => Some(319),
		"minecraft:deepslate" => Some(8),
		"minecraft:deepslate_brick_slab" => Some(714),
		"minecraft:deepslate_brick_stairs" => Some(697),
		"minecraft:deepslate_brick_wall" => Some(476),
		"minecraft:deepslate_bricks" => Some(381),
		"minecraft:deepslate_coal_ore" => Some(65),
		"minecraft:deepslate_copper_ore" => Some(69),
		"minecraft:deepslate_diamond_ore" => Some(79),
		"minecraft:deepslate_emerald_ore" => Some(75),
		"minecraft:deepslate_gold_ore" => Some(71),
		"minecraft:deepslate_iron_ore" => Some(67),
		"minecraft:deepslate_lapis_ore" => Some(77),
		"minecraft:deepslate_redstone_ore" => Some(73),
		"minecraft:deepslate_tile_slab" => Some(715),
		"minecraft:deepslate_tile_stairs" => Some(698),
		"minecraft:deepslate_tile_wall" => Some(477),
		"minecraft:deepslate_tiles" => Some(383),
		"minecraft:detector_rail" => Some(834),
		"minecraft:diamond" => Some(898),
		"minecraft:diamond_axe" => Some(939),
		"minecraft:diamond_block" => Some(93),
		"minecraft:diamond_boots" => Some(973),
		"minecraft:diamond_chestplate" => Some(971),
		"minecraft:diamond_helmet" => Some(970),
		"minecraft:diamond_hoe" => Some(940),
		"minecraft:diamond_horse_armor" => Some(1254),
		"minecraft:diamond_leggings" => Some(972),
		"minecraft:diamond_ore" => Some(78),
		"minecraft:diamond_pickaxe" => Some(938),
		"minecraft:diamond_shovel" => Some(937),
		"minecraft:diamond_sword" => Some(936),
		"minecraft:diorite" => Some(4),
		"minecraft:diorite_slab" => Some(711),
		"minecraft:diorite_stairs" => Some(694),
		"minecraft:diorite_wall" => Some(470),
		"minecraft:dirt" => Some(28),
		"minecraft:dirt_path" => Some(523),
		"minecraft:disc_fragment_5" => Some(1318),
		"minecraft:dispenser" => Some(728),
		"minecraft:dolphin_spawn_egg" => Some(1146),
		"minecraft:donkey_spawn_egg" => Some(1147),
		"minecraft:dragon_breath" => Some(1285),
		"minecraft:dragon_egg" => Some(437),
		"minecraft:dragon_head" => Some(1234),
		"minecraft:dried_ghast" => Some(648),
		"minecraft:dried_kelp" => Some(1107),
		"minecraft:dried_kelp_block" => Some(1027),
		"minecraft:dripstone_block" => Some(26),
		"minecraft:dropper" => Some(729),
		"minecraft:drowned_spawn_egg" => Some(1148),
		"minecraft:dune_armor_trim_smithing_template" => Some(1411),
		"minecraft:echo_shard" => Some(1407),
		"minecraft:egg" => Some(1031),
		"minecraft:elder_guardian_spawn_egg" => Some(1149),
		"minecraft:elytra" => Some(862),
		"minecraft:emerald" => Some(899),
		"minecraft:emerald_block" => Some(440),
		"minecraft:emerald_ore" => Some(74),
		"minecraft:enchanted_book" => Some(1240),
		"minecraft:enchanted_golden_apple" => Some(987),
		"minecraft:enchanting_table" => Some(433),
		"minecraft:end_crystal" => Some(1277),
		"minecraft:end_portal_frame" => Some(434),
		"minecraft:end_rod" => Some(323),
		"minecraft:end_stone" => Some(435),
		"minecraft:end_stone_brick_slab" => Some(704),
		"minecraft:end_stone_brick_stairs" => Some(686),
		"minecraft:end_stone_brick_wall" => Some(469),
		"minecraft:end_stone_bricks" => Some(436),
		"minecraft:ender_chest" => Some(439),
		"minecraft:ender_dragon_spawn_egg" => Some(1150),
		"minecraft:ender_eye" => Some(1128),
		"minecraft:ender_pearl" => Some(1115),
		"minecraft:enderman_spawn_egg" => Some(1151),
		"minecraft:endermite_spawn_egg" => Some(1152),
		"minecraft:evoker_spawn_egg" => Some(1153),
		"minecraft:experience_bottle" => Some(1213),
		"minecraft:explorer_pottery_sherd" => Some(1435),
		"minecraft:exposed_chiseled_copper" => Some(99),
		"minecraft:exposed_copper" => Some(95),
		"minecraft:exposed_copper_bars" => Some(392),
		"minecraft:exposed_copper_bulb" => Some(1460),
		"minecraft:exposed_copper_chain" => Some(401),
		"minecraft:exposed_copper_chest" => Some(1468),
		"minecraft:exposed_copper_door" => Some(793),
		"minecraft:exposed_copper_golem_statue" => Some(1476),
		"minecraft:exposed_copper_grate" => Some(1452),
		"minecraft:exposed_copper_lantern" => Some(1349),
		"minecraft:exposed_copper_trapdoor" => Some(814),
		"minecraft:exposed_cut_copper" => Some(103),
		"minecraft:exposed_cut_copper_slab" => Some(111),
		"minecraft:exposed_cut_copper_stairs" => Some(107),
		"minecraft:exposed_lightning_rod" => Some(734),
		"minecraft:eye_armor_trim_smithing_template" => Some(1415),
		"minecraft:farmland" => Some(333),
		"minecraft:feather" => Some(949),
		"minecraft:fermented_spider_eye" => Some(1123),
		"minecraft:fern" => Some(203),
		"minecraft:field_masoned_banner_pattern" => Some(1333),
		"minecraft:filled_map" => Some(1104),
		"minecraft:fire_charge" => Some(1214),
		"minecraft:fire_coral" => Some(662),
		"minecraft:fire_coral_block" => Some(657),
		"minecraft:fire_coral_fan" => Some(672),
		"minecraft:firefly_bush" => Some(208),
		"minecraft:firework_rocket" => Some(1238),
		"minecraft:firework_star" => Some(1239),
		"minecraft:fishing_rod" => Some(1053),
		"minecraft:fletching_table" => Some(1341),
		"minecraft:flint" => Some(982),
		"minecraft:flint_and_steel" => Some(891),
		"minecraft:flow_armor_trim_smithing_template" => Some(1426),
		"minecraft:flow_banner_pattern" => Some(1331),
		"minecraft:flow_pottery_sherd" => Some(1436),
		"minecraft:flower_banner_pattern" => Some(1325),
		"minecraft:flower_pot" => Some(1222),
		"minecraft:flowering_azalea" => Some(206),
		"minecraft:flowering_azalea_leaves" => Some(192),
		"minecraft:fox_spawn_egg" => Some(1154),
		"minecraft:friend_pottery_sherd" => Some(1437),
		"minecraft:frog_spawn_egg" => Some(1155),
		"minecraft:frogspawn" => Some(1406),
		"minecraft:furnace" => Some(334),
		"minecraft:furnace_minecart" => Some(856),
		"minecraft:ghast_spawn_egg" => Some(1156),
		"minecraft:ghast_tear" => Some(1117),
		"minecraft:gilded_blackstone" => Some(1371),
		"minecraft:glass" => Some(195),
		"minecraft:glass_bottle" => Some(1120),
		"minecraft:glass_pane" => Some(408),
		"minecraft:glistering_melon_slice" => Some(1129),
		"minecraft:globe_banner_pattern" => Some(1329),
		"minecraft:glow_berries" => Some(1357),
		"minecraft:glow_ink_sac" => Some(1064),
		"minecraft:glow_item_frame" => Some(1221),
		"minecraft:glow_lichen" => Some(411),
		"minecraft:glow_squid_spawn_egg" => Some(1158),
		"minecraft:glowstone" => Some(367),
		"minecraft:glowstone_dust" => Some(1056),
		"minecraft:goat_horn" => Some(1335),
		"minecraft:goat_spawn_egg" => Some(1159),
		"minecraft:gold_block" => Some(92),
		"minecraft:gold_ingot" => Some(908),
		"minecraft:gold_nugget" => Some(1118),
		"minecraft:gold_ore" => Some(70),
		"minecraft:golden_apple" => Some(986),
		"minecraft:golden_axe" => Some(929),
		"minecraft:golden_boots" => Some(977),
		"minecraft:golden_carrot" => Some(1228),
		"minecraft:golden_chestplate" => Some(975),
		"minecraft:golden_helmet" => Some(974),
		"minecraft:golden_hoe" => Some(930),
		"minecraft:golden_horse_armor" => Some(1253),
		"minecraft:golden_leggings" => Some(976),
		"minecraft:golden_pickaxe" => Some(928),
		"minecraft:golden_shovel" => Some(927),
		"minecraft:golden_sword" => Some(926),
		"minecraft:granite" => Some(2),
		"minecraft:granite_slab" => Some(707),
		"minecraft:granite_stairs" => Some(690),
		"minecraft:granite_wall" => Some(462),
		"minecraft:grass_block" => Some(27),
		"minecraft:gravel" => Some(63),
		"minecraft:gray_banner" => Some(1268),
		"minecraft:gray_bed" => Some(1093),
		"minecraft:gray_bundle" => Some(1044),
		"minecraft:gray_candle" => Some(1389),
		"minecraft:gray_carpet" => Some(512),
		"minecraft:gray_concrete" => Some(621),
		"minecraft:gray_concrete_powder" => Some(637),
		"minecraft:gray_dye" => Some(1073),
		"minecraft:gray_glazed_terracotta" => Some(605),
		"minecraft:gray_harness" => Some(845),
		"minecraft:gray_shulker_box" => Some(589),
		"minecraft:gray_stained_glass" => Some(537),
		"minecraft:gray_stained_glass_pane" => Some(553),
		"minecraft:gray_terracotta" => Some(493),
		"minecraft:gray_wool" => Some(220),
		"minecraft:green_banner" => Some(1274),
		"minecraft:green_bed" => Some(1099),
		"minecraft:green_bundle" => Some(1050),
		"minecraft:green_candle" => Some(1395),
		"minecraft:green_carpet" => Some(518),
		"minecraft:green_concrete" => Some(627),
		"minecraft:green_concrete_powder" => Some(643),
		"minecraft:green_dye" => Some(1079),
		"minecraft:green_glazed_terracotta" => Some(611),
		"minecraft:green_harness" => Some(851),
		"minecraft:green_shulker_box" => Some(595),
		"minecraft:green_stained_glass" => Some(543),
		"minecraft:green_stained_glass_pane" => Some(559),
		"minecraft:green_terracotta" => Some(499),
		"minecraft:green_wool" => Some(226),
		"minecraft:grindstone" => Some(1342),
		"minecraft:guardian_spawn_egg" => Some(1160),
		"minecraft:gunpowder" => Some(950),
		"minecraft:guster_banner_pattern" => Some(1332),
		"minecraft:guster_pottery_sherd" => Some(1438),
		"minecraft:hanging_roots" => Some(266),
		"minecraft:happy_ghast_spawn_egg" => Some(1157),
		"minecraft:hay_block" => Some(504),
		"minecraft:heart_of_the_sea" => Some(1321),
		"minecraft:heart_pottery_sherd" => Some(1439),
		"minecraft:heartbreak_pottery_sherd" => Some(1440),
		"minecraft:heavy_core" => Some(87),
		"minecraft:heavy_weighted_pressure_plate" => Some(766),
		"minecraft:hoglin_spawn_egg" => Some(1161),
		"minecraft:honey_block" => Some(725),
		"minecraft:honey_bottle" => Some(1364),
		"minecraft:honeycomb" => Some(1361),
		"minecraft:honeycomb_block" => Some(1365),
		"minecraft:hopper" => Some(727),
		"minecraft:hopper_minecart" => Some(858),
		"minecraft:horn_coral" => Some(663),
		"minecraft:horn_coral_block" => Some(658),
		"minecraft:horn_coral_fan" => Some(673),
		"minecraft:horse_spawn_egg" => Some(1162),
		"minecraft:host_armor_trim_smithing_template" => Some(1425),
		"minecraft:howl_pottery_sherd" => Some(1441),
		"minecraft:husk_spawn_egg" => Some(1163),
		"minecraft:ice" => Some(338),
		"minecraft:infested_chiseled_stone_bricks" => Some(373),
		"minecraft:infested_cobblestone" => Some(369),
		"minecraft:infested_cracked_stone_bricks" => Some(372),
		"minecraft:infested_deepslate" => Some(374),
		"minecraft:infested_mossy_stone_bricks" => Some(371),
		"minecraft:infested_stone" => Some(368),
		"minecraft:infested_stone_bricks" => Some(370),
		"minecraft:ink_sac" => Some(1063),
		"minecraft:iron_axe" => Some(934),
		"minecraft:iron_bars" => Some(390),
		"minecraft:iron_block" => Some(90),
		"minecraft:iron_boots" => Some(969),
		"minecraft:iron_chain" => Some(399),
		"minecraft:iron_chestplate" => Some(967),
		"minecraft:iron_door" => Some(779),
		"minecraft:iron_golem_spawn_egg" => Some(1164),
		"minecraft:iron_helmet" => Some(966),
		"minecraft:iron_hoe" => Some(935),
		"minecraft:iron_horse_armor" => Some(1252),
		"minecraft:iron_ingot" => Some(904),
		"minecraft:iron_leggings" => Some(968),
		"minecraft:iron_nugget" => Some(1293),
		"minecraft:iron_ore" => Some(66),
		"minecraft:iron_pickaxe" => Some(933),
		"minecraft:iron_shovel" => Some(932),
		"minecraft:iron_sword" => Some(931),
		"minecraft:iron_trapdoor" => Some(800),
		"minecraft:item_frame" => Some(1220),
		"minecraft:jack_o_lantern" => Some(358),
		"minecraft:jigsaw" => Some(884),
		"minecraft:jukebox" => Some(343),
		"minecraft:jungle_boat" => Some(869),
		"minecraft:jungle_button" => Some(754),
		"minecraft:jungle_chest_boat" => Some(870),
		"minecraft:jungle_door" => Some(783),
		"minecraft:jungle_fence" => Some(347),
		"minecraft:jungle_fence_gate" => Some(824),
		"minecraft:jungle_hanging_sign" => Some(1003),
		"minecraft:jungle_leaves" => Some(185),
		"minecraft:jungle_log" => Some(137),
		"minecraft:jungle_planks" => Some(39),
		"minecraft:jungle_pressure_plate" => Some(770),
		"minecraft:jungle_sapling" => Some(52),
		"minecraft:jungle_shelf" => Some(311),
		"minecraft:jungle_sign" => Some(991),
		"minecraft:jungle_slab" => Some(273),
		"minecraft:jungle_stairs" => Some(444),
		"minecraft:jungle_trapdoor" => Some(804),
		"minecraft:jungle_wood" => Some(174),
		"minecraft:kelp" => Some(257),
		"minecraft:knowledge_book" => Some(1295),
		"minecraft:ladder" => Some(335),
		"minecraft:lantern" => Some(1346),
		"minecraft:lapis_block" => Some(197),
		"minecraft:lapis_lazuli" => Some(900),
		"minecraft:lapis_ore" => Some(76),
		"minecraft:large_amethyst_bud" => Some(1400),
		"minecraft:large_fern" => Some(529),
		"minecraft:lava_bucket" => Some(1014),
		"minecraft:lead" => Some(1256),
		"minecraft:leaf_litter" => Some(260),
		"minecraft:leather" => Some(1017),
		"minecraft:leather_boots" => Some(957),
		"minecraft:leather_chestplate" => Some(955),
		"minecraft:leather_helmet" => Some(954),
		"minecraft:leather_horse_armor" => Some(1255),
		"minecraft:leather_leggings" => Some(956),
		"minecraft:lectern" => Some(730),
		"minecraft:lever" => Some(732),
		"minecraft:light" => Some(503),
		"minecraft:light_blue_banner" => Some(1264),
		"minecraft:light_blue_bed" => Some(1089),
		"minecraft:light_blue_bundle" => Some(1040),
		"minecraft:light_blue_candle" => Some(1385),
		"minecraft:light_blue_carpet" => Some(508),
		"minecraft:light_blue_concrete" => Some(617),
		"minecraft:light_blue_concrete_powder" => Some(633),
		"minecraft:light_blue_dye" => Some(1069),
		"minecraft:light_blue_glazed_terracotta" => Some(601),
		"minecraft:light_blue_harness" => Some(841),
		"minecraft:light_blue_shulker_box" => Some(585),
		"minecraft:light_blue_stained_glass" => Some(533),
		"minecraft:light_blue_stained_glass_pane" => Some(549),
		"minecraft:light_blue_terracotta" => Some(489),
		"minecraft:light_blue_wool" => Some(216),
		"minecraft:light_gray_banner" => Some(1269),
		"minecraft:light_gray_bed" => Some(1094),
		"minecraft:light_gray_bundle" => Some(1045),
		"minecraft:light_gray_candle" => Some(1390),
		"minecraft:light_gray_carpet" => Some(513),
		"minecraft:light_gray_concrete" => Some(622),
		"minecraft:light_gray_concrete_powder" => Some(638),
		"minecraft:light_gray_dye" => Some(1074),
		"minecraft:light_gray_glazed_terracotta" => Some(606),
		"minecraft:light_gray_harness" => Some(846),
		"minecraft:light_gray_shulker_box" => Some(590),
		"minecraft:light_gray_stained_glass" => Some(538),
		"minecraft:light_gray_stained_glass_pane" => Some(554),
		"minecraft:light_gray_terracotta" => Some(494),
		"minecraft:light_gray_wool" => Some(221),
		"minecraft:light_weighted_pressure_plate" => Some(765),
		"minecraft:lightning_rod" => Some(733),
		"minecraft:lilac" => Some(525),
		"minecraft:lily_of_the_valley" => Some(242),
		"minecraft:lily_pad" => Some(423),
		"minecraft:lime_banner" => Some(1266),
		"minecraft:lime_bed" => Some(1091),
		"minecraft:lime_bundle" => Some(1042),
		"minecraft:lime_candle" => Some(1387),
		"minecraft:lime_carpet" => Some(510),
		"minecraft:lime_concrete" => Some(619),
		"minecraft:lime_concrete_powder" => Some(635),
		"minecraft:lime_dye" => Some(1071),
		"minecraft:lime_glazed_terracotta" => Some(603),
		"minecraft:lime_harness" => Some(843),
		"minecraft:lime_shulker_box" => Some(587),
		"minecraft:lime_stained_glass" => Some(535),
		"minecraft:lime_stained_glass_pane" => Some(551),
		"minecraft:lime_terracotta" => Some(491),
		"minecraft:lime_wool" => Some(218),
		"minecraft:lingering_potion" => Some(1289),
		"minecraft:llama_spawn_egg" => Some(1165),
		"minecraft:lodestone" => Some(1366),
		"minecraft:loom" => Some(1324),
		"minecraft:mace" => Some(1219),
		"minecraft:magenta_banner" => Some(1263),
		"minecraft:magenta_bed" => Some(1088),
		"minecraft:magenta_bundle" => Some(1039),
		"minecraft:magenta_candle" => Some(1384),
		"minecraft:magenta_carpet" => Some(507),
		"minecraft:magenta_concrete" => Some(616),
		"minecraft:magenta_concrete_powder" => Some(632),
		"minecraft:magenta_dye" => Some(1068),
		"minecraft:magenta_glazed_terracotta" => Some(600),
		"minecraft:magenta_harness" => Some(840),
		"minecraft:magenta_shulker_box" => Some(584),
		"minecraft:magenta_stained_glass" => Some(532),
		"minecraft:magenta_stained_glass_pane" => Some(548),
		"minecraft:magenta_terracotta" => Some(488),
		"minecraft:magenta_wool" => Some(215),
		"minecraft:magma_block" => Some(575),
		"minecraft:magma_cream" => Some(1125),
		"minecraft:magma_cube_spawn_egg" => Some(1166),
		"minecraft:mangrove_boat" => Some(879),
		"minecraft:mangrove_button" => Some(759),
		"minecraft:mangrove_chest_boat" => Some(880),
		"minecraft:mangrove_door" => Some(788),
		"minecraft:mangrove_fence" => Some(352),
		"minecraft:mangrove_fence_gate" => Some(829),
		"minecraft:mangrove_hanging_sign" => Some(1008),
		"minecraft:mangrove_leaves" => Some(190),
		"minecraft:mangrove_log" => Some(142),
		"minecraft:mangrove_planks" => Some(44),
		"minecraft:mangrove_pressure_plate" => Some(775),
		"minecraft:mangrove_propagule" => Some(57),
		"minecraft:mangrove_roots" => Some(143),
		"minecraft:mangrove_shelf" => Some(312),
		"minecraft:mangrove_sign" => Some(996),
		"minecraft:mangrove_slab" => Some(278),
		"minecraft:mangrove_stairs" => Some(449),
		"minecraft:mangrove_trapdoor" => Some(809),
		"minecraft:mangrove_wood" => Some(179),
		"minecraft:map" => Some(1227),
		"minecraft:medium_amethyst_bud" => Some(1399),
		"minecraft:melon" => Some(409),
		"minecraft:melon_seeds" => Some(1109),
		"minecraft:melon_slice" => Some(1106),
		"minecraft:milk_bucket" => Some(1018),
		"minecraft:minecart" => Some(854),
		"minecraft:miner_pottery_sherd" => Some(1442),
		"minecraft:mojang_banner_pattern" => Some(1328),
		"minecraft:mooshroom_spawn_egg" => Some(1167),
		"minecraft:moss_block" => Some(262),
		"minecraft:moss_carpet" => Some(261),
		"minecraft:mossy_cobblestone" => Some(320),
		"minecraft:mossy_cobblestone_slab" => Some(703),
		"minecraft:mossy_cobblestone_stairs" => Some(685),
		"minecraft:mossy_cobblestone_wall" => Some(457),
		"minecraft:mossy_stone_brick_slab" => Some(701),
		"minecraft:mossy_stone_brick_stairs" => Some(683),
		"minecraft:mossy_stone_brick_wall" => Some(461),
		"minecraft:mossy_stone_bricks" => Some(376),
		"minecraft:mourner_pottery_sherd" => Some(1443),
		"minecraft:mud" => Some(32),
		"minecraft:mud_brick_slab" => Some(291),
		"minecraft:mud_brick_stairs" => Some(421),
		"minecraft:mud_brick_wall" => Some(464),
		"minecraft:mud_bricks" => Some(380),
		"minecraft:muddy_mangrove_roots" => Some(144),
		"minecraft:mule_spawn_egg" => Some(1168),
		"minecraft:mushroom_stem" => Some(389),
		"minecraft:mushroom_stew" => Some(947),
		"minecraft:music_disc_11" => Some(1310),
		"minecraft:music_disc_13" => Some(1297),
		"minecraft:music_disc_5" => Some(1314),
		"minecraft:music_disc_blocks" => Some(1299),
		"minecraft:music_disc_cat" => Some(1298),
		"minecraft:music_disc_chirp" => Some(1300),
		"minecraft:music_disc_creator" => Some(1301),
		"minecraft:music_disc_creator_music_box" => Some(1302),
		"minecraft:music_disc_far" => Some(1303),
		"minecraft:music_disc_lava_chicken" => Some(1304),
		"minecraft:music_disc_mall" => Some(1305),
		"minecraft:music_disc_mellohi" => Some(1306),
		"minecraft:music_disc_otherside" => Some(1312),
		"minecraft:music_disc_pigstep" => Some(1315),
		"minecraft:music_disc_precipice" => Some(1316),
		"minecraft:music_disc_relic" => Some(1313),
		"minecraft:music_disc_stal" => Some(1307),
		"minecraft:music_disc_strad" => Some(1308),
		"minecraft:music_disc_tears" => Some(1317),
		"minecraft:music_disc_wait" => Some(1311),
		"minecraft:music_disc_ward" => Some(1309),
		"minecraft:mutton" => Some(1259),
		"minecraft:mycelium" => Some(422),
		"minecraft:name_tag" => Some(1257),
		"minecraft:nautilus_shell" => Some(1320),
		"minecraft:nether_brick" => Some(1241),
		"minecraft:nether_brick_fence" => Some(427),
		"minecraft:nether_brick_slab" => Some(292),
		"minecraft:nether_brick_stairs" => Some(428),
		"minecraft:nether_brick_wall" => Some(465),
		"minecraft:nether_bricks" => Some(424),
		"minecraft:nether_gold_ore" => Some(80),
		"minecraft:nether_quartz_ore" => Some(81),
		"minecraft:nether_sprouts" => Some(253),
		"minecraft:nether_star" => Some(1236),
		"minecraft:nether_wart" => Some(1119),
		"minecraft:nether_wart_block" => Some(576),
		"minecraft:netherite_axe" => Some(944),
		"minecraft:netherite_block" => Some(94),
		"minecraft:netherite_boots" => Some(981),
		"minecraft:netherite_chestplate" => Some(979),
		"minecraft:netherite_helmet" => Some(978),
		"minecraft:netherite_hoe" => Some(945),
		"minecraft:netherite_ingot" => Some(909),
		"minecraft:netherite_leggings" => Some(980),
		"minecraft:netherite_pickaxe" => Some(943),
		"minecraft:netherite_scrap" => Some(910),
		"minecraft:netherite_shovel" => Some(942),
		"minecraft:netherite_sword" => Some(941),
		"minecraft:netherite_upgrade_smithing_template" => Some(1409),
		"minecraft:netherrack" => Some(359),
		"minecraft:note_block" => Some(748),
		"minecraft:oak_boat" => Some(863),
		"minecraft:oak_button" => Some(751),
		"minecraft:oak_chest_boat" => Some(864),
		"minecraft:oak_door" => Some(780),
		"minecraft:oak_fence" => Some(344),
		"minecraft:oak_fence_gate" => Some(821),
		"minecraft:oak_hanging_sign" => Some(1000),
		"minecraft:oak_leaves" => Some(182),
		"minecraft:oak_log" => Some(134),
		"minecraft:oak_planks" => Some(36),
		"minecraft:oak_pressure_plate" => Some(767),
		"minecraft:oak_sapling" => Some(49),
		"minecraft:oak_shelf" => Some(313),
		"minecraft:oak_sign" => Some(988),
		"minecraft:oak_slab" => Some(270),
		"minecraft:oak_stairs" => Some(441),
		"minecraft:oak_trapdoor" => Some(801),
		"minecraft:oak_wood" => Some(171),
		"minecraft:observer" => Some(726),
		"minecraft:obsidian" => Some(321),
		"minecraft:ocelot_spawn_egg" => Some(1169),
		"minecraft:ochre_froglight" => Some(1403),
		"minecraft:ominous_bottle" => Some(1487),
		"minecraft:ominous_trial_key" => Some(1485),
		"minecraft:open_eyeblossom" => Some(230),
		"minecraft:orange_banner" => Some(1262),
		"minecraft:orange_bed" => Some(1087),
		"minecraft:orange_bundle" => Some(1038),
		"minecraft:orange_candle" => Some(1383),
		"minecraft:orange_carpet" => Some(506),
		"minecraft:orange_concrete" => Some(615),
		"minecraft:orange_concrete_powder" => Some(631),
		"minecraft:orange_dye" => Some(1067),
		"minecraft:orange_glazed_terracotta" => Some(599),
		"minecraft:orange_harness" => Some(839),
		"minecraft:orange_shulker_box" => Some(583),
		"minecraft:orange_stained_glass" => Some(531),
		"minecraft:orange_stained_glass_pane" => Some(547),
		"minecraft:orange_terracotta" => Some(487),
		"minecraft:orange_tulip" => Some(237),
		"minecraft:orange_wool" => Some(214),
		"minecraft:oxeye_daisy" => Some(240),
		"minecraft:oxidized_chiseled_copper" => Some(101),
		"minecraft:oxidized_copper" => Some(97),
		"minecraft:oxidized_copper_bars" => Some(394),
		"minecraft:oxidized_copper_bulb" => Some(1462),
		"minecraft:oxidized_copper_chain" => Some(403),
		"minecraft:oxidized_copper_chest" => Some(1470),
		"minecraft:oxidized_copper_door" => Some(795),
		"minecraft:oxidized_copper_golem_statue" => Some(1478),
		"minecraft:oxidized_copper_grate" => Some(1454),
		"minecraft:oxidized_copper_lantern" => Some(1351),
		"minecraft:oxidized_copper_trapdoor" => Some(816),
		"minecraft:oxidized_cut_copper" => Some(105),
		"minecraft:oxidized_cut_copper_slab" => Some(113),
		"minecraft:oxidized_cut_copper_stairs" => Some(109),
		"minecraft:oxidized_lightning_rod" => Some(736),
		"minecraft:packed_ice" => Some(522),
		"minecraft:packed_mud" => Some(379),
		"minecraft:painting" => Some(985),
		"minecraft:pale_hanging_moss" => Some(264),
		"minecraft:pale_moss_block" => Some(265),
		"minecraft:pale_moss_carpet" => Some(263),
		"minecraft:pale_oak_boat" => Some(877),
		"minecraft:pale_oak_button" => Some(758),
		"minecraft:pale_oak_chest_boat" => Some(878),
		"minecraft:pale_oak_door" => Some(787),
		"minecraft:pale_oak_fence" => Some(351),
		"minecraft:pale_oak_fence_gate" => Some(828),
		"minecraft:pale_oak_hanging_sign" => Some(1007),
		"minecraft:pale_oak_leaves" => Some(189),
		"minecraft:pale_oak_log" => Some(140),
		"minecraft:pale_oak_planks" => Some(43),
		"minecraft:pale_oak_pressure_plate" => Some(774),
		"minecraft:pale_oak_sapling" => Some(56),
		"minecraft:pale_oak_shelf" => Some(314),
		"minecraft:pale_oak_sign" => Some(995),
		"minecraft:pale_oak_slab" => Some(277),
		"minecraft:pale_oak_stairs" => Some(448),
		"minecraft:pale_oak_trapdoor" => Some(808),
		"minecraft:pale_oak_wood" => Some(177),
		"minecraft:panda_spawn_egg" => Some(1170),
		"minecraft:paper" => Some(1028),
		"minecraft:parrot_spawn_egg" => Some(1171),
		"minecraft:pearlescent_froglight" => Some(1405),
		"minecraft:peony" => Some(527),
		"minecraft:petrified_oak_slab" => Some(287),
		"minecraft:phantom_membrane" => Some(861),
		"minecraft:phantom_spawn_egg" => Some(1172),
		"minecraft:pig_spawn_egg" => Some(1173),
		"minecraft:piglin_banner_pattern" => Some(1330),
		"minecraft:piglin_brute_spawn_egg" => Some(1175),
		"minecraft:piglin_head" => Some(1235),
		"minecraft:piglin_spawn_egg" => Some(1174),
		"minecraft:pillager_spawn_egg" => Some(1176),
		"minecraft:pink_banner" => Some(1267),
		"minecraft:pink_bed" => Some(1092),
		"minecraft:pink_bundle" => Some(1043),
		"minecraft:pink_candle" => Some(1388),
		"minecraft:pink_carpet" => Some(511),
		"minecraft:pink_concrete" => Some(620),
		"minecraft:pink_concrete_powder" => Some(636),
		"minecraft:pink_dye" => Some(1072),
		"minecraft:pink_glazed_terracotta" => Some(604),
		"minecraft:pink_harness" => Some(844),
		"minecraft:pink_petals" => Some(258),
		"minecraft:pink_shulker_box" => Some(588),
		"minecraft:pink_stained_glass" => Some(536),
		"minecraft:pink_stained_glass_pane" => Some(552),
		"minecraft:pink_terracotta" => Some(492),
		"minecraft:pink_tulip" => Some(239),
		"minecraft:pink_wool" => Some(219),
		"minecraft:piston" => Some(722),
		"minecraft:pitcher_plant" => Some(245),
		"minecraft:pitcher_pod" => Some(1281),
		"minecraft:player_head" => Some(1231),
		"minecraft:plenty_pottery_sherd" => Some(1444),
		"minecraft:podzol" => Some(30),
		"minecraft:pointed_dripstone" => Some(1402),
		"minecraft:poisonous_potato" => Some(1226),
		"minecraft:polar_bear_spawn_egg" => Some(1177),
		"minecraft:polished_andesite" => Some(7),
		"minecraft:polished_andesite_slab" => Some(710),
		"minecraft:polished_andesite_stairs" => Some(693),
		"minecraft:polished_basalt" => Some(363),
		"minecraft:polished_blackstone" => Some(1372),
		"minecraft:polished_blackstone_brick_slab" => Some(1377),
		"minecraft:polished_blackstone_brick_stairs" => Some(1378),
		"minecraft:polished_blackstone_brick_wall" => Some(473),
		"minecraft:polished_blackstone_bricks" => Some(1376),
		"minecraft:polished_blackstone_button" => Some(750),
		"minecraft:polished_blackstone_pressure_plate" => Some(764),
		"minecraft:polished_blackstone_slab" => Some(1373),
		"minecraft:polished_blackstone_stairs" => Some(1374),
		"minecraft:polished_blackstone_wall" => Some(472),
		"minecraft:polished_deepslate" => Some(10),
		"minecraft:polished_deepslate_slab" => Some(713),
		"minecraft:polished_deepslate_stairs" => Some(696),
		"minecraft:polished_deepslate_wall" => Some(475),
		"minecraft:polished_diorite" => Some(5),
		"minecraft:polished_diorite_slab" => Some(702),
		"minecraft:polished_diorite_stairs" => Some(684),
		"minecraft:polished_granite" => Some(3),
		"minecraft:polished_granite_slab" => Some(699),
		"minecraft:polished_granite_stairs" => Some(681),
		"minecraft:polished_tuff" => Some(17),
		"minecraft:polished_tuff_slab" => Some(18),
		"minecraft:polished_tuff_stairs" => Some(19),
		"minecraft:polished_tuff_wall" => Some(20),
		"minecraft:popped_chorus_fruit" => Some(1279),
		"minecraft:poppy" => Some(232),
		"minecraft:porkchop" => Some(983),
		"minecraft:potato" => Some(1224),
		"minecraft:potion" => Some(1121),
		"minecraft:powder_snow_bucket" => Some(1015),
		"minecraft:powered_rail" => Some(833),
		"minecraft:prismarine" => Some(562),
		"minecraft:prismarine_brick_slab" => Some(298),
		"minecraft:prismarine_brick_stairs" => Some(566),
		"minecraft:prismarine_bricks" => Some(563),
		"minecraft:prismarine_crystals" => Some(1244),
		"minecraft:prismarine_shard" => Some(1243),
		"minecraft:prismarine_slab" => Some(297),
		"minecraft:prismarine_stairs" => Some(565),
		"minecraft:prismarine_wall" => Some(459),
		"minecraft:prize_pottery_sherd" => Some(1445),
		"minecraft:pufferfish" => Some(1060),
		"minecraft:pufferfish_bucket" => Some(1019),
		"minecraft:pufferfish_spawn_egg" => Some(1178),
		"minecraft:pumpkin" => Some(356),
		"minecraft:pumpkin_pie" => Some(1237),
		"minecraft:pumpkin_seeds" => Some(1108),
		"minecraft:purple_banner" => Some(1271),
		"minecraft:purple_bed" => Some(1096),
		"minecraft:purple_bundle" => Some(1047),
		"minecraft:purple_candle" => Some(1392),
		"minecraft:purple_carpet" => Some(515),
		"minecraft:purple_concrete" => Some(624),
		"minecraft:purple_concrete_powder" => Some(640),
		"minecraft:purple_dye" => Some(1076),
		"minecraft:purple_glazed_terracotta" => Some(608),
		"minecraft:purple_harness" => Some(848),
		"minecraft:purple_shulker_box" => Some(592),
		"minecraft:purple_stained_glass" => Some(540),
		"minecraft:purple_stained_glass_pane" => Some(556),
		"minecraft:purple_terracotta" => Some(496),
		"minecraft:purple_wool" => Some(223),
		"minecraft:purpur_block" => Some(326),
		"minecraft:purpur_pillar" => Some(327),
		"minecraft:purpur_slab" => Some(296),
		"minecraft:purpur_stairs" => Some(328),
		"minecraft:quartz" => Some(901),
		"minecraft:quartz_block" => Some(482),
		"minecraft:quartz_bricks" => Some(483),
		"minecraft:quartz_pillar" => Some(484),
		"minecraft:quartz_slab" => Some(293),
		"minecraft:quartz_stairs" => Some(485),
		"minecraft:rabbit" => Some(1245),
		"minecraft:rabbit_foot" => Some(1248),
		"minecraft:rabbit_hide" => Some(1249),
		"minecraft:rabbit_spawn_egg" => Some(1179),
		"minecraft:rabbit_stew" => Some(1247),
		"minecraft:rail" => Some(835),
		"minecraft:raiser_armor_trim_smithing_template" => Some(1424),
		"minecraft:ravager_spawn_egg" => Some(1180),
		"minecraft:raw_copper" => Some(905),
		"minecraft:raw_copper_block" => Some(85),
		"minecraft:raw_gold" => Some(907),
		"minecraft:raw_gold_block" => Some(86),
		"minecraft:raw_iron" => Some(903),
		"minecraft:raw_iron_block" => Some(84),
		"minecraft:recovery_compass" => Some(1035),
		"minecraft:red_banner" => Some(1275),
		"minecraft:red_bed" => Some(1100),
		"minecraft:red_bundle" => Some(1051),
		"minecraft:red_candle" => Some(1396),
		"minecraft:red_carpet" => Some(519),
		"minecraft:red_concrete" => Some(628),
		"minecraft:red_concrete_powder" => Some(644),
		"minecraft:red_dye" => Some(1080),
		"minecraft:red_glazed_terracotta" => Some(612),
		"minecraft:red_harness" => Some(852),
		"minecraft:red_mushroom" => Some(248),
		"minecraft:red_mushroom_block" => Some(388),
		"minecraft:red_nether_brick_slab" => Some(709),
		"minecraft:red_nether_brick_stairs" => Some(692),
		"minecraft:red_nether_brick_wall" => Some(467),
		"minecraft:red_nether_bricks" => Some(578),
		"minecraft:red_sand" => Some(62),
		"minecraft:red_sandstone" => Some(569),
		"minecraft:red_sandstone_slab" => Some(294),
		"minecraft:red_sandstone_stairs" => Some(572),
		"minecraft:red_sandstone_wall" => Some(460),
		"minecraft:red_shulker_box" => Some(596),
		"minecraft:red_stained_glass" => Some(544),
		"minecraft:red_stained_glass_pane" => Some(560),
		"minecraft:red_terracotta" => Some(500),
		"minecraft:red_tulip" => Some(236),
		"minecraft:red_wool" => Some(227),
		"minecraft:redstone" => Some(717),
		"minecraft:redstone_block" => Some(719),
		"minecraft:redstone_lamp" => Some(747),
		"minecraft:redstone_ore" => Some(72),
		"minecraft:redstone_torch" => Some(718),
		"minecraft:reinforced_deepslate" => Some(386),
		"minecraft:repeater" => Some(720),
		"minecraft:repeating_command_block" => Some(573),
		"minecraft:resin_block" => Some(413),
		"minecraft:resin_brick" => Some(1242),
		"minecraft:resin_brick_slab" => Some(416),
		"minecraft:resin_brick_stairs" => Some(415),
		"minecraft:resin_brick_wall" => Some(417),
		"minecraft:resin_bricks" => Some(414),
		"minecraft:resin_clump" => Some(412),
		"minecraft:respawn_anchor" => Some(1380),
		"minecraft:rib_armor_trim_smithing_template" => Some(1419),
		"minecraft:rooted_dirt" => Some(31),
		"minecraft:rose_bush" => Some(526),
		"minecraft:rotten_flesh" => Some(1114),
		"minecraft:saddle" => Some(837),
		"minecraft:salmon" => Some(1058),
		"minecraft:salmon_bucket" => Some(1020),
		"minecraft:salmon_spawn_egg" => Some(1181),
		"minecraft:sand" => Some(59),
		"minecraft:sandstone" => Some(198),
		"minecraft:sandstone_slab" => Some(285),
		"minecraft:sandstone_stairs" => Some(438),
		"minecraft:sandstone_wall" => Some(468),
		"minecraft:scaffolding" => Some(716),
		"minecraft:scrape_pottery_sherd" => Some(1446),
		"minecraft:sculk" => Some(429),
		"minecraft:sculk_catalyst" => Some(431),
		"minecraft:sculk_sensor" => Some(742),
		"minecraft:sculk_shrieker" => Some(432),
		"minecraft:sculk_vein" => Some(430),
		"minecraft:sea_lantern" => Some(568),
		"minecraft:sea_pickle" => Some(212),
		"minecraft:seagrass" => Some(211),
		"minecraft:sentry_armor_trim_smithing_template" => Some(1410),
		"minecraft:shaper_armor_trim_smithing_template" => Some(1422),
		"minecraft:sheaf_pottery_sherd" => Some(1447),
		"minecraft:shears" => Some(1105),
		"minecraft:sheep_spawn_egg" => Some(1182),
		"minecraft:shelter_pottery_sherd" => Some(1448),
		"minecraft:shield" => Some(1290),
		"minecraft:short_dry_grass" => Some(209),
		"minecraft:short_grass" => Some(202),
		"minecraft:shroomlight" => Some(1360),
		"minecraft:shulker_box" => Some(581),
		"minecraft:shulker_shell" => Some(1292),
		"minecraft:shulker_spawn_egg" => Some(1183),
		"minecraft:silence_armor_trim_smithing_template" => Some(1423),
		"minecraft:silverfish_spawn_egg" => Some(1184),
		"minecraft:skeleton_horse_spawn_egg" => Some(1186),
		"minecraft:skeleton_skull" => Some(1229),
		"minecraft:skeleton_spawn_egg" => Some(1185),
		"minecraft:skull_banner_pattern" => Some(1327),
		"minecraft:skull_pottery_sherd" => Some(1449),
		"minecraft:slime_ball" => Some(1030),
		"minecraft:slime_block" => Some(724),
		"minecraft:slime_spawn_egg" => Some(1187),
		"minecraft:small_amethyst_bud" => Some(1398),
		"minecraft:small_dripleaf" => Some(268),
		"minecraft:smithing_table" => Some(1343),
		"minecraft:smoker" => Some(1338),
		"minecraft:smooth_basalt" => Some(364),
		"minecraft:smooth_quartz" => Some(300),
		"minecraft:smooth_quartz_slab" => Some(706),
		"minecraft:smooth_quartz_stairs" => Some(689),
		"minecraft:smooth_red_sandstone" => Some(301),
		"minecraft:smooth_red_sandstone_slab" => Some(700),
		"minecraft:smooth_red_sandstone_stairs" => Some(682),
		"minecraft:smooth_sandstone" => Some(302),
		"minecraft:smooth_sandstone_slab" => Some(705),
		"minecraft:smooth_sandstone_stairs" => Some(688),
		"minecraft:smooth_stone" => Some(303),
		"minecraft:smooth_stone_slab" => Some(284),
		"minecraft:sniffer_egg" => Some(647),
		"minecraft:sniffer_spawn_egg" => Some(1188),
		"minecraft:snort_pottery_sherd" => Some(1450),
		"minecraft:snout_armor_trim_smithing_template" => Some(1418),
		"minecraft:snow" => Some(337),
		"minecraft:snow_block" => Some(339),
		"minecraft:snow_golem_spawn_egg" => Some(1189),
		"minecraft:snowball" => Some(1016),
		"minecraft:soul_campfire" => Some(1359),
		"minecraft:soul_lantern" => Some(1347),
		"minecraft:soul_sand" => Some(360),
		"minecraft:soul_soil" => Some(361),
		"minecraft:soul_torch" => Some(365),
		"minecraft:spawner" => Some(329),
		"minecraft:spectral_arrow" => Some(1287),
		"minecraft:spider_eye" => Some(1122),
		"minecraft:spider_spawn_egg" => Some(1190),
		"minecraft:spire_armor_trim_smithing_template" => Some(1420),
		"minecraft:splash_potion" => Some(1286),
		"minecraft:sponge" => Some(193),
		"minecraft:spore_blossom" => Some(246),
		"minecraft:spruce_boat" => Some(865),
		"minecraft:spruce_button" => Some(752),
		"minecraft:spruce_chest_boat" => Some(866),
		"minecraft:spruce_door" => Some(781),
		"minecraft:spruce_fence" => Some(345),
		"minecraft:spruce_fence_gate" => Some(822),
		"minecraft:spruce_hanging_sign" => Some(1001),
		"minecraft:spruce_leaves" => Some(183),
		"minecraft:spruce_log" => Some(135),
		"minecraft:spruce_planks" => Some(37),
		"minecraft:spruce_pressure_plate" => Some(768),
		"minecraft:spruce_sapling" => Some(50),
		"minecraft:spruce_shelf" => Some(315),
		"minecraft:spruce_sign" => Some(989),
		"minecraft:spruce_slab" => Some(271),
		"minecraft:spruce_stairs" => Some(442),
		"minecraft:spruce_trapdoor" => Some(802),
		"minecraft:spruce_wood" => Some(172),
		"minecraft:spyglass" => Some(1055),
		"minecraft:squid_spawn_egg" => Some(1191),
		"minecraft:stick" => Some(946),
		"minecraft:sticky_piston" => Some(723),
		"minecraft:stone" => Some(1),
		"minecraft:stone_axe" => Some(924),
		"minecraft:stone_brick_slab" => Some(290),
		"minecraft:stone_brick_stairs" => Some(420),
		"minecraft:stone_brick_wall" => Some(463),
		"minecraft:stone_bricks" => Some(375),
		"minecraft:stone_button" => Some(749),
		"minecraft:stone_hoe" => Some(925),
		"minecraft:stone_pickaxe" => Some(923),
		"minecraft:stone_pressure_plate" => Some(763),
		"minecraft:stone_shovel" => Some(922),
		"minecraft:stone_slab" => Some(283),
		"minecraft:stone_stairs" => Some(687),
		"minecraft:stone_sword" => Some(921),
		"minecraft:stonecutter" => Some(1344),
		"minecraft:stray_spawn_egg" => Some(1192),
		"minecraft:strider_spawn_egg" => Some(1193),
		"minecraft:string" => Some(948),
		"minecraft:stripped_acacia_log" => Some(152),
		"minecraft:stripped_acacia_wood" => Some(163),
		"minecraft:stripped_bamboo_block" => Some(170),
		"minecraft:stripped_birch_log" => Some(150),
		"minecraft:stripped_birch_wood" => Some(161),
		"minecraft:stripped_cherry_log" => Some(153),
		"minecraft:stripped_cherry_wood" => Some(164),
		"minecraft:stripped_crimson_hyphae" => Some(168),
		"minecraft:stripped_crimson_stem" => Some(157),
		"minecraft:stripped_dark_oak_log" => Some(154),
		"minecraft:stripped_dark_oak_wood" => Some(165),
		"minecraft:stripped_jungle_log" => Some(151),
		"minecraft:stripped_jungle_wood" => Some(162),
		"minecraft:stripped_mangrove_log" => Some(156),
		"minecraft:stripped_mangrove_wood" => Some(167),
		"minecraft:stripped_oak_log" => Some(148),
		"minecraft:stripped_oak_wood" => Some(159),
		"minecraft:stripped_pale_oak_log" => Some(155),
		"minecraft:stripped_pale_oak_wood" => Some(166),
		"minecraft:stripped_spruce_log" => Some(149),
		"minecraft:stripped_spruce_wood" => Some(160),
		"minecraft:stripped_warped_hyphae" => Some(169),
		"minecraft:stripped_warped_stem" => Some(158),
		"minecraft:structure_block" => Some(883),
		"minecraft:structure_void" => Some(580),
		"minecraft:sugar" => Some(1084),
		"minecraft:sugar_cane" => Some(256),
		"minecraft:sunflower" => Some(524),
		"minecraft:suspicious_gravel" => Some(61),
		"minecraft:suspicious_sand" => Some(60),
		"minecraft:suspicious_stew" => Some(1323),
		"minecraft:sweet_berries" => Some(1356),
		"minecraft:tadpole_bucket" => Some(1024),
		"minecraft:tadpole_spawn_egg" => Some(1194),
		"minecraft:tall_dry_grass" => Some(210),
		"minecraft:tall_grass" => Some(528),
		"minecraft:target" => Some(731),
		"minecraft:terracotta" => Some(521),
		"minecraft:test_block" => Some(885),
		"minecraft:test_instance_block" => Some(886),
		"minecraft:tide_armor_trim_smithing_template" => Some(1417),
		"minecraft:tinted_glass" => Some(196),
		"minecraft:tipped_arrow" => Some(1288),
		"minecraft:tnt" => Some(746),
		"minecraft:tnt_minecart" => Some(857),
		"minecraft:torch" => Some(322),
		"minecraft:torchflower" => Some(244),
		"minecraft:torchflower_seeds" => Some(1280),
		"minecraft:totem_of_undying" => Some(1291),
		"minecraft:trader_llama_spawn_egg" => Some(1195),
		"minecraft:trapped_chest" => Some(745),
		"minecraft:trial_key" => Some(1484),
		"minecraft:trial_spawner" => Some(1483),
		"minecraft:trident" => Some(1319),
		"minecraft:tripwire_hook" => Some(744),
		"minecraft:tropical_fish" => Some(1059),
		"minecraft:tropical_fish_bucket" => Some(1022),
		"minecraft:tropical_fish_spawn_egg" => Some(1196),
		"minecraft:tube_coral" => Some(659),
		"minecraft:tube_coral_block" => Some(654),
		"minecraft:tube_coral_fan" => Some(669),
		"minecraft:tuff" => Some(12),
		"minecraft:tuff_brick_slab" => Some(22),
		"minecraft:tuff_brick_stairs" => Some(23),
		"minecraft:tuff_brick_wall" => Some(24),
		"minecraft:tuff_bricks" => Some(21),
		"minecraft:tuff_slab" => Some(13),
		"minecraft:tuff_stairs" => Some(14),
		"minecraft:tuff_wall" => Some(15),
		"minecraft:turtle_egg" => Some(646),
		"minecraft:turtle_helmet" => Some(887),
		"minecraft:turtle_scute" => Some(888),
		"minecraft:turtle_spawn_egg" => Some(1197),
		"minecraft:twisting_vines" => Some(255),
		"minecraft:vault" => Some(1486),
		"minecraft:verdant_froglight" => Some(1404),
		"minecraft:vex_armor_trim_smithing_template" => Some(1416),
		"minecraft:vex_spawn_egg" => Some(1198),
		"minecraft:villager_spawn_egg" => Some(1199),
		"minecraft:vindicator_spawn_egg" => Some(1200),
		"minecraft:vine" => Some(410),
		"minecraft:wandering_trader_spawn_egg" => Some(1201),
		"minecraft:ward_armor_trim_smithing_template" => Some(1414),
		"minecraft:warden_spawn_egg" => Some(1202),
		"minecraft:warped_button" => Some(762),
		"minecraft:warped_door" => Some(791),
		"minecraft:warped_fence" => Some(355),
		"minecraft:warped_fence_gate" => Some(832),
		"minecraft:warped_fungus" => Some(250),
		"minecraft:warped_fungus_on_a_stick" => Some(860),
		"minecraft:warped_hanging_sign" => Some(1011),
		"minecraft:warped_hyphae" => Some(181),
		"minecraft:warped_nylium" => Some(34),
		"minecraft:warped_planks" => Some(47),
		"minecraft:warped_pressure_plate" => Some(778),
		"minecraft:warped_roots" => Some(252),
		"minecraft:warped_shelf" => Some(316),
		"minecraft:warped_sign" => Some(999),
		"minecraft:warped_slab" => Some(282),
		"minecraft:warped_stairs" => Some(453),
		"minecraft:warped_stem" => Some(146),
		"minecraft:warped_trapdoor" => Some(812),
		"minecraft:warped_wart_block" => Some(577),
		"minecraft:water_bucket" => Some(1013),
		"minecraft:waxed_chiseled_copper" => Some(118),
		"minecraft:waxed_copper_bars" => Some(395),
		"minecraft:waxed_copper_block" => Some(114),
		"minecraft:waxed_copper_bulb" => Some(1463),
		"minecraft:waxed_copper_chain" => Some(404),
		"minecraft:waxed_copper_chest" => Some(1471),
		"minecraft:waxed_copper_door" => Some(796),
		"minecraft:waxed_copper_golem_statue" => Some(1479),
		"minecraft:waxed_copper_grate" => Some(1455),
		"minecraft:waxed_copper_lantern" => Some(1352),
		"minecraft:waxed_copper_trapdoor" => Some(817),
		"minecraft:waxed_cut_copper" => Some(122),
		"minecraft:waxed_cut_copper_slab" => Some(130),
		"minecraft:waxed_cut_copper_stairs" => Some(126),
		"minecraft:waxed_exposed_chiseled_copper" => Some(119),
		"minecraft:waxed_exposed_copper" => Some(115),
		"minecraft:waxed_exposed_copper_bars" => Some(396),
		"minecraft:waxed_exposed_copper_bulb" => Some(1464),
		"minecraft:waxed_exposed_copper_chain" => Some(405),
		"minecraft:waxed_exposed_copper_chest" => Some(1472),
		"minecraft:waxed_exposed_copper_door" => Some(797),
		"minecraft:waxed_exposed_copper_golem_statue" => Some(1480),
		"minecraft:waxed_exposed_copper_grate" => Some(1456),
		"minecraft:waxed_exposed_copper_lantern" => Some(1353),
		"minecraft:waxed_exposed_copper_trapdoor" => Some(818),
		"minecraft:waxed_exposed_cut_copper" => Some(123),
		"minecraft:waxed_exposed_cut_copper_slab" => Some(131),
		"minecraft:waxed_exposed_cut_copper_stairs" => Some(127),
		"minecraft:waxed_exposed_lightning_rod" => Some(738),
		"minecraft:waxed_lightning_rod" => Some(737),
		"minecraft:waxed_oxidized_chiseled_copper" => Some(121),
		"minecraft:waxed_oxidized_copper" => Some(117),
		"minecraft:waxed_oxidized_copper_bars" => Some(398),
		"minecraft:waxed_oxidized_copper_bulb" => Some(1466),
		"minecraft:waxed_oxidized_copper_chain" => Some(407),
		"minecraft:waxed_oxidized_copper_chest" => Some(1474),
		"minecraft:waxed_oxidized_copper_door" => Some(799),
		"minecraft:waxed_oxidized_copper_golem_statue" => Some(1482),
		"minecraft:waxed_oxidized_copper_grate" => Some(1458),
		"minecraft:waxed_oxidized_copper_lantern" => Some(1355),
		"minecraft:waxed_oxidized_copper_trapdoor" => Some(820),
		"minecraft:waxed_oxidized_cut_copper" => Some(125),
		"minecraft:waxed_oxidized_cut_copper_slab" => Some(133),
		"minecraft:waxed_oxidized_cut_copper_stairs" => Some(129),
		"minecraft:waxed_oxidized_lightning_rod" => Some(740),
		"minecraft:waxed_weathered_chiseled_copper" => Some(120),
		"minecraft:waxed_weathered_copper" => Some(116),
		"minecraft:waxed_weathered_copper_bars" => Some(397),
		"minecraft:waxed_weathered_copper_bulb" => Some(1465),
		"minecraft:waxed_weathered_copper_chain" => Some(406),
		"minecraft:waxed_weathered_copper_chest" => Some(1473),
		"minecraft:waxed_weathered_copper_door" => Some(798),
		"minecraft:waxed_weathered_copper_golem_statue" => Some(1481),
		"minecraft:waxed_weathered_copper_grate" => Some(1457),
		"minecraft:waxed_weathered_copper_lantern" => Some(1354),
		"minecraft:waxed_weathered_copper_trapdoor" => Some(819),
		"minecraft:waxed_weathered_cut_copper" => Some(124),
		"minecraft:waxed_weathered_cut_copper_slab" => Some(132),
		"minecraft:waxed_weathered_cut_copper_stairs" => Some(128),
		"minecraft:waxed_weathered_lightning_rod" => Some(739),
		"minecraft:wayfinder_armor_trim_smithing_template" => Some(1421),
		"minecraft:weathered_chiseled_copper" => Some(100),
		"minecraft:weathered_copper" => Some(96),
		"minecraft:weathered_copper_bars" => Some(393),
		"minecraft:weathered_copper_bulb" => Some(1461),
		"minecraft:weathered_copper_chain" => Some(402),
		"minecraft:weathered_copper_chest" => Some(1469),
		"minecraft:weathered_copper_door" => Some(794),
		"minecraft:weathered_copper_golem_statue" => Some(1477),
		"minecraft:weathered_copper_grate" => Some(1453),
		"minecraft:weathered_copper_lantern" => Some(1350),
		"minecraft:weathered_copper_trapdoor" => Some(815),
		"minecraft:weathered_cut_copper" => Some(104),
		"minecraft:weathered_cut_copper_slab" => Some(112),
		"minecraft:weathered_cut_copper_stairs" => Some(108),
		"minecraft:weathered_lightning_rod" => Some(735),
		"minecraft:weeping_vines" => Some(254),
		"minecraft:wet_sponge" => Some(194),
		"minecraft:wheat" => Some(952),
		"minecraft:wheat_seeds" => Some(951),
		"minecraft:white_banner" => Some(1261),
		"minecraft:white_bed" => Some(1086),
		"minecraft:white_bundle" => Some(1037),
		"minecraft:white_candle" => Some(1382),
		"minecraft:white_carpet" => Some(505),
		"minecraft:white_concrete" => Some(614),
		"minecraft:white_concrete_powder" => Some(630),
		"minecraft:white_dye" => Some(1066),
		"minecraft:white_glazed_terracotta" => Some(598),
		"minecraft:white_harness" => Some(838),
		"minecraft:white_shulker_box" => Some(582),
		"minecraft:white_stained_glass" => Some(530),
		"minecraft:white_stained_glass_pane" => Some(546),
		"minecraft:white_terracotta" => Some(486),
		"minecraft:white_tulip" => Some(238),
		"minecraft:white_wool" => Some(213),
		"minecraft:wild_armor_trim_smithing_template" => Some(1413),
		"minecraft:wildflowers" => Some(259),
		"minecraft:wind_charge" => Some(1215),
		"minecraft:witch_spawn_egg" => Some(1203),
		"minecraft:wither_rose" => Some(243),
		"minecraft:wither_skeleton_skull" => Some(1230),
		"minecraft:wither_skeleton_spawn_egg" => Some(1205),
		"minecraft:wither_spawn_egg" => Some(1204),
		"minecraft:wolf_armor" => Some(890),
		"minecraft:wolf_spawn_egg" => Some(1206),
		"minecraft:wooden_axe" => Some(914),
		"minecraft:wooden_hoe" => Some(915),
		"minecraft:wooden_pickaxe" => Some(913),
		"minecraft:wooden_shovel" => Some(912),
		"minecraft:wooden_sword" => Some(911),
		"minecraft:writable_book" => Some(1216),
		"minecraft:written_book" => Some(1217),
		"minecraft:yellow_banner" => Some(1265),
		"minecraft:yellow_bed" => Some(1090),
		"minecraft:yellow_bundle" => Some(1041),
		"minecraft:yellow_candle" => Some(1386),
		"minecraft:yellow_carpet" => Some(509),
		"minecraft:yellow_concrete" => Some(618),
		"minecraft:yellow_concrete_powder" => Some(634),
		"minecraft:yellow_dye" => Some(1070),
		"minecraft:yellow_glazed_terracotta" => Some(602),
		"minecraft:yellow_harness" => Some(842),
		"minecraft:yellow_shulker_box" => Some(586),
		"minecraft:yellow_stained_glass" => Some(534),
		"minecraft:yellow_stained_glass_pane" => Some(550),
		"minecraft:yellow_terracotta" => Some(490),
		"minecraft:yellow_wool" => Some(217),
		"minecraft:zoglin_spawn_egg" => Some(1207),
		"minecraft:zombie_head" => Some(1232),
		"minecraft:zombie_horse_spawn_egg" => Some(1210),
		"minecraft:zombie_spawn_egg" => Some(1209),
		"minecraft:zombie_villager_spawn_egg" => Some(1211),
		"minecraft:zombified_piglin_spawn_egg" => Some(1212),
    _ => None,
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
