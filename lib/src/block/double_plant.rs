use super::*;

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	_block_states: &HashMap<String, data::blocks::Block>,
) -> Item {
	let all_items = data::items::get_items();
	if used_tool.id == all_items.get("minecraft:shears").unwrap().id || block.block_name == "pitcher_plant" {
		let item = all_items.get(block.block_name).unwrap_or(all_items.get("minecraft:air").unwrap()).clone();

		return Item {
			id: data::items::get_item_name_by_id(item.id).to_string(),
			count: 1,
			components: Vec::new(),
		};
	} else {
		return Item::default();
	}
}


#[cfg(test)]
mod test {

	#[test]
	fn short_grass_drops_self_with_shears() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:short_grass").unwrap().clone(),
			all_items.get("minecraft:shears").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:short_grass");
		assert_eq!(res.count, 1);
	}

	#[test]
	fn diamond_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();

		let res = super::get_item_drop(
			block_states.get("minecraft:short_grass").unwrap().clone(),
			all_items.get("minecraft:air").unwrap(),
			&block_states,
		);

		assert_eq!(res.id, "minecraft:air");
		assert_eq!(res.count, 0);
	}
}
