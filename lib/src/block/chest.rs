use super::*;

pub fn get_block_state_id(
	cardinal_direction: CardinalDirection,
	position: BlockPosition,
	used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let facing = match cardinal_direction {
		CardinalDirection::North => ChestFacing::South,
		CardinalDirection::East => ChestFacing::West,
		CardinalDirection::South => ChestFacing::North,
		CardinalDirection::West => ChestFacing::East,
	};

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::ChestFacing(facing.clone()))
					&& x.properties.contains(&Property::ChestType(ChestType::Single))
					&& x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False))
			})
			.unwrap()
			.id,
		position,
	));

	return output;
}

#[cfg(test)]
mod test {
	use super::*;

	mod get_block_state_id {
		use super::*;

		#[test]
		fn north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:chest", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::ChestFacing(ChestFacing::North))
						&& x.properties.contains(&Property::ChestType(ChestType::Single))
						&& x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(CardinalDirection::South, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:chest", &block_states);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}

		#[test]
		fn east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:chest", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::ChestFacing(ChestFacing::East))
						&& x.properties.contains(&Property::ChestType(ChestType::Single))
						&& x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(CardinalDirection::West, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:chest", &block_states);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}
	}
}
