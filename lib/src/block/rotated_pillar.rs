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
}
