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
		1253 => Some("minecraft:abandoned_camp_map"),
		983 => Some("minecraft:acacia_boat"),
		862 => Some("minecraft:acacia_button"),
		984 => Some("minecraft:acacia_chest_boat"),
		893 => Some("minecraft:acacia_door"),
		421 => Some("minecraft:acacia_fence"),
		936 => Some("minecraft:acacia_fence_gate"),
		1119 => Some("minecraft:acacia_hanging_sign"),
		219 => Some("minecraft:acacia_leaves"),
		167 => Some("minecraft:acacia_log"),
		67 => Some("minecraft:acacia_planks"),
		879 => Some("minecraft:acacia_pressure_plate"),
		81 => Some("minecraft:acacia_sapling"),
		377 => Some("minecraft:acacia_shelf"),
		1106 => Some("minecraft:acacia_sign"),
		345 => Some("minecraft:acacia_slab"),
		519 => Some("minecraft:acacia_stairs"),
		915 => Some("minecraft:acacia_trapdoor"),
		207 => Some("minecraft:acacia_wood"),
		948 => Some("minecraft:activator_rail"),
		0 => Some("minecraft:air"),
		1313 => Some("minecraft:allay_spawn_egg"),
		304 => Some("minecraft:allium"),
		117 => Some("minecraft:amethyst_block"),
		1570 => Some("minecraft:amethyst_cluster"),
		1016 => Some("minecraft:amethyst_shard"),
		111 => Some("minecraft:ancient_debris"),
		6 => Some("minecraft:andesite"),
		815 => Some("minecraft:andesite_slab"),
		798 => Some("minecraft:andesite_stairs"),
		541 => Some("minecraft:andesite_wall"),
		1598 => Some("minecraft:angler_pottery_sherd"),
		553 => Some("minecraft:anvil"),
		1007 => Some("minecraft:apple"),
		1599 => Some("minecraft:archer_pottery_sherd"),
		1003 => Some("minecraft:armadillo_scute"),
		1291 => Some("minecraft:armadillo_spawn_egg"),
		1405 => Some("minecraft:armor_stand"),
		1600 => Some("minecraft:arms_up_pottery_sherd"),
		1009 => Some("minecraft:arrow"),
		1139 => Some("minecraft:axolotl_bucket"),
		1301 => Some("minecraft:axolotl_spawn_egg"),
		242 => Some("minecraft:azalea"),
		227 => Some("minecraft:azalea_leaves"),
		305 => Some("minecraft:azure_bluet"),
		1380 => Some("minecraft:baked_potato"),
		340 => Some("minecraft:bamboo"),
		177 => Some("minecraft:bamboo_block"),
		868 => Some("minecraft:bamboo_button"),
		996 => Some("minecraft:bamboo_chest_raft"),
		899 => Some("minecraft:bamboo_door"),
		427 => Some("minecraft:bamboo_fence"),
		942 => Some("minecraft:bamboo_fence_gate"),
		1125 => Some("minecraft:bamboo_hanging_sign"),
		76 => Some("minecraft:bamboo_mosaic"),
		352 => Some("minecraft:bamboo_mosaic_slab"),
		526 => Some("minecraft:bamboo_mosaic_stairs"),
		73 => Some("minecraft:bamboo_planks"),
		885 => Some("minecraft:bamboo_pressure_plate"),
		995 => Some("minecraft:bamboo_raft"),
		378 => Some("minecraft:bamboo_shelf"),
		1112 => Some("minecraft:bamboo_sign"),
		351 => Some("minecraft:bamboo_slab"),
		525 => Some("minecraft:bamboo_stairs"),
		921 => Some("minecraft:bamboo_trapdoor"),
		1506 => Some("minecraft:barrel"),
		577 => Some("minecraft:barrier"),
		436 => Some("minecraft:basalt"),
		1292 => Some("minecraft:bat_spawn_egg"),
		530 => Some("minecraft:beacon"),
		87 => Some("minecraft:bedrock"),
		1531 => Some("minecraft:bee_nest"),
		1293 => Some("minecraft:bee_spawn_egg"),
		1260 => Some("minecraft:beef"),
		1532 => Some("minecraft:beehive"),
		1438 => Some("minecraft:beetroot"),
		1439 => Some("minecraft:beetroot_seeds"),
		1440 => Some("minecraft:beetroot_soup"),
		1514 => Some("minecraft:bell"),
		338 => Some("minecraft:big_dripleaf"),
		979 => Some("minecraft:birch_boat"),
		860 => Some("minecraft:birch_button"),
		980 => Some("minecraft:birch_chest_boat"),
		891 => Some("minecraft:birch_door"),
		419 => Some("minecraft:birch_fence"),
		934 => Some("minecraft:birch_fence_gate"),
		1117 => Some("minecraft:birch_hanging_sign"),
		217 => Some("minecraft:birch_leaves"),
		165 => Some("minecraft:birch_log"),
		65 => Some("minecraft:birch_planks"),
		877 => Some("minecraft:birch_pressure_plate"),
		79 => Some("minecraft:birch_sapling"),
		379 => Some("minecraft:birch_shelf"),
		1104 => Some("minecraft:birch_sign"),
		343 => Some("minecraft:birch_slab"),
		517 => Some("minecraft:birch_stairs"),
		913 => Some("minecraft:birch_trapdoor"),
		205 => Some("minecraft:birch_wood"),
		1432 => Some("minecraft:black_banner"),
		1234 => Some("minecraft:black_bed"),
		1169 => Some("minecraft:black_bundle"),
		1566 => Some("minecraft:black_candle"),
		595 => Some("minecraft:black_carpet"),
		704 => Some("minecraft:black_concrete"),
		752 => Some("minecraft:black_concrete_powder"),
		736 => Some("minecraft:black_concrete_slab"),
		720 => Some("minecraft:black_concrete_stairs"),
		1185 => Some("minecraft:black_cushion"),
		1214 => Some("minecraft:black_dye"),
		688 => Some("minecraft:black_glazed_terracotta"),
		965 => Some("minecraft:black_harness"),
		672 => Some("minecraft:black_shulker_box"),
		620 => Some("minecraft:black_stained_glass"),
		636 => Some("minecraft:black_stained_glass_pane"),
		576 => Some("minecraft:black_terracotta"),
		265 => Some("minecraft:black_wool"),
		297 => Some("minecraft:black_wool_slab"),
		281 => Some("minecraft:black_wool_stairs"),
		1537 => Some("minecraft:blackstone"),
		1538 => Some("minecraft:blackstone_slab"),
		1539 => Some("minecraft:blackstone_stairs"),
		546 => Some("minecraft:blackstone_wall"),
		1601 => Some("minecraft:blade_pottery_sherd"),
		1508 => Some("minecraft:blast_furnace"),
		1274 => Some("minecraft:blaze_powder"),
		1266 => Some("minecraft:blaze_rod"),
		1354 => Some("minecraft:blaze_spawn_egg"),
		1428 => Some("minecraft:blue_banner"),
		1230 => Some("minecraft:blue_bed"),
		1165 => Some("minecraft:blue_bundle"),
		1562 => Some("minecraft:blue_candle"),
		591 => Some("minecraft:blue_carpet"),
		700 => Some("minecraft:blue_concrete"),
		748 => Some("minecraft:blue_concrete_powder"),
		732 => Some("minecraft:blue_concrete_slab"),
		716 => Some("minecraft:blue_concrete_stairs"),
		1181 => Some("minecraft:blue_cushion"),
		1210 => Some("minecraft:blue_dye"),
		1149 => Some("minecraft:blue_egg"),
		684 => Some("minecraft:blue_glazed_terracotta"),
		961 => Some("minecraft:blue_harness"),
		786 => Some("minecraft:blue_ice"),
		303 => Some("minecraft:blue_orchid"),
		668 => Some("minecraft:blue_shulker_box"),
		616 => Some("minecraft:blue_stained_glass"),
		632 => Some("minecraft:blue_stained_glass_pane"),
		572 => Some("minecraft:blue_terracotta"),
		261 => Some("minecraft:blue_wool"),
		293 => Some("minecraft:blue_wool_slab"),
		277 => Some("minecraft:blue_wool_stairs"),
		1323 => Some("minecraft:bogged_spawn_egg"),
		1597 => Some("minecraft:bolt_armor_trim_smithing_template"),
		1216 => Some("minecraft:bone"),
		654 => Some("minecraft:bone_block"),
		1215 => Some("minecraft:bone_meal"),
		1146 => Some("minecraft:book"),
		390 => Some("minecraft:bookshelf"),
		1503 => Some("minecraft:bordure_indented_banner_pattern"),
		1008 => Some("minecraft:bow"),
		1006 => Some("minecraft:bowl"),
		767 => Some("minecraft:brain_coral"),
		762 => Some("minecraft:brain_coral_block"),
		777 => Some("minecraft:brain_coral_fan"),
		1067 => Some("minecraft:bread"),
		1373 => Some("minecraft:breeze_rod"),
		1339 => Some("minecraft:breeze_spawn_egg"),
		1602 => Some("minecraft:brewer_pottery_sherd"),
		1276 => Some("minecraft:brewing_stand"),
		1142 => Some("minecraft:brick"),
		361 => Some("minecraft:brick_slab"),
		493 => Some("minecraft:brick_stairs"),
		533 => Some("minecraft:brick_wall"),
		376 => Some("minecraft:bricks"),
		1429 => Some("minecraft:brown_banner"),
		1231 => Some("minecraft:brown_bed"),
		1166 => Some("minecraft:brown_bundle"),
		1563 => Some("minecraft:brown_candle"),
		592 => Some("minecraft:brown_carpet"),
		701 => Some("minecraft:brown_concrete"),
		749 => Some("minecraft:brown_concrete_powder"),
		733 => Some("minecraft:brown_concrete_slab"),
		717 => Some("minecraft:brown_concrete_stairs"),
		1182 => Some("minecraft:brown_cushion"),
		1211 => Some("minecraft:brown_dye"),
		1150 => Some("minecraft:brown_egg"),
		685 => Some("minecraft:brown_glazed_terracotta"),
		962 => Some("minecraft:brown_harness"),
		317 => Some("minecraft:brown_mushroom"),
		461 => Some("minecraft:brown_mushroom_block"),
		669 => Some("minecraft:brown_shulker_box"),
		617 => Some("minecraft:brown_stained_glass"),
		633 => Some("minecraft:brown_stained_glass_pane"),
		573 => Some("minecraft:brown_terracotta"),
		262 => Some("minecraft:brown_wool"),
		294 => Some("minecraft:brown_wool_slab"),
		278 => Some("minecraft:brown_wool_stairs"),
		1578 => Some("minecraft:brush"),
		768 => Some("minecraft:bubble_coral"),
		763 => Some("minecraft:bubble_coral_block"),
		778 => Some("minecraft:bubble_coral_fan"),
		1128 => Some("minecraft:bucket"),
		118 => Some("minecraft:budding_amethyst"),
		1153 => Some("minecraft:bundle"),
		1250 => Some("minecraft:buried_ancient_city_map"),
		1251 => Some("minecraft:buried_mineshaft_map"),
		1249 => Some("minecraft:buried_treasure_map"),
		1241 => Some("minecraft:buried_trial_chambers_map"),
		1603 => Some("minecraft:burn_pottery_sherd"),
		240 => Some("minecraft:bush"),
		413 => Some("minecraft:cactus"),
		414 => Some("minecraft:cactus_flower"),
		1218 => Some("minecraft:cake"),
		11 => Some("minecraft:calcite"),
		850 => Some("minecraft:calibrated_sculk_sensor"),
		1324 => Some("minecraft:camel_husk_spawn_egg"),
		1284 => Some("minecraft:camel_spawn_egg"),
		1527 => Some("minecraft:campfire"),
		1550 => Some("minecraft:candle"),
		1378 => Some("minecraft:carrot"),
		971 => Some("minecraft:carrot_on_a_stick"),
		1509 => Some("minecraft:cartography_table"),
		431 => Some("minecraft:carved_pumpkin"),
		1288 => Some("minecraft:cat_spawn_egg"),
		1277 => Some("minecraft:cauldron"),
		1337 => Some("minecraft:cave_spider_spawn_egg"),
		649 => Some("minecraft:chain_command_block"),
		1079 => Some("minecraft:chainmail_boots"),
		1077 => Some("minecraft:chainmail_chestplate"),
		1076 => Some("minecraft:chainmail_helmet"),
		1078 => Some("minecraft:chainmail_leggings"),
		1011 => Some("minecraft:charcoal"),
		985 => Some("minecraft:cherry_boat"),
		863 => Some("minecraft:cherry_button"),
		986 => Some("minecraft:cherry_chest_boat"),
		894 => Some("minecraft:cherry_door"),
		422 => Some("minecraft:cherry_fence"),
		937 => Some("minecraft:cherry_fence_gate"),
		1120 => Some("minecraft:cherry_hanging_sign"),
		220 => Some("minecraft:cherry_leaves"),
		168 => Some("minecraft:cherry_log"),
		68 => Some("minecraft:cherry_planks"),
		880 => Some("minecraft:cherry_pressure_plate"),
		82 => Some("minecraft:cherry_sapling"),
		380 => Some("minecraft:cherry_shelf"),
		1107 => Some("minecraft:cherry_sign"),
		346 => Some("minecraft:cherry_slab"),
		520 => Some("minecraft:cherry_stairs"),
		916 => Some("minecraft:cherry_trapdoor"),
		208 => Some("minecraft:cherry_wood"),
		404 => Some("minecraft:chest"),
		967 => Some("minecraft:chest_minecart"),
		1262 => Some("minecraft:chicken"),
		1280 => Some("minecraft:chicken_spawn_egg"),
		554 => Some("minecraft:chipped_anvil"),
		391 => Some("minecraft:chiseled_bookshelf"),
		52 => Some("minecraft:chiseled_cinnabar"),
		131 => Some("minecraft:chiseled_copper"),
		459 => Some("minecraft:chiseled_deepslate"),
		500 => Some("minecraft:chiseled_nether_bricks"),
		1544 => Some("minecraft:chiseled_polished_blackstone"),
		556 => Some("minecraft:chiseled_quartz_block"),
		645 => Some("minecraft:chiseled_red_sandstone"),
		492 => Some("minecraft:chiseled_resin_bricks"),
		235 => Some("minecraft:chiseled_sandstone"),
		452 => Some("minecraft:chiseled_stone_bricks"),
		39 => Some("minecraft:chiseled_sulfur"),
		16 => Some("minecraft:chiseled_tuff"),
		25 => Some("minecraft:chiseled_tuff_bricks"),
		398 => Some("minecraft:chorus_flower"),
		1434 => Some("minecraft:chorus_fruit"),
		397 => Some("minecraft:chorus_plant"),
		40 => Some("minecraft:cinnabar"),
		49 => Some("minecraft:cinnabar_brick_slab"),
		50 => Some("minecraft:cinnabar_brick_stairs"),
		51 => Some("minecraft:cinnabar_brick_wall"),
		48 => Some("minecraft:cinnabar_bricks"),
		41 => Some("minecraft:cinnabar_slab"),
		42 => Some("minecraft:cinnabar_stairs"),
		43 => Some("minecraft:cinnabar_wall"),
		415 => Some("minecraft:clay"),
		1143 => Some("minecraft:clay_ball"),
		1187 => Some("minecraft:clock"),
		301 => Some("minecraft:closed_eyeblossom"),
		1010 => Some("minecraft:coal"),
		112 => Some("minecraft:coal_block"),
		93 => Some("minecraft:coal_ore"),
		56 => Some("minecraft:coarse_dirt"),
		1582 => Some("minecraft:coast_armor_trim_smithing_template"),
		9 => Some("minecraft:cobbled_deepslate"),
		819 => Some("minecraft:cobbled_deepslate_slab"),
		802 => Some("minecraft:cobbled_deepslate_stairs"),
		549 => Some("minecraft:cobbled_deepslate_wall"),
		62 => Some("minecraft:cobblestone"),
		360 => Some("minecraft:cobblestone_slab"),
		409 => Some("minecraft:cobblestone_stairs"),
		531 => Some("minecraft:cobblestone_wall"),
		237 => Some("minecraft:cobweb"),
		1198 => Some("minecraft:cocoa_beans"),
		1190 => Some("minecraft:cod"),
		1137 => Some("minecraft:cod_bucket"),
		1302 => Some("minecraft:cod_spawn_egg"),
		529 => Some("minecraft:command_block"),
		1414 => Some("minecraft:command_block_minecart"),
		828 => Some("minecraft:comparator"),
		1151 => Some("minecraft:compass"),
		1505 => Some("minecraft:composter"),
		787 => Some("minecraft:conduit"),
		1261 => Some("minecraft:cooked_beef"),
		1263 => Some("minecraft:cooked_chicken"),
		1194 => Some("minecraft:cooked_cod"),
		1416 => Some("minecraft:cooked_mutton"),
		1098 => Some("minecraft:cooked_porkchop"),
		1401 => Some("minecraft:cooked_rabbit"),
		1195 => Some("minecraft:cooked_salmon"),
		1236 => Some("minecraft:cookie"),
		1033 => Some("minecraft:copper_axe"),
		465 => Some("minecraft:copper_bars"),
		120 => Some("minecraft:copper_block"),
		1075 => Some("minecraft:copper_boots"),
		1629 => Some("minecraft:copper_bulb"),
		474 => Some("minecraft:copper_chain"),
		1637 => Some("minecraft:copper_chest"),
		1073 => Some("minecraft:copper_chestplate"),
		902 => Some("minecraft:copper_door"),
		1317 => Some("minecraft:copper_golem_spawn_egg"),
		1645 => Some("minecraft:copper_golem_statue"),
		1621 => Some("minecraft:copper_grate"),
		1072 => Some("minecraft:copper_helmet"),
		1034 => Some("minecraft:copper_hoe"),
		1406 => Some("minecraft:copper_horse_armor"),
		1020 => Some("minecraft:copper_ingot"),
		1517 => Some("minecraft:copper_lantern"),
		1074 => Some("minecraft:copper_leggings"),
		1489 => Some("minecraft:copper_nautilus_armor"),
		1457 => Some("minecraft:copper_nugget"),
		97 => Some("minecraft:copper_ore"),
		1032 => Some("minecraft:copper_pickaxe"),
		1031 => Some("minecraft:copper_shovel"),
		1449 => Some("minecraft:copper_spear"),
		1030 => Some("minecraft:copper_sword"),
		440 => Some("minecraft:copper_torch"),
		924 => Some("minecraft:copper_trapdoor"),
		311 => Some("minecraft:cornflower"),
		1281 => Some("minecraft:cow_spawn_egg"),
		456 => Some("minecraft:cracked_deepslate_bricks"),
		458 => Some("minecraft:cracked_deepslate_tiles"),
		499 => Some("minecraft:cracked_nether_bricks"),
		1548 => Some("minecraft:cracked_polished_blackstone_bricks"),
		451 => Some("minecraft:cracked_stone_bricks"),
		1237 => Some("minecraft:crafter"),
		405 => Some("minecraft:crafting_table"),
		403 => Some("minecraft:creaking_heart"),
		1340 => Some("minecraft:creaking_spawn_egg"),
		1495 => Some("minecraft:creeper_banner_pattern"),
		1388 => Some("minecraft:creeper_head"),
		1341 => Some("minecraft:creeper_spawn_egg"),
		869 => Some("minecraft:crimson_button"),
		900 => Some("minecraft:crimson_door"),
		428 => Some("minecraft:crimson_fence"),
		943 => Some("minecraft:crimson_fence_gate"),
		320 => Some("minecraft:crimson_fungus"),
		1126 => Some("minecraft:crimson_hanging_sign"),
		213 => Some("minecraft:crimson_hyphae"),
		60 => Some("minecraft:crimson_nylium"),
		74 => Some("minecraft:crimson_planks"),
		886 => Some("minecraft:crimson_pressure_plate"),
		322 => Some("minecraft:crimson_roots"),
		381 => Some("minecraft:crimson_shelf"),
		1113 => Some("minecraft:crimson_sign"),
		353 => Some("minecraft:crimson_slab"),
		527 => Some("minecraft:crimson_stairs"),
		175 => Some("minecraft:crimson_stem"),
		922 => Some("minecraft:crimson_trapdoor"),
		1491 => Some("minecraft:crossbow"),
		1536 => Some("minecraft:crying_obsidian"),
		139 => Some("minecraft:cut_copper"),
		155 => Some("minecraft:cut_copper_slab"),
		147 => Some("minecraft:cut_copper_stairs"),
		646 => Some("minecraft:cut_red_sandstone"),
		367 => Some("minecraft:cut_red_sandstone_slab"),
		236 => Some("minecraft:cut_sandstone"),
		358 => Some("minecraft:cut_sandstone_slab"),
		1426 => Some("minecraft:cyan_banner"),
		1228 => Some("minecraft:cyan_bed"),
		1163 => Some("minecraft:cyan_bundle"),
		1560 => Some("minecraft:cyan_candle"),
		589 => Some("minecraft:cyan_carpet"),
		698 => Some("minecraft:cyan_concrete"),
		746 => Some("minecraft:cyan_concrete_powder"),
		730 => Some("minecraft:cyan_concrete_slab"),
		714 => Some("minecraft:cyan_concrete_stairs"),
		1179 => Some("minecraft:cyan_cushion"),
		1208 => Some("minecraft:cyan_dye"),
		682 => Some("minecraft:cyan_glazed_terracotta"),
		959 => Some("minecraft:cyan_harness"),
		666 => Some("minecraft:cyan_shulker_box"),
		614 => Some("minecraft:cyan_stained_glass"),
		630 => Some("minecraft:cyan_stained_glass_pane"),
		570 => Some("minecraft:cyan_terracotta"),
		259 => Some("minecraft:cyan_wool"),
		291 => Some("minecraft:cyan_wool_slab"),
		275 => Some("minecraft:cyan_wool_stairs"),
		555 => Some("minecraft:damaged_anvil"),
		298 => Some("minecraft:dandelion"),
		1604 => Some("minecraft:danger_pottery_sherd"),
		987 => Some("minecraft:dark_oak_boat"),
		864 => Some("minecraft:dark_oak_button"),
		988 => Some("minecraft:dark_oak_chest_boat"),
		895 => Some("minecraft:dark_oak_door"),
		423 => Some("minecraft:dark_oak_fence"),
		938 => Some("minecraft:dark_oak_fence_gate"),
		1121 => Some("minecraft:dark_oak_hanging_sign"),
		221 => Some("minecraft:dark_oak_leaves"),
		170 => Some("minecraft:dark_oak_log"),
		69 => Some("minecraft:dark_oak_planks"),
		881 => Some("minecraft:dark_oak_pressure_plate"),
		83 => Some("minecraft:dark_oak_sapling"),
		382 => Some("minecraft:dark_oak_shelf"),
		1108 => Some("minecraft:dark_oak_sign"),
		347 => Some("minecraft:dark_oak_slab"),
		521 => Some("minecraft:dark_oak_stairs"),
		917 => Some("minecraft:dark_oak_trapdoor"),
		210 => Some("minecraft:dark_oak_wood"),
		639 => Some("minecraft:dark_prismarine"),
		371 => Some("minecraft:dark_prismarine_slab"),
		642 => Some("minecraft:dark_prismarine_stairs"),
		848 => Some("minecraft:daylight_detector"),
		771 => Some("minecraft:dead_brain_coral"),
		757 => Some("minecraft:dead_brain_coral_block"),
		782 => Some("minecraft:dead_brain_coral_fan"),
		772 => Some("minecraft:dead_bubble_coral"),
		758 => Some("minecraft:dead_bubble_coral_block"),
		783 => Some("minecraft:dead_bubble_coral_fan"),
		244 => Some("minecraft:dead_bush"),
		773 => Some("minecraft:dead_fire_coral"),
		759 => Some("minecraft:dead_fire_coral_block"),
		784 => Some("minecraft:dead_fire_coral_fan"),
		774 => Some("minecraft:dead_horn_coral"),
		760 => Some("minecraft:dead_horn_coral_block"),
		785 => Some("minecraft:dead_horn_coral_fan"),
		775 => Some("minecraft:dead_tube_coral"),
		756 => Some("minecraft:dead_tube_coral_block"),
		781 => Some("minecraft:dead_tube_coral_fan"),
		1459 => Some("minecraft:debug_stick"),
		392 => Some("minecraft:decorated_pot"),
		8 => Some("minecraft:deepslate"),
		821 => Some("minecraft:deepslate_brick_slab"),
		804 => Some("minecraft:deepslate_brick_stairs"),
		551 => Some("minecraft:deepslate_brick_wall"),
		455 => Some("minecraft:deepslate_bricks"),
		94 => Some("minecraft:deepslate_coal_ore"),
		98 => Some("minecraft:deepslate_copper_ore"),
		108 => Some("minecraft:deepslate_diamond_ore"),
		104 => Some("minecraft:deepslate_emerald_ore"),
		100 => Some("minecraft:deepslate_gold_ore"),
		96 => Some("minecraft:deepslate_iron_ore"),
		106 => Some("minecraft:deepslate_lapis_ore"),
		102 => Some("minecraft:deepslate_redstone_ore"),
		822 => Some("minecraft:deepslate_tile_slab"),
		805 => Some("minecraft:deepslate_tile_stairs"),
		552 => Some("minecraft:deepslate_tile_wall"),
		457 => Some("minecraft:deepslate_tiles"),
		1252 => Some("minecraft:desert_pyramid_map"),
		1244 => Some("minecraft:desert_village_map"),
		946 => Some("minecraft:detector_rail"),
		1012 => Some("minecraft:diamond"),
		1053 => Some("minecraft:diamond_axe"),
		129 => Some("minecraft:diamond_block"),
		1087 => Some("minecraft:diamond_boots"),
		1085 => Some("minecraft:diamond_chestplate"),
		1084 => Some("minecraft:diamond_helmet"),
		1054 => Some("minecraft:diamond_hoe"),
		1409 => Some("minecraft:diamond_horse_armor"),
		1086 => Some("minecraft:diamond_leggings"),
		1487 => Some("minecraft:diamond_nautilus_armor"),
		107 => Some("minecraft:diamond_ore"),
		1052 => Some("minecraft:diamond_pickaxe"),
		1051 => Some("minecraft:diamond_shovel"),
		1452 => Some("minecraft:diamond_spear"),
		1050 => Some("minecraft:diamond_sword"),
		4 => Some("minecraft:diorite"),
		818 => Some("minecraft:diorite_slab"),
		801 => Some("minecraft:diorite_stairs"),
		545 => Some("minecraft:diorite_wall"),
		55 => Some("minecraft:dirt"),
		598 => Some("minecraft:dirt_path"),
		1482 => Some("minecraft:disc_fragment_5"),
		835 => Some("minecraft:dispenser"),
		1303 => Some("minecraft:dolphin_spawn_egg"),
		1285 => Some("minecraft:donkey_spawn_egg"),
		1441 => Some("minecraft:dragon_breath"),
		511 => Some("minecraft:dragon_egg"),
		1389 => Some("minecraft:dragon_head"),
		755 => Some("minecraft:dried_ghast"),
		1257 => Some("minecraft:dried_kelp"),
		1144 => Some("minecraft:dried_kelp_block"),
		53 => Some("minecraft:dripstone_block"),
		836 => Some("minecraft:dropper"),
		1325 => Some("minecraft:drowned_spawn_egg"),
		1581 => Some("minecraft:dune_armor_trim_smithing_template"),
		1577 => Some("minecraft:echo_shard"),
		1148 => Some("minecraft:egg"),
		1342 => Some("minecraft:elder_guardian_spawn_egg"),
		974 => Some("minecraft:elytra"),
		1013 => Some("minecraft:emerald"),
		514 => Some("minecraft:emerald_block"),
		103 => Some("minecraft:emerald_ore"),
		1395 => Some("minecraft:enchanted_book"),
		1101 => Some("minecraft:enchanted_golden_apple"),
		507 => Some("minecraft:enchanting_table"),
		1433 => Some("minecraft:end_crystal"),
		508 => Some("minecraft:end_portal_frame"),
		396 => Some("minecraft:end_rod"),
		509 => Some("minecraft:end_stone"),
		811 => Some("minecraft:end_stone_brick_slab"),
		793 => Some("minecraft:end_stone_brick_stairs"),
		544 => Some("minecraft:end_stone_brick_wall"),
		510 => Some("minecraft:end_stone_bricks"),
		513 => Some("minecraft:ender_chest"),
		1364 => Some("minecraft:ender_dragon_spawn_egg"),
		1278 => Some("minecraft:ender_eye"),
		1265 => Some("minecraft:ender_pearl"),
		1365 => Some("minecraft:enderman_spawn_egg"),
		1366 => Some("minecraft:endermite_spawn_egg"),
		1349 => Some("minecraft:evoker_spawn_egg"),
		1368 => Some("minecraft:experience_bottle"),
		1605 => Some("minecraft:explorer_pottery_sherd"),
		132 => Some("minecraft:exposed_chiseled_copper"),
		121 => Some("minecraft:exposed_copper"),
		466 => Some("minecraft:exposed_copper_bars"),
		1630 => Some("minecraft:exposed_copper_bulb"),
		475 => Some("minecraft:exposed_copper_chain"),
		1638 => Some("minecraft:exposed_copper_chest"),
		903 => Some("minecraft:exposed_copper_door"),
		1646 => Some("minecraft:exposed_copper_golem_statue"),
		1622 => Some("minecraft:exposed_copper_grate"),
		1518 => Some("minecraft:exposed_copper_lantern"),
		925 => Some("minecraft:exposed_copper_trapdoor"),
		140 => Some("minecraft:exposed_cut_copper"),
		156 => Some("minecraft:exposed_cut_copper_slab"),
		148 => Some("minecraft:exposed_cut_copper_stairs"),
		841 => Some("minecraft:exposed_lightning_rod"),
		1585 => Some("minecraft:eye_armor_trim_smithing_template"),
		406 => Some("minecraft:farmland"),
		1063 => Some("minecraft:feather"),
		1273 => Some("minecraft:fermented_spider_eye"),
		239 => Some("minecraft:fern"),
		1502 => Some("minecraft:field_masoned_banner_pattern"),
		1238 => Some("minecraft:filled_map"),
		1369 => Some("minecraft:fire_charge"),
		769 => Some("minecraft:fire_coral"),
		764 => Some("minecraft:fire_coral_block"),
		779 => Some("minecraft:fire_coral_fan"),
		245 => Some("minecraft:firefly_bush"),
		1393 => Some("minecraft:firework_rocket"),
		1394 => Some("minecraft:firework_star"),
		1186 => Some("minecraft:fishing_rod"),
		1510 => Some("minecraft:fletching_table"),
		1096 => Some("minecraft:flint"),
		1005 => Some("minecraft:flint_and_steel"),
		1596 => Some("minecraft:flow_armor_trim_smithing_template"),
		1500 => Some("minecraft:flow_banner_pattern"),
		1606 => Some("minecraft:flow_pottery_sherd"),
		1494 => Some("minecraft:flower_banner_pattern"),
		1377 => Some("minecraft:flower_pot"),
		243 => Some("minecraft:flowering_azalea"),
		228 => Some("minecraft:flowering_azalea_leaves"),
		1294 => Some("minecraft:fox_spawn_egg"),
		1607 => Some("minecraft:friend_pottery_sherd"),
		1304 => Some("minecraft:frog_spawn_egg"),
		1576 => Some("minecraft:frogspawn"),
		407 => Some("minecraft:furnace"),
		968 => Some("minecraft:furnace_minecart"),
		1355 => Some("minecraft:ghast_spawn_egg"),
		1267 => Some("minecraft:ghast_tear"),
		1540 => Some("minecraft:gilded_blackstone"),
		231 => Some("minecraft:glass"),
		1270 => Some("minecraft:glass_bottle"),
		482 => Some("minecraft:glass_pane"),
		1279 => Some("minecraft:glistering_melon_slice"),
		1498 => Some("minecraft:globe_banner_pattern"),
		1526 => Some("minecraft:glow_berries"),
		1197 => Some("minecraft:glow_ink_sac"),
		1376 => Some("minecraft:glow_item_frame"),
		485 => Some("minecraft:glow_lichen"),
		1305 => Some("minecraft:glow_squid_spawn_egg"),
		441 => Some("minecraft:glowstone"),
		1189 => Some("minecraft:glowstone_dust"),
		1504 => Some("minecraft:goat_horn"),
		1295 => Some("minecraft:goat_spawn_egg"),
		128 => Some("minecraft:gold_block"),
		1022 => Some("minecraft:gold_ingot"),
		1268 => Some("minecraft:gold_nugget"),
		99 => Some("minecraft:gold_ore"),
		1100 => Some("minecraft:golden_apple"),
		1043 => Some("minecraft:golden_axe"),
		1091 => Some("minecraft:golden_boots"),
		1383 => Some("minecraft:golden_carrot"),
		1089 => Some("minecraft:golden_chestplate"),
		299 => Some("minecraft:golden_dandelion"),
		1088 => Some("minecraft:golden_helmet"),
		1044 => Some("minecraft:golden_hoe"),
		1408 => Some("minecraft:golden_horse_armor"),
		1090 => Some("minecraft:golden_leggings"),
		1486 => Some("minecraft:golden_nautilus_armor"),
		1042 => Some("minecraft:golden_pickaxe"),
		1041 => Some("minecraft:golden_shovel"),
		1451 => Some("minecraft:golden_spear"),
		1040 => Some("minecraft:golden_sword"),
		2 => Some("minecraft:granite"),
		814 => Some("minecraft:granite_slab"),
		797 => Some("minecraft:granite_stairs"),
		537 => Some("minecraft:granite_wall"),
		54 => Some("minecraft:grass_block"),
		92 => Some("minecraft:gravel"),
		1424 => Some("minecraft:gray_banner"),
		1226 => Some("minecraft:gray_bed"),
		1161 => Some("minecraft:gray_bundle"),
		1558 => Some("minecraft:gray_candle"),
		587 => Some("minecraft:gray_carpet"),
		696 => Some("minecraft:gray_concrete"),
		744 => Some("minecraft:gray_concrete_powder"),
		728 => Some("minecraft:gray_concrete_slab"),
		712 => Some("minecraft:gray_concrete_stairs"),
		1177 => Some("minecraft:gray_cushion"),
		1206 => Some("minecraft:gray_dye"),
		680 => Some("minecraft:gray_glazed_terracotta"),
		957 => Some("minecraft:gray_harness"),
		664 => Some("minecraft:gray_shulker_box"),
		612 => Some("minecraft:gray_stained_glass"),
		628 => Some("minecraft:gray_stained_glass_pane"),
		568 => Some("minecraft:gray_terracotta"),
		257 => Some("minecraft:gray_wool"),
		289 => Some("minecraft:gray_wool_slab"),
		273 => Some("minecraft:gray_wool_stairs"),
		1430 => Some("minecraft:green_banner"),
		1232 => Some("minecraft:green_bed"),
		1167 => Some("minecraft:green_bundle"),
		1564 => Some("minecraft:green_candle"),
		593 => Some("minecraft:green_carpet"),
		702 => Some("minecraft:green_concrete"),
		750 => Some("minecraft:green_concrete_powder"),
		734 => Some("minecraft:green_concrete_slab"),
		718 => Some("minecraft:green_concrete_stairs"),
		1183 => Some("minecraft:green_cushion"),
		1212 => Some("minecraft:green_dye"),
		686 => Some("minecraft:green_glazed_terracotta"),
		963 => Some("minecraft:green_harness"),
		670 => Some("minecraft:green_shulker_box"),
		618 => Some("minecraft:green_stained_glass"),
		634 => Some("minecraft:green_stained_glass_pane"),
		574 => Some("minecraft:green_terracotta"),
		263 => Some("minecraft:green_wool"),
		295 => Some("minecraft:green_wool_slab"),
		279 => Some("minecraft:green_wool_stairs"),
		1511 => Some("minecraft:grindstone"),
		1343 => Some("minecraft:guardian_spawn_egg"),
		1064 => Some("minecraft:gunpowder"),
		1501 => Some("minecraft:guster_banner_pattern"),
		1608 => Some("minecraft:guster_pottery_sherd"),
		337 => Some("minecraft:hanging_roots"),
		1356 => Some("minecraft:happy_ghast_spawn_egg"),
		579 => Some("minecraft:hay_block"),
		1490 => Some("minecraft:heart_of_the_sea"),
		1609 => Some("minecraft:heart_pottery_sherd"),
		1610 => Some("minecraft:heartbreak_pottery_sherd"),
		116 => Some("minecraft:heavy_core"),
		874 => Some("minecraft:heavy_weighted_pressure_plate"),
		1357 => Some("minecraft:hoglin_spawn_egg"),
		832 => Some("minecraft:honey_block"),
		1533 => Some("minecraft:honey_bottle"),
		1530 => Some("minecraft:honeycomb"),
		1534 => Some("minecraft:honeycomb_block"),
		834 => Some("minecraft:hopper"),
		970 => Some("minecraft:hopper_minecart"),
		770 => Some("minecraft:horn_coral"),
		765 => Some("minecraft:horn_coral_block"),
		780 => Some("minecraft:horn_coral_fan"),
		1286 => Some("minecraft:horse_spawn_egg"),
		1595 => Some("minecraft:host_armor_trim_smithing_template"),
		1611 => Some("minecraft:howl_pottery_sherd"),
		1326 => Some("minecraft:husk_spawn_egg"),
		411 => Some("minecraft:ice"),
		447 => Some("minecraft:infested_chiseled_stone_bricks"),
		443 => Some("minecraft:infested_cobblestone"),
		446 => Some("minecraft:infested_cracked_stone_bricks"),
		448 => Some("minecraft:infested_deepslate"),
		445 => Some("minecraft:infested_mossy_stone_bricks"),
		442 => Some("minecraft:infested_stone"),
		444 => Some("minecraft:infested_stone_bricks"),
		1196 => Some("minecraft:ink_sac"),
		1048 => Some("minecraft:iron_axe"),
		464 => Some("minecraft:iron_bars"),
		119 => Some("minecraft:iron_block"),
		1083 => Some("minecraft:iron_boots"),
		473 => Some("minecraft:iron_chain"),
		1081 => Some("minecraft:iron_chestplate"),
		888 => Some("minecraft:iron_door"),
		1318 => Some("minecraft:iron_golem_spawn_egg"),
		1080 => Some("minecraft:iron_helmet"),
		1049 => Some("minecraft:iron_hoe"),
		1407 => Some("minecraft:iron_horse_armor"),
		1018 => Some("minecraft:iron_ingot"),
		1082 => Some("minecraft:iron_leggings"),
		1485 => Some("minecraft:iron_nautilus_armor"),
		1456 => Some("minecraft:iron_nugget"),
		95 => Some("minecraft:iron_ore"),
		1047 => Some("minecraft:iron_pickaxe"),
		1046 => Some("minecraft:iron_shovel"),
		1450 => Some("minecraft:iron_spear"),
		1045 => Some("minecraft:iron_sword"),
		910 => Some("minecraft:iron_trapdoor"),
		1375 => Some("minecraft:item_frame"),
		432 => Some("minecraft:jack_o_lantern"),
		998 => Some("minecraft:jigsaw"),
		416 => Some("minecraft:jukebox"),
		981 => Some("minecraft:jungle_boat"),
		861 => Some("minecraft:jungle_button"),
		982 => Some("minecraft:jungle_chest_boat"),
		892 => Some("minecraft:jungle_door"),
		420 => Some("minecraft:jungle_fence"),
		935 => Some("minecraft:jungle_fence_gate"),
		1118 => Some("minecraft:jungle_hanging_sign"),
		218 => Some("minecraft:jungle_leaves"),
		166 => Some("minecraft:jungle_log"),
		66 => Some("minecraft:jungle_planks"),
		878 => Some("minecraft:jungle_pressure_plate"),
		1242 => Some("minecraft:jungle_pyramid_map"),
		80 => Some("minecraft:jungle_sapling"),
		383 => Some("minecraft:jungle_shelf"),
		1105 => Some("minecraft:jungle_sign"),
		344 => Some("minecraft:jungle_slab"),
		518 => Some("minecraft:jungle_stairs"),
		914 => Some("minecraft:jungle_trapdoor"),
		206 => Some("minecraft:jungle_wood"),
		328 => Some("minecraft:kelp"),
		1458 => Some("minecraft:knowledge_book"),
		408 => Some("minecraft:ladder"),
		1515 => Some("minecraft:lantern"),
		233 => Some("minecraft:lapis_block"),
		1014 => Some("minecraft:lapis_lazuli"),
		105 => Some("minecraft:lapis_ore"),
		1569 => Some("minecraft:large_amethyst_bud"),
		604 => Some("minecraft:large_fern"),
		1130 => Some("minecraft:lava_bucket"),
		1412 => Some("minecraft:lead"),
		331 => Some("minecraft:leaf_litter"),
		1133 => Some("minecraft:leather"),
		1071 => Some("minecraft:leather_boots"),
		1069 => Some("minecraft:leather_chestplate"),
		1068 => Some("minecraft:leather_helmet"),
		1411 => Some("minecraft:leather_horse_armor"),
		1070 => Some("minecraft:leather_leggings"),
		837 => Some("minecraft:lectern"),
		839 => Some("minecraft:lever"),
		578 => Some("minecraft:light"),
		1420 => Some("minecraft:light_blue_banner"),
		1222 => Some("minecraft:light_blue_bed"),
		1157 => Some("minecraft:light_blue_bundle"),
		1554 => Some("minecraft:light_blue_candle"),
		583 => Some("minecraft:light_blue_carpet"),
		692 => Some("minecraft:light_blue_concrete"),
		740 => Some("minecraft:light_blue_concrete_powder"),
		724 => Some("minecraft:light_blue_concrete_slab"),
		708 => Some("minecraft:light_blue_concrete_stairs"),
		1173 => Some("minecraft:light_blue_cushion"),
		1202 => Some("minecraft:light_blue_dye"),
		676 => Some("minecraft:light_blue_glazed_terracotta"),
		953 => Some("minecraft:light_blue_harness"),
		660 => Some("minecraft:light_blue_shulker_box"),
		608 => Some("minecraft:light_blue_stained_glass"),
		624 => Some("minecraft:light_blue_stained_glass_pane"),
		564 => Some("minecraft:light_blue_terracotta"),
		253 => Some("minecraft:light_blue_wool"),
		285 => Some("minecraft:light_blue_wool_slab"),
		269 => Some("minecraft:light_blue_wool_stairs"),
		1425 => Some("minecraft:light_gray_banner"),
		1227 => Some("minecraft:light_gray_bed"),
		1162 => Some("minecraft:light_gray_bundle"),
		1559 => Some("minecraft:light_gray_candle"),
		588 => Some("minecraft:light_gray_carpet"),
		697 => Some("minecraft:light_gray_concrete"),
		745 => Some("minecraft:light_gray_concrete_powder"),
		729 => Some("minecraft:light_gray_concrete_slab"),
		713 => Some("minecraft:light_gray_concrete_stairs"),
		1178 => Some("minecraft:light_gray_cushion"),
		1207 => Some("minecraft:light_gray_dye"),
		681 => Some("minecraft:light_gray_glazed_terracotta"),
		958 => Some("minecraft:light_gray_harness"),
		665 => Some("minecraft:light_gray_shulker_box"),
		613 => Some("minecraft:light_gray_stained_glass"),
		629 => Some("minecraft:light_gray_stained_glass_pane"),
		569 => Some("minecraft:light_gray_terracotta"),
		258 => Some("minecraft:light_gray_wool"),
		290 => Some("minecraft:light_gray_wool_slab"),
		274 => Some("minecraft:light_gray_wool_stairs"),
		873 => Some("minecraft:light_weighted_pressure_plate"),
		840 => Some("minecraft:lightning_rod"),
		600 => Some("minecraft:lilac"),
		312 => Some("minecraft:lily_of_the_valley"),
		497 => Some("minecraft:lily_pad"),
		1422 => Some("minecraft:lime_banner"),
		1224 => Some("minecraft:lime_bed"),
		1159 => Some("minecraft:lime_bundle"),
		1556 => Some("minecraft:lime_candle"),
		585 => Some("minecraft:lime_carpet"),
		694 => Some("minecraft:lime_concrete"),
		742 => Some("minecraft:lime_concrete_powder"),
		726 => Some("minecraft:lime_concrete_slab"),
		710 => Some("minecraft:lime_concrete_stairs"),
		1175 => Some("minecraft:lime_cushion"),
		1204 => Some("minecraft:lime_dye"),
		678 => Some("minecraft:lime_glazed_terracotta"),
		955 => Some("minecraft:lime_harness"),
		662 => Some("minecraft:lime_shulker_box"),
		610 => Some("minecraft:lime_stained_glass"),
		626 => Some("minecraft:lime_stained_glass_pane"),
		566 => Some("minecraft:lime_terracotta"),
		255 => Some("minecraft:lime_wool"),
		287 => Some("minecraft:lime_wool_slab"),
		271 => Some("minecraft:lime_wool_stairs"),
		1445 => Some("minecraft:lingering_potion"),
		1296 => Some("minecraft:llama_spawn_egg"),
		1535 => Some("minecraft:lodestone"),
		1493 => Some("minecraft:loom"),
		1374 => Some("minecraft:mace"),
		1419 => Some("minecraft:magenta_banner"),
		1221 => Some("minecraft:magenta_bed"),
		1156 => Some("minecraft:magenta_bundle"),
		1553 => Some("minecraft:magenta_candle"),
		582 => Some("minecraft:magenta_carpet"),
		691 => Some("minecraft:magenta_concrete"),
		739 => Some("minecraft:magenta_concrete_powder"),
		723 => Some("minecraft:magenta_concrete_slab"),
		707 => Some("minecraft:magenta_concrete_stairs"),
		1172 => Some("minecraft:magenta_cushion"),
		1201 => Some("minecraft:magenta_dye"),
		675 => Some("minecraft:magenta_glazed_terracotta"),
		952 => Some("minecraft:magenta_harness"),
		659 => Some("minecraft:magenta_shulker_box"),
		607 => Some("minecraft:magenta_stained_glass"),
		623 => Some("minecraft:magenta_stained_glass_pane"),
		563 => Some("minecraft:magenta_terracotta"),
		252 => Some("minecraft:magenta_wool"),
		284 => Some("minecraft:magenta_wool_slab"),
		268 => Some("minecraft:magenta_wool_stairs"),
		650 => Some("minecraft:magma_block"),
		1275 => Some("minecraft:magma_cream"),
		1358 => Some("minecraft:magma_cube_spawn_egg"),
		991 => Some("minecraft:mangrove_boat"),
		866 => Some("minecraft:mangrove_button"),
		992 => Some("minecraft:mangrove_chest_boat"),
		897 => Some("minecraft:mangrove_door"),
		425 => Some("minecraft:mangrove_fence"),
		940 => Some("minecraft:mangrove_fence_gate"),
		1123 => Some("minecraft:mangrove_hanging_sign"),
		223 => Some("minecraft:mangrove_leaves"),
		171 => Some("minecraft:mangrove_log"),
		71 => Some("minecraft:mangrove_planks"),
		883 => Some("minecraft:mangrove_pressure_plate"),
		85 => Some("minecraft:mangrove_propagule"),
		173 => Some("minecraft:mangrove_roots"),
		384 => Some("minecraft:mangrove_shelf"),
		1110 => Some("minecraft:mangrove_sign"),
		349 => Some("minecraft:mangrove_slab"),
		523 => Some("minecraft:mangrove_stairs"),
		919 => Some("minecraft:mangrove_trapdoor"),
		211 => Some("minecraft:mangrove_wood"),
		1382 => Some("minecraft:map"),
		1568 => Some("minecraft:medium_amethyst_bud"),
		483 => Some("minecraft:melon"),
		1259 => Some("minecraft:melon_seeds"),
		1256 => Some("minecraft:melon_slice"),
		1134 => Some("minecraft:milk_bucket"),
		966 => Some("minecraft:minecart"),
		1612 => Some("minecraft:miner_pottery_sherd"),
		1497 => Some("minecraft:mojang_banner_pattern"),
		1314 => Some("minecraft:mooshroom_spawn_egg"),
		333 => Some("minecraft:moss_block"),
		332 => Some("minecraft:moss_carpet"),
		393 => Some("minecraft:mossy_cobblestone"),
		810 => Some("minecraft:mossy_cobblestone_slab"),
		792 => Some("minecraft:mossy_cobblestone_stairs"),
		532 => Some("minecraft:mossy_cobblestone_wall"),
		808 => Some("minecraft:mossy_stone_brick_slab"),
		790 => Some("minecraft:mossy_stone_brick_stairs"),
		536 => Some("minecraft:mossy_stone_brick_wall"),
		450 => Some("minecraft:mossy_stone_bricks"),
		1613 => Some("minecraft:mourner_pottery_sherd"),
		59 => Some("minecraft:mud"),
		363 => Some("minecraft:mud_brick_slab"),
		495 => Some("minecraft:mud_brick_stairs"),
		539 => Some("minecraft:mud_brick_wall"),
		454 => Some("minecraft:mud_bricks"),
		174 => Some("minecraft:muddy_mangrove_roots"),
		1287 => Some("minecraft:mule_spawn_egg"),
		463 => Some("minecraft:mushroom_stem"),
		1061 => Some("minecraft:mushroom_stew"),
		1474 => Some("minecraft:music_disc_11"),
		1460 => Some("minecraft:music_disc_13"),
		1478 => Some("minecraft:music_disc_5"),
		1462 => Some("minecraft:music_disc_blocks"),
		1463 => Some("minecraft:music_disc_bounce"),
		1461 => Some("minecraft:music_disc_cat"),
		1464 => Some("minecraft:music_disc_chirp"),
		1465 => Some("minecraft:music_disc_creator"),
		1466 => Some("minecraft:music_disc_creator_music_box"),
		1467 => Some("minecraft:music_disc_far"),
		1468 => Some("minecraft:music_disc_lava_chicken"),
		1469 => Some("minecraft:music_disc_mall"),
		1470 => Some("minecraft:music_disc_mellohi"),
		1476 => Some("minecraft:music_disc_otherside"),
		1479 => Some("minecraft:music_disc_pigstep"),
		1480 => Some("minecraft:music_disc_precipice"),
		1477 => Some("minecraft:music_disc_relic"),
		1471 => Some("minecraft:music_disc_stal"),
		1472 => Some("minecraft:music_disc_strad"),
		1481 => Some("minecraft:music_disc_tears"),
		1475 => Some("minecraft:music_disc_wait"),
		1473 => Some("minecraft:music_disc_ward"),
		1415 => Some("minecraft:mutton"),
		496 => Some("minecraft:mycelium"),
		1413 => Some("minecraft:name_tag"),
		1484 => Some("minecraft:nautilus_shell"),
		1306 => Some("minecraft:nautilus_spawn_egg"),
		1396 => Some("minecraft:nether_brick"),
		501 => Some("minecraft:nether_brick_fence"),
		364 => Some("minecraft:nether_brick_slab"),
		502 => Some("minecraft:nether_brick_stairs"),
		540 => Some("minecraft:nether_brick_wall"),
		498 => Some("minecraft:nether_bricks"),
		109 => Some("minecraft:nether_gold_ore"),
		110 => Some("minecraft:nether_quartz_ore"),
		324 => Some("minecraft:nether_sprouts"),
		1391 => Some("minecraft:nether_star"),
		1269 => Some("minecraft:nether_wart"),
		651 => Some("minecraft:nether_wart_block"),
		1058 => Some("minecraft:netherite_axe"),
		130 => Some("minecraft:netherite_block"),
		1095 => Some("minecraft:netherite_boots"),
		1093 => Some("minecraft:netherite_chestplate"),
		1092 => Some("minecraft:netherite_helmet"),
		1059 => Some("minecraft:netherite_hoe"),
		1410 => Some("minecraft:netherite_horse_armor"),
		1023 => Some("minecraft:netherite_ingot"),
		1094 => Some("minecraft:netherite_leggings"),
		1488 => Some("minecraft:netherite_nautilus_armor"),
		1057 => Some("minecraft:netherite_pickaxe"),
		1024 => Some("minecraft:netherite_scrap"),
		1056 => Some("minecraft:netherite_shovel"),
		1453 => Some("minecraft:netherite_spear"),
		1055 => Some("minecraft:netherite_sword"),
		1579 => Some("minecraft:netherite_upgrade_smithing_template"),
		433 => Some("minecraft:netherrack"),
		855 => Some("minecraft:note_block"),
		975 => Some("minecraft:oak_boat"),
		858 => Some("minecraft:oak_button"),
		976 => Some("minecraft:oak_chest_boat"),
		889 => Some("minecraft:oak_door"),
		417 => Some("minecraft:oak_fence"),
		932 => Some("minecraft:oak_fence_gate"),
		1115 => Some("minecraft:oak_hanging_sign"),
		215 => Some("minecraft:oak_leaves"),
		163 => Some("minecraft:oak_log"),
		63 => Some("minecraft:oak_planks"),
		875 => Some("minecraft:oak_pressure_plate"),
		77 => Some("minecraft:oak_sapling"),
		386 => Some("minecraft:oak_shelf"),
		1102 => Some("minecraft:oak_sign"),
		341 => Some("minecraft:oak_slab"),
		515 => Some("minecraft:oak_stairs"),
		911 => Some("minecraft:oak_trapdoor"),
		203 => Some("minecraft:oak_wood"),
		833 => Some("minecraft:observer"),
		394 => Some("minecraft:obsidian"),
		1239 => Some("minecraft:ocean_monument_map"),
		1297 => Some("minecraft:ocelot_spawn_egg"),
		1573 => Some("minecraft:ochre_froglight"),
		1657 => Some("minecraft:ominous_bottle"),
		1655 => Some("minecraft:ominous_trial_key"),
		300 => Some("minecraft:open_eyeblossom"),
		1418 => Some("minecraft:orange_banner"),
		1220 => Some("minecraft:orange_bed"),
		1155 => Some("minecraft:orange_bundle"),
		1552 => Some("minecraft:orange_candle"),
		581 => Some("minecraft:orange_carpet"),
		690 => Some("minecraft:orange_concrete"),
		738 => Some("minecraft:orange_concrete_powder"),
		722 => Some("minecraft:orange_concrete_slab"),
		706 => Some("minecraft:orange_concrete_stairs"),
		1171 => Some("minecraft:orange_cushion"),
		1200 => Some("minecraft:orange_dye"),
		674 => Some("minecraft:orange_glazed_terracotta"),
		951 => Some("minecraft:orange_harness"),
		225 => Some("minecraft:orange_poplar_leaves"),
		658 => Some("minecraft:orange_shulker_box"),
		606 => Some("minecraft:orange_stained_glass"),
		622 => Some("minecraft:orange_stained_glass_pane"),
		562 => Some("minecraft:orange_terracotta"),
		307 => Some("minecraft:orange_tulip"),
		251 => Some("minecraft:orange_wool"),
		283 => Some("minecraft:orange_wool_slab"),
		267 => Some("minecraft:orange_wool_stairs"),
		310 => Some("minecraft:oxeye_daisy"),
		134 => Some("minecraft:oxidized_chiseled_copper"),
		123 => Some("minecraft:oxidized_copper"),
		468 => Some("minecraft:oxidized_copper_bars"),
		1632 => Some("minecraft:oxidized_copper_bulb"),
		477 => Some("minecraft:oxidized_copper_chain"),
		1640 => Some("minecraft:oxidized_copper_chest"),
		905 => Some("minecraft:oxidized_copper_door"),
		1648 => Some("minecraft:oxidized_copper_golem_statue"),
		1624 => Some("minecraft:oxidized_copper_grate"),
		1520 => Some("minecraft:oxidized_copper_lantern"),
		927 => Some("minecraft:oxidized_copper_trapdoor"),
		142 => Some("minecraft:oxidized_cut_copper"),
		158 => Some("minecraft:oxidized_cut_copper_slab"),
		150 => Some("minecraft:oxidized_cut_copper_stairs"),
		843 => Some("minecraft:oxidized_lightning_rod"),
		597 => Some("minecraft:packed_ice"),
		453 => Some("minecraft:packed_mud"),
		1099 => Some("minecraft:painting"),
		335 => Some("minecraft:pale_hanging_moss"),
		336 => Some("minecraft:pale_moss_block"),
		334 => Some("minecraft:pale_moss_carpet"),
		989 => Some("minecraft:pale_oak_boat"),
		865 => Some("minecraft:pale_oak_button"),
		990 => Some("minecraft:pale_oak_chest_boat"),
		896 => Some("minecraft:pale_oak_door"),
		424 => Some("minecraft:pale_oak_fence"),
		939 => Some("minecraft:pale_oak_fence_gate"),
		1122 => Some("minecraft:pale_oak_hanging_sign"),
		222 => Some("minecraft:pale_oak_leaves"),
		169 => Some("minecraft:pale_oak_log"),
		70 => Some("minecraft:pale_oak_planks"),
		882 => Some("minecraft:pale_oak_pressure_plate"),
		84 => Some("minecraft:pale_oak_sapling"),
		387 => Some("minecraft:pale_oak_shelf"),
		1109 => Some("minecraft:pale_oak_sign"),
		348 => Some("minecraft:pale_oak_slab"),
		522 => Some("minecraft:pale_oak_stairs"),
		918 => Some("minecraft:pale_oak_trapdoor"),
		209 => Some("minecraft:pale_oak_wood"),
		1298 => Some("minecraft:panda_spawn_egg"),
		1145 => Some("minecraft:paper"),
		1327 => Some("minecraft:parched_spawn_egg"),
		1289 => Some("minecraft:parrot_spawn_egg"),
		1575 => Some("minecraft:pearlescent_froglight"),
		602 => Some("minecraft:peony"),
		359 => Some("minecraft:petrified_oak_slab"),
		973 => Some("minecraft:phantom_membrane"),
		1344 => Some("minecraft:phantom_spawn_egg"),
		1282 => Some("minecraft:pig_spawn_egg"),
		1499 => Some("minecraft:piglin_banner_pattern"),
		1360 => Some("minecraft:piglin_brute_spawn_egg"),
		1390 => Some("minecraft:piglin_head"),
		1359 => Some("minecraft:piglin_spawn_egg"),
		1350 => Some("minecraft:pillager_spawn_egg"),
		1423 => Some("minecraft:pink_banner"),
		1225 => Some("minecraft:pink_bed"),
		1160 => Some("minecraft:pink_bundle"),
		1557 => Some("minecraft:pink_candle"),
		586 => Some("minecraft:pink_carpet"),
		695 => Some("minecraft:pink_concrete"),
		743 => Some("minecraft:pink_concrete_powder"),
		727 => Some("minecraft:pink_concrete_slab"),
		711 => Some("minecraft:pink_concrete_stairs"),
		1176 => Some("minecraft:pink_cushion"),
		1205 => Some("minecraft:pink_dye"),
		679 => Some("minecraft:pink_glazed_terracotta"),
		956 => Some("minecraft:pink_harness"),
		329 => Some("minecraft:pink_petals"),
		663 => Some("minecraft:pink_shulker_box"),
		611 => Some("minecraft:pink_stained_glass"),
		627 => Some("minecraft:pink_stained_glass_pane"),
		567 => Some("minecraft:pink_terracotta"),
		309 => Some("minecraft:pink_tulip"),
		256 => Some("minecraft:pink_wool"),
		288 => Some("minecraft:pink_wool_slab"),
		272 => Some("minecraft:pink_wool_stairs"),
		829 => Some("minecraft:piston"),
		315 => Some("minecraft:pitcher_plant"),
		1437 => Some("minecraft:pitcher_pod"),
		1245 => Some("minecraft:plains_village_map"),
		1386 => Some("minecraft:player_head"),
		1614 => Some("minecraft:plenty_pottery_sherd"),
		57 => Some("minecraft:podzol"),
		1571 => Some("minecraft:pointed_dripstone"),
		1381 => Some("minecraft:poisonous_potato"),
		1299 => Some("minecraft:polar_bear_spawn_egg"),
		7 => Some("minecraft:polished_andesite"),
		817 => Some("minecraft:polished_andesite_slab"),
		800 => Some("minecraft:polished_andesite_stairs"),
		437 => Some("minecraft:polished_basalt"),
		1541 => Some("minecraft:polished_blackstone"),
		1546 => Some("minecraft:polished_blackstone_brick_slab"),
		1547 => Some("minecraft:polished_blackstone_brick_stairs"),
		548 => Some("minecraft:polished_blackstone_brick_wall"),
		1545 => Some("minecraft:polished_blackstone_bricks"),
		857 => Some("minecraft:polished_blackstone_button"),
		872 => Some("minecraft:polished_blackstone_pressure_plate"),
		1542 => Some("minecraft:polished_blackstone_slab"),
		1543 => Some("minecraft:polished_blackstone_stairs"),
		547 => Some("minecraft:polished_blackstone_wall"),
		44 => Some("minecraft:polished_cinnabar"),
		45 => Some("minecraft:polished_cinnabar_slab"),
		46 => Some("minecraft:polished_cinnabar_stairs"),
		47 => Some("minecraft:polished_cinnabar_wall"),
		10 => Some("minecraft:polished_deepslate"),
		820 => Some("minecraft:polished_deepslate_slab"),
		803 => Some("minecraft:polished_deepslate_stairs"),
		550 => Some("minecraft:polished_deepslate_wall"),
		5 => Some("minecraft:polished_diorite"),
		809 => Some("minecraft:polished_diorite_slab"),
		791 => Some("minecraft:polished_diorite_stairs"),
		3 => Some("minecraft:polished_granite"),
		806 => Some("minecraft:polished_granite_slab"),
		788 => Some("minecraft:polished_granite_stairs"),
		31 => Some("minecraft:polished_sulfur"),
		32 => Some("minecraft:polished_sulfur_slab"),
		33 => Some("minecraft:polished_sulfur_stairs"),
		34 => Some("minecraft:polished_sulfur_wall"),
		17 => Some("minecraft:polished_tuff"),
		18 => Some("minecraft:polished_tuff_slab"),
		19 => Some("minecraft:polished_tuff_stairs"),
		20 => Some("minecraft:polished_tuff_wall"),
		993 => Some("minecraft:poplar_boat"),
		867 => Some("minecraft:poplar_button"),
		994 => Some("minecraft:poplar_chest_boat"),
		898 => Some("minecraft:poplar_door"),
		426 => Some("minecraft:poplar_fence"),
		941 => Some("minecraft:poplar_fence_gate"),
		1124 => Some("minecraft:poplar_hanging_sign"),
		172 => Some("minecraft:poplar_log"),
		72 => Some("minecraft:poplar_planks"),
		884 => Some("minecraft:poplar_pressure_plate"),
		86 => Some("minecraft:poplar_sapling"),
		385 => Some("minecraft:poplar_shelf"),
		1111 => Some("minecraft:poplar_sign"),
		350 => Some("minecraft:poplar_slab"),
		524 => Some("minecraft:poplar_stairs"),
		920 => Some("minecraft:poplar_trapdoor"),
		212 => Some("minecraft:poplar_wood"),
		1435 => Some("minecraft:popped_chorus_fruit"),
		302 => Some("minecraft:poppy"),
		1097 => Some("minecraft:porkchop"),
		1379 => Some("minecraft:potato"),
		27 => Some("minecraft:potent_sulfur"),
		1271 => Some("minecraft:potion"),
		1131 => Some("minecraft:powder_snow_bucket"),
		945 => Some("minecraft:powered_rail"),
		637 => Some("minecraft:prismarine"),
		370 => Some("minecraft:prismarine_brick_slab"),
		641 => Some("minecraft:prismarine_brick_stairs"),
		638 => Some("minecraft:prismarine_bricks"),
		1399 => Some("minecraft:prismarine_crystals"),
		1398 => Some("minecraft:prismarine_shard"),
		369 => Some("minecraft:prismarine_slab"),
		640 => Some("minecraft:prismarine_stairs"),
		534 => Some("minecraft:prismarine_wall"),
		1615 => Some("minecraft:prize_pottery_sherd"),
		1193 => Some("minecraft:pufferfish"),
		1135 => Some("minecraft:pufferfish_bucket"),
		1307 => Some("minecraft:pufferfish_spawn_egg"),
		430 => Some("minecraft:pumpkin"),
		1392 => Some("minecraft:pumpkin_pie"),
		1258 => Some("minecraft:pumpkin_seeds"),
		1427 => Some("minecraft:purple_banner"),
		1229 => Some("minecraft:purple_bed"),
		1164 => Some("minecraft:purple_bundle"),
		1561 => Some("minecraft:purple_candle"),
		590 => Some("minecraft:purple_carpet"),
		699 => Some("minecraft:purple_concrete"),
		747 => Some("minecraft:purple_concrete_powder"),
		731 => Some("minecraft:purple_concrete_slab"),
		715 => Some("minecraft:purple_concrete_stairs"),
		1180 => Some("minecraft:purple_cushion"),
		1209 => Some("minecraft:purple_dye"),
		683 => Some("minecraft:purple_glazed_terracotta"),
		960 => Some("minecraft:purple_harness"),
		667 => Some("minecraft:purple_shulker_box"),
		615 => Some("minecraft:purple_stained_glass"),
		631 => Some("minecraft:purple_stained_glass_pane"),
		571 => Some("minecraft:purple_terracotta"),
		260 => Some("minecraft:purple_wool"),
		292 => Some("minecraft:purple_wool_slab"),
		276 => Some("minecraft:purple_wool_stairs"),
		399 => Some("minecraft:purpur_block"),
		400 => Some("minecraft:purpur_pillar"),
		368 => Some("minecraft:purpur_slab"),
		401 => Some("minecraft:purpur_stairs"),
		1015 => Some("minecraft:quartz"),
		557 => Some("minecraft:quartz_block"),
		558 => Some("minecraft:quartz_bricks"),
		559 => Some("minecraft:quartz_pillar"),
		365 => Some("minecraft:quartz_slab"),
		560 => Some("minecraft:quartz_stairs"),
		1400 => Some("minecraft:rabbit"),
		1403 => Some("minecraft:rabbit_foot"),
		1404 => Some("minecraft:rabbit_hide"),
		1300 => Some("minecraft:rabbit_spawn_egg"),
		1402 => Some("minecraft:rabbit_stew"),
		947 => Some("minecraft:rail"),
		1594 => Some("minecraft:raiser_armor_trim_smithing_template"),
		1351 => Some("minecraft:ravager_spawn_egg"),
		1019 => Some("minecraft:raw_copper"),
		114 => Some("minecraft:raw_copper_block"),
		1021 => Some("minecraft:raw_gold"),
		115 => Some("minecraft:raw_gold_block"),
		1017 => Some("minecraft:raw_iron"),
		113 => Some("minecraft:raw_iron_block"),
		1152 => Some("minecraft:recovery_compass"),
		1431 => Some("minecraft:red_banner"),
		1233 => Some("minecraft:red_bed"),
		1168 => Some("minecraft:red_bundle"),
		1565 => Some("minecraft:red_candle"),
		594 => Some("minecraft:red_carpet"),
		703 => Some("minecraft:red_concrete"),
		751 => Some("minecraft:red_concrete_powder"),
		735 => Some("minecraft:red_concrete_slab"),
		719 => Some("minecraft:red_concrete_stairs"),
		1184 => Some("minecraft:red_cushion"),
		1213 => Some("minecraft:red_dye"),
		687 => Some("minecraft:red_glazed_terracotta"),
		964 => Some("minecraft:red_harness"),
		318 => Some("minecraft:red_mushroom"),
		462 => Some("minecraft:red_mushroom_block"),
		816 => Some("minecraft:red_nether_brick_slab"),
		799 => Some("minecraft:red_nether_brick_stairs"),
		542 => Some("minecraft:red_nether_brick_wall"),
		653 => Some("minecraft:red_nether_bricks"),
		224 => Some("minecraft:red_poplar_leaves"),
		91 => Some("minecraft:red_sand"),
		644 => Some("minecraft:red_sandstone"),
		366 => Some("minecraft:red_sandstone_slab"),
		647 => Some("minecraft:red_sandstone_stairs"),
		535 => Some("minecraft:red_sandstone_wall"),
		241 => Some("minecraft:red_shrub"),
		671 => Some("minecraft:red_shulker_box"),
		619 => Some("minecraft:red_stained_glass"),
		635 => Some("minecraft:red_stained_glass_pane"),
		575 => Some("minecraft:red_terracotta"),
		306 => Some("minecraft:red_tulip"),
		264 => Some("minecraft:red_wool"),
		296 => Some("minecraft:red_wool_slab"),
		280 => Some("minecraft:red_wool_stairs"),
		824 => Some("minecraft:redstone"),
		826 => Some("minecraft:redstone_block"),
		854 => Some("minecraft:redstone_lamp"),
		101 => Some("minecraft:redstone_ore"),
		825 => Some("minecraft:redstone_torch"),
		460 => Some("minecraft:reinforced_deepslate"),
		827 => Some("minecraft:repeater"),
		648 => Some("minecraft:repeating_command_block"),
		487 => Some("minecraft:resin_block"),
		1397 => Some("minecraft:resin_brick"),
		490 => Some("minecraft:resin_brick_slab"),
		489 => Some("minecraft:resin_brick_stairs"),
		491 => Some("minecraft:resin_brick_wall"),
		488 => Some("minecraft:resin_bricks"),
		486 => Some("minecraft:resin_clump"),
		1549 => Some("minecraft:respawn_anchor"),
		1589 => Some("minecraft:rib_armor_trim_smithing_template"),
		58 => Some("minecraft:rooted_dirt"),
		601 => Some("minecraft:rose_bush"),
		1264 => Some("minecraft:rotten_flesh"),
		949 => Some("minecraft:saddle"),
		1191 => Some("minecraft:salmon"),
		1136 => Some("minecraft:salmon_bucket"),
		1308 => Some("minecraft:salmon_spawn_egg"),
		88 => Some("minecraft:sand"),
		234 => Some("minecraft:sandstone"),
		357 => Some("minecraft:sandstone_slab"),
		512 => Some("minecraft:sandstone_stairs"),
		543 => Some("minecraft:sandstone_wall"),
		1246 => Some("minecraft:savanna_village_map"),
		823 => Some("minecraft:scaffolding"),
		1616 => Some("minecraft:scrape_pottery_sherd"),
		503 => Some("minecraft:sculk"),
		505 => Some("minecraft:sculk_catalyst"),
		849 => Some("minecraft:sculk_sensor"),
		506 => Some("minecraft:sculk_shrieker"),
		504 => Some("minecraft:sculk_vein"),
		643 => Some("minecraft:sea_lantern"),
		249 => Some("minecraft:sea_pickle"),
		248 => Some("minecraft:seagrass"),
		1580 => Some("minecraft:sentry_armor_trim_smithing_template"),
		1592 => Some("minecraft:shaper_armor_trim_smithing_template"),
		1617 => Some("minecraft:sheaf_pottery_sherd"),
		1255 => Some("minecraft:shears"),
		1283 => Some("minecraft:sheep_spawn_egg"),
		319 => Some("minecraft:shelf_mushroom"),
		1618 => Some("minecraft:shelter_pottery_sherd"),
		1446 => Some("minecraft:shield"),
		246 => Some("minecraft:short_dry_grass"),
		238 => Some("minecraft:short_grass"),
		1529 => Some("minecraft:shroomlight"),
		656 => Some("minecraft:shulker_box"),
		1455 => Some("minecraft:shulker_shell"),
		1367 => Some("minecraft:shulker_spawn_egg"),
		1593 => Some("minecraft:silence_armor_trim_smithing_template"),
		1345 => Some("minecraft:silverfish_spawn_egg"),
		1329 => Some("minecraft:skeleton_horse_spawn_egg"),
		1384 => Some("minecraft:skeleton_skull"),
		1328 => Some("minecraft:skeleton_spawn_egg"),
		1496 => Some("minecraft:skull_banner_pattern"),
		1619 => Some("minecraft:skull_pottery_sherd"),
		1147 => Some("minecraft:slime_ball"),
		831 => Some("minecraft:slime_block"),
		1346 => Some("minecraft:slime_spawn_egg"),
		1567 => Some("minecraft:small_amethyst_bud"),
		339 => Some("minecraft:small_dripleaf"),
		1512 => Some("minecraft:smithing_table"),
		1507 => Some("minecraft:smoker"),
		438 => Some("minecraft:smooth_basalt"),
		372 => Some("minecraft:smooth_quartz"),
		813 => Some("minecraft:smooth_quartz_slab"),
		796 => Some("minecraft:smooth_quartz_stairs"),
		373 => Some("minecraft:smooth_red_sandstone"),
		807 => Some("minecraft:smooth_red_sandstone_slab"),
		789 => Some("minecraft:smooth_red_sandstone_stairs"),
		374 => Some("minecraft:smooth_sandstone"),
		812 => Some("minecraft:smooth_sandstone_slab"),
		795 => Some("minecraft:smooth_sandstone_stairs"),
		375 => Some("minecraft:smooth_stone"),
		356 => Some("minecraft:smooth_stone_slab"),
		754 => Some("minecraft:sniffer_egg"),
		1315 => Some("minecraft:sniffer_spawn_egg"),
		1620 => Some("minecraft:snort_pottery_sherd"),
		1588 => Some("minecraft:snout_armor_trim_smithing_template"),
		410 => Some("minecraft:snow"),
		412 => Some("minecraft:snow_block"),
		1319 => Some("minecraft:snow_golem_spawn_egg"),
		1132 => Some("minecraft:snowball"),
		1247 => Some("minecraft:snowy_village_map"),
		1528 => Some("minecraft:soul_campfire"),
		1516 => Some("minecraft:soul_lantern"),
		434 => Some("minecraft:soul_sand"),
		435 => Some("minecraft:soul_soil"),
		439 => Some("minecraft:soul_torch"),
		402 => Some("minecraft:spawner"),
		1443 => Some("minecraft:spectral_arrow"),
		1272 => Some("minecraft:spider_eye"),
		1338 => Some("minecraft:spider_spawn_egg"),
		1590 => Some("minecraft:spire_armor_trim_smithing_template"),
		1442 => Some("minecraft:splash_potion"),
		229 => Some("minecraft:sponge"),
		316 => Some("minecraft:spore_blossom"),
		977 => Some("minecraft:spruce_boat"),
		859 => Some("minecraft:spruce_button"),
		978 => Some("minecraft:spruce_chest_boat"),
		890 => Some("minecraft:spruce_door"),
		418 => Some("minecraft:spruce_fence"),
		933 => Some("minecraft:spruce_fence_gate"),
		1116 => Some("minecraft:spruce_hanging_sign"),
		216 => Some("minecraft:spruce_leaves"),
		164 => Some("minecraft:spruce_log"),
		64 => Some("minecraft:spruce_planks"),
		876 => Some("minecraft:spruce_pressure_plate"),
		78 => Some("minecraft:spruce_sapling"),
		388 => Some("minecraft:spruce_shelf"),
		1103 => Some("minecraft:spruce_sign"),
		342 => Some("minecraft:spruce_slab"),
		516 => Some("minecraft:spruce_stairs"),
		912 => Some("minecraft:spruce_trapdoor"),
		204 => Some("minecraft:spruce_wood"),
		1188 => Some("minecraft:spyglass"),
		1309 => Some("minecraft:squid_spawn_egg"),
		1060 => Some("minecraft:stick"),
		830 => Some("minecraft:sticky_piston"),
		1 => Some("minecraft:stone"),
		1038 => Some("minecraft:stone_axe"),
		362 => Some("minecraft:stone_brick_slab"),
		494 => Some("minecraft:stone_brick_stairs"),
		538 => Some("minecraft:stone_brick_wall"),
		449 => Some("minecraft:stone_bricks"),
		856 => Some("minecraft:stone_button"),
		1039 => Some("minecraft:stone_hoe"),
		1037 => Some("minecraft:stone_pickaxe"),
		871 => Some("minecraft:stone_pressure_plate"),
		1036 => Some("minecraft:stone_shovel"),
		355 => Some("minecraft:stone_slab"),
		1448 => Some("minecraft:stone_spear"),
		794 => Some("minecraft:stone_stairs"),
		1035 => Some("minecraft:stone_sword"),
		1513 => Some("minecraft:stonecutter"),
		1235 => Some("minecraft:straw_bed"),
		1330 => Some("minecraft:stray_spawn_egg"),
		1361 => Some("minecraft:strider_spawn_egg"),
		1062 => Some("minecraft:string"),
		182 => Some("minecraft:stripped_acacia_log"),
		194 => Some("minecraft:stripped_acacia_wood"),
		202 => Some("minecraft:stripped_bamboo_block"),
		180 => Some("minecraft:stripped_birch_log"),
		192 => Some("minecraft:stripped_birch_wood"),
		183 => Some("minecraft:stripped_cherry_log"),
		195 => Some("minecraft:stripped_cherry_wood"),
		200 => Some("minecraft:stripped_crimson_hyphae"),
		188 => Some("minecraft:stripped_crimson_stem"),
		184 => Some("minecraft:stripped_dark_oak_log"),
		196 => Some("minecraft:stripped_dark_oak_wood"),
		181 => Some("minecraft:stripped_jungle_log"),
		193 => Some("minecraft:stripped_jungle_wood"),
		186 => Some("minecraft:stripped_mangrove_log"),
		198 => Some("minecraft:stripped_mangrove_wood"),
		178 => Some("minecraft:stripped_oak_log"),
		190 => Some("minecraft:stripped_oak_wood"),
		185 => Some("minecraft:stripped_pale_oak_log"),
		197 => Some("minecraft:stripped_pale_oak_wood"),
		187 => Some("minecraft:stripped_poplar_log"),
		199 => Some("minecraft:stripped_poplar_wood"),
		179 => Some("minecraft:stripped_spruce_log"),
		191 => Some("minecraft:stripped_spruce_wood"),
		201 => Some("minecraft:stripped_warped_hyphae"),
		189 => Some("minecraft:stripped_warped_stem"),
		997 => Some("minecraft:structure_block"),
		655 => Some("minecraft:structure_void"),
		1217 => Some("minecraft:sugar"),
		327 => Some("minecraft:sugar_cane"),
		26 => Some("minecraft:sulfur"),
		36 => Some("minecraft:sulfur_brick_slab"),
		37 => Some("minecraft:sulfur_brick_stairs"),
		38 => Some("minecraft:sulfur_brick_wall"),
		35 => Some("minecraft:sulfur_bricks"),
		1140 => Some("minecraft:sulfur_cube_bucket"),
		1316 => Some("minecraft:sulfur_cube_spawn_egg"),
		28 => Some("minecraft:sulfur_slab"),
		1572 => Some("minecraft:sulfur_spike"),
		29 => Some("minecraft:sulfur_stairs"),
		30 => Some("minecraft:sulfur_wall"),
		599 => Some("minecraft:sunflower"),
		90 => Some("minecraft:suspicious_gravel"),
		89 => Some("minecraft:suspicious_sand"),
		1492 => Some("minecraft:suspicious_stew"),
		1243 => Some("minecraft:swamp_hut_map"),
		1525 => Some("minecraft:sweet_berries"),
		1141 => Some("minecraft:tadpole_bucket"),
		1310 => Some("minecraft:tadpole_spawn_egg"),
		1248 => Some("minecraft:taiga_village_map"),
		247 => Some("minecraft:tall_dry_grass"),
		603 => Some("minecraft:tall_grass"),
		838 => Some("minecraft:target"),
		596 => Some("minecraft:terracotta"),
		999 => Some("minecraft:test_block"),
		1000 => Some("minecraft:test_instance_block"),
		1587 => Some("minecraft:tide_armor_trim_smithing_template"),
		232 => Some("minecraft:tinted_glass"),
		1444 => Some("minecraft:tipped_arrow"),
		853 => Some("minecraft:tnt"),
		969 => Some("minecraft:tnt_minecart"),
		395 => Some("minecraft:torch"),
		314 => Some("minecraft:torchflower"),
		1436 => Some("minecraft:torchflower_seeds"),
		1454 => Some("minecraft:totem_of_undying"),
		1320 => Some("minecraft:trader_llama_spawn_egg"),
		852 => Some("minecraft:trapped_chest"),
		1654 => Some("minecraft:trial_key"),
		1653 => Some("minecraft:trial_spawner"),
		1483 => Some("minecraft:trident"),
		851 => Some("minecraft:tripwire_hook"),
		1192 => Some("minecraft:tropical_fish"),
		1138 => Some("minecraft:tropical_fish_bucket"),
		1311 => Some("minecraft:tropical_fish_spawn_egg"),
		766 => Some("minecraft:tube_coral"),
		761 => Some("minecraft:tube_coral_block"),
		776 => Some("minecraft:tube_coral_fan"),
		12 => Some("minecraft:tuff"),
		22 => Some("minecraft:tuff_brick_slab"),
		23 => Some("minecraft:tuff_brick_stairs"),
		24 => Some("minecraft:tuff_brick_wall"),
		21 => Some("minecraft:tuff_bricks"),
		13 => Some("minecraft:tuff_slab"),
		14 => Some("minecraft:tuff_stairs"),
		15 => Some("minecraft:tuff_wall"),
		753 => Some("minecraft:turtle_egg"),
		1001 => Some("minecraft:turtle_helmet"),
		1002 => Some("minecraft:turtle_scute"),
		1312 => Some("minecraft:turtle_spawn_egg"),
		326 => Some("minecraft:twisting_vines"),
		1656 => Some("minecraft:vault"),
		1574 => Some("minecraft:verdant_froglight"),
		1586 => Some("minecraft:vex_armor_trim_smithing_template"),
		1353 => Some("minecraft:vex_spawn_egg"),
		1321 => Some("minecraft:villager_spawn_egg"),
		1352 => Some("minecraft:vindicator_spawn_egg"),
		484 => Some("minecraft:vine"),
		1322 => Some("minecraft:wandering_trader_spawn_egg"),
		1584 => Some("minecraft:ward_armor_trim_smithing_template"),
		1347 => Some("minecraft:warden_spawn_egg"),
		1254 => Some("minecraft:warm_ocean_ruins_map"),
		870 => Some("minecraft:warped_button"),
		901 => Some("minecraft:warped_door"),
		429 => Some("minecraft:warped_fence"),
		944 => Some("minecraft:warped_fence_gate"),
		321 => Some("minecraft:warped_fungus"),
		972 => Some("minecraft:warped_fungus_on_a_stick"),
		1127 => Some("minecraft:warped_hanging_sign"),
		214 => Some("minecraft:warped_hyphae"),
		61 => Some("minecraft:warped_nylium"),
		75 => Some("minecraft:warped_planks"),
		887 => Some("minecraft:warped_pressure_plate"),
		323 => Some("minecraft:warped_roots"),
		389 => Some("minecraft:warped_shelf"),
		1114 => Some("minecraft:warped_sign"),
		354 => Some("minecraft:warped_slab"),
		528 => Some("minecraft:warped_stairs"),
		176 => Some("minecraft:warped_stem"),
		923 => Some("minecraft:warped_trapdoor"),
		652 => Some("minecraft:warped_wart_block"),
		1129 => Some("minecraft:water_bucket"),
		135 => Some("minecraft:waxed_chiseled_copper"),
		469 => Some("minecraft:waxed_copper_bars"),
		124 => Some("minecraft:waxed_copper_block"),
		1633 => Some("minecraft:waxed_copper_bulb"),
		478 => Some("minecraft:waxed_copper_chain"),
		1641 => Some("minecraft:waxed_copper_chest"),
		906 => Some("minecraft:waxed_copper_door"),
		1649 => Some("minecraft:waxed_copper_golem_statue"),
		1625 => Some("minecraft:waxed_copper_grate"),
		1521 => Some("minecraft:waxed_copper_lantern"),
		928 => Some("minecraft:waxed_copper_trapdoor"),
		143 => Some("minecraft:waxed_cut_copper"),
		159 => Some("minecraft:waxed_cut_copper_slab"),
		151 => Some("minecraft:waxed_cut_copper_stairs"),
		136 => Some("minecraft:waxed_exposed_chiseled_copper"),
		125 => Some("minecraft:waxed_exposed_copper"),
		470 => Some("minecraft:waxed_exposed_copper_bars"),
		1634 => Some("minecraft:waxed_exposed_copper_bulb"),
		479 => Some("minecraft:waxed_exposed_copper_chain"),
		1642 => Some("minecraft:waxed_exposed_copper_chest"),
		907 => Some("minecraft:waxed_exposed_copper_door"),
		1650 => Some("minecraft:waxed_exposed_copper_golem_statue"),
		1626 => Some("minecraft:waxed_exposed_copper_grate"),
		1522 => Some("minecraft:waxed_exposed_copper_lantern"),
		929 => Some("minecraft:waxed_exposed_copper_trapdoor"),
		144 => Some("minecraft:waxed_exposed_cut_copper"),
		160 => Some("minecraft:waxed_exposed_cut_copper_slab"),
		152 => Some("minecraft:waxed_exposed_cut_copper_stairs"),
		845 => Some("minecraft:waxed_exposed_lightning_rod"),
		844 => Some("minecraft:waxed_lightning_rod"),
		138 => Some("minecraft:waxed_oxidized_chiseled_copper"),
		127 => Some("minecraft:waxed_oxidized_copper"),
		472 => Some("minecraft:waxed_oxidized_copper_bars"),
		1636 => Some("minecraft:waxed_oxidized_copper_bulb"),
		481 => Some("minecraft:waxed_oxidized_copper_chain"),
		1644 => Some("minecraft:waxed_oxidized_copper_chest"),
		909 => Some("minecraft:waxed_oxidized_copper_door"),
		1652 => Some("minecraft:waxed_oxidized_copper_golem_statue"),
		1628 => Some("minecraft:waxed_oxidized_copper_grate"),
		1524 => Some("minecraft:waxed_oxidized_copper_lantern"),
		931 => Some("minecraft:waxed_oxidized_copper_trapdoor"),
		146 => Some("minecraft:waxed_oxidized_cut_copper"),
		162 => Some("minecraft:waxed_oxidized_cut_copper_slab"),
		154 => Some("minecraft:waxed_oxidized_cut_copper_stairs"),
		847 => Some("minecraft:waxed_oxidized_lightning_rod"),
		137 => Some("minecraft:waxed_weathered_chiseled_copper"),
		126 => Some("minecraft:waxed_weathered_copper"),
		471 => Some("minecraft:waxed_weathered_copper_bars"),
		1635 => Some("minecraft:waxed_weathered_copper_bulb"),
		480 => Some("minecraft:waxed_weathered_copper_chain"),
		1643 => Some("minecraft:waxed_weathered_copper_chest"),
		908 => Some("minecraft:waxed_weathered_copper_door"),
		1651 => Some("minecraft:waxed_weathered_copper_golem_statue"),
		1627 => Some("minecraft:waxed_weathered_copper_grate"),
		1523 => Some("minecraft:waxed_weathered_copper_lantern"),
		930 => Some("minecraft:waxed_weathered_copper_trapdoor"),
		145 => Some("minecraft:waxed_weathered_cut_copper"),
		161 => Some("minecraft:waxed_weathered_cut_copper_slab"),
		153 => Some("minecraft:waxed_weathered_cut_copper_stairs"),
		846 => Some("minecraft:waxed_weathered_lightning_rod"),
		1591 => Some("minecraft:wayfinder_armor_trim_smithing_template"),
		133 => Some("minecraft:weathered_chiseled_copper"),
		122 => Some("minecraft:weathered_copper"),
		467 => Some("minecraft:weathered_copper_bars"),
		1631 => Some("minecraft:weathered_copper_bulb"),
		476 => Some("minecraft:weathered_copper_chain"),
		1639 => Some("minecraft:weathered_copper_chest"),
		904 => Some("minecraft:weathered_copper_door"),
		1647 => Some("minecraft:weathered_copper_golem_statue"),
		1623 => Some("minecraft:weathered_copper_grate"),
		1519 => Some("minecraft:weathered_copper_lantern"),
		926 => Some("minecraft:weathered_copper_trapdoor"),
		141 => Some("minecraft:weathered_cut_copper"),
		157 => Some("minecraft:weathered_cut_copper_slab"),
		149 => Some("minecraft:weathered_cut_copper_stairs"),
		842 => Some("minecraft:weathered_lightning_rod"),
		325 => Some("minecraft:weeping_vines"),
		230 => Some("minecraft:wet_sponge"),
		1066 => Some("minecraft:wheat"),
		1065 => Some("minecraft:wheat_seeds"),
		1417 => Some("minecraft:white_banner"),
		1219 => Some("minecraft:white_bed"),
		1154 => Some("minecraft:white_bundle"),
		1551 => Some("minecraft:white_candle"),
		580 => Some("minecraft:white_carpet"),
		689 => Some("minecraft:white_concrete"),
		737 => Some("minecraft:white_concrete_powder"),
		721 => Some("minecraft:white_concrete_slab"),
		705 => Some("minecraft:white_concrete_stairs"),
		1170 => Some("minecraft:white_cushion"),
		1199 => Some("minecraft:white_dye"),
		673 => Some("minecraft:white_glazed_terracotta"),
		950 => Some("minecraft:white_harness"),
		657 => Some("minecraft:white_shulker_box"),
		605 => Some("minecraft:white_stained_glass"),
		621 => Some("minecraft:white_stained_glass_pane"),
		561 => Some("minecraft:white_terracotta"),
		308 => Some("minecraft:white_tulip"),
		250 => Some("minecraft:white_wool"),
		282 => Some("minecraft:white_wool_slab"),
		266 => Some("minecraft:white_wool_stairs"),
		1583 => Some("minecraft:wild_armor_trim_smithing_template"),
		330 => Some("minecraft:wildflowers"),
		1370 => Some("minecraft:wind_charge"),
		1348 => Some("minecraft:witch_spawn_egg"),
		313 => Some("minecraft:wither_rose"),
		1385 => Some("minecraft:wither_skeleton_skull"),
		1332 => Some("minecraft:wither_skeleton_spawn_egg"),
		1331 => Some("minecraft:wither_spawn_egg"),
		1004 => Some("minecraft:wolf_armor"),
		1290 => Some("minecraft:wolf_spawn_egg"),
		1028 => Some("minecraft:wooden_axe"),
		1029 => Some("minecraft:wooden_hoe"),
		1027 => Some("minecraft:wooden_pickaxe"),
		1026 => Some("minecraft:wooden_shovel"),
		1447 => Some("minecraft:wooden_spear"),
		1025 => Some("minecraft:wooden_sword"),
		1240 => Some("minecraft:woodland_mansion_map"),
		1371 => Some("minecraft:writable_book"),
		1372 => Some("minecraft:written_book"),
		1421 => Some("minecraft:yellow_banner"),
		1223 => Some("minecraft:yellow_bed"),
		1158 => Some("minecraft:yellow_bundle"),
		1555 => Some("minecraft:yellow_candle"),
		584 => Some("minecraft:yellow_carpet"),
		693 => Some("minecraft:yellow_concrete"),
		741 => Some("minecraft:yellow_concrete_powder"),
		725 => Some("minecraft:yellow_concrete_slab"),
		709 => Some("minecraft:yellow_concrete_stairs"),
		1174 => Some("minecraft:yellow_cushion"),
		1203 => Some("minecraft:yellow_dye"),
		677 => Some("minecraft:yellow_glazed_terracotta"),
		954 => Some("minecraft:yellow_harness"),
		226 => Some("minecraft:yellow_poplar_leaves"),
		661 => Some("minecraft:yellow_shulker_box"),
		609 => Some("minecraft:yellow_stained_glass"),
		625 => Some("minecraft:yellow_stained_glass_pane"),
		565 => Some("minecraft:yellow_terracotta"),
		254 => Some("minecraft:yellow_wool"),
		286 => Some("minecraft:yellow_wool_slab"),
		270 => Some("minecraft:yellow_wool_stairs"),
		1362 => Some("minecraft:zoglin_spawn_egg"),
		1387 => Some("minecraft:zombie_head"),
		1334 => Some("minecraft:zombie_horse_spawn_egg"),
		1335 => Some("minecraft:zombie_nautilus_spawn_egg"),
		1333 => Some("minecraft:zombie_spawn_egg"),
		1336 => Some("minecraft:zombie_villager_spawn_egg"),
		1363 => Some("minecraft:zombified_piglin_spawn_egg"),
    _ => None,
	};
}
pub fn get_item_id_by_name(name: &str) -> Option<i32> {
  return match name {
		"minecraft:abandoned_camp_map" => Some(1253),
		"minecraft:acacia_boat" => Some(983),
		"minecraft:acacia_button" => Some(862),
		"minecraft:acacia_chest_boat" => Some(984),
		"minecraft:acacia_door" => Some(893),
		"minecraft:acacia_fence" => Some(421),
		"minecraft:acacia_fence_gate" => Some(936),
		"minecraft:acacia_hanging_sign" => Some(1119),
		"minecraft:acacia_leaves" => Some(219),
		"minecraft:acacia_log" => Some(167),
		"minecraft:acacia_planks" => Some(67),
		"minecraft:acacia_pressure_plate" => Some(879),
		"minecraft:acacia_sapling" => Some(81),
		"minecraft:acacia_shelf" => Some(377),
		"minecraft:acacia_sign" => Some(1106),
		"minecraft:acacia_slab" => Some(345),
		"minecraft:acacia_stairs" => Some(519),
		"minecraft:acacia_trapdoor" => Some(915),
		"minecraft:acacia_wood" => Some(207),
		"minecraft:activator_rail" => Some(948),
		"minecraft:air" => Some(0),
		"minecraft:allay_spawn_egg" => Some(1313),
		"minecraft:allium" => Some(304),
		"minecraft:amethyst_block" => Some(117),
		"minecraft:amethyst_cluster" => Some(1570),
		"minecraft:amethyst_shard" => Some(1016),
		"minecraft:ancient_debris" => Some(111),
		"minecraft:andesite" => Some(6),
		"minecraft:andesite_slab" => Some(815),
		"minecraft:andesite_stairs" => Some(798),
		"minecraft:andesite_wall" => Some(541),
		"minecraft:angler_pottery_sherd" => Some(1598),
		"minecraft:anvil" => Some(553),
		"minecraft:apple" => Some(1007),
		"minecraft:archer_pottery_sherd" => Some(1599),
		"minecraft:armadillo_scute" => Some(1003),
		"minecraft:armadillo_spawn_egg" => Some(1291),
		"minecraft:armor_stand" => Some(1405),
		"minecraft:arms_up_pottery_sherd" => Some(1600),
		"minecraft:arrow" => Some(1009),
		"minecraft:axolotl_bucket" => Some(1139),
		"minecraft:axolotl_spawn_egg" => Some(1301),
		"minecraft:azalea" => Some(242),
		"minecraft:azalea_leaves" => Some(227),
		"minecraft:azure_bluet" => Some(305),
		"minecraft:baked_potato" => Some(1380),
		"minecraft:bamboo" => Some(340),
		"minecraft:bamboo_block" => Some(177),
		"minecraft:bamboo_button" => Some(868),
		"minecraft:bamboo_chest_raft" => Some(996),
		"minecraft:bamboo_door" => Some(899),
		"minecraft:bamboo_fence" => Some(427),
		"minecraft:bamboo_fence_gate" => Some(942),
		"minecraft:bamboo_hanging_sign" => Some(1125),
		"minecraft:bamboo_mosaic" => Some(76),
		"minecraft:bamboo_mosaic_slab" => Some(352),
		"minecraft:bamboo_mosaic_stairs" => Some(526),
		"minecraft:bamboo_planks" => Some(73),
		"minecraft:bamboo_pressure_plate" => Some(885),
		"minecraft:bamboo_raft" => Some(995),
		"minecraft:bamboo_shelf" => Some(378),
		"minecraft:bamboo_sign" => Some(1112),
		"minecraft:bamboo_slab" => Some(351),
		"minecraft:bamboo_stairs" => Some(525),
		"minecraft:bamboo_trapdoor" => Some(921),
		"minecraft:barrel" => Some(1506),
		"minecraft:barrier" => Some(577),
		"minecraft:basalt" => Some(436),
		"minecraft:bat_spawn_egg" => Some(1292),
		"minecraft:beacon" => Some(530),
		"minecraft:bedrock" => Some(87),
		"minecraft:bee_nest" => Some(1531),
		"minecraft:bee_spawn_egg" => Some(1293),
		"minecraft:beef" => Some(1260),
		"minecraft:beehive" => Some(1532),
		"minecraft:beetroot" => Some(1438),
		"minecraft:beetroot_seeds" => Some(1439),
		"minecraft:beetroot_soup" => Some(1440),
		"minecraft:bell" => Some(1514),
		"minecraft:big_dripleaf" => Some(338),
		"minecraft:birch_boat" => Some(979),
		"minecraft:birch_button" => Some(860),
		"minecraft:birch_chest_boat" => Some(980),
		"minecraft:birch_door" => Some(891),
		"minecraft:birch_fence" => Some(419),
		"minecraft:birch_fence_gate" => Some(934),
		"minecraft:birch_hanging_sign" => Some(1117),
		"minecraft:birch_leaves" => Some(217),
		"minecraft:birch_log" => Some(165),
		"minecraft:birch_planks" => Some(65),
		"minecraft:birch_pressure_plate" => Some(877),
		"minecraft:birch_sapling" => Some(79),
		"minecraft:birch_shelf" => Some(379),
		"minecraft:birch_sign" => Some(1104),
		"minecraft:birch_slab" => Some(343),
		"minecraft:birch_stairs" => Some(517),
		"minecraft:birch_trapdoor" => Some(913),
		"minecraft:birch_wood" => Some(205),
		"minecraft:black_banner" => Some(1432),
		"minecraft:black_bed" => Some(1234),
		"minecraft:black_bundle" => Some(1169),
		"minecraft:black_candle" => Some(1566),
		"minecraft:black_carpet" => Some(595),
		"minecraft:black_concrete" => Some(704),
		"minecraft:black_concrete_powder" => Some(752),
		"minecraft:black_concrete_slab" => Some(736),
		"minecraft:black_concrete_stairs" => Some(720),
		"minecraft:black_cushion" => Some(1185),
		"minecraft:black_dye" => Some(1214),
		"minecraft:black_glazed_terracotta" => Some(688),
		"minecraft:black_harness" => Some(965),
		"minecraft:black_shulker_box" => Some(672),
		"minecraft:black_stained_glass" => Some(620),
		"minecraft:black_stained_glass_pane" => Some(636),
		"minecraft:black_terracotta" => Some(576),
		"minecraft:black_wool" => Some(265),
		"minecraft:black_wool_slab" => Some(297),
		"minecraft:black_wool_stairs" => Some(281),
		"minecraft:blackstone" => Some(1537),
		"minecraft:blackstone_slab" => Some(1538),
		"minecraft:blackstone_stairs" => Some(1539),
		"minecraft:blackstone_wall" => Some(546),
		"minecraft:blade_pottery_sherd" => Some(1601),
		"minecraft:blast_furnace" => Some(1508),
		"minecraft:blaze_powder" => Some(1274),
		"minecraft:blaze_rod" => Some(1266),
		"minecraft:blaze_spawn_egg" => Some(1354),
		"minecraft:blue_banner" => Some(1428),
		"minecraft:blue_bed" => Some(1230),
		"minecraft:blue_bundle" => Some(1165),
		"minecraft:blue_candle" => Some(1562),
		"minecraft:blue_carpet" => Some(591),
		"minecraft:blue_concrete" => Some(700),
		"minecraft:blue_concrete_powder" => Some(748),
		"minecraft:blue_concrete_slab" => Some(732),
		"minecraft:blue_concrete_stairs" => Some(716),
		"minecraft:blue_cushion" => Some(1181),
		"minecraft:blue_dye" => Some(1210),
		"minecraft:blue_egg" => Some(1149),
		"minecraft:blue_glazed_terracotta" => Some(684),
		"minecraft:blue_harness" => Some(961),
		"minecraft:blue_ice" => Some(786),
		"minecraft:blue_orchid" => Some(303),
		"minecraft:blue_shulker_box" => Some(668),
		"minecraft:blue_stained_glass" => Some(616),
		"minecraft:blue_stained_glass_pane" => Some(632),
		"minecraft:blue_terracotta" => Some(572),
		"minecraft:blue_wool" => Some(261),
		"minecraft:blue_wool_slab" => Some(293),
		"minecraft:blue_wool_stairs" => Some(277),
		"minecraft:bogged_spawn_egg" => Some(1323),
		"minecraft:bolt_armor_trim_smithing_template" => Some(1597),
		"minecraft:bone" => Some(1216),
		"minecraft:bone_block" => Some(654),
		"minecraft:bone_meal" => Some(1215),
		"minecraft:book" => Some(1146),
		"minecraft:bookshelf" => Some(390),
		"minecraft:bordure_indented_banner_pattern" => Some(1503),
		"minecraft:bow" => Some(1008),
		"minecraft:bowl" => Some(1006),
		"minecraft:brain_coral" => Some(767),
		"minecraft:brain_coral_block" => Some(762),
		"minecraft:brain_coral_fan" => Some(777),
		"minecraft:bread" => Some(1067),
		"minecraft:breeze_rod" => Some(1373),
		"minecraft:breeze_spawn_egg" => Some(1339),
		"minecraft:brewer_pottery_sherd" => Some(1602),
		"minecraft:brewing_stand" => Some(1276),
		"minecraft:brick" => Some(1142),
		"minecraft:brick_slab" => Some(361),
		"minecraft:brick_stairs" => Some(493),
		"minecraft:brick_wall" => Some(533),
		"minecraft:bricks" => Some(376),
		"minecraft:brown_banner" => Some(1429),
		"minecraft:brown_bed" => Some(1231),
		"minecraft:brown_bundle" => Some(1166),
		"minecraft:brown_candle" => Some(1563),
		"minecraft:brown_carpet" => Some(592),
		"minecraft:brown_concrete" => Some(701),
		"minecraft:brown_concrete_powder" => Some(749),
		"minecraft:brown_concrete_slab" => Some(733),
		"minecraft:brown_concrete_stairs" => Some(717),
		"minecraft:brown_cushion" => Some(1182),
		"minecraft:brown_dye" => Some(1211),
		"minecraft:brown_egg" => Some(1150),
		"minecraft:brown_glazed_terracotta" => Some(685),
		"minecraft:brown_harness" => Some(962),
		"minecraft:brown_mushroom" => Some(317),
		"minecraft:brown_mushroom_block" => Some(461),
		"minecraft:brown_shulker_box" => Some(669),
		"minecraft:brown_stained_glass" => Some(617),
		"minecraft:brown_stained_glass_pane" => Some(633),
		"minecraft:brown_terracotta" => Some(573),
		"minecraft:brown_wool" => Some(262),
		"minecraft:brown_wool_slab" => Some(294),
		"minecraft:brown_wool_stairs" => Some(278),
		"minecraft:brush" => Some(1578),
		"minecraft:bubble_coral" => Some(768),
		"minecraft:bubble_coral_block" => Some(763),
		"minecraft:bubble_coral_fan" => Some(778),
		"minecraft:bucket" => Some(1128),
		"minecraft:budding_amethyst" => Some(118),
		"minecraft:bundle" => Some(1153),
		"minecraft:buried_ancient_city_map" => Some(1250),
		"minecraft:buried_mineshaft_map" => Some(1251),
		"minecraft:buried_treasure_map" => Some(1249),
		"minecraft:buried_trial_chambers_map" => Some(1241),
		"minecraft:burn_pottery_sherd" => Some(1603),
		"minecraft:bush" => Some(240),
		"minecraft:cactus" => Some(413),
		"minecraft:cactus_flower" => Some(414),
		"minecraft:cake" => Some(1218),
		"minecraft:calcite" => Some(11),
		"minecraft:calibrated_sculk_sensor" => Some(850),
		"minecraft:camel_husk_spawn_egg" => Some(1324),
		"minecraft:camel_spawn_egg" => Some(1284),
		"minecraft:campfire" => Some(1527),
		"minecraft:candle" => Some(1550),
		"minecraft:carrot" => Some(1378),
		"minecraft:carrot_on_a_stick" => Some(971),
		"minecraft:cartography_table" => Some(1509),
		"minecraft:carved_pumpkin" => Some(431),
		"minecraft:cat_spawn_egg" => Some(1288),
		"minecraft:cauldron" => Some(1277),
		"minecraft:cave_spider_spawn_egg" => Some(1337),
		"minecraft:chain_command_block" => Some(649),
		"minecraft:chainmail_boots" => Some(1079),
		"minecraft:chainmail_chestplate" => Some(1077),
		"minecraft:chainmail_helmet" => Some(1076),
		"minecraft:chainmail_leggings" => Some(1078),
		"minecraft:charcoal" => Some(1011),
		"minecraft:cherry_boat" => Some(985),
		"minecraft:cherry_button" => Some(863),
		"minecraft:cherry_chest_boat" => Some(986),
		"minecraft:cherry_door" => Some(894),
		"minecraft:cherry_fence" => Some(422),
		"minecraft:cherry_fence_gate" => Some(937),
		"minecraft:cherry_hanging_sign" => Some(1120),
		"minecraft:cherry_leaves" => Some(220),
		"minecraft:cherry_log" => Some(168),
		"minecraft:cherry_planks" => Some(68),
		"minecraft:cherry_pressure_plate" => Some(880),
		"minecraft:cherry_sapling" => Some(82),
		"minecraft:cherry_shelf" => Some(380),
		"minecraft:cherry_sign" => Some(1107),
		"minecraft:cherry_slab" => Some(346),
		"minecraft:cherry_stairs" => Some(520),
		"minecraft:cherry_trapdoor" => Some(916),
		"minecraft:cherry_wood" => Some(208),
		"minecraft:chest" => Some(404),
		"minecraft:chest_minecart" => Some(967),
		"minecraft:chicken" => Some(1262),
		"minecraft:chicken_spawn_egg" => Some(1280),
		"minecraft:chipped_anvil" => Some(554),
		"minecraft:chiseled_bookshelf" => Some(391),
		"minecraft:chiseled_cinnabar" => Some(52),
		"minecraft:chiseled_copper" => Some(131),
		"minecraft:chiseled_deepslate" => Some(459),
		"minecraft:chiseled_nether_bricks" => Some(500),
		"minecraft:chiseled_polished_blackstone" => Some(1544),
		"minecraft:chiseled_quartz_block" => Some(556),
		"minecraft:chiseled_red_sandstone" => Some(645),
		"minecraft:chiseled_resin_bricks" => Some(492),
		"minecraft:chiseled_sandstone" => Some(235),
		"minecraft:chiseled_stone_bricks" => Some(452),
		"minecraft:chiseled_sulfur" => Some(39),
		"minecraft:chiseled_tuff" => Some(16),
		"minecraft:chiseled_tuff_bricks" => Some(25),
		"minecraft:chorus_flower" => Some(398),
		"minecraft:chorus_fruit" => Some(1434),
		"minecraft:chorus_plant" => Some(397),
		"minecraft:cinnabar" => Some(40),
		"minecraft:cinnabar_brick_slab" => Some(49),
		"minecraft:cinnabar_brick_stairs" => Some(50),
		"minecraft:cinnabar_brick_wall" => Some(51),
		"minecraft:cinnabar_bricks" => Some(48),
		"minecraft:cinnabar_slab" => Some(41),
		"minecraft:cinnabar_stairs" => Some(42),
		"minecraft:cinnabar_wall" => Some(43),
		"minecraft:clay" => Some(415),
		"minecraft:clay_ball" => Some(1143),
		"minecraft:clock" => Some(1187),
		"minecraft:closed_eyeblossom" => Some(301),
		"minecraft:coal" => Some(1010),
		"minecraft:coal_block" => Some(112),
		"minecraft:coal_ore" => Some(93),
		"minecraft:coarse_dirt" => Some(56),
		"minecraft:coast_armor_trim_smithing_template" => Some(1582),
		"minecraft:cobbled_deepslate" => Some(9),
		"minecraft:cobbled_deepslate_slab" => Some(819),
		"minecraft:cobbled_deepslate_stairs" => Some(802),
		"minecraft:cobbled_deepslate_wall" => Some(549),
		"minecraft:cobblestone" => Some(62),
		"minecraft:cobblestone_slab" => Some(360),
		"minecraft:cobblestone_stairs" => Some(409),
		"minecraft:cobblestone_wall" => Some(531),
		"minecraft:cobweb" => Some(237),
		"minecraft:cocoa_beans" => Some(1198),
		"minecraft:cod" => Some(1190),
		"minecraft:cod_bucket" => Some(1137),
		"minecraft:cod_spawn_egg" => Some(1302),
		"minecraft:command_block" => Some(529),
		"minecraft:command_block_minecart" => Some(1414),
		"minecraft:comparator" => Some(828),
		"minecraft:compass" => Some(1151),
		"minecraft:composter" => Some(1505),
		"minecraft:conduit" => Some(787),
		"minecraft:cooked_beef" => Some(1261),
		"minecraft:cooked_chicken" => Some(1263),
		"minecraft:cooked_cod" => Some(1194),
		"minecraft:cooked_mutton" => Some(1416),
		"minecraft:cooked_porkchop" => Some(1098),
		"minecraft:cooked_rabbit" => Some(1401),
		"minecraft:cooked_salmon" => Some(1195),
		"minecraft:cookie" => Some(1236),
		"minecraft:copper_axe" => Some(1033),
		"minecraft:copper_bars" => Some(465),
		"minecraft:copper_block" => Some(120),
		"minecraft:copper_boots" => Some(1075),
		"minecraft:copper_bulb" => Some(1629),
		"minecraft:copper_chain" => Some(474),
		"minecraft:copper_chest" => Some(1637),
		"minecraft:copper_chestplate" => Some(1073),
		"minecraft:copper_door" => Some(902),
		"minecraft:copper_golem_spawn_egg" => Some(1317),
		"minecraft:copper_golem_statue" => Some(1645),
		"minecraft:copper_grate" => Some(1621),
		"minecraft:copper_helmet" => Some(1072),
		"minecraft:copper_hoe" => Some(1034),
		"minecraft:copper_horse_armor" => Some(1406),
		"minecraft:copper_ingot" => Some(1020),
		"minecraft:copper_lantern" => Some(1517),
		"minecraft:copper_leggings" => Some(1074),
		"minecraft:copper_nautilus_armor" => Some(1489),
		"minecraft:copper_nugget" => Some(1457),
		"minecraft:copper_ore" => Some(97),
		"minecraft:copper_pickaxe" => Some(1032),
		"minecraft:copper_shovel" => Some(1031),
		"minecraft:copper_spear" => Some(1449),
		"minecraft:copper_sword" => Some(1030),
		"minecraft:copper_torch" => Some(440),
		"minecraft:copper_trapdoor" => Some(924),
		"minecraft:cornflower" => Some(311),
		"minecraft:cow_spawn_egg" => Some(1281),
		"minecraft:cracked_deepslate_bricks" => Some(456),
		"minecraft:cracked_deepslate_tiles" => Some(458),
		"minecraft:cracked_nether_bricks" => Some(499),
		"minecraft:cracked_polished_blackstone_bricks" => Some(1548),
		"minecraft:cracked_stone_bricks" => Some(451),
		"minecraft:crafter" => Some(1237),
		"minecraft:crafting_table" => Some(405),
		"minecraft:creaking_heart" => Some(403),
		"minecraft:creaking_spawn_egg" => Some(1340),
		"minecraft:creeper_banner_pattern" => Some(1495),
		"minecraft:creeper_head" => Some(1388),
		"minecraft:creeper_spawn_egg" => Some(1341),
		"minecraft:crimson_button" => Some(869),
		"minecraft:crimson_door" => Some(900),
		"minecraft:crimson_fence" => Some(428),
		"minecraft:crimson_fence_gate" => Some(943),
		"minecraft:crimson_fungus" => Some(320),
		"minecraft:crimson_hanging_sign" => Some(1126),
		"minecraft:crimson_hyphae" => Some(213),
		"minecraft:crimson_nylium" => Some(60),
		"minecraft:crimson_planks" => Some(74),
		"minecraft:crimson_pressure_plate" => Some(886),
		"minecraft:crimson_roots" => Some(322),
		"minecraft:crimson_shelf" => Some(381),
		"minecraft:crimson_sign" => Some(1113),
		"minecraft:crimson_slab" => Some(353),
		"minecraft:crimson_stairs" => Some(527),
		"minecraft:crimson_stem" => Some(175),
		"minecraft:crimson_trapdoor" => Some(922),
		"minecraft:crossbow" => Some(1491),
		"minecraft:crying_obsidian" => Some(1536),
		"minecraft:cut_copper" => Some(139),
		"minecraft:cut_copper_slab" => Some(155),
		"minecraft:cut_copper_stairs" => Some(147),
		"minecraft:cut_red_sandstone" => Some(646),
		"minecraft:cut_red_sandstone_slab" => Some(367),
		"minecraft:cut_sandstone" => Some(236),
		"minecraft:cut_sandstone_slab" => Some(358),
		"minecraft:cyan_banner" => Some(1426),
		"minecraft:cyan_bed" => Some(1228),
		"minecraft:cyan_bundle" => Some(1163),
		"minecraft:cyan_candle" => Some(1560),
		"minecraft:cyan_carpet" => Some(589),
		"minecraft:cyan_concrete" => Some(698),
		"minecraft:cyan_concrete_powder" => Some(746),
		"minecraft:cyan_concrete_slab" => Some(730),
		"minecraft:cyan_concrete_stairs" => Some(714),
		"minecraft:cyan_cushion" => Some(1179),
		"minecraft:cyan_dye" => Some(1208),
		"minecraft:cyan_glazed_terracotta" => Some(682),
		"minecraft:cyan_harness" => Some(959),
		"minecraft:cyan_shulker_box" => Some(666),
		"minecraft:cyan_stained_glass" => Some(614),
		"minecraft:cyan_stained_glass_pane" => Some(630),
		"minecraft:cyan_terracotta" => Some(570),
		"minecraft:cyan_wool" => Some(259),
		"minecraft:cyan_wool_slab" => Some(291),
		"minecraft:cyan_wool_stairs" => Some(275),
		"minecraft:damaged_anvil" => Some(555),
		"minecraft:dandelion" => Some(298),
		"minecraft:danger_pottery_sherd" => Some(1604),
		"minecraft:dark_oak_boat" => Some(987),
		"minecraft:dark_oak_button" => Some(864),
		"minecraft:dark_oak_chest_boat" => Some(988),
		"minecraft:dark_oak_door" => Some(895),
		"minecraft:dark_oak_fence" => Some(423),
		"minecraft:dark_oak_fence_gate" => Some(938),
		"minecraft:dark_oak_hanging_sign" => Some(1121),
		"minecraft:dark_oak_leaves" => Some(221),
		"minecraft:dark_oak_log" => Some(170),
		"minecraft:dark_oak_planks" => Some(69),
		"minecraft:dark_oak_pressure_plate" => Some(881),
		"minecraft:dark_oak_sapling" => Some(83),
		"minecraft:dark_oak_shelf" => Some(382),
		"minecraft:dark_oak_sign" => Some(1108),
		"minecraft:dark_oak_slab" => Some(347),
		"minecraft:dark_oak_stairs" => Some(521),
		"minecraft:dark_oak_trapdoor" => Some(917),
		"minecraft:dark_oak_wood" => Some(210),
		"minecraft:dark_prismarine" => Some(639),
		"minecraft:dark_prismarine_slab" => Some(371),
		"minecraft:dark_prismarine_stairs" => Some(642),
		"minecraft:daylight_detector" => Some(848),
		"minecraft:dead_brain_coral" => Some(771),
		"minecraft:dead_brain_coral_block" => Some(757),
		"minecraft:dead_brain_coral_fan" => Some(782),
		"minecraft:dead_bubble_coral" => Some(772),
		"minecraft:dead_bubble_coral_block" => Some(758),
		"minecraft:dead_bubble_coral_fan" => Some(783),
		"minecraft:dead_bush" => Some(244),
		"minecraft:dead_fire_coral" => Some(773),
		"minecraft:dead_fire_coral_block" => Some(759),
		"minecraft:dead_fire_coral_fan" => Some(784),
		"minecraft:dead_horn_coral" => Some(774),
		"minecraft:dead_horn_coral_block" => Some(760),
		"minecraft:dead_horn_coral_fan" => Some(785),
		"minecraft:dead_tube_coral" => Some(775),
		"minecraft:dead_tube_coral_block" => Some(756),
		"minecraft:dead_tube_coral_fan" => Some(781),
		"minecraft:debug_stick" => Some(1459),
		"minecraft:decorated_pot" => Some(392),
		"minecraft:deepslate" => Some(8),
		"minecraft:deepslate_brick_slab" => Some(821),
		"minecraft:deepslate_brick_stairs" => Some(804),
		"minecraft:deepslate_brick_wall" => Some(551),
		"minecraft:deepslate_bricks" => Some(455),
		"minecraft:deepslate_coal_ore" => Some(94),
		"minecraft:deepslate_copper_ore" => Some(98),
		"minecraft:deepslate_diamond_ore" => Some(108),
		"minecraft:deepslate_emerald_ore" => Some(104),
		"minecraft:deepslate_gold_ore" => Some(100),
		"minecraft:deepslate_iron_ore" => Some(96),
		"minecraft:deepslate_lapis_ore" => Some(106),
		"minecraft:deepslate_redstone_ore" => Some(102),
		"minecraft:deepslate_tile_slab" => Some(822),
		"minecraft:deepslate_tile_stairs" => Some(805),
		"minecraft:deepslate_tile_wall" => Some(552),
		"minecraft:deepslate_tiles" => Some(457),
		"minecraft:desert_pyramid_map" => Some(1252),
		"minecraft:desert_village_map" => Some(1244),
		"minecraft:detector_rail" => Some(946),
		"minecraft:diamond" => Some(1012),
		"minecraft:diamond_axe" => Some(1053),
		"minecraft:diamond_block" => Some(129),
		"minecraft:diamond_boots" => Some(1087),
		"minecraft:diamond_chestplate" => Some(1085),
		"minecraft:diamond_helmet" => Some(1084),
		"minecraft:diamond_hoe" => Some(1054),
		"minecraft:diamond_horse_armor" => Some(1409),
		"minecraft:diamond_leggings" => Some(1086),
		"minecraft:diamond_nautilus_armor" => Some(1487),
		"minecraft:diamond_ore" => Some(107),
		"minecraft:diamond_pickaxe" => Some(1052),
		"minecraft:diamond_shovel" => Some(1051),
		"minecraft:diamond_spear" => Some(1452),
		"minecraft:diamond_sword" => Some(1050),
		"minecraft:diorite" => Some(4),
		"minecraft:diorite_slab" => Some(818),
		"minecraft:diorite_stairs" => Some(801),
		"minecraft:diorite_wall" => Some(545),
		"minecraft:dirt" => Some(55),
		"minecraft:dirt_path" => Some(598),
		"minecraft:disc_fragment_5" => Some(1482),
		"minecraft:dispenser" => Some(835),
		"minecraft:dolphin_spawn_egg" => Some(1303),
		"minecraft:donkey_spawn_egg" => Some(1285),
		"minecraft:dragon_breath" => Some(1441),
		"minecraft:dragon_egg" => Some(511),
		"minecraft:dragon_head" => Some(1389),
		"minecraft:dried_ghast" => Some(755),
		"minecraft:dried_kelp" => Some(1257),
		"minecraft:dried_kelp_block" => Some(1144),
		"minecraft:dripstone_block" => Some(53),
		"minecraft:dropper" => Some(836),
		"minecraft:drowned_spawn_egg" => Some(1325),
		"minecraft:dune_armor_trim_smithing_template" => Some(1581),
		"minecraft:echo_shard" => Some(1577),
		"minecraft:egg" => Some(1148),
		"minecraft:elder_guardian_spawn_egg" => Some(1342),
		"minecraft:elytra" => Some(974),
		"minecraft:emerald" => Some(1013),
		"minecraft:emerald_block" => Some(514),
		"minecraft:emerald_ore" => Some(103),
		"minecraft:enchanted_book" => Some(1395),
		"minecraft:enchanted_golden_apple" => Some(1101),
		"minecraft:enchanting_table" => Some(507),
		"minecraft:end_crystal" => Some(1433),
		"minecraft:end_portal_frame" => Some(508),
		"minecraft:end_rod" => Some(396),
		"minecraft:end_stone" => Some(509),
		"minecraft:end_stone_brick_slab" => Some(811),
		"minecraft:end_stone_brick_stairs" => Some(793),
		"minecraft:end_stone_brick_wall" => Some(544),
		"minecraft:end_stone_bricks" => Some(510),
		"minecraft:ender_chest" => Some(513),
		"minecraft:ender_dragon_spawn_egg" => Some(1364),
		"minecraft:ender_eye" => Some(1278),
		"minecraft:ender_pearl" => Some(1265),
		"minecraft:enderman_spawn_egg" => Some(1365),
		"minecraft:endermite_spawn_egg" => Some(1366),
		"minecraft:evoker_spawn_egg" => Some(1349),
		"minecraft:experience_bottle" => Some(1368),
		"minecraft:explorer_pottery_sherd" => Some(1605),
		"minecraft:exposed_chiseled_copper" => Some(132),
		"minecraft:exposed_copper" => Some(121),
		"minecraft:exposed_copper_bars" => Some(466),
		"minecraft:exposed_copper_bulb" => Some(1630),
		"minecraft:exposed_copper_chain" => Some(475),
		"minecraft:exposed_copper_chest" => Some(1638),
		"minecraft:exposed_copper_door" => Some(903),
		"minecraft:exposed_copper_golem_statue" => Some(1646),
		"minecraft:exposed_copper_grate" => Some(1622),
		"minecraft:exposed_copper_lantern" => Some(1518),
		"minecraft:exposed_copper_trapdoor" => Some(925),
		"minecraft:exposed_cut_copper" => Some(140),
		"minecraft:exposed_cut_copper_slab" => Some(156),
		"minecraft:exposed_cut_copper_stairs" => Some(148),
		"minecraft:exposed_lightning_rod" => Some(841),
		"minecraft:eye_armor_trim_smithing_template" => Some(1585),
		"minecraft:farmland" => Some(406),
		"minecraft:feather" => Some(1063),
		"minecraft:fermented_spider_eye" => Some(1273),
		"minecraft:fern" => Some(239),
		"minecraft:field_masoned_banner_pattern" => Some(1502),
		"minecraft:filled_map" => Some(1238),
		"minecraft:fire_charge" => Some(1369),
		"minecraft:fire_coral" => Some(769),
		"minecraft:fire_coral_block" => Some(764),
		"minecraft:fire_coral_fan" => Some(779),
		"minecraft:firefly_bush" => Some(245),
		"minecraft:firework_rocket" => Some(1393),
		"minecraft:firework_star" => Some(1394),
		"minecraft:fishing_rod" => Some(1186),
		"minecraft:fletching_table" => Some(1510),
		"minecraft:flint" => Some(1096),
		"minecraft:flint_and_steel" => Some(1005),
		"minecraft:flow_armor_trim_smithing_template" => Some(1596),
		"minecraft:flow_banner_pattern" => Some(1500),
		"minecraft:flow_pottery_sherd" => Some(1606),
		"minecraft:flower_banner_pattern" => Some(1494),
		"minecraft:flower_pot" => Some(1377),
		"minecraft:flowering_azalea" => Some(243),
		"minecraft:flowering_azalea_leaves" => Some(228),
		"minecraft:fox_spawn_egg" => Some(1294),
		"minecraft:friend_pottery_sherd" => Some(1607),
		"minecraft:frog_spawn_egg" => Some(1304),
		"minecraft:frogspawn" => Some(1576),
		"minecraft:furnace" => Some(407),
		"minecraft:furnace_minecart" => Some(968),
		"minecraft:ghast_spawn_egg" => Some(1355),
		"minecraft:ghast_tear" => Some(1267),
		"minecraft:gilded_blackstone" => Some(1540),
		"minecraft:glass" => Some(231),
		"minecraft:glass_bottle" => Some(1270),
		"minecraft:glass_pane" => Some(482),
		"minecraft:glistering_melon_slice" => Some(1279),
		"minecraft:globe_banner_pattern" => Some(1498),
		"minecraft:glow_berries" => Some(1526),
		"minecraft:glow_ink_sac" => Some(1197),
		"minecraft:glow_item_frame" => Some(1376),
		"minecraft:glow_lichen" => Some(485),
		"minecraft:glow_squid_spawn_egg" => Some(1305),
		"minecraft:glowstone" => Some(441),
		"minecraft:glowstone_dust" => Some(1189),
		"minecraft:goat_horn" => Some(1504),
		"minecraft:goat_spawn_egg" => Some(1295),
		"minecraft:gold_block" => Some(128),
		"minecraft:gold_ingot" => Some(1022),
		"minecraft:gold_nugget" => Some(1268),
		"minecraft:gold_ore" => Some(99),
		"minecraft:golden_apple" => Some(1100),
		"minecraft:golden_axe" => Some(1043),
		"minecraft:golden_boots" => Some(1091),
		"minecraft:golden_carrot" => Some(1383),
		"minecraft:golden_chestplate" => Some(1089),
		"minecraft:golden_dandelion" => Some(299),
		"minecraft:golden_helmet" => Some(1088),
		"minecraft:golden_hoe" => Some(1044),
		"minecraft:golden_horse_armor" => Some(1408),
		"minecraft:golden_leggings" => Some(1090),
		"minecraft:golden_nautilus_armor" => Some(1486),
		"minecraft:golden_pickaxe" => Some(1042),
		"minecraft:golden_shovel" => Some(1041),
		"minecraft:golden_spear" => Some(1451),
		"minecraft:golden_sword" => Some(1040),
		"minecraft:granite" => Some(2),
		"minecraft:granite_slab" => Some(814),
		"minecraft:granite_stairs" => Some(797),
		"minecraft:granite_wall" => Some(537),
		"minecraft:grass_block" => Some(54),
		"minecraft:gravel" => Some(92),
		"minecraft:gray_banner" => Some(1424),
		"minecraft:gray_bed" => Some(1226),
		"minecraft:gray_bundle" => Some(1161),
		"minecraft:gray_candle" => Some(1558),
		"minecraft:gray_carpet" => Some(587),
		"minecraft:gray_concrete" => Some(696),
		"minecraft:gray_concrete_powder" => Some(744),
		"minecraft:gray_concrete_slab" => Some(728),
		"minecraft:gray_concrete_stairs" => Some(712),
		"minecraft:gray_cushion" => Some(1177),
		"minecraft:gray_dye" => Some(1206),
		"minecraft:gray_glazed_terracotta" => Some(680),
		"minecraft:gray_harness" => Some(957),
		"minecraft:gray_shulker_box" => Some(664),
		"minecraft:gray_stained_glass" => Some(612),
		"minecraft:gray_stained_glass_pane" => Some(628),
		"minecraft:gray_terracotta" => Some(568),
		"minecraft:gray_wool" => Some(257),
		"minecraft:gray_wool_slab" => Some(289),
		"minecraft:gray_wool_stairs" => Some(273),
		"minecraft:green_banner" => Some(1430),
		"minecraft:green_bed" => Some(1232),
		"minecraft:green_bundle" => Some(1167),
		"minecraft:green_candle" => Some(1564),
		"minecraft:green_carpet" => Some(593),
		"minecraft:green_concrete" => Some(702),
		"minecraft:green_concrete_powder" => Some(750),
		"minecraft:green_concrete_slab" => Some(734),
		"minecraft:green_concrete_stairs" => Some(718),
		"minecraft:green_cushion" => Some(1183),
		"minecraft:green_dye" => Some(1212),
		"minecraft:green_glazed_terracotta" => Some(686),
		"minecraft:green_harness" => Some(963),
		"minecraft:green_shulker_box" => Some(670),
		"minecraft:green_stained_glass" => Some(618),
		"minecraft:green_stained_glass_pane" => Some(634),
		"minecraft:green_terracotta" => Some(574),
		"minecraft:green_wool" => Some(263),
		"minecraft:green_wool_slab" => Some(295),
		"minecraft:green_wool_stairs" => Some(279),
		"minecraft:grindstone" => Some(1511),
		"minecraft:guardian_spawn_egg" => Some(1343),
		"minecraft:gunpowder" => Some(1064),
		"minecraft:guster_banner_pattern" => Some(1501),
		"minecraft:guster_pottery_sherd" => Some(1608),
		"minecraft:hanging_roots" => Some(337),
		"minecraft:happy_ghast_spawn_egg" => Some(1356),
		"minecraft:hay_block" => Some(579),
		"minecraft:heart_of_the_sea" => Some(1490),
		"minecraft:heart_pottery_sherd" => Some(1609),
		"minecraft:heartbreak_pottery_sherd" => Some(1610),
		"minecraft:heavy_core" => Some(116),
		"minecraft:heavy_weighted_pressure_plate" => Some(874),
		"minecraft:hoglin_spawn_egg" => Some(1357),
		"minecraft:honey_block" => Some(832),
		"minecraft:honey_bottle" => Some(1533),
		"minecraft:honeycomb" => Some(1530),
		"minecraft:honeycomb_block" => Some(1534),
		"minecraft:hopper" => Some(834),
		"minecraft:hopper_minecart" => Some(970),
		"minecraft:horn_coral" => Some(770),
		"minecraft:horn_coral_block" => Some(765),
		"minecraft:horn_coral_fan" => Some(780),
		"minecraft:horse_spawn_egg" => Some(1286),
		"minecraft:host_armor_trim_smithing_template" => Some(1595),
		"minecraft:howl_pottery_sherd" => Some(1611),
		"minecraft:husk_spawn_egg" => Some(1326),
		"minecraft:ice" => Some(411),
		"minecraft:infested_chiseled_stone_bricks" => Some(447),
		"minecraft:infested_cobblestone" => Some(443),
		"minecraft:infested_cracked_stone_bricks" => Some(446),
		"minecraft:infested_deepslate" => Some(448),
		"minecraft:infested_mossy_stone_bricks" => Some(445),
		"minecraft:infested_stone" => Some(442),
		"minecraft:infested_stone_bricks" => Some(444),
		"minecraft:ink_sac" => Some(1196),
		"minecraft:iron_axe" => Some(1048),
		"minecraft:iron_bars" => Some(464),
		"minecraft:iron_block" => Some(119),
		"minecraft:iron_boots" => Some(1083),
		"minecraft:iron_chain" => Some(473),
		"minecraft:iron_chestplate" => Some(1081),
		"minecraft:iron_door" => Some(888),
		"minecraft:iron_golem_spawn_egg" => Some(1318),
		"minecraft:iron_helmet" => Some(1080),
		"minecraft:iron_hoe" => Some(1049),
		"minecraft:iron_horse_armor" => Some(1407),
		"minecraft:iron_ingot" => Some(1018),
		"minecraft:iron_leggings" => Some(1082),
		"minecraft:iron_nautilus_armor" => Some(1485),
		"minecraft:iron_nugget" => Some(1456),
		"minecraft:iron_ore" => Some(95),
		"minecraft:iron_pickaxe" => Some(1047),
		"minecraft:iron_shovel" => Some(1046),
		"minecraft:iron_spear" => Some(1450),
		"minecraft:iron_sword" => Some(1045),
		"minecraft:iron_trapdoor" => Some(910),
		"minecraft:item_frame" => Some(1375),
		"minecraft:jack_o_lantern" => Some(432),
		"minecraft:jigsaw" => Some(998),
		"minecraft:jukebox" => Some(416),
		"minecraft:jungle_boat" => Some(981),
		"minecraft:jungle_button" => Some(861),
		"minecraft:jungle_chest_boat" => Some(982),
		"minecraft:jungle_door" => Some(892),
		"minecraft:jungle_fence" => Some(420),
		"minecraft:jungle_fence_gate" => Some(935),
		"minecraft:jungle_hanging_sign" => Some(1118),
		"minecraft:jungle_leaves" => Some(218),
		"minecraft:jungle_log" => Some(166),
		"minecraft:jungle_planks" => Some(66),
		"minecraft:jungle_pressure_plate" => Some(878),
		"minecraft:jungle_pyramid_map" => Some(1242),
		"minecraft:jungle_sapling" => Some(80),
		"minecraft:jungle_shelf" => Some(383),
		"minecraft:jungle_sign" => Some(1105),
		"minecraft:jungle_slab" => Some(344),
		"minecraft:jungle_stairs" => Some(518),
		"minecraft:jungle_trapdoor" => Some(914),
		"minecraft:jungle_wood" => Some(206),
		"minecraft:kelp" => Some(328),
		"minecraft:knowledge_book" => Some(1458),
		"minecraft:ladder" => Some(408),
		"minecraft:lantern" => Some(1515),
		"minecraft:lapis_block" => Some(233),
		"minecraft:lapis_lazuli" => Some(1014),
		"minecraft:lapis_ore" => Some(105),
		"minecraft:large_amethyst_bud" => Some(1569),
		"minecraft:large_fern" => Some(604),
		"minecraft:lava_bucket" => Some(1130),
		"minecraft:lead" => Some(1412),
		"minecraft:leaf_litter" => Some(331),
		"minecraft:leather" => Some(1133),
		"minecraft:leather_boots" => Some(1071),
		"minecraft:leather_chestplate" => Some(1069),
		"minecraft:leather_helmet" => Some(1068),
		"minecraft:leather_horse_armor" => Some(1411),
		"minecraft:leather_leggings" => Some(1070),
		"minecraft:lectern" => Some(837),
		"minecraft:lever" => Some(839),
		"minecraft:light" => Some(578),
		"minecraft:light_blue_banner" => Some(1420),
		"minecraft:light_blue_bed" => Some(1222),
		"minecraft:light_blue_bundle" => Some(1157),
		"minecraft:light_blue_candle" => Some(1554),
		"minecraft:light_blue_carpet" => Some(583),
		"minecraft:light_blue_concrete" => Some(692),
		"minecraft:light_blue_concrete_powder" => Some(740),
		"minecraft:light_blue_concrete_slab" => Some(724),
		"minecraft:light_blue_concrete_stairs" => Some(708),
		"minecraft:light_blue_cushion" => Some(1173),
		"minecraft:light_blue_dye" => Some(1202),
		"minecraft:light_blue_glazed_terracotta" => Some(676),
		"minecraft:light_blue_harness" => Some(953),
		"minecraft:light_blue_shulker_box" => Some(660),
		"minecraft:light_blue_stained_glass" => Some(608),
		"minecraft:light_blue_stained_glass_pane" => Some(624),
		"minecraft:light_blue_terracotta" => Some(564),
		"minecraft:light_blue_wool" => Some(253),
		"minecraft:light_blue_wool_slab" => Some(285),
		"minecraft:light_blue_wool_stairs" => Some(269),
		"minecraft:light_gray_banner" => Some(1425),
		"minecraft:light_gray_bed" => Some(1227),
		"minecraft:light_gray_bundle" => Some(1162),
		"minecraft:light_gray_candle" => Some(1559),
		"minecraft:light_gray_carpet" => Some(588),
		"minecraft:light_gray_concrete" => Some(697),
		"minecraft:light_gray_concrete_powder" => Some(745),
		"minecraft:light_gray_concrete_slab" => Some(729),
		"minecraft:light_gray_concrete_stairs" => Some(713),
		"minecraft:light_gray_cushion" => Some(1178),
		"minecraft:light_gray_dye" => Some(1207),
		"minecraft:light_gray_glazed_terracotta" => Some(681),
		"minecraft:light_gray_harness" => Some(958),
		"minecraft:light_gray_shulker_box" => Some(665),
		"minecraft:light_gray_stained_glass" => Some(613),
		"minecraft:light_gray_stained_glass_pane" => Some(629),
		"minecraft:light_gray_terracotta" => Some(569),
		"minecraft:light_gray_wool" => Some(258),
		"minecraft:light_gray_wool_slab" => Some(290),
		"minecraft:light_gray_wool_stairs" => Some(274),
		"minecraft:light_weighted_pressure_plate" => Some(873),
		"minecraft:lightning_rod" => Some(840),
		"minecraft:lilac" => Some(600),
		"minecraft:lily_of_the_valley" => Some(312),
		"minecraft:lily_pad" => Some(497),
		"minecraft:lime_banner" => Some(1422),
		"minecraft:lime_bed" => Some(1224),
		"minecraft:lime_bundle" => Some(1159),
		"minecraft:lime_candle" => Some(1556),
		"minecraft:lime_carpet" => Some(585),
		"minecraft:lime_concrete" => Some(694),
		"minecraft:lime_concrete_powder" => Some(742),
		"minecraft:lime_concrete_slab" => Some(726),
		"minecraft:lime_concrete_stairs" => Some(710),
		"minecraft:lime_cushion" => Some(1175),
		"minecraft:lime_dye" => Some(1204),
		"minecraft:lime_glazed_terracotta" => Some(678),
		"minecraft:lime_harness" => Some(955),
		"minecraft:lime_shulker_box" => Some(662),
		"minecraft:lime_stained_glass" => Some(610),
		"minecraft:lime_stained_glass_pane" => Some(626),
		"minecraft:lime_terracotta" => Some(566),
		"minecraft:lime_wool" => Some(255),
		"minecraft:lime_wool_slab" => Some(287),
		"minecraft:lime_wool_stairs" => Some(271),
		"minecraft:lingering_potion" => Some(1445),
		"minecraft:llama_spawn_egg" => Some(1296),
		"minecraft:lodestone" => Some(1535),
		"minecraft:loom" => Some(1493),
		"minecraft:mace" => Some(1374),
		"minecraft:magenta_banner" => Some(1419),
		"minecraft:magenta_bed" => Some(1221),
		"minecraft:magenta_bundle" => Some(1156),
		"minecraft:magenta_candle" => Some(1553),
		"minecraft:magenta_carpet" => Some(582),
		"minecraft:magenta_concrete" => Some(691),
		"minecraft:magenta_concrete_powder" => Some(739),
		"minecraft:magenta_concrete_slab" => Some(723),
		"minecraft:magenta_concrete_stairs" => Some(707),
		"minecraft:magenta_cushion" => Some(1172),
		"minecraft:magenta_dye" => Some(1201),
		"minecraft:magenta_glazed_terracotta" => Some(675),
		"minecraft:magenta_harness" => Some(952),
		"minecraft:magenta_shulker_box" => Some(659),
		"minecraft:magenta_stained_glass" => Some(607),
		"minecraft:magenta_stained_glass_pane" => Some(623),
		"minecraft:magenta_terracotta" => Some(563),
		"minecraft:magenta_wool" => Some(252),
		"minecraft:magenta_wool_slab" => Some(284),
		"minecraft:magenta_wool_stairs" => Some(268),
		"minecraft:magma_block" => Some(650),
		"minecraft:magma_cream" => Some(1275),
		"minecraft:magma_cube_spawn_egg" => Some(1358),
		"minecraft:mangrove_boat" => Some(991),
		"minecraft:mangrove_button" => Some(866),
		"minecraft:mangrove_chest_boat" => Some(992),
		"minecraft:mangrove_door" => Some(897),
		"minecraft:mangrove_fence" => Some(425),
		"minecraft:mangrove_fence_gate" => Some(940),
		"minecraft:mangrove_hanging_sign" => Some(1123),
		"minecraft:mangrove_leaves" => Some(223),
		"minecraft:mangrove_log" => Some(171),
		"minecraft:mangrove_planks" => Some(71),
		"minecraft:mangrove_pressure_plate" => Some(883),
		"minecraft:mangrove_propagule" => Some(85),
		"minecraft:mangrove_roots" => Some(173),
		"minecraft:mangrove_shelf" => Some(384),
		"minecraft:mangrove_sign" => Some(1110),
		"minecraft:mangrove_slab" => Some(349),
		"minecraft:mangrove_stairs" => Some(523),
		"minecraft:mangrove_trapdoor" => Some(919),
		"minecraft:mangrove_wood" => Some(211),
		"minecraft:map" => Some(1382),
		"minecraft:medium_amethyst_bud" => Some(1568),
		"minecraft:melon" => Some(483),
		"minecraft:melon_seeds" => Some(1259),
		"minecraft:melon_slice" => Some(1256),
		"minecraft:milk_bucket" => Some(1134),
		"minecraft:minecart" => Some(966),
		"minecraft:miner_pottery_sherd" => Some(1612),
		"minecraft:mojang_banner_pattern" => Some(1497),
		"minecraft:mooshroom_spawn_egg" => Some(1314),
		"minecraft:moss_block" => Some(333),
		"minecraft:moss_carpet" => Some(332),
		"minecraft:mossy_cobblestone" => Some(393),
		"minecraft:mossy_cobblestone_slab" => Some(810),
		"minecraft:mossy_cobblestone_stairs" => Some(792),
		"minecraft:mossy_cobblestone_wall" => Some(532),
		"minecraft:mossy_stone_brick_slab" => Some(808),
		"minecraft:mossy_stone_brick_stairs" => Some(790),
		"minecraft:mossy_stone_brick_wall" => Some(536),
		"minecraft:mossy_stone_bricks" => Some(450),
		"minecraft:mourner_pottery_sherd" => Some(1613),
		"minecraft:mud" => Some(59),
		"minecraft:mud_brick_slab" => Some(363),
		"minecraft:mud_brick_stairs" => Some(495),
		"minecraft:mud_brick_wall" => Some(539),
		"minecraft:mud_bricks" => Some(454),
		"minecraft:muddy_mangrove_roots" => Some(174),
		"minecraft:mule_spawn_egg" => Some(1287),
		"minecraft:mushroom_stem" => Some(463),
		"minecraft:mushroom_stew" => Some(1061),
		"minecraft:music_disc_11" => Some(1474),
		"minecraft:music_disc_13" => Some(1460),
		"minecraft:music_disc_5" => Some(1478),
		"minecraft:music_disc_blocks" => Some(1462),
		"minecraft:music_disc_bounce" => Some(1463),
		"minecraft:music_disc_cat" => Some(1461),
		"minecraft:music_disc_chirp" => Some(1464),
		"minecraft:music_disc_creator" => Some(1465),
		"minecraft:music_disc_creator_music_box" => Some(1466),
		"minecraft:music_disc_far" => Some(1467),
		"minecraft:music_disc_lava_chicken" => Some(1468),
		"minecraft:music_disc_mall" => Some(1469),
		"minecraft:music_disc_mellohi" => Some(1470),
		"minecraft:music_disc_otherside" => Some(1476),
		"minecraft:music_disc_pigstep" => Some(1479),
		"minecraft:music_disc_precipice" => Some(1480),
		"minecraft:music_disc_relic" => Some(1477),
		"minecraft:music_disc_stal" => Some(1471),
		"minecraft:music_disc_strad" => Some(1472),
		"minecraft:music_disc_tears" => Some(1481),
		"minecraft:music_disc_wait" => Some(1475),
		"minecraft:music_disc_ward" => Some(1473),
		"minecraft:mutton" => Some(1415),
		"minecraft:mycelium" => Some(496),
		"minecraft:name_tag" => Some(1413),
		"minecraft:nautilus_shell" => Some(1484),
		"minecraft:nautilus_spawn_egg" => Some(1306),
		"minecraft:nether_brick" => Some(1396),
		"minecraft:nether_brick_fence" => Some(501),
		"minecraft:nether_brick_slab" => Some(364),
		"minecraft:nether_brick_stairs" => Some(502),
		"minecraft:nether_brick_wall" => Some(540),
		"minecraft:nether_bricks" => Some(498),
		"minecraft:nether_gold_ore" => Some(109),
		"minecraft:nether_quartz_ore" => Some(110),
		"minecraft:nether_sprouts" => Some(324),
		"minecraft:nether_star" => Some(1391),
		"minecraft:nether_wart" => Some(1269),
		"minecraft:nether_wart_block" => Some(651),
		"minecraft:netherite_axe" => Some(1058),
		"minecraft:netherite_block" => Some(130),
		"minecraft:netherite_boots" => Some(1095),
		"minecraft:netherite_chestplate" => Some(1093),
		"minecraft:netherite_helmet" => Some(1092),
		"minecraft:netherite_hoe" => Some(1059),
		"minecraft:netherite_horse_armor" => Some(1410),
		"minecraft:netherite_ingot" => Some(1023),
		"minecraft:netherite_leggings" => Some(1094),
		"minecraft:netherite_nautilus_armor" => Some(1488),
		"minecraft:netherite_pickaxe" => Some(1057),
		"minecraft:netherite_scrap" => Some(1024),
		"minecraft:netherite_shovel" => Some(1056),
		"minecraft:netherite_spear" => Some(1453),
		"minecraft:netherite_sword" => Some(1055),
		"minecraft:netherite_upgrade_smithing_template" => Some(1579),
		"minecraft:netherrack" => Some(433),
		"minecraft:note_block" => Some(855),
		"minecraft:oak_boat" => Some(975),
		"minecraft:oak_button" => Some(858),
		"minecraft:oak_chest_boat" => Some(976),
		"minecraft:oak_door" => Some(889),
		"minecraft:oak_fence" => Some(417),
		"minecraft:oak_fence_gate" => Some(932),
		"minecraft:oak_hanging_sign" => Some(1115),
		"minecraft:oak_leaves" => Some(215),
		"minecraft:oak_log" => Some(163),
		"minecraft:oak_planks" => Some(63),
		"minecraft:oak_pressure_plate" => Some(875),
		"minecraft:oak_sapling" => Some(77),
		"minecraft:oak_shelf" => Some(386),
		"minecraft:oak_sign" => Some(1102),
		"minecraft:oak_slab" => Some(341),
		"minecraft:oak_stairs" => Some(515),
		"minecraft:oak_trapdoor" => Some(911),
		"minecraft:oak_wood" => Some(203),
		"minecraft:observer" => Some(833),
		"minecraft:obsidian" => Some(394),
		"minecraft:ocean_monument_map" => Some(1239),
		"minecraft:ocelot_spawn_egg" => Some(1297),
		"minecraft:ochre_froglight" => Some(1573),
		"minecraft:ominous_bottle" => Some(1657),
		"minecraft:ominous_trial_key" => Some(1655),
		"minecraft:open_eyeblossom" => Some(300),
		"minecraft:orange_banner" => Some(1418),
		"minecraft:orange_bed" => Some(1220),
		"minecraft:orange_bundle" => Some(1155),
		"minecraft:orange_candle" => Some(1552),
		"minecraft:orange_carpet" => Some(581),
		"minecraft:orange_concrete" => Some(690),
		"minecraft:orange_concrete_powder" => Some(738),
		"minecraft:orange_concrete_slab" => Some(722),
		"minecraft:orange_concrete_stairs" => Some(706),
		"minecraft:orange_cushion" => Some(1171),
		"minecraft:orange_dye" => Some(1200),
		"minecraft:orange_glazed_terracotta" => Some(674),
		"minecraft:orange_harness" => Some(951),
		"minecraft:orange_poplar_leaves" => Some(225),
		"minecraft:orange_shulker_box" => Some(658),
		"minecraft:orange_stained_glass" => Some(606),
		"minecraft:orange_stained_glass_pane" => Some(622),
		"minecraft:orange_terracotta" => Some(562),
		"minecraft:orange_tulip" => Some(307),
		"minecraft:orange_wool" => Some(251),
		"minecraft:orange_wool_slab" => Some(283),
		"minecraft:orange_wool_stairs" => Some(267),
		"minecraft:oxeye_daisy" => Some(310),
		"minecraft:oxidized_chiseled_copper" => Some(134),
		"minecraft:oxidized_copper" => Some(123),
		"minecraft:oxidized_copper_bars" => Some(468),
		"minecraft:oxidized_copper_bulb" => Some(1632),
		"minecraft:oxidized_copper_chain" => Some(477),
		"minecraft:oxidized_copper_chest" => Some(1640),
		"minecraft:oxidized_copper_door" => Some(905),
		"minecraft:oxidized_copper_golem_statue" => Some(1648),
		"minecraft:oxidized_copper_grate" => Some(1624),
		"minecraft:oxidized_copper_lantern" => Some(1520),
		"minecraft:oxidized_copper_trapdoor" => Some(927),
		"minecraft:oxidized_cut_copper" => Some(142),
		"minecraft:oxidized_cut_copper_slab" => Some(158),
		"minecraft:oxidized_cut_copper_stairs" => Some(150),
		"minecraft:oxidized_lightning_rod" => Some(843),
		"minecraft:packed_ice" => Some(597),
		"minecraft:packed_mud" => Some(453),
		"minecraft:painting" => Some(1099),
		"minecraft:pale_hanging_moss" => Some(335),
		"minecraft:pale_moss_block" => Some(336),
		"minecraft:pale_moss_carpet" => Some(334),
		"minecraft:pale_oak_boat" => Some(989),
		"minecraft:pale_oak_button" => Some(865),
		"minecraft:pale_oak_chest_boat" => Some(990),
		"minecraft:pale_oak_door" => Some(896),
		"minecraft:pale_oak_fence" => Some(424),
		"minecraft:pale_oak_fence_gate" => Some(939),
		"minecraft:pale_oak_hanging_sign" => Some(1122),
		"minecraft:pale_oak_leaves" => Some(222),
		"minecraft:pale_oak_log" => Some(169),
		"minecraft:pale_oak_planks" => Some(70),
		"minecraft:pale_oak_pressure_plate" => Some(882),
		"minecraft:pale_oak_sapling" => Some(84),
		"minecraft:pale_oak_shelf" => Some(387),
		"minecraft:pale_oak_sign" => Some(1109),
		"minecraft:pale_oak_slab" => Some(348),
		"minecraft:pale_oak_stairs" => Some(522),
		"minecraft:pale_oak_trapdoor" => Some(918),
		"minecraft:pale_oak_wood" => Some(209),
		"minecraft:panda_spawn_egg" => Some(1298),
		"minecraft:paper" => Some(1145),
		"minecraft:parched_spawn_egg" => Some(1327),
		"minecraft:parrot_spawn_egg" => Some(1289),
		"minecraft:pearlescent_froglight" => Some(1575),
		"minecraft:peony" => Some(602),
		"minecraft:petrified_oak_slab" => Some(359),
		"minecraft:phantom_membrane" => Some(973),
		"minecraft:phantom_spawn_egg" => Some(1344),
		"minecraft:pig_spawn_egg" => Some(1282),
		"minecraft:piglin_banner_pattern" => Some(1499),
		"minecraft:piglin_brute_spawn_egg" => Some(1360),
		"minecraft:piglin_head" => Some(1390),
		"minecraft:piglin_spawn_egg" => Some(1359),
		"minecraft:pillager_spawn_egg" => Some(1350),
		"minecraft:pink_banner" => Some(1423),
		"minecraft:pink_bed" => Some(1225),
		"minecraft:pink_bundle" => Some(1160),
		"minecraft:pink_candle" => Some(1557),
		"minecraft:pink_carpet" => Some(586),
		"minecraft:pink_concrete" => Some(695),
		"minecraft:pink_concrete_powder" => Some(743),
		"minecraft:pink_concrete_slab" => Some(727),
		"minecraft:pink_concrete_stairs" => Some(711),
		"minecraft:pink_cushion" => Some(1176),
		"minecraft:pink_dye" => Some(1205),
		"minecraft:pink_glazed_terracotta" => Some(679),
		"minecraft:pink_harness" => Some(956),
		"minecraft:pink_petals" => Some(329),
		"minecraft:pink_shulker_box" => Some(663),
		"minecraft:pink_stained_glass" => Some(611),
		"minecraft:pink_stained_glass_pane" => Some(627),
		"minecraft:pink_terracotta" => Some(567),
		"minecraft:pink_tulip" => Some(309),
		"minecraft:pink_wool" => Some(256),
		"minecraft:pink_wool_slab" => Some(288),
		"minecraft:pink_wool_stairs" => Some(272),
		"minecraft:piston" => Some(829),
		"minecraft:pitcher_plant" => Some(315),
		"minecraft:pitcher_pod" => Some(1437),
		"minecraft:plains_village_map" => Some(1245),
		"minecraft:player_head" => Some(1386),
		"minecraft:plenty_pottery_sherd" => Some(1614),
		"minecraft:podzol" => Some(57),
		"minecraft:pointed_dripstone" => Some(1571),
		"minecraft:poisonous_potato" => Some(1381),
		"minecraft:polar_bear_spawn_egg" => Some(1299),
		"minecraft:polished_andesite" => Some(7),
		"minecraft:polished_andesite_slab" => Some(817),
		"minecraft:polished_andesite_stairs" => Some(800),
		"minecraft:polished_basalt" => Some(437),
		"minecraft:polished_blackstone" => Some(1541),
		"minecraft:polished_blackstone_brick_slab" => Some(1546),
		"minecraft:polished_blackstone_brick_stairs" => Some(1547),
		"minecraft:polished_blackstone_brick_wall" => Some(548),
		"minecraft:polished_blackstone_bricks" => Some(1545),
		"minecraft:polished_blackstone_button" => Some(857),
		"minecraft:polished_blackstone_pressure_plate" => Some(872),
		"minecraft:polished_blackstone_slab" => Some(1542),
		"minecraft:polished_blackstone_stairs" => Some(1543),
		"minecraft:polished_blackstone_wall" => Some(547),
		"minecraft:polished_cinnabar" => Some(44),
		"minecraft:polished_cinnabar_slab" => Some(45),
		"minecraft:polished_cinnabar_stairs" => Some(46),
		"minecraft:polished_cinnabar_wall" => Some(47),
		"minecraft:polished_deepslate" => Some(10),
		"minecraft:polished_deepslate_slab" => Some(820),
		"minecraft:polished_deepslate_stairs" => Some(803),
		"minecraft:polished_deepslate_wall" => Some(550),
		"minecraft:polished_diorite" => Some(5),
		"minecraft:polished_diorite_slab" => Some(809),
		"minecraft:polished_diorite_stairs" => Some(791),
		"minecraft:polished_granite" => Some(3),
		"minecraft:polished_granite_slab" => Some(806),
		"minecraft:polished_granite_stairs" => Some(788),
		"minecraft:polished_sulfur" => Some(31),
		"minecraft:polished_sulfur_slab" => Some(32),
		"minecraft:polished_sulfur_stairs" => Some(33),
		"minecraft:polished_sulfur_wall" => Some(34),
		"minecraft:polished_tuff" => Some(17),
		"minecraft:polished_tuff_slab" => Some(18),
		"minecraft:polished_tuff_stairs" => Some(19),
		"minecraft:polished_tuff_wall" => Some(20),
		"minecraft:poplar_boat" => Some(993),
		"minecraft:poplar_button" => Some(867),
		"minecraft:poplar_chest_boat" => Some(994),
		"minecraft:poplar_door" => Some(898),
		"minecraft:poplar_fence" => Some(426),
		"minecraft:poplar_fence_gate" => Some(941),
		"minecraft:poplar_hanging_sign" => Some(1124),
		"minecraft:poplar_log" => Some(172),
		"minecraft:poplar_planks" => Some(72),
		"minecraft:poplar_pressure_plate" => Some(884),
		"minecraft:poplar_sapling" => Some(86),
		"minecraft:poplar_shelf" => Some(385),
		"minecraft:poplar_sign" => Some(1111),
		"minecraft:poplar_slab" => Some(350),
		"minecraft:poplar_stairs" => Some(524),
		"minecraft:poplar_trapdoor" => Some(920),
		"minecraft:poplar_wood" => Some(212),
		"minecraft:popped_chorus_fruit" => Some(1435),
		"minecraft:poppy" => Some(302),
		"minecraft:porkchop" => Some(1097),
		"minecraft:potato" => Some(1379),
		"minecraft:potent_sulfur" => Some(27),
		"minecraft:potion" => Some(1271),
		"minecraft:powder_snow_bucket" => Some(1131),
		"minecraft:powered_rail" => Some(945),
		"minecraft:prismarine" => Some(637),
		"minecraft:prismarine_brick_slab" => Some(370),
		"minecraft:prismarine_brick_stairs" => Some(641),
		"minecraft:prismarine_bricks" => Some(638),
		"minecraft:prismarine_crystals" => Some(1399),
		"minecraft:prismarine_shard" => Some(1398),
		"minecraft:prismarine_slab" => Some(369),
		"minecraft:prismarine_stairs" => Some(640),
		"minecraft:prismarine_wall" => Some(534),
		"minecraft:prize_pottery_sherd" => Some(1615),
		"minecraft:pufferfish" => Some(1193),
		"minecraft:pufferfish_bucket" => Some(1135),
		"minecraft:pufferfish_spawn_egg" => Some(1307),
		"minecraft:pumpkin" => Some(430),
		"minecraft:pumpkin_pie" => Some(1392),
		"minecraft:pumpkin_seeds" => Some(1258),
		"minecraft:purple_banner" => Some(1427),
		"minecraft:purple_bed" => Some(1229),
		"minecraft:purple_bundle" => Some(1164),
		"minecraft:purple_candle" => Some(1561),
		"minecraft:purple_carpet" => Some(590),
		"minecraft:purple_concrete" => Some(699),
		"minecraft:purple_concrete_powder" => Some(747),
		"minecraft:purple_concrete_slab" => Some(731),
		"minecraft:purple_concrete_stairs" => Some(715),
		"minecraft:purple_cushion" => Some(1180),
		"minecraft:purple_dye" => Some(1209),
		"minecraft:purple_glazed_terracotta" => Some(683),
		"minecraft:purple_harness" => Some(960),
		"minecraft:purple_shulker_box" => Some(667),
		"minecraft:purple_stained_glass" => Some(615),
		"minecraft:purple_stained_glass_pane" => Some(631),
		"minecraft:purple_terracotta" => Some(571),
		"minecraft:purple_wool" => Some(260),
		"minecraft:purple_wool_slab" => Some(292),
		"minecraft:purple_wool_stairs" => Some(276),
		"minecraft:purpur_block" => Some(399),
		"minecraft:purpur_pillar" => Some(400),
		"minecraft:purpur_slab" => Some(368),
		"minecraft:purpur_stairs" => Some(401),
		"minecraft:quartz" => Some(1015),
		"minecraft:quartz_block" => Some(557),
		"minecraft:quartz_bricks" => Some(558),
		"minecraft:quartz_pillar" => Some(559),
		"minecraft:quartz_slab" => Some(365),
		"minecraft:quartz_stairs" => Some(560),
		"minecraft:rabbit" => Some(1400),
		"minecraft:rabbit_foot" => Some(1403),
		"minecraft:rabbit_hide" => Some(1404),
		"minecraft:rabbit_spawn_egg" => Some(1300),
		"minecraft:rabbit_stew" => Some(1402),
		"minecraft:rail" => Some(947),
		"minecraft:raiser_armor_trim_smithing_template" => Some(1594),
		"minecraft:ravager_spawn_egg" => Some(1351),
		"minecraft:raw_copper" => Some(1019),
		"minecraft:raw_copper_block" => Some(114),
		"minecraft:raw_gold" => Some(1021),
		"minecraft:raw_gold_block" => Some(115),
		"minecraft:raw_iron" => Some(1017),
		"minecraft:raw_iron_block" => Some(113),
		"minecraft:recovery_compass" => Some(1152),
		"minecraft:red_banner" => Some(1431),
		"minecraft:red_bed" => Some(1233),
		"minecraft:red_bundle" => Some(1168),
		"minecraft:red_candle" => Some(1565),
		"minecraft:red_carpet" => Some(594),
		"minecraft:red_concrete" => Some(703),
		"minecraft:red_concrete_powder" => Some(751),
		"minecraft:red_concrete_slab" => Some(735),
		"minecraft:red_concrete_stairs" => Some(719),
		"minecraft:red_cushion" => Some(1184),
		"minecraft:red_dye" => Some(1213),
		"minecraft:red_glazed_terracotta" => Some(687),
		"minecraft:red_harness" => Some(964),
		"minecraft:red_mushroom" => Some(318),
		"minecraft:red_mushroom_block" => Some(462),
		"minecraft:red_nether_brick_slab" => Some(816),
		"minecraft:red_nether_brick_stairs" => Some(799),
		"minecraft:red_nether_brick_wall" => Some(542),
		"minecraft:red_nether_bricks" => Some(653),
		"minecraft:red_poplar_leaves" => Some(224),
		"minecraft:red_sand" => Some(91),
		"minecraft:red_sandstone" => Some(644),
		"minecraft:red_sandstone_slab" => Some(366),
		"minecraft:red_sandstone_stairs" => Some(647),
		"minecraft:red_sandstone_wall" => Some(535),
		"minecraft:red_shrub" => Some(241),
		"minecraft:red_shulker_box" => Some(671),
		"minecraft:red_stained_glass" => Some(619),
		"minecraft:red_stained_glass_pane" => Some(635),
		"minecraft:red_terracotta" => Some(575),
		"minecraft:red_tulip" => Some(306),
		"minecraft:red_wool" => Some(264),
		"minecraft:red_wool_slab" => Some(296),
		"minecraft:red_wool_stairs" => Some(280),
		"minecraft:redstone" => Some(824),
		"minecraft:redstone_block" => Some(826),
		"minecraft:redstone_lamp" => Some(854),
		"minecraft:redstone_ore" => Some(101),
		"minecraft:redstone_torch" => Some(825),
		"minecraft:reinforced_deepslate" => Some(460),
		"minecraft:repeater" => Some(827),
		"minecraft:repeating_command_block" => Some(648),
		"minecraft:resin_block" => Some(487),
		"minecraft:resin_brick" => Some(1397),
		"minecraft:resin_brick_slab" => Some(490),
		"minecraft:resin_brick_stairs" => Some(489),
		"minecraft:resin_brick_wall" => Some(491),
		"minecraft:resin_bricks" => Some(488),
		"minecraft:resin_clump" => Some(486),
		"minecraft:respawn_anchor" => Some(1549),
		"minecraft:rib_armor_trim_smithing_template" => Some(1589),
		"minecraft:rooted_dirt" => Some(58),
		"minecraft:rose_bush" => Some(601),
		"minecraft:rotten_flesh" => Some(1264),
		"minecraft:saddle" => Some(949),
		"minecraft:salmon" => Some(1191),
		"minecraft:salmon_bucket" => Some(1136),
		"minecraft:salmon_spawn_egg" => Some(1308),
		"minecraft:sand" => Some(88),
		"minecraft:sandstone" => Some(234),
		"minecraft:sandstone_slab" => Some(357),
		"minecraft:sandstone_stairs" => Some(512),
		"minecraft:sandstone_wall" => Some(543),
		"minecraft:savanna_village_map" => Some(1246),
		"minecraft:scaffolding" => Some(823),
		"minecraft:scrape_pottery_sherd" => Some(1616),
		"minecraft:sculk" => Some(503),
		"minecraft:sculk_catalyst" => Some(505),
		"minecraft:sculk_sensor" => Some(849),
		"minecraft:sculk_shrieker" => Some(506),
		"minecraft:sculk_vein" => Some(504),
		"minecraft:sea_lantern" => Some(643),
		"minecraft:sea_pickle" => Some(249),
		"minecraft:seagrass" => Some(248),
		"minecraft:sentry_armor_trim_smithing_template" => Some(1580),
		"minecraft:shaper_armor_trim_smithing_template" => Some(1592),
		"minecraft:sheaf_pottery_sherd" => Some(1617),
		"minecraft:shears" => Some(1255),
		"minecraft:sheep_spawn_egg" => Some(1283),
		"minecraft:shelf_mushroom" => Some(319),
		"minecraft:shelter_pottery_sherd" => Some(1618),
		"minecraft:shield" => Some(1446),
		"minecraft:short_dry_grass" => Some(246),
		"minecraft:short_grass" => Some(238),
		"minecraft:shroomlight" => Some(1529),
		"minecraft:shulker_box" => Some(656),
		"minecraft:shulker_shell" => Some(1455),
		"minecraft:shulker_spawn_egg" => Some(1367),
		"minecraft:silence_armor_trim_smithing_template" => Some(1593),
		"minecraft:silverfish_spawn_egg" => Some(1345),
		"minecraft:skeleton_horse_spawn_egg" => Some(1329),
		"minecraft:skeleton_skull" => Some(1384),
		"minecraft:skeleton_spawn_egg" => Some(1328),
		"minecraft:skull_banner_pattern" => Some(1496),
		"minecraft:skull_pottery_sherd" => Some(1619),
		"minecraft:slime_ball" => Some(1147),
		"minecraft:slime_block" => Some(831),
		"minecraft:slime_spawn_egg" => Some(1346),
		"minecraft:small_amethyst_bud" => Some(1567),
		"minecraft:small_dripleaf" => Some(339),
		"minecraft:smithing_table" => Some(1512),
		"minecraft:smoker" => Some(1507),
		"minecraft:smooth_basalt" => Some(438),
		"minecraft:smooth_quartz" => Some(372),
		"minecraft:smooth_quartz_slab" => Some(813),
		"minecraft:smooth_quartz_stairs" => Some(796),
		"minecraft:smooth_red_sandstone" => Some(373),
		"minecraft:smooth_red_sandstone_slab" => Some(807),
		"minecraft:smooth_red_sandstone_stairs" => Some(789),
		"minecraft:smooth_sandstone" => Some(374),
		"minecraft:smooth_sandstone_slab" => Some(812),
		"minecraft:smooth_sandstone_stairs" => Some(795),
		"minecraft:smooth_stone" => Some(375),
		"minecraft:smooth_stone_slab" => Some(356),
		"minecraft:sniffer_egg" => Some(754),
		"minecraft:sniffer_spawn_egg" => Some(1315),
		"minecraft:snort_pottery_sherd" => Some(1620),
		"minecraft:snout_armor_trim_smithing_template" => Some(1588),
		"minecraft:snow" => Some(410),
		"minecraft:snow_block" => Some(412),
		"minecraft:snow_golem_spawn_egg" => Some(1319),
		"minecraft:snowball" => Some(1132),
		"minecraft:snowy_village_map" => Some(1247),
		"minecraft:soul_campfire" => Some(1528),
		"minecraft:soul_lantern" => Some(1516),
		"minecraft:soul_sand" => Some(434),
		"minecraft:soul_soil" => Some(435),
		"minecraft:soul_torch" => Some(439),
		"minecraft:spawner" => Some(402),
		"minecraft:spectral_arrow" => Some(1443),
		"minecraft:spider_eye" => Some(1272),
		"minecraft:spider_spawn_egg" => Some(1338),
		"minecraft:spire_armor_trim_smithing_template" => Some(1590),
		"minecraft:splash_potion" => Some(1442),
		"minecraft:sponge" => Some(229),
		"minecraft:spore_blossom" => Some(316),
		"minecraft:spruce_boat" => Some(977),
		"minecraft:spruce_button" => Some(859),
		"minecraft:spruce_chest_boat" => Some(978),
		"minecraft:spruce_door" => Some(890),
		"minecraft:spruce_fence" => Some(418),
		"minecraft:spruce_fence_gate" => Some(933),
		"minecraft:spruce_hanging_sign" => Some(1116),
		"minecraft:spruce_leaves" => Some(216),
		"minecraft:spruce_log" => Some(164),
		"minecraft:spruce_planks" => Some(64),
		"minecraft:spruce_pressure_plate" => Some(876),
		"minecraft:spruce_sapling" => Some(78),
		"minecraft:spruce_shelf" => Some(388),
		"minecraft:spruce_sign" => Some(1103),
		"minecraft:spruce_slab" => Some(342),
		"minecraft:spruce_stairs" => Some(516),
		"minecraft:spruce_trapdoor" => Some(912),
		"minecraft:spruce_wood" => Some(204),
		"minecraft:spyglass" => Some(1188),
		"minecraft:squid_spawn_egg" => Some(1309),
		"minecraft:stick" => Some(1060),
		"minecraft:sticky_piston" => Some(830),
		"minecraft:stone" => Some(1),
		"minecraft:stone_axe" => Some(1038),
		"minecraft:stone_brick_slab" => Some(362),
		"minecraft:stone_brick_stairs" => Some(494),
		"minecraft:stone_brick_wall" => Some(538),
		"minecraft:stone_bricks" => Some(449),
		"minecraft:stone_button" => Some(856),
		"minecraft:stone_hoe" => Some(1039),
		"minecraft:stone_pickaxe" => Some(1037),
		"minecraft:stone_pressure_plate" => Some(871),
		"minecraft:stone_shovel" => Some(1036),
		"minecraft:stone_slab" => Some(355),
		"minecraft:stone_spear" => Some(1448),
		"minecraft:stone_stairs" => Some(794),
		"minecraft:stone_sword" => Some(1035),
		"minecraft:stonecutter" => Some(1513),
		"minecraft:straw_bed" => Some(1235),
		"minecraft:stray_spawn_egg" => Some(1330),
		"minecraft:strider_spawn_egg" => Some(1361),
		"minecraft:string" => Some(1062),
		"minecraft:stripped_acacia_log" => Some(182),
		"minecraft:stripped_acacia_wood" => Some(194),
		"minecraft:stripped_bamboo_block" => Some(202),
		"minecraft:stripped_birch_log" => Some(180),
		"minecraft:stripped_birch_wood" => Some(192),
		"minecraft:stripped_cherry_log" => Some(183),
		"minecraft:stripped_cherry_wood" => Some(195),
		"minecraft:stripped_crimson_hyphae" => Some(200),
		"minecraft:stripped_crimson_stem" => Some(188),
		"minecraft:stripped_dark_oak_log" => Some(184),
		"minecraft:stripped_dark_oak_wood" => Some(196),
		"minecraft:stripped_jungle_log" => Some(181),
		"minecraft:stripped_jungle_wood" => Some(193),
		"minecraft:stripped_mangrove_log" => Some(186),
		"minecraft:stripped_mangrove_wood" => Some(198),
		"minecraft:stripped_oak_log" => Some(178),
		"minecraft:stripped_oak_wood" => Some(190),
		"minecraft:stripped_pale_oak_log" => Some(185),
		"minecraft:stripped_pale_oak_wood" => Some(197),
		"minecraft:stripped_poplar_log" => Some(187),
		"minecraft:stripped_poplar_wood" => Some(199),
		"minecraft:stripped_spruce_log" => Some(179),
		"minecraft:stripped_spruce_wood" => Some(191),
		"minecraft:stripped_warped_hyphae" => Some(201),
		"minecraft:stripped_warped_stem" => Some(189),
		"minecraft:structure_block" => Some(997),
		"minecraft:structure_void" => Some(655),
		"minecraft:sugar" => Some(1217),
		"minecraft:sugar_cane" => Some(327),
		"minecraft:sulfur" => Some(26),
		"minecraft:sulfur_brick_slab" => Some(36),
		"minecraft:sulfur_brick_stairs" => Some(37),
		"minecraft:sulfur_brick_wall" => Some(38),
		"minecraft:sulfur_bricks" => Some(35),
		"minecraft:sulfur_cube_bucket" => Some(1140),
		"minecraft:sulfur_cube_spawn_egg" => Some(1316),
		"minecraft:sulfur_slab" => Some(28),
		"minecraft:sulfur_spike" => Some(1572),
		"minecraft:sulfur_stairs" => Some(29),
		"minecraft:sulfur_wall" => Some(30),
		"minecraft:sunflower" => Some(599),
		"minecraft:suspicious_gravel" => Some(90),
		"minecraft:suspicious_sand" => Some(89),
		"minecraft:suspicious_stew" => Some(1492),
		"minecraft:swamp_hut_map" => Some(1243),
		"minecraft:sweet_berries" => Some(1525),
		"minecraft:tadpole_bucket" => Some(1141),
		"minecraft:tadpole_spawn_egg" => Some(1310),
		"minecraft:taiga_village_map" => Some(1248),
		"minecraft:tall_dry_grass" => Some(247),
		"minecraft:tall_grass" => Some(603),
		"minecraft:target" => Some(838),
		"minecraft:terracotta" => Some(596),
		"minecraft:test_block" => Some(999),
		"minecraft:test_instance_block" => Some(1000),
		"minecraft:tide_armor_trim_smithing_template" => Some(1587),
		"minecraft:tinted_glass" => Some(232),
		"minecraft:tipped_arrow" => Some(1444),
		"minecraft:tnt" => Some(853),
		"minecraft:tnt_minecart" => Some(969),
		"minecraft:torch" => Some(395),
		"minecraft:torchflower" => Some(314),
		"minecraft:torchflower_seeds" => Some(1436),
		"minecraft:totem_of_undying" => Some(1454),
		"minecraft:trader_llama_spawn_egg" => Some(1320),
		"minecraft:trapped_chest" => Some(852),
		"minecraft:trial_key" => Some(1654),
		"minecraft:trial_spawner" => Some(1653),
		"minecraft:trident" => Some(1483),
		"minecraft:tripwire_hook" => Some(851),
		"minecraft:tropical_fish" => Some(1192),
		"minecraft:tropical_fish_bucket" => Some(1138),
		"minecraft:tropical_fish_spawn_egg" => Some(1311),
		"minecraft:tube_coral" => Some(766),
		"minecraft:tube_coral_block" => Some(761),
		"minecraft:tube_coral_fan" => Some(776),
		"minecraft:tuff" => Some(12),
		"minecraft:tuff_brick_slab" => Some(22),
		"minecraft:tuff_brick_stairs" => Some(23),
		"minecraft:tuff_brick_wall" => Some(24),
		"minecraft:tuff_bricks" => Some(21),
		"minecraft:tuff_slab" => Some(13),
		"minecraft:tuff_stairs" => Some(14),
		"minecraft:tuff_wall" => Some(15),
		"minecraft:turtle_egg" => Some(753),
		"minecraft:turtle_helmet" => Some(1001),
		"minecraft:turtle_scute" => Some(1002),
		"minecraft:turtle_spawn_egg" => Some(1312),
		"minecraft:twisting_vines" => Some(326),
		"minecraft:vault" => Some(1656),
		"minecraft:verdant_froglight" => Some(1574),
		"minecraft:vex_armor_trim_smithing_template" => Some(1586),
		"minecraft:vex_spawn_egg" => Some(1353),
		"minecraft:villager_spawn_egg" => Some(1321),
		"minecraft:vindicator_spawn_egg" => Some(1352),
		"minecraft:vine" => Some(484),
		"minecraft:wandering_trader_spawn_egg" => Some(1322),
		"minecraft:ward_armor_trim_smithing_template" => Some(1584),
		"minecraft:warden_spawn_egg" => Some(1347),
		"minecraft:warm_ocean_ruins_map" => Some(1254),
		"minecraft:warped_button" => Some(870),
		"minecraft:warped_door" => Some(901),
		"minecraft:warped_fence" => Some(429),
		"minecraft:warped_fence_gate" => Some(944),
		"minecraft:warped_fungus" => Some(321),
		"minecraft:warped_fungus_on_a_stick" => Some(972),
		"minecraft:warped_hanging_sign" => Some(1127),
		"minecraft:warped_hyphae" => Some(214),
		"minecraft:warped_nylium" => Some(61),
		"minecraft:warped_planks" => Some(75),
		"minecraft:warped_pressure_plate" => Some(887),
		"minecraft:warped_roots" => Some(323),
		"minecraft:warped_shelf" => Some(389),
		"minecraft:warped_sign" => Some(1114),
		"minecraft:warped_slab" => Some(354),
		"minecraft:warped_stairs" => Some(528),
		"minecraft:warped_stem" => Some(176),
		"minecraft:warped_trapdoor" => Some(923),
		"minecraft:warped_wart_block" => Some(652),
		"minecraft:water_bucket" => Some(1129),
		"minecraft:waxed_chiseled_copper" => Some(135),
		"minecraft:waxed_copper_bars" => Some(469),
		"minecraft:waxed_copper_block" => Some(124),
		"minecraft:waxed_copper_bulb" => Some(1633),
		"minecraft:waxed_copper_chain" => Some(478),
		"minecraft:waxed_copper_chest" => Some(1641),
		"minecraft:waxed_copper_door" => Some(906),
		"minecraft:waxed_copper_golem_statue" => Some(1649),
		"minecraft:waxed_copper_grate" => Some(1625),
		"minecraft:waxed_copper_lantern" => Some(1521),
		"minecraft:waxed_copper_trapdoor" => Some(928),
		"minecraft:waxed_cut_copper" => Some(143),
		"minecraft:waxed_cut_copper_slab" => Some(159),
		"minecraft:waxed_cut_copper_stairs" => Some(151),
		"minecraft:waxed_exposed_chiseled_copper" => Some(136),
		"minecraft:waxed_exposed_copper" => Some(125),
		"minecraft:waxed_exposed_copper_bars" => Some(470),
		"minecraft:waxed_exposed_copper_bulb" => Some(1634),
		"minecraft:waxed_exposed_copper_chain" => Some(479),
		"minecraft:waxed_exposed_copper_chest" => Some(1642),
		"minecraft:waxed_exposed_copper_door" => Some(907),
		"minecraft:waxed_exposed_copper_golem_statue" => Some(1650),
		"minecraft:waxed_exposed_copper_grate" => Some(1626),
		"minecraft:waxed_exposed_copper_lantern" => Some(1522),
		"minecraft:waxed_exposed_copper_trapdoor" => Some(929),
		"minecraft:waxed_exposed_cut_copper" => Some(144),
		"minecraft:waxed_exposed_cut_copper_slab" => Some(160),
		"minecraft:waxed_exposed_cut_copper_stairs" => Some(152),
		"minecraft:waxed_exposed_lightning_rod" => Some(845),
		"minecraft:waxed_lightning_rod" => Some(844),
		"minecraft:waxed_oxidized_chiseled_copper" => Some(138),
		"minecraft:waxed_oxidized_copper" => Some(127),
		"minecraft:waxed_oxidized_copper_bars" => Some(472),
		"minecraft:waxed_oxidized_copper_bulb" => Some(1636),
		"minecraft:waxed_oxidized_copper_chain" => Some(481),
		"minecraft:waxed_oxidized_copper_chest" => Some(1644),
		"minecraft:waxed_oxidized_copper_door" => Some(909),
		"minecraft:waxed_oxidized_copper_golem_statue" => Some(1652),
		"minecraft:waxed_oxidized_copper_grate" => Some(1628),
		"minecraft:waxed_oxidized_copper_lantern" => Some(1524),
		"minecraft:waxed_oxidized_copper_trapdoor" => Some(931),
		"minecraft:waxed_oxidized_cut_copper" => Some(146),
		"minecraft:waxed_oxidized_cut_copper_slab" => Some(162),
		"minecraft:waxed_oxidized_cut_copper_stairs" => Some(154),
		"minecraft:waxed_oxidized_lightning_rod" => Some(847),
		"minecraft:waxed_weathered_chiseled_copper" => Some(137),
		"minecraft:waxed_weathered_copper" => Some(126),
		"minecraft:waxed_weathered_copper_bars" => Some(471),
		"minecraft:waxed_weathered_copper_bulb" => Some(1635),
		"minecraft:waxed_weathered_copper_chain" => Some(480),
		"minecraft:waxed_weathered_copper_chest" => Some(1643),
		"minecraft:waxed_weathered_copper_door" => Some(908),
		"minecraft:waxed_weathered_copper_golem_statue" => Some(1651),
		"minecraft:waxed_weathered_copper_grate" => Some(1627),
		"minecraft:waxed_weathered_copper_lantern" => Some(1523),
		"minecraft:waxed_weathered_copper_trapdoor" => Some(930),
		"minecraft:waxed_weathered_cut_copper" => Some(145),
		"minecraft:waxed_weathered_cut_copper_slab" => Some(161),
		"minecraft:waxed_weathered_cut_copper_stairs" => Some(153),
		"minecraft:waxed_weathered_lightning_rod" => Some(846),
		"minecraft:wayfinder_armor_trim_smithing_template" => Some(1591),
		"minecraft:weathered_chiseled_copper" => Some(133),
		"minecraft:weathered_copper" => Some(122),
		"minecraft:weathered_copper_bars" => Some(467),
		"minecraft:weathered_copper_bulb" => Some(1631),
		"minecraft:weathered_copper_chain" => Some(476),
		"minecraft:weathered_copper_chest" => Some(1639),
		"minecraft:weathered_copper_door" => Some(904),
		"minecraft:weathered_copper_golem_statue" => Some(1647),
		"minecraft:weathered_copper_grate" => Some(1623),
		"minecraft:weathered_copper_lantern" => Some(1519),
		"minecraft:weathered_copper_trapdoor" => Some(926),
		"minecraft:weathered_cut_copper" => Some(141),
		"minecraft:weathered_cut_copper_slab" => Some(157),
		"minecraft:weathered_cut_copper_stairs" => Some(149),
		"minecraft:weathered_lightning_rod" => Some(842),
		"minecraft:weeping_vines" => Some(325),
		"minecraft:wet_sponge" => Some(230),
		"minecraft:wheat" => Some(1066),
		"minecraft:wheat_seeds" => Some(1065),
		"minecraft:white_banner" => Some(1417),
		"minecraft:white_bed" => Some(1219),
		"minecraft:white_bundle" => Some(1154),
		"minecraft:white_candle" => Some(1551),
		"minecraft:white_carpet" => Some(580),
		"minecraft:white_concrete" => Some(689),
		"minecraft:white_concrete_powder" => Some(737),
		"minecraft:white_concrete_slab" => Some(721),
		"minecraft:white_concrete_stairs" => Some(705),
		"minecraft:white_cushion" => Some(1170),
		"minecraft:white_dye" => Some(1199),
		"minecraft:white_glazed_terracotta" => Some(673),
		"minecraft:white_harness" => Some(950),
		"minecraft:white_shulker_box" => Some(657),
		"minecraft:white_stained_glass" => Some(605),
		"minecraft:white_stained_glass_pane" => Some(621),
		"minecraft:white_terracotta" => Some(561),
		"minecraft:white_tulip" => Some(308),
		"minecraft:white_wool" => Some(250),
		"minecraft:white_wool_slab" => Some(282),
		"minecraft:white_wool_stairs" => Some(266),
		"minecraft:wild_armor_trim_smithing_template" => Some(1583),
		"minecraft:wildflowers" => Some(330),
		"minecraft:wind_charge" => Some(1370),
		"minecraft:witch_spawn_egg" => Some(1348),
		"minecraft:wither_rose" => Some(313),
		"minecraft:wither_skeleton_skull" => Some(1385),
		"minecraft:wither_skeleton_spawn_egg" => Some(1332),
		"minecraft:wither_spawn_egg" => Some(1331),
		"minecraft:wolf_armor" => Some(1004),
		"minecraft:wolf_spawn_egg" => Some(1290),
		"minecraft:wooden_axe" => Some(1028),
		"minecraft:wooden_hoe" => Some(1029),
		"minecraft:wooden_pickaxe" => Some(1027),
		"minecraft:wooden_shovel" => Some(1026),
		"minecraft:wooden_spear" => Some(1447),
		"minecraft:wooden_sword" => Some(1025),
		"minecraft:woodland_mansion_map" => Some(1240),
		"minecraft:writable_book" => Some(1371),
		"minecraft:written_book" => Some(1372),
		"minecraft:yellow_banner" => Some(1421),
		"minecraft:yellow_bed" => Some(1223),
		"minecraft:yellow_bundle" => Some(1158),
		"minecraft:yellow_candle" => Some(1555),
		"minecraft:yellow_carpet" => Some(584),
		"minecraft:yellow_concrete" => Some(693),
		"minecraft:yellow_concrete_powder" => Some(741),
		"minecraft:yellow_concrete_slab" => Some(725),
		"minecraft:yellow_concrete_stairs" => Some(709),
		"minecraft:yellow_cushion" => Some(1174),
		"minecraft:yellow_dye" => Some(1203),
		"minecraft:yellow_glazed_terracotta" => Some(677),
		"minecraft:yellow_harness" => Some(954),
		"minecraft:yellow_poplar_leaves" => Some(226),
		"minecraft:yellow_shulker_box" => Some(661),
		"minecraft:yellow_stained_glass" => Some(609),
		"minecraft:yellow_stained_glass_pane" => Some(625),
		"minecraft:yellow_terracotta" => Some(565),
		"minecraft:yellow_wool" => Some(254),
		"minecraft:yellow_wool_slab" => Some(286),
		"minecraft:yellow_wool_stairs" => Some(270),
		"minecraft:zoglin_spawn_egg" => Some(1362),
		"minecraft:zombie_head" => Some(1387),
		"minecraft:zombie_horse_spawn_egg" => Some(1334),
		"minecraft:zombie_nautilus_spawn_egg" => Some(1335),
		"minecraft:zombie_spawn_egg" => Some(1333),
		"minecraft:zombie_villager_spawn_egg" => Some(1336),
		"minecraft:zombified_piglin_spawn_egg" => Some(1363),
    _ => None,
	};
}
pub fn get_items() -> HashMap<&'static str, Item> {
	let mut items = HashMap::new();
	items.insert("minecraft:dropper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 836, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 284, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 935, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 871, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 177, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_porkchop", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1098, nutrition: Some(8), saturation: Some(12.80), tool_rules: vec![] });
	items.insert("minecraft:ghast_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1355, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1639, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dune_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1581, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 893, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 696, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 342, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_deepslate_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 456, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 529, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:buried_treasure_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1249, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 568, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 687, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 749, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pufferfish_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1135, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:dandelion", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 298, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glowstone_dust", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1189, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 567, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 438, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:abandoned_camp_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1253, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_birch_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 180, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 781, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 288, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cave_spider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1337, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:furnace_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 968, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1074, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 69, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bucket", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1128, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:porkchop", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1097, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:cobweb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 237, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 632, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beehive", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1532, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_crimson_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 188, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:campfire", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1527, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pufferfish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1307, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mutton", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1415, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:white_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 561, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 645, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_cinnabar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 52, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 220, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1417, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:parched_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1327, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:powered_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 945, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:experience_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1368, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:twisting_vines", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 326, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 203, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:llama_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1296, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 297, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heart_of_the_sea", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1490, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zoglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1362, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 575, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 233, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1539, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1190, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:honey_bottle", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1533, nutrition: Some(6), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:emerald_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 103, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 266, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:amethyst_shard", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1016, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:azalea_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 227, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 714, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 573, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:reinforced_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 460, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 531, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cauldron", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1277, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:player_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1386, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 932, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 572, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 535, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:elder_guardian_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1342, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1036, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:packed_mud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 453, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1270, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spawner", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 402, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 348, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1268, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 518, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 516, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:debug_stick", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1459, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:frogspawn", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1576, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1318, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 844, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 731, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1076, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 933, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1543, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1229, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:straw_bed", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1235, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1069, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_carrot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1383, nutrition: Some(6), saturation: Some(14.40), tool_rules: vec![] });
	items.insert("minecraft:totem_of_undying", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1454, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 813, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 158, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 675, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 551, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fletching_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1510, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ocelot_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1297, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_item_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1376, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_wart", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1269, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:axolotl_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1301, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1173, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_dark_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 196, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 657, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:decorated_pot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 392, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 533, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 82, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_lamp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 854, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1429, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:villager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1321, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 613, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sniffer_egg", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 754, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enchanted_golden_apple", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1101, nutrition: Some(4), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:blue_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1428, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 900, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clock", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1187, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:breeze_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1339, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lilac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 600, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:podzol", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 57, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1420, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1524, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 399, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 35, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 543, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 214, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 842, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 289, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:structure_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 997, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1123, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_chicken", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1263, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:azure_bluet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 305, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1184, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wayfinder_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1591, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 160, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1124, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_lazuli", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1014, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:apple", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1007, nutrition: Some(4), saturation: Some(2.40), tool_rules: vec![] });
	items.insert("minecraft:infested_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 448, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 843, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 88, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1625, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 467, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:observer", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 833, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1644, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1423, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_head", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 1389, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:parrot_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1289, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:baked_potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1380, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:mourner_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1613, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_cinnabar_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 45, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 374, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 70, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:grass_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 54, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hoglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1357, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 917, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1541, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:filled_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1238, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1209, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombified_piglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1363, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1550, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1171, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1515, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 788, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1185, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 764, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_petals", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 329, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 153, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1638, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 436, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_weighted_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 873, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 574, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 499, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 765, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 171, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 422, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1094, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sunflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 599, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 24, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lily_of_the_valley", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 312, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_ball", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1147, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 13, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 877, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hopper_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 970, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 120, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 364, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_crystal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1433, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 983, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 141, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 377, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1102, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:plains_village_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1245, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:field_masoned_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1502, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tall_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 603, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 780, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 906, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 741, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1050, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:bolt_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1597, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coarse_dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 56, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 429, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 170, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cactus_flower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 414, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 885, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 452, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 478, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 6, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 509, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rooted_dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 58, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brush", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1578, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vault", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1656, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vex_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1353, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 617, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 746, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1035, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:waxed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 143, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 669, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 891, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1632, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1388, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 826, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_cracked_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 446, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:comparator", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 828, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 897, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 886, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:target", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 838, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_nautilus_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1335, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_warped_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 201, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1561, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1221, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 966, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1154, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bell", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1514, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 210, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beetroot_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1439, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 685, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:short_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 238, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1114, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1085, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 14, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 763, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 255, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fishing_rod", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1186, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1121, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:small_dripleaf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 339, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_nautilus_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1487, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mushroom_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1061, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:cobblestone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 360, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_cinnabar_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 46, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cookie", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1236, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:sentry_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1580, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 439, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 366, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1516, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:buried_ancient_city_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1250, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_sensor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 849, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:phantom_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1344, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sweet_berries", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1525, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:poplar_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 86, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_hanging_moss", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 335, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 251, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pearlescent_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1575, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 717, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1227, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 909, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 418, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 908, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 821, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 76, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fungus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 321, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 581, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 870, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 16, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blade_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1601, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 215, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1205, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:evoker_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1349, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 681, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 671, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 656, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:big_dripleaf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 338, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_5", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1478, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1165, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 483, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1108, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1408, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:danger_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1604, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_salmon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1195, nutrition: Some(6), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:sticky_piston", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 830, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trapped_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 852, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 938, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:damaged_anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 555, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 357, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 17, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1166, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 513, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 691, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:strider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1361, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon_slice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1256, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 373, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1359, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 176, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 879, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1201, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shroomlight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1529, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 918, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 361, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 526, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:emerald", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1013, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 611, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 147, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1157, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 368, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 724, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 866, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1068, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_pale_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 185, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 806, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crying_obsidian", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1536, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_rose", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 313, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 862, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:amethyst_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 117, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:arms_up_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1600, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 980, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:breeze_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1373, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 959, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1552, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 280, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_acacia_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 194, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ancient_debris", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 111, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_chiseled_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 447, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 443, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heavy_core", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 116, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:test_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 999, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1160, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1456, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dried_kelp_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1144, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 798, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:explorer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1605, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beetroot_soup", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1440, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:cooked_cod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1194, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:orange_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 674, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone_meal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1215, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 709, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_otherside", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1476, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1409, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 278, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jack_o_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 432, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_nugget", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1457, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1225, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 381, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 466, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_quartz_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 556, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 285, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_cinnabar_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 47, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spider_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1338, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1081, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 10, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guster_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1608, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 991, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 952, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ink_sac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1196, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 65, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chicken", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1262, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:angler_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1598, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 845, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1207, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 640, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1634, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gravel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 92, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:writable_book", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1371, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 125, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 961, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dispenser", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 835, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_diamond_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 108, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_far", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1467, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trader_llama_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1320, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 590, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_spike", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1572, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1112, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 979, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_polished_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1544, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gunpowder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1064, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 22, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:popped_chorus_fruit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1435, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1142, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 498, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 15, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 81, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 4, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 560, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 434, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 540, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 975, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 264, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1106, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 350, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:potent_sulfur", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 27, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 887, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_moss_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 336, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_bamboo_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 202, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 778, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pumpkin_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1258, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 804, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 512, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 100, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 583, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 273, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clay", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 415, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1278, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cactus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 413, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1018, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_mossy_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 445, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 591, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1178, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 869, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fungus", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 320, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1627, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1213, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armadillo_scute", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1003, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 592, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_chirp", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1464, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_poplar_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 224, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 465, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_propagule", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 85, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wandering_trader_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1322, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 586, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:taiga_village_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1248, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:calibrated_sculk_sensor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 850, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 66, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 404, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 235, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 683, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_mangrove_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 198, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 747, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 562, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1224, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 64, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_13", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1460, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dried_kelp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1257, nutrition: Some(1), saturation: Some(0.60), tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1648, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:knowledge_book", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1458, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 175, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 614, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 565, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 926, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 279, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shears", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1255, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:shears_extreme_breaking_speed"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:shears_major_breaking_speed"], correct_for_drops: true, speed: Some(5.0)},ToolRule {blocks: vec!["#minecraft:shears_minor_breaking_speed"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:deepslate_redstone_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 102, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_wart_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 652, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 867, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honeycomb_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1534, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_mellohi", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1470, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1647, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:eye_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1585, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 428, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1637, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:torchflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 314, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 677, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 726, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smoker", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1507, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1596, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 48, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 921, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tnt_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 969, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:repeating_command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 648, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ominous_bottle", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1657, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_creator", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1465, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1624, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 937, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1542, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 500, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 270, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guardian_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1343, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heavy_weighted_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 874, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_mushroom_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 462, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1023, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1551, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 941, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 239, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 725, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 521, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_cube_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1316, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 863, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 960, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 912, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 295, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 907, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1234, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 450, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_charge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1369, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firework_rocket", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1393, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lapis_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 105, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1164, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 99, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coast_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1582, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 127, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 389, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 522, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 911, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 631, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blaze_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1274, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1113, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_warped_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 189, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:barrel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1506, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 619, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 905, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1329, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 258, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 790, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1204, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:closed_eyeblossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 301, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 875, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 612, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 923, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 700, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_lichen", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 485, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_berries", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1526, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:snow_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1319, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 236, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 26, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 680, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1233, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honey_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 832, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:salmon", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1191, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:white_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 580, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1030, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:netherite_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1453, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 814, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_blocks", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1462, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 688, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gilded_blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1540, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 661, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ominous_trial_key", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1655, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:name_tag", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1413, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fungus_on_a_stick", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 972, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 786, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1091, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 207, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tnt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 853, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 49, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 358, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1499, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 810, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spyglass", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1188, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 129, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_resin_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 492, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 190, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 43, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 664, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 396, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 28, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1073, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:howl_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1611, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1214, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1554, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pumpkin", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 430, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_pillar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 400, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 133, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1090, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hanging_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 337, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1562, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1126, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snout_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1588, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_mangrove_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 186, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:compass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1151, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 340, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1450, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 868, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1563, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:medium_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1568, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 963, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1557, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 283, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tropical_fish", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1192, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:bamboo_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1125, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 388, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 122, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 670, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 146, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1120, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 779, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 799, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1635, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 78, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 721, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:verdant_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1574, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1636, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:moss_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 332, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1093, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 889, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chipped_anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 554, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1025, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:nether_wart_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 651, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 231, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1155, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:book", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1146, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 259, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poppy", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 302, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_catalyst", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 505, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_dark_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 184, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1083, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 660, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1646, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:respawn_anchor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1549, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sea_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 643, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 23, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1034, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:budding_amethyst", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 118, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 756, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wildflowers", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 330, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 77, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 802, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weeping_vines", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 325, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 793, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_birch_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 192, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:milk_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1134, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 131, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 797, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:desert_village_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1244, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lava_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1130, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1048, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:red_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 91, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_mutton", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1416, nutrition: Some(6), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:gray_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1206, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1200, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 145, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bookshelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 390, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 992, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1104, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 831, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 777, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1053, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:lodestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1535, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 346, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 858, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1110, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 606, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 950, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 628, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 694, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 876, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 901, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1148, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1212, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1642, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creaking_heart", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 403, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 371, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poisonous_potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1381, nutrition: Some(2), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:blaze_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1266, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:note_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 855, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 276, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 464, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 547, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trial_spawner", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1653, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pillager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1350, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1001, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 7, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1547, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shield", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1446, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 625, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 294, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1054, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:iron_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 473, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 633, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 93, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1500, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:powder_snow_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1131, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:scaffolding", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 823, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 380, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 344, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lever", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 839, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 444, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wolf_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1290, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 166, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1049, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:purple_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 715, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:written_book", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1372, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1060, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1451, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_cherry_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 183, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1633, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1176, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 62, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1033, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:purple_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1427, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 421, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_gold_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 109, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 957, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1095, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:command_block_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1414, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 987, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sugar_cane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 327, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1009, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 773, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 159, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 164, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 162, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 59, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sugar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1217, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1029, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:end_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 510, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 261, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 732, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 962, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_crystals", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1399, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1537, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 607, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 173, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 895, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 564, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1232, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_ink_sac", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1197, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_boots", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1079, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 644, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fire_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 769, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 161, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1522, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 169, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 794, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1153, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 667, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 211, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 517, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:paper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1145, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_granite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 3, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_skeleton_skull", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1385, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 480, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1651, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1220, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1226, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 51, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chorus_fruit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1434, nutrition: Some(4), saturation: Some(2.40), tool_rules: vec![] });
	items.insert("minecraft:birch_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 205, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 532, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jigsaw", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 998, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_vein", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 504, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_lapis_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 106, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1645, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_foot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1403, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 537, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_poplar_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 225, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 698, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:donkey_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1285, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 154, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:savanna_village_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1246, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 639, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1521, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_spruce_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 179, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bowl", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1006, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 569, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 156, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 936, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tropical_fish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1311, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shaper_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1592, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cocoa_beans", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1198, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:swamp_hut_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1243, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_wait", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1475, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cod_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1302, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 223, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 846, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 736, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:desert_pyramid_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1252, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 168, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 41, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 269, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_iron_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 113, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_coal_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 94, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 990, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 30, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 152, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_dragon_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1364, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1174, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mule_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1287, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:obsidian", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 394, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1410, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_jungle_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 181, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 494, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 882, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 796, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 618, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 767, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 213, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 899, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 477, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 80, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 955, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 459, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 128, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flow_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1606, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_shell", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1455, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1418, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mycelium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 496, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 219, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1119, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flint", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1096, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_upgrade_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1579, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1520, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 924, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1028, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:green_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 686, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 563, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 653, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:peony", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 602, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1558, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1163, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 890, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1431, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smithing_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1512, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1026, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:acacia_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 345, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1111, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:barrier", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 577, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blast_furnace", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1508, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1039, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:bamboo_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 427, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 20, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 782, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1027, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_wooden_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(2.0)},] });
	items.insert("minecraft:scrape_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1616, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pig_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1282, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 856, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1559, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 860, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 97, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:large_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1569, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:string", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1062, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1051, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:moss_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 333, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1626, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warden_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1347, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1183, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1037, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:lime_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 626, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 84, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 425, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 841, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1042, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:golden_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1088, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1649, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 942, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 668, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk_shrieker", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 506, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_mosaic_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 352, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 916, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 840, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crafter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1237, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 578, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1495, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 8, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1396, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:loom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1493, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1538, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 362, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1397, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 95, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 880, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_skull", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1384, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 706, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_beef", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1261, nutrition: Some(8), saturation: Some(12.80), tool_rules: vec![] });
	items.insert("minecraft:music_disc_mall", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1469, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 12, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_nylium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 61, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 615, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_11", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1474, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1230, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 355, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tuff_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 21, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jukebox", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 416, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_golem_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1317, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lead", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1412, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 493, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1419, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 650, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_nautilus_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1489, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dirt_path", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 598, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:goat_horn", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1504, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 281, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pitcher_plant", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 315, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_red_sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 646, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 71, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1211, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pointed_dripstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1571, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sniffer_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1315, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_cube_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1358, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 40, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 67, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 101, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 417, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armadillo_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1291, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 914, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1012, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wet_sponge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 230, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:composter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1505, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1556, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 712, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1038, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_stone_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(4.0)},] });
	items.insert("minecraft:allium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 304, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 776, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 896, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 812, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 423, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 944, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cornflower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 311, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 107, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rose_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 601, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 702, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cartography_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1509, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:grindstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1511, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 795, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bogged_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1323, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1056, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:kelp", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 328, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 291, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:echo_shard", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1577, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tiles", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 457, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 148, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flint_and_steel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1005, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 495, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_brute_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1360, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ladder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 408, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spore_blossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 316, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 672, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 558, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 254, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 353, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chainmail_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1077, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 922, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 740, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tadpole_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1141, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 719, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 393, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:honeycomb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1530, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warm_ocean_ruins_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1254, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 904, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 989, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1519, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 638, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 811, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_sulfur_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 32, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 622, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enderman_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1365, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:painting", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1099, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:happy_ghast_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1356, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 411, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:drowned_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1325, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 515, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 263, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 585, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1040, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:music_disc_precipice", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1480, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 75, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 679, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 489, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1180, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pumpkin_pie", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1392, nutrition: Some(8), saturation: Some(4.80), tool_rules: vec![] });
	items.insert("minecraft:dripstone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 53, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 36, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chest_minecart", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 967, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 126, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 743, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 729, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_basalt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 437, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 212, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 785, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crafting_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 405, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_chest_raft", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 996, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_scrap", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1024, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1075, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 420, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_villager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1336, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 369, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1116, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_tears", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1481, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 260, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 157, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:test_instance_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 1000, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1432, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 587, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_iron_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 96, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_bookshelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 391, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 820, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1312, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 881, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chicken_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1280, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 958, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1517, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 934, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polar_bear_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1299, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_cat", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1461, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 710, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 469, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spire_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1590, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cow_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1281, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spider_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1272, nutrition: Some(2), saturation: Some(3.20), tool_rules: vec![] });
	items.insert("minecraft:polished_diorite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 5, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 695, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_copper_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 98, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 588, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1231, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 277, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 474, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snowball", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1132, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creeper_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1341, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 114, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cooked_rabbit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1401, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:poplar_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 426, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 704, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_dandelion", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 299, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 502, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 178, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:plenty_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1614, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 771, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1421, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 545, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:enchanting_table", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 507, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1210, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 977, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glow_squid_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1305, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 988, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 123, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:small_amethyst_bud", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1567, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:creaking_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1340, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 865, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 738, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 124, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 978, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1168, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 155, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1545, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1426, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raiser_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1594, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 308, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 627, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1643, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 18, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 262, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 637, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 593, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crossbow", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1491, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:phantom_membrane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 973, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nautilus_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1306, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_chestplate", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1089, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1045, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:disc_fragment_5", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1482, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flower_pot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1377, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 343, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1133, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shelter_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1618, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tipped_arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1444, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 807, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 571, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_cube_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1140, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1082, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_sand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 89, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 954, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_polished_blackstone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1548, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brewer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1602, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 167, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:miner_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1612, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1149, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 744, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:activator_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 948, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rib_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1589, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:woodland_mansion_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1240, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 349, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 274, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1107, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_deepslate_tiles", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 458, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:azalea", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 242, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blackstone_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 546, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shelf_mushroom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 319, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 693, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1407, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 976, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:infested_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 442, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1175, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_mushroom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 318, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 733, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wooden_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1447, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 663, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1387, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 943, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ward_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1584, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 692, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 925, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 487, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 267, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bubble_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 768, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 140, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1228, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 701, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 472, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:farmland", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 406, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1288, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 471, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_pillar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 559, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blaze_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1354, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1182, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diorite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 801, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1640, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_campfire", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1528, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 538, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_egg", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 511, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piglin_head", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1390, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 760, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 488, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1052, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_diamond_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(8.0)},] });
	items.insert("minecraft:enchanted_book", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1395, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1156, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1621, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 525, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 759, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 629, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1449, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_sword", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1055, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["minecraft:cobweb"], correct_for_drops: true, speed: Some(15.0)},ToolRule {blocks: vec!["#minecraft:sword_instantly_mines"], correct_for_drops: true, speed: Some(3.4028235e38)},ToolRule {blocks: vec!["#minecraft:sword_efficient"], correct_for_drops: true, speed: Some(1.5)},] });
	items.insert("minecraft:jungle_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1118, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snowy_village_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1247, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:camel_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1284, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 135, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1105, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 817, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:husk_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1326, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 930, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 138, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lingering_potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1445, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 982, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 142, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:buried_mineshaft_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1251, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1203, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:melon_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1259, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:salmon_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1136, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:resin_clump", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 486, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:allay_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1313, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glistering_melon_slice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1279, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1058, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:purple_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 292, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 928, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bee_nest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1531, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_prismarine_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 642, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 137, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vex_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1586, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_pyramid_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1242, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 931, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nautilus_shell", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1484, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:elytra", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 974, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_sulfur_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 34, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_iron", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1017, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 544, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:charcoal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1011, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hay_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 579, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 527, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 149, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_acacia_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 182, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 479, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 742, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dried_ghast", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 755, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 208, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 630, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 608, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_lightning_rod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 847, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 727, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1020, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1629, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 956, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1031, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:torchflower_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1436, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 534, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_spruce_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 191, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_egg", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1150, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1177, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_tuff_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 25, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 218, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:saddle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 949, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flowering_azalea_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 228, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 424, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 38, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 410, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 216, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_emerald_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 104, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 524, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1331, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:guster_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1501, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cake", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1218, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:friend_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1607, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 753, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1169, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1223, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1564, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1057, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:mangrove_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 883, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1046, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:chainmail_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1078, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_moss_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 334, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 884, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 132, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 708, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_hide", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1404, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 419, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brain_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 762, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 751, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 244, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fermented_spider_eye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1273, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ender_pearl", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1265, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leaf_litter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 331, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 206, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wind_charge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1370, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vine", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 484, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 550, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_pigstep", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1479, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 272, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 684, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 322, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1158, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 864, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_red_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 789, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 482, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ocean_monument_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1239, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 903, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:potato", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1379, nutrition: Some(1), saturation: Some(0.60), tool_rules: vec![] });
	items.insert("minecraft:cherry_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 520, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 658, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wild_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1583, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 29, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1170, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_brain_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 757, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 665, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firework_star", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1394, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sulfur_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 37, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 711, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1070, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cod_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1137, nutrition: Some(2), saturation: Some(0.40), tool_rules: vec![] });
	items.insert("minecraft:pufferfish", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1193, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:sandstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 234, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:endermite_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1366, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1333, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1059, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_netherite_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(9.0)},] });
	items.insert("minecraft:diorite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 818, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1208, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chain_command_block", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 649, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_star", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 1391, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1103, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 286, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:salmon_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1308, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 898, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1019, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1202, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 730, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beacon", Item { max_stack_size: 64, rarity: ItemRarity::Rare, repair_cost: 0, id: 530, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 951, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_mushroom_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 461, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 376, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brewing_stand", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1276, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1553, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 805, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 965, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 287, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:furnace", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 407, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bordure_indented_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1503, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 481, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dirt", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 55, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 549, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 523, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1072, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 83, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 375, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 9, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 519, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bread", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1067, nutrition: Some(5), saturation: Some(6.00), tool_rules: vec![] });
	items.insert("minecraft:dead_tube_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 775, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:emerald_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 514, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:archer_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1599, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mushroom_stem", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 463, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_deepslate_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 803, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 673, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chiseled_sulfur", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 39, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snow_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 412, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stray_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1330, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 707, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 542, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_cinnabar", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 44, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 676, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wither_skeleton_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1332, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 557, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1422, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 902, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 387, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_stal", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1471, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_relic", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1477, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_gold", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1021, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 165, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:silverfish_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1345, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1425, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:camel_husk_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1324, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 953, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_orchid", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 303, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:feather", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1063, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 163, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1650, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 282, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1127, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 217, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dragon_breath", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1441, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_gravel", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 90, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tall_dry_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 247, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 470, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 929, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 112, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:open_eyeblossom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 300, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 379, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1565, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:hopper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 834, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1219, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 609, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:calcite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 11, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1424, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 595, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_shard", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1398, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1080, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 635, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_harness", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 964, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1109, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 150, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:goat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1295, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lectern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 837, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purpur_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 401, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 134, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 293, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 271, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bow", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1008, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 610, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 256, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 940, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_quartz", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 372, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 824, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 705, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 275, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tropical_fish_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1138, nutrition: Some(1), saturation: Some(0.20), tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 475, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 584, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_nautilus_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1486, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_cobblestone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 792, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 624, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 920, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1300, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:carrot_on_a_stick", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 971, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 634, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 815, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ghast_tear", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1267, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 713, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1630, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 927, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_diorite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 791, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 761, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:panda_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1298, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:granite", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 2, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 748, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1015, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1047, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_iron_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(6.0)},] });
	items.insert("minecraft:skull_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1496, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mojang_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1497, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1122, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_lava_chicken", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1468, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tinted_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 232, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:buried_trial_chambers_map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1241, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_pickaxe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1032, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_copper_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/pickaxe"], correct_for_drops: true, speed: Some(5.0)},] });
	items.insert("minecraft:black_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 720, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 594, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spectral_arrow", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1443, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 919, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 395, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1518, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_cherry_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 195, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1167, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beetroot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1438, nutrition: Some(1), saturation: Some(1.20), tool_rules: vec![] });
	items.insert("minecraft:birch_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1117, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prize_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1615, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:zombie_horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1334, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1566, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 666, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:silence_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 1593, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 750, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_fence_gate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 939, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_fire_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 784, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 50, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_chain", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 476, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 323, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gold_ingot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1022, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 552, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 728, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 536, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_golem_statue", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1652, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 548, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_quartz_ore", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 110, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxidized_copper_bars", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 468, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 913, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_sandstone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 647, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1411, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 250, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:purple_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 699, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_ward", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1473, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 204, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 735, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:detector_rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 946, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1555, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 984, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_bed", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1222, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stonecutter", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1513, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 570, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_chiseled_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 136, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1216, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 252, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 662, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:quartz_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 365, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 566, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_apple", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1100, nutrition: Some(4), saturation: Some(9.60), tool_rules: vec![] });
	items.insert("minecraft:amethyst_cluster", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1570, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 910, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_diorite_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 809, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:water_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1129, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_blue_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 253, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 605, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:yellow_poplar_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 226, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tube_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 766, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 582, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_sulfur", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 31, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:daylight_detector", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 848, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 209, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_cut_copper_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 151, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:slime_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1346, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sculk", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 503, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pink_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 309, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_red_sandstone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 367, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rail", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 947, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sheaf_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1617, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:turtle_scute", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1002, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wolf_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1004, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_jungle_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 193, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1448, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 296, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flower_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1494, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_dye", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1199, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 63, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 490, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1087, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 528, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cracked_stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 451, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:squid_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1309, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stone_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 449, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_nautilus_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1488, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 440, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 121, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bedrock", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 87, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:end_portal_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 508, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 307, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 363, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heartbreak_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1610, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_raft", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 995, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 993, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tripwire_hook", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 851, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_sulfur_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 33, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 341, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:leather_boots", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1071, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:redstone_torch", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 825, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_crimson_hyphae", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 200, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_bulb", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1631, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:splash_potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1442, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobbled_deepslate_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 819, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bee_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1293, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 703, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sea_pickle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 249, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_exposed_cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 144, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:shulker_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1367, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_wool_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 268, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:glowstone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 441, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 722, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:weathered_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1623, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 994, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lily_pad", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 497, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cobblestone_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 409, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:coal", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1010, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_wool_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 290, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:suspicious_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1492, nutrition: Some(6), saturation: Some(7.20), tool_rules: vec![] });
	items.insert("minecraft:bamboo_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 73, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1172, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chorus_plant", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 397, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_nether_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 816, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 894, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horn_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 770, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1181, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 130, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:structure_void", Item { max_stack_size: 64, rarity: ItemRarity::Epic, repair_cost: 0, id: 655, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cinnabar_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 42, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:heart_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1609, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skeleton_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1328, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bubble_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 772, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_chest_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 986, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:snort_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1620, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_spear", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1452, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:orange_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 690, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:air", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 0, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:chorus_flower", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 398, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_nylium", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 60, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1159, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 737, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:short_dry_grass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 246, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:anvil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 553, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bat_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1292, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:smooth_stone_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 356, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:warped_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 354, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trial_key", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1654, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:host_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1595, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_hanging_sign", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1115, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 240, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:petrified_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 359, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 745, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:birch_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 79, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 383, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ravager_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1351, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:map", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1382, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:item_frame", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1375, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1162, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oxeye_daisy", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 310, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cut_copper", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 139, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:vindicator_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1352, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mooshroom_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1314, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:potion", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1271, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 119, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pale_oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 222, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_bundle", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1161, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sponge", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 229, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 172, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_candle", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1560, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:brown_mushroom", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 317, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_tuff_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 19, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:axolotl_bucket", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1139, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 385, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 892, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:tadpole_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1310, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:conduit", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 787, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_pale_oak_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 197, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:carrot", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1378, nutrition: Some(3), saturation: Some(3.60), tool_rules: vec![] });
	items.insert("minecraft:muddy_mangrove_roots", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 174, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_cushion", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1179, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 455, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_door", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 888, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:raw_gold_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 115, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:andesite_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 541, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:copper_horse_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1406, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magma_cream", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1275, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:armor_stand", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1405, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:large_fern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 604, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 739, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_creator_music_box", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1466, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_leggings", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1086, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dolphin_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1303, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:piston", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 829, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:trident", Item { max_stack_size: 1, rarity: ItemRarity::Rare, repair_cost: 0, id: 1483, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:lime_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 678, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_bubble_coral_fan", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 783, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:poplar_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 72, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rotten_flesh", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1264, nutrition: Some(4), saturation: Some(0.80), tool_rules: vec![] });
	items.insert("minecraft:prismarine_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 370, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 347, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:acacia_trapdoor", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 915, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:soul_soil", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 435, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_hoe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1044, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/hoe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:wheat_seeds", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1065, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_weathered_copper_lantern", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1523, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:diamond_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1084, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_banner", Item { max_stack_size: 16, rarity: ItemRarity::Common, repair_cost: 0, id: 1430, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 878, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 378, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:fox_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1294, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_sprouts", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 324, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 636, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_carpet", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 589, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 981, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_shovel", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1041, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/shovel"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:dead_bubble_coral_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 758, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:nether_brick_fence", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 501, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherrack", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 433, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_copper_chest", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1641, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:light_gray_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 697, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:waxed_oxidized_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1628, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:repeater", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 827, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mangrove_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 384, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 68, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:witch_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1348, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:burn_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1603, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:beef", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1260, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:tide_armor_trim_smithing_template", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1587, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:deepslate_tile_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 822, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:packed_ice", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 597, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_leaves", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 221, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:skull_pottery_sherd", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1619, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:magenta_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 723, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 596, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1546, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mossy_stone_brick_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 808, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_poplar_wood", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 199, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mace", Item { max_stack_size: 1, rarity: ItemRarity::Epic, repair_cost: 0, id: 1374, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 616, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_concrete", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 689, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1400, nutrition: Some(3), saturation: Some(1.80), tool_rules: vec![] });
	items.insert("minecraft:magenta_shulker_box", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 659, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:gray_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 257, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_bricks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 454, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_shrub", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 241, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:iron_nautilus_armor", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1485, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:carved_pumpkin", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 431, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_strad", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1472, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:exposed_copper_grate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1622, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:wheat", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1066, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dead_horn_coral", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 774, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:netherite_helmet", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1092, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:blue_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 716, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:rabbit_stew", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1402, nutrition: Some(10), saturation: Some(12.00), tool_rules: vec![] });
	items.insert("minecraft:black_concrete_powder", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 752, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:sheep_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1283, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bone_block", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 654, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:prismarine_brick_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 641, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cyan_glazed_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 682, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:clay_ball", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1143, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:ochre_froglight", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1573, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:horse_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1286, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:crimson_planks", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 74, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:cherry_boat", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 985, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_andesite_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 800, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 734, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_pressure_plate", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 872, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:seagrass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 248, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:bamboo_slab", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 351, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:firefly_bush", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 245, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_terracotta", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 576, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:green_concrete_stairs", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 718, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 386, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:stripped_poplar_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 187, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:mud_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 539, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:music_disc_bounce", Item { max_stack_size: 1, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1463, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:jungle_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 861, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:frog_spawn_egg", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1304, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_stained_glass", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 620, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:spruce_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 859, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:dark_oak_shelf", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 382, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:golden_axe", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1043, nutrition: None, saturation: None, tool_rules: vec![ToolRule {blocks: vec!["#minecraft:incorrect_for_gold_tool"], correct_for_drops: false, speed: None},ToolRule {blocks: vec!["#minecraft:mineable/axe"], correct_for_drops: true, speed: Some(12.0)},] });
	items.insert("minecraft:magenta_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 623, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:polished_blackstone_button", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 857, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:flowering_azalea", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 243, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:black_wool", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 265, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:resin_brick_wall", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 491, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:pitcher_pod", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1437, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:white_stained_glass_pane", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 621, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:globe_banner_pattern", Item { max_stack_size: 1, rarity: ItemRarity::Common, repair_cost: 0, id: 1498, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:red_tulip", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 306, nutrition: None, saturation: None, tool_rules: vec![] });
	items.insert("minecraft:recovery_compass", Item { max_stack_size: 64, rarity: ItemRarity::Uncommon, repair_cost: 0, id: 1152, nutrition: None, saturation: None, tool_rules: vec![] });

	return items;
}
