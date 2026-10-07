use super::*;

pub fn get_block_state_id(
	cardinal_direction: CardinalDirection,
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);

	let bed_facing = match cardinal_direction {
		CardinalDirection::North => BedFacing::North,
		CardinalDirection::East => BedFacing::East,
		CardinalDirection::South => BedFacing::South,
		CardinalDirection::West => BedFacing::West,
	};

	let head_position = match cardinal_direction {
		CardinalDirection::North => BlockPosition { z: position.z - 1, ..position },
		CardinalDirection::East => BlockPosition { x: position.x + 1, ..position },
		CardinalDirection::South => BlockPosition { z: position.z + 1, ..position },
		CardinalDirection::West => BlockPosition { x: position.x - 1, ..position },
	};

	let block_at_head_position = data::blocks::get_block_name_from_block_state_id(dimension.get_block(head_position).unwrap_or_default());
	if !data::tags::get_block().get("replaceable").unwrap().contains(&block_at_head_position) {
		return vec![(0, position)];
	}

	let foot_part_id: u16 = block
		.states
		.iter()
		.find(|x| {
			x.properties.contains(&Property::BedPart(BedPart::Foot))
				&& x.properties.contains(&Property::BedFacing(bed_facing.clone()))
				&& x.properties.contains(&Property::BedOccupied(BedOccupied::False))
		})
		.unwrap()
		.id;

	let head_part_id = block
		.states
		.iter()
		.find(|x| {
			x.properties.contains(&Property::BedPart(BedPart::Head))
				&& x.properties.contains(&Property::BedFacing(bed_facing.clone()))
				&& x.properties.contains(&Property::BedOccupied(BedOccupied::False))
		})
		.unwrap()
		.id;

	return vec![(foot_part_id, position), (head_part_id, head_position)];
}

pub fn update(
	position: BlockPosition,
	dimension: &Dimension,
	_block_states: &HashMap<String, Block>,
	_block_id: u16,
) -> BlockUpdateOutcome {
	let Ok(block_state_id) = dimension.get_block(position) else {
		return BlockUpdateOutcome::DoNothing;
	};

	let block_state = data::blocks::get_block_state_from_block_state_id(block_state_id);

	if block_state.properties.contains(&Property::BedPart(BedPart::Head)) {
		let position_to_check = if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::North)).is_some() {
			BlockPosition { z: position.z + 1, ..position }
		} else if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::East)).is_some() {
			BlockPosition { x: position.x - 1, ..position }
		} else if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::South)).is_some() {
			BlockPosition { z: position.z - 1, ..position }
		} else {
			BlockPosition { x: position.x + 1, ..position }
		};

		let Ok(block_state_id) = dimension.get_block(position_to_check) else {
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		};

		let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id);

		if !block_name.ends_with("_bed") {
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		}
	}

	if block_state.properties.contains(&Property::BedPart(BedPart::Foot)) {
		let position_to_check = if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::North)).is_some() {
			BlockPosition { z: position.z - 1, ..position }
		} else if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::East)).is_some() {
			BlockPosition { x: position.x + 1, ..position }
		} else if block_state.properties.iter().find(|x| **x == Property::BedFacing(BedFacing::South)).is_some() {
			BlockPosition { z: position.z + 1, ..position }
		} else {
			BlockPosition { x: position.x - 1, ..position }
		};

		let Ok(block_state_id) = dimension.get_block(position_to_check) else {
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		};

		let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id);

		if !block_name.ends_with("_bed") {
			return BlockUpdateOutcome::ChangeOwnBlockId(0);
		}
	}

	return BlockUpdateOutcome::DoNothing;
}
