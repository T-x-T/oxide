use super::*;

pub fn get_block_state_id(
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let block_ids_to_check: Vec<u16> = block_states
		.iter()
		.filter(|x| x.0.ends_with("glass_pane") || x.0 == "minecraft:iron_bars")
		.flat_map(|x| x.1.states.iter().map(|x| x.id))
		.collect();

	let north = if block_ids_to_check.contains(
		&dimension
			.get_block(BlockPosition {
				z: position.z - 1,
				..position
			})
			.unwrap_or(0),
	) {
		IronBarsNorth::True
	} else {
		IronBarsNorth::False
	};
	let south = if block_ids_to_check.contains(
		&dimension
			.get_block(BlockPosition {
				z: position.z + 1,
				..position
			})
			.unwrap_or(0),
	) {
		IronBarsSouth::True
	} else {
		IronBarsSouth::False
	};
	let east = if block_ids_to_check.contains(
		&dimension
			.get_block(BlockPosition {
				x: position.x + 1,
				..position
			})
			.unwrap_or(0),
	) {
		IronBarsEast::True
	} else {
		IronBarsEast::False
	};
	let west = if block_ids_to_check.contains(
		&dimension
			.get_block(BlockPosition {
				x: position.x - 1,
				..position
			})
			.unwrap_or(0),
	) {
		IronBarsWest::True
	} else {
		IronBarsWest::False
	};
	let water_logged = IronBarsWaterlogged::False;

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::IronBarsNorth(north.clone()))
					&& x.properties.contains(&Property::IronBarsSouth(south.clone()))
					&& x.properties.contains(&Property::IronBarsEast(east.clone()))
					&& x.properties.contains(&Property::IronBarsWest(west.clone()))
					&& x.properties.contains(&Property::IronBarsWaterlogged(water_logged.clone()))
			})
			.unwrap()
			.id,
		position,
	));

	return output;
}

pub fn update(position: BlockPosition, dimension: &Dimension, block_states: &HashMap<String, Block>) -> Option<u16> {
	let Ok(block_state_id) = dimension.get_block(position) else {
		return None;
	};

	let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id, block_states);

	let res = get_block_state_id(dimension, position, &block_name, block_states);

	let new_block_state_id = res.first().unwrap().0;

	if block_state_id == new_block_state_id {
		return None;
	} else {
		return Some(new_block_state_id);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	mod get_block_state_id {
		use super::*;

		#[test]
		fn no_connections() {
			let block_states = data::blocks::get_blocks();
			let dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:iron_bars", &block_states);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:iron_bars",
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
		fn north_and_west_connected() {
			let block_states = data::blocks::get_blocks();
			let mut dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:iron_bars", &block_states);

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::True))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: -1,
					},
					block_state_id_to_place,
				)
				.unwrap();
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::True))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::True))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::True))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:iron_bars",
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
		fn all_sides_connected() {
			let block_states = data::blocks::get_blocks();
			let mut dimension = Dimension::new();
			let block = data::blocks::get_block_from_name("minecraft:iron_bars", &block_states);

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::True))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: -1,
					},
					block_state_id_to_place,
				)
				.unwrap();
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::True))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::True))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::False))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 1,
					},
					block_state_id_to_place,
				)
				.unwrap();
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::False))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::True))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::True))
						&& x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::True))
						&& x.properties.contains(&Property::IronBarsEast(IronBarsEast::True))
						&& x.properties.contains(&Property::IronBarsWest(IronBarsWest::True))
						&& x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:iron_bars",
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
