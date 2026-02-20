use rand::Rng;

use super::*;

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	block_states: &HashMap<String, data::blocks::Block>,
) -> Item {
	let all_items = data::items::get_items();
	if data::tags::get_block().get("needs_iron_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let iron_tools = [
			all_items.get("minecraft:iron_pickaxe").unwrap().id,
			all_items.get("minecraft:golden_pickaxe").unwrap().id,
			all_items.get("minecraft:diamond_pickaxe").unwrap().id,
			all_items.get("minecraft:netherite_pickaxe").unwrap().id,
		];

		if !iron_tools.contains(&used_tool.id) {
			return Item::default();
		}
	} else if data::tags::get_block().get("needs_stone_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let stone_tools = [
			all_items.get("minecraft:stone_pickaxe").unwrap().id,
			all_items.get("minecraft:copper_pickaxe").unwrap().id,
			all_items.get("minecraft:iron_pickaxe").unwrap().id,
			all_items.get("minecraft:golden_pickaxe").unwrap().id,
			all_items.get("minecraft:diamond_pickaxe").unwrap().id,
			all_items.get("minecraft:netherite_pickaxe").unwrap().id,
		];

		if !stone_tools.contains(&used_tool.id) {
			return Item::default();
		}
	} else if data::tags::get_block().get("needs_diamond_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let diamond_tools = [all_items.get("minecraft:diamond_pickaxe").unwrap().id, all_items.get("minecraft:netherite_pickaxe").unwrap().id];
		if !diamond_tools.contains(&used_tool.id) {
			return Item::default();
		}
	} else if data::tags::get_block().get("mineable/pickaxe").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let pickaxes: Vec<i32> = data::tags::get_item().get("pickaxes").unwrap().iter().map(|x| all_items.get(x).unwrap().id).collect();

		if !pickaxes.contains(&used_tool.id) {
			return Item::default();
		}
	}

	let mut rng = rand::rng();

	match block.block_name {
		"minecraft:stone" => Item {
			id: "minecraft:cobblestone".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:bedrock" => Item {
			id: "minecraft:dragon_egg".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:bookshelf" => Item {
			id: "minecraft:book".to_string(),
			count: 3,
			components: Vec::new(),
		},
		"minecraft:clay" => Item {
			id: "minecraft:clay_ball".to_string(),
			count: 4,
			components: Vec::new(),
		},
		"minecraft:glowstone" => Item {
			id: "minecraft:clay_ball".to_string(),
			count: 4,
			components: Vec::new(),
		},
		_ => {
			let block_name = block.block_name;
			let item = all_items.get(block_name).unwrap_or(all_items.get("minecraft:air").unwrap()).clone();

			Item {
				id: data::items::get_item_name_by_id(item.id).to_string(),
				count: 1,
				components: Vec::new(),
			}
		}
	}
}

pub fn get_hardness(_block_id: u16, block: data::blocks::Block, _block_states: &HashMap<String, data::blocks::Block>) -> f32 {
	if block.block_name.ends_with("_concrete") {
		return 1.8;
	} else if block.block_name.ends_with("terrcotta") {
		return 1.25;
	} else if block.block_name.ends_with("wool") {
		return 0.8;
	}

	match block.block_name {
		"minecraft:stone" => 1.5,
		"minecraft:andesite" => 1.5,
		"minecraft:ancient_debris" => 30.0,
		"minecraft:bedrock" => -1.0,
		"minecraft:blackstone" => 1.5,
		"minecraft:bookshelf" => 1.5,
		"minecraft:calcite" => 0.75,
		"minecraft:chiseled_deepslate" => 3.5,
		"minecraft:chiseled_polished_blackstone" => 1.5,
		"minecraft:chiseled_blackstone" => 3.5,
		"minecraft:chiseled_quartz_block" => 0.8,
		"minecraft:chiseled_red_sandstone" => 0.8,
		"minecraft:chiseled_resin_bricks" => 1.5,
		"minecraft:chiseled_sandstone" => 1.5,
		"minecraft:chiseled_stone_bricks" => 1.5,
		"minecraft:chiseled_tuff" => 1.5,
		"minecraft:chiseled_tuff_bricks" => 1.5,
		"minecraft:clay" => 0.6,
		"minecraft:coal_block" => 5.0,
		"minecraft:coarse_dirt" => 0.5,
		"minecraft:cobbled_deepslate" => 3.5,
		"minecraft:cracked_deepslate_bricks" => 3.5,
		"minecraft:cracked_deepslate_tiles" => 3.5,
		"minecraft:cracked_polished_blackstone_bricks" => 3.5,
		"minecraft:cracked_stone_bricks" => 1.5,
		"minecraft:cut_red_sandstone" => 0.8,
		"minecraft:cut_sandstone" => 0.8,
		"minecraft:dark_prismarine" => 1.5,
		"minecraft:dead_brain_coral_block" => 1.5,
		_ => 2.0,
	}
}


#[cfg(test)]
mod test {

	#[test]
	fn ancient_debris_drops_itself_with_diamond_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:ancient_debris").unwrap().clone(),
			all_items.get("minecraft:diamond_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:ancient_debris");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn ancient_debris_drops_nothing_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:ancient_debris").unwrap().clone(),
			all_items.get("minecraft:iron_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}
}
