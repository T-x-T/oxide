use super::*;

pub fn get_block_state_id(
	dimension: &Dimension,
	position: BlockPosition,
	_used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let obsidian_block_state_id = data::blocks::get_block_from_name("minecraft:obsidian", block_states).states.first().unwrap().id;
	let nether_portal_block_state_id = data::blocks::get_block_from_name("minecraft:nether_portal", block_states).states.first().unwrap().id;

	let block_state_id_below = dimension
		.get_block(BlockPosition {
			y: position.y - 1,
			..position
		})
		.unwrap_or_default();

	if obsidian_block_state_id == block_state_id_below {
		let next_blocks_to_check = vec![
			BlockPosition {
				x: position.x + 1,
				y: position.y - 1,
				..position
			},
			BlockPosition {
				x: position.x + 2,
				..position
			},
			BlockPosition {
				x: position.x + 2,
				y: position.y + 1,
				..position
			},
			BlockPosition {
				x: position.x + 2,
				y: position.y + 2,
				..position
			},
			BlockPosition {
				x: position.x + 1,
				y: position.y + 3,
				..position
			},
			BlockPosition {
				y: position.y + 3,
				..position
			},
			BlockPosition {
				x: position.x - 1,
				y: position.y + 2,
				..position
			},
			BlockPosition {
				x: position.x - 1,
				..position
			},
		];

		for next_block_to_check in next_blocks_to_check {
			let block_state_id = dimension.get_block(next_block_to_check).unwrap_or_default();
			if block_state_id != obsidian_block_state_id {
				let fire_block = data::blocks::get_block_from_name("minecraft:fire", block_states);
				let fire_block_state_id = fire_block.states[fire_block.default_state].id;
				return vec![(fire_block_state_id, position)];
			}
		}
		output = vec![
			(nether_portal_block_state_id, position),
			(
				nether_portal_block_state_id,
				BlockPosition {
					x: position.x + 1,
					..position
				},
			),
			(
				nether_portal_block_state_id,
				BlockPosition {
					y: position.y + 1,
					..position
				},
			),
			(
				nether_portal_block_state_id,
				BlockPosition {
					y: position.y + 1,
					x: position.x + 1,
					..position
				},
			),
			(
				nether_portal_block_state_id,
				BlockPosition {
					x: position.x + 1,
					y: position.y + 2,
					..position
				},
			),
			(
				nether_portal_block_state_id,
				BlockPosition {
					y: position.y + 2,
					..position
				},
			),
		]
	}


	return output;
}
