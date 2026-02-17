use super::*;

pub fn get_block_state_id(
	face: u8,
	_cardinal_direction: CardinalDirection,
	_dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	_cursor_position_x: f32,
	_cursor_position_y: f32,
	_cursor_position_z: f32,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let axis = match face {
		0 | 1 => RotatedPillarAxis::Y,
		2 | 3 => RotatedPillarAxis::Z,
		_ => RotatedPillarAxis::X,
	};

	output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(axis.clone()))).unwrap().id, position));

	return output;
}

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	_block_states: &HashMap<String, data::blocks::Block>,
) -> Item {
	let all_items = data::items::get_items();
	let pickaxes: Vec<i32> = data::tags::get_item().get("pickaxes").unwrap().iter().map(|x| all_items.get(x).unwrap().id).collect();

	let needs_to_be_mined_with_wooden_pickaxe = [
		"minecraft:purpur_pillar",
		"minecraft:basalt",
		"minecraft:bone_block",
		"minecraft:deepslate",
		"minecraft:polished_basalt",
		"minecraft:quartz_pillar",
	];
	if needs_to_be_mined_with_wooden_pickaxe.contains(&block.block_name) && !pickaxes.contains(&used_tool.id) {
		return Item::default();
	} else {
		return Item {
			id: block.block_name.to_string(),
			count: 1,
			components: Vec::new(),
		};
	}
}

pub fn get_hardness(_block_id: u16, block: data::blocks::Block, _block_states: &HashMap<String, data::blocks::Block>) -> f32 {
	match block.block_name {
		"minecraft:purpur_pillar" => 1.5,
		"minecraft:basalt" => 1.25,
		"minecraft:polished_basalt" => 1.25,
		"minecraft:deepslate" => 3.0,
		"minecraft:ochre_froglight" => 0.3,
		"minecraft:pearlescent_froglight" => 0.3,
		"minecraft:verdant_froglight" => 0.3,
		"minecraft:quartz_pillar" => 0.8,
		_ => 2.0,
	}
}

#[cfg(test)]
mod test {
	use super::*;

	mod get_block_state_id {
		use super::*;

		#[test]
		fn top() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:oak_log", &block_states);

			let block_state_id =
				block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(RotatedPillarAxis::Y))).unwrap().id;

			let res = get_block_state_id(
				0,
				CardinalDirection::North,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_log",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn east() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:oak_log", &block_states);

			let block_state_id =
				block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(RotatedPillarAxis::X))).unwrap().id;

			let res = get_block_state_id(
				4,
				CardinalDirection::North,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_log",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}
	}

	mod get_item_drop {
		#[test]
		fn basalt_drops_basalt_with_wooden_pickaxe() {
			let block_states = data::blocks::get_blocks();
			let all_items = data::items::get_items();

			let res = super::get_item_drop(
				block_states.get("minecraft:basalt").unwrap().clone(),
				all_items.get("minecraft:wooden_pickaxe").unwrap(),
				&block_states,
			);

			assert_eq!(res.id, "minecraft:basalt");
			assert_eq!(res.count, 1);
		}

		#[test]
		fn basalt_drops_nothing_with_no_tool() {
			let block_states = data::blocks::get_blocks();
			let all_items = data::items::get_items();

			let res =
				super::get_item_drop(block_states.get("minecraft:basalt").unwrap().clone(), all_items.get("minecraft:air").unwrap(), &block_states);

			assert_eq!(res.id, "minecraft:air");
			assert_eq!(res.count, 0);
		}
	}
}
