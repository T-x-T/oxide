use super::*;

pub fn update(position: BlockPosition, dimension: &Dimension, block_states: &HashMap<String, Block>, block_id: u16) -> BlockUpdateOutcome {
	let state = data::blocks::get_block_state_from_block_state_id(block_id, block_states);
	//println!("running for: {position:?}");
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

	for position_to_check in positions_to_check {
		//println!("checking: {position_to_check:?}");
		if !dimension.get_block(position_to_check).is_ok_and(|x| x == 0 || x == obsidian_block_state_id) {
			//println!("delete me");
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		}
	}

	return BlockUpdateOutcome::DoNothing;
}
