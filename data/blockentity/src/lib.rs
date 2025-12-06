#![allow(clippy::needless_return)]
pub fn get_block_entity_types() -> std::collections::HashMap<String, u8> {{
	let mut output: std::collections::HashMap<String, u8> = std::collections::HashMap::new();

	output.insert("minecraft:banner".to_string(), 20);
	output.insert("minecraft:barrel".to_string(), 27);
	output.insert("minecraft:beacon".to_string(), 15);
	output.insert("minecraft:bed".to_string(), 25);
	output.insert("minecraft:beehive".to_string(), 34);
	output.insert("minecraft:bell".to_string(), 31);
	output.insert("minecraft:blast_furnace".to_string(), 29);
	output.insert("minecraft:brewing_stand".to_string(), 12);
	output.insert("minecraft:brushable_block".to_string(), 41);
	output.insert("minecraft:calibrated_sculk_sensor".to_string(), 36);
	output.insert("minecraft:campfire".to_string(), 33);
	output.insert("minecraft:chest".to_string(), 1);
	output.insert("minecraft:chiseled_bookshelf".to_string(), 39);
	output.insert("minecraft:command_block".to_string(), 23);
	output.insert("minecraft:comparator".to_string(), 19);
	output.insert("minecraft:conduit".to_string(), 26);
	output.insert("minecraft:copper_golem_statue".to_string(), 48);
	output.insert("minecraft:crafter".to_string(), 43);
	output.insert("minecraft:creaking_heart".to_string(), 10);
	output.insert("minecraft:daylight_detector".to_string(), 17);
	output.insert("minecraft:decorated_pot".to_string(), 42);
	output.insert("minecraft:dispenser".to_string(), 5);
	output.insert("minecraft:dropper".to_string(), 6);
	output.insert("minecraft:enchanting_table".to_string(), 13);
	output.insert("minecraft:end_gateway".to_string(), 22);
	output.insert("minecraft:end_portal".to_string(), 14);
	output.insert("minecraft:ender_chest".to_string(), 3);
	output.insert("minecraft:furnace".to_string(), 0);
	output.insert("minecraft:hanging_sign".to_string(), 8);
	output.insert("minecraft:hopper".to_string(), 18);
	output.insert("minecraft:jigsaw".to_string(), 32);
	output.insert("minecraft:jukebox".to_string(), 4);
	output.insert("minecraft:lectern".to_string(), 30);
	output.insert("minecraft:mob_spawner".to_string(), 9);
	output.insert("minecraft:piston".to_string(), 11);
	output.insert("minecraft:sculk_catalyst".to_string(), 37);
	output.insert("minecraft:sculk_sensor".to_string(), 35);
	output.insert("minecraft:sculk_shrieker".to_string(), 38);
	output.insert("minecraft:shelf".to_string(), 40);
	output.insert("minecraft:shulker_box".to_string(), 24);
	output.insert("minecraft:sign".to_string(), 7);
	output.insert("minecraft:skull".to_string(), 16);
	output.insert("minecraft:smoker".to_string(), 28);
	output.insert("minecraft:structure_block".to_string(), 21);
	output.insert("minecraft:test_block".to_string(), 46);
	output.insert("minecraft:test_instance_block".to_string(), 47);
	output.insert("minecraft:trapped_chest".to_string(), 2);
	output.insert("minecraft:trial_spawner".to_string(), 44);
	output.insert("minecraft:vault".to_string(), 45);

	return output;
}}