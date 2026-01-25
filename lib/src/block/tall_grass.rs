use super::*;

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	_block_states: &HashMap<String, data::blocks::Block>,
) -> Item {
	let all_items = data::items::get_items();
	if used_tool.id == all_items.get("minecraft:shears").unwrap().id {
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
