use super::*;

pub fn get_block_state_id(
	_face: u8,
	cardinal_direction: CardinalDirection,
	pitch: f32,
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

	let mut axis = match cardinal_direction {
		CardinalDirection::North => BarrelFacing::South,
		CardinalDirection::East => BarrelFacing::West,
		CardinalDirection::South => BarrelFacing::North,
		CardinalDirection::West => BarrelFacing::East,
	};

	if pitch > 45.0 {
		axis = BarrelFacing::Up;
	} else if pitch < -45.0 {
		axis = BarrelFacing::Down;
	};

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::BarrelFacing(axis.clone())) && x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
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
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:barrel", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::BarrelFacing(BarrelFacing::North))
						&& x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::South,
				44.0,
				&dimension,
				BlockPosition { x: 10, y: 80, z: 0 },
				"minecraft:barrel",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}

		#[test]
		fn east() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:barrel", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::BarrelFacing(BarrelFacing::East))
						&& x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::West,
				-44.0,
				&dimension,
				BlockPosition { x: 10, y: 80, z: 0 },
				"minecraft:barrel",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}

		#[test]
		fn up() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:barrel", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::BarrelFacing(BarrelFacing::Up))
						&& x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::West,
				46.0,
				&dimension,
				BlockPosition { x: 10, y: 80, z: 0 },
				"minecraft:barrel",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}

		#[test]
		fn down() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:barrel", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::BarrelFacing(BarrelFacing::Down))
						&& x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::West,
				-45.5,
				&dimension,
				BlockPosition { x: 10, y: 80, z: 0 },
				"minecraft:barrel",
				0.0,
				0.0,
				0.0,
				&block_states,
			);

			let expected = vec![(block_state_id, BlockPosition { x: 10, y: 80, z: 0 })];

			assert_eq!(res, expected);
		}
	}
}
