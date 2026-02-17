use rand::Rng;

use super::*;

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	_block_states: &HashMap<String, data::blocks::Block>,
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
	} else if data::tags::get_block().get("mineable/pickaxe").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let pickaxes: Vec<i32> = data::tags::get_item().get("pickaxes").unwrap().iter().map(|x| all_items.get(x).unwrap().id).collect();

		if !pickaxes.contains(&used_tool.id) {
			return Item::default();
		}
	}

	let mut rng = rand::rng();

	match block.block_name {
		"minecraft:diamond_ore" => Item {
			id: "minecraft:diamond".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:coal_ore" => Item {
			id: "minecraft:coal".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:copper_ore" => Item {
			id: "minecraft:raw_copper".to_string(),
			count: rng.random_range(2..=5),
			components: Vec::new(),
		},
		"minecraft:deepslate_coal_ore" => Item {
			id: "minecraft:coal".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:deepslate_copper_ore" => Item {
			id: "minecraft:raw_copper".to_string(),
			count: rng.random_range(2..=5),
			components: Vec::new(),
		},
		"minecraft:deepslate_diamond_ore" => Item {
			id: "minecraft:diamond".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:deepslate_emerald_ore" => Item {
			id: "minecraft:emerald".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:deepslate_gold_ore" => Item {
			id: "minecraft:raw_gold".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:deepslate_iron_ore" => Item {
			id: "minecraft:raw_iron".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:deepslate_lapis_ore" => Item {
			id: "minecraft:lapis_lazuli".to_string(),
			count: rng.random_range(4..=9),
			components: Vec::new(),
		},
		"minecraft:emerald_ore" => Item {
			id: "minecraft:emerald".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:gold_ore" => Item {
			id: "minecraft:raw_gold".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:iron_ore" => Item {
			id: "minecraft:raw_iron".to_string(),
			count: 1,
			components: Vec::new(),
		},
		"minecraft:lapis_ore" => Item {
			id: "minecraft:lapis_lazuli".to_string(),
			count: rng.random_range(4..=9),
			components: Vec::new(),
		},
		"minecraft:nether_gold_ore" => Item {
			id: "minecraft:gold_nugget".to_string(),
			count: rng.random_range(2..=6),
			components: Vec::new(),
		},
		"minecraft:nether_quartz_ore" => Item {
			id: "minecraft:quartz".to_string(),
			count: 1,
			components: Vec::new(),
		},
		_ => Item::default(),
	}
}


#[cfg(test)]
mod test {

	#[test]
	fn diamond_ore_drops_diamond_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:diamond_ore").unwrap().clone(),
			all_items.get("minecraft:iron_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:diamond");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn diamond_ore_drops_nothing_with_stone_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:diamond_ore").unwrap().clone(),
			all_items.get("minecraft:stone_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}

	#[test]
	fn diamond_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:diamond_ore").unwrap().clone(),
			all_items.get("minecraft:air").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}

	#[test]
	fn coal_ore_drops_coal_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:coal_ore").unwrap().clone(),
			all_items.get("minecraft:iron_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:coal");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn coal_ore_drops_coal_ore_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:coal_ore").unwrap().clone(),
			all_items.get("minecraft:wooden_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:coal");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn coal_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res =
			super::get_item_drop(block_states.get("minecraft:coal_ore").unwrap().clone(), all_items.get("minecraft:air").unwrap(), &block_states);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}

	#[test]
	fn copper_ore_drops_2_to_5_raw_copper_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let mut counts: Vec<u8> = Vec::new();
		for _ in 0..100 {
			let res = super::get_item_drop(
				block_states.get("minecraft:copper_ore").unwrap().clone(),
				all_items.get("minecraft:iron_pickaxe").unwrap(),
				&block_states,
			);

			assert_eq!(res.id, "minecraft:raw_copper");
			counts.push(res.count);
		}
		counts.sort();
		counts.dedup();
		assert!(counts.contains(&2));
		assert!(counts.contains(&3));
		assert!(counts.contains(&4));
		assert!(counts.contains(&5));
	}

	#[test]
	fn deepslate_coal_ore_drops_coal_ore_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:deepslate_coal_ore").unwrap().clone(),
			all_items.get("minecraft:wooden_pickaxe").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:coal");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn deepslate_coal_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:deepslate_coal_ore").unwrap().clone(),
			all_items.get("minecraft:air").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}
}
