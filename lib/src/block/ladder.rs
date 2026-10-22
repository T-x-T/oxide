use super::*;

pub fn get_block_state_id(
	face: u8,
	_cardinal_direction: CardinalDirection,
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	_cursor_position_x: f32,
	_cursor_position_y: f32,
	_cursor_position_z: f32,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let position_to_check = match face {
		0 => BlockPosition { y: position.y + 1, ..position },
		1 => BlockPosition { y: position.y - 1, ..position },
		2 => BlockPosition { z: position.z + 1, ..position },
		3 => BlockPosition { z: position.z - 1, ..position },
		4 => BlockPosition { x: position.x + 1, ..position },
		5 => BlockPosition { x: position.x - 1, ..position },
		_ => panic!("invalid face {face}"),
	};

	let block_placed_against = dimension.get_block(position_to_check).unwrap_or_default();
	let block_type_placed_against = data::blocks::get_type_from_block_state_id(block_placed_against);
	if block_type_placed_against.has_no_collision_box() || block_type_placed_against == Type::Ladder {
		return vec![(0, position)];
	}

	let block = data::blocks::get_block_from_name(used_item_name, block_states);

	let axis = match face {
		2 => LadderFacing::North,
		3 => LadderFacing::South,
		4 => LadderFacing::West,
		5 => LadderFacing::East,
		_ => return vec![(0, position)],
	};

	return vec![(
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::LadderFacing(axis.clone()))
					&& x.properties.contains(&Property::LadderWaterlogged(LadderWaterlogged::False))
			})
			.unwrap()
			.id,
		position,
	)];
}

pub fn update(position: BlockPosition, dimension: &Dimension, _block_states: &HashMap<String, Block>, block_id: u16) -> BlockUpdateOutcome {
	let state = data::blocks::get_block_state_from_block_state_id(block_id);
	let position_to_check = if state.properties.contains(&Property::LadderFacing(LadderFacing::North)) {
		BlockPosition { z: position.z + 1, ..position }
	} else if state.properties.contains(&Property::LadderFacing(LadderFacing::South)) {
		BlockPosition { z: position.z - 1, ..position }
	} else if state.properties.contains(&Property::LadderFacing(LadderFacing::East)) {
		BlockPosition { x: position.x - 1, ..position }
	} else {
		BlockPosition { x: position.x + 1, ..position }
	};

	let block_placed_against = dimension.get_block(position_to_check).unwrap_or_default();
	let block_type_placed_against = data::blocks::get_type_from_block_state_id(block_placed_against);
	if block_type_placed_against.has_no_collision_box() || block_type_placed_against == Type::Ladder {
		return BlockUpdateOutcome::DestroyAndDropSelf(block_id);
	}
	return BlockUpdateOutcome::DoNothing;
}
