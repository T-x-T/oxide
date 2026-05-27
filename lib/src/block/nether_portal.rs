use super::*;

pub fn update(position: BlockPosition, dimension: &Dimension, block_states: &HashMap<String, Block>, block_id: u16) -> BlockUpdateOutcome {
	let state = data::blocks::get_block_state_from_block_state_id(block_id, block_states);
	let positions_to_check = if state.properties.contains(&Property::NetherPortalAxis(NetherPortalAxis::X)) {
		vec![
			BlockPosition {
				x: position.x + 1,
				..position
			},
			BlockPosition {
				x: position.x - 1,
				..position
			},
			BlockPosition {
				y: position.y + 1,
				..position
			},
			BlockPosition {
				y: position.y - 1,
				..position
			},
		]
	} else {
		vec![
			BlockPosition {
				z: position.z + 1,
				..position
			},
			BlockPosition {
				z: position.z - 1,
				..position
			},
			BlockPosition {
				y: position.y + 1,
				..position
			},
			BlockPosition {
				y: position.y - 1,
				..position
			},
		]
	};

	let obsidian_block_state_id = data::blocks::get_block_from_name("minecraft:obsidian", block_states).states.first().unwrap().id;
	let portal_block_state_id_1 = data::blocks::get_block_from_name("minecraft:nether_portal", block_states).states.first().unwrap().id;
	let portal_block_state_id_2 = data::blocks::get_block_from_name("minecraft:nether_portal", block_states).states.last().unwrap().id;


	for position_to_check in positions_to_check {
		let actual_block = dimension.get_block(position_to_check);
		if actual_block.is_ok_and(|x| x != obsidian_block_state_id && x != portal_block_state_id_1 && x != portal_block_state_id_2) {
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		}
	}

	return BlockUpdateOutcome::DoNothing;
}
