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
	}

	match block.block_name {
		"minecraft:diamond_ore" => Item {
			id: "minecraft:diamond".to_string(),
			count: 1,
			components: Vec::new(),
		},
		_ => Item::default(),
	}
}
