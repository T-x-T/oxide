use super::*;

pub fn get_block_state_id(
	face: u8,
	cardinal_direction: CardinalDirection,
	cursor_position_y: f32,
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let facing = match cardinal_direction {
		CardinalDirection::North => StairFacing::North,
		CardinalDirection::East => StairFacing::East,
		CardinalDirection::South => StairFacing::South,
		CardinalDirection::West => StairFacing::West,
	};

	let flip_it = face == 0 || (cursor_position_y > 0.5 && cursor_position_y < 0.9999);
	let stair_half = if flip_it { StairHalf::Top } else { StairHalf::Bottom };

	let north_block = dimension
		.get_block(BlockPosition {
			z: position.z - 1,
			..position
		})
		.unwrap_or(0);
	let east_block = dimension
		.get_block(BlockPosition {
			x: position.x + 1,
			..position
		})
		.unwrap_or(0);
	let south_block = dimension
		.get_block(BlockPosition {
			z: position.z + 1,
			..position
		})
		.unwrap_or(0);
	let west_block = dimension
		.get_block(BlockPosition {
			x: position.x - 1,
			..position
		})
		.unwrap_or(0);

	let stair_block_ids: Vec<u16> = block_states
		.iter()
		.filter(|x| x.1.block_type == Type::Stair)
		.flat_map(|x| {
			x.1.states.iter().filter(|y| y.properties.contains(&Property::StairHalf(stair_half.clone()))).map(|y| y.id).collect::<Vec<u16>>()
		})
		.collect();

	let mut shape = StairShape::Straight;

	if stair_block_ids.contains(&north_block) {
		let north_block_state = data::blocks::get_block_state_from_block_state_id(north_block, block_states);

		if north_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) {
			if cardinal_direction == CardinalDirection::South {
				if stair_block_ids.contains(&west_block) {
					let west_block_state = data::blocks::get_block_state_from_block_state_id(west_block, block_states);
					if west_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::OuterRight;
					}
				} else {
					shape = StairShape::InnerRight;
				}
			} else if cardinal_direction != CardinalDirection::West
				|| (north_block_state.properties.contains(&Property::StairShape(StairShape::OuterRight))
					|| north_block_state.properties.contains(&Property::StairShape(StairShape::InnerLeft)))
			{
				shape = StairShape::OuterLeft;
			}
		} else if north_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) {
			if cardinal_direction == CardinalDirection::North {
				if stair_block_ids.contains(&west_block) {
					let west_block_state = data::blocks::get_block_state_from_block_state_id(west_block, block_states);
					if west_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::InnerRight;
					}
				} else {
					shape = StairShape::OuterRight;
				}
			} else if cardinal_direction != CardinalDirection::East
				|| (north_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft))
					|| north_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)))
			{
				shape = StairShape::InnerLeft;
			}
		}
	}

	if stair_block_ids.contains(&east_block) {
		let east_block_state = data::blocks::get_block_state_from_block_state_id(east_block, block_states);
		if east_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) {
			if cardinal_direction == CardinalDirection::West {
				if stair_block_ids.contains(&north_block) {
					let north_block_state = data::blocks::get_block_state_from_block_state_id(north_block, block_states);
					if north_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::OuterRight;
					}
				} else {
					shape = StairShape::InnerRight;
				}
			} else if cardinal_direction != CardinalDirection::North
				|| (east_block_state.properties.contains(&Property::StairShape(StairShape::OuterRight))
					|| east_block_state.properties.contains(&Property::StairShape(StairShape::InnerLeft)))
			{
				shape = StairShape::OuterLeft;
			}
		} else if east_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) {
			if cardinal_direction == CardinalDirection::East {
				if stair_block_ids.contains(&north_block) {
					let north_block_state = data::blocks::get_block_state_from_block_state_id(north_block, block_states);
					if north_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::InnerRight;
					}
				} else {
					shape = StairShape::OuterRight;
				}
			} else if cardinal_direction != CardinalDirection::South
				|| (east_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft))
					|| east_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)))
			{
				shape = StairShape::InnerLeft;
			}
		}
	}

	if stair_block_ids.contains(&south_block) {
		let south_block_state = data::blocks::get_block_state_from_block_state_id(south_block, block_states);
		if south_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) {
			if cardinal_direction == CardinalDirection::North {
				if stair_block_ids.contains(&east_block) {
					let east_block_state = data::blocks::get_block_state_from_block_state_id(east_block, block_states);
					if east_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::OuterRight;
					}
				} else {
					shape = StairShape::InnerRight;
				}
			} else if cardinal_direction != CardinalDirection::East
				|| (south_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft))
					|| south_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)))
			{
				shape = StairShape::OuterLeft;
			}
		} else if south_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) {
			if cardinal_direction == CardinalDirection::South {
				if stair_block_ids.contains(&east_block) {
					let east_block_state = data::blocks::get_block_state_from_block_state_id(east_block, block_states);
					if east_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::InnerRight;
					}
				} else {
					shape = StairShape::OuterRight;
				}
			} else if cardinal_direction != CardinalDirection::West
				|| (south_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft))
					|| south_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)))
			{
				shape = StairShape::InnerLeft;
			}
		}
	}

	if stair_block_ids.contains(&west_block) {
		let west_block_state = data::blocks::get_block_state_from_block_state_id(west_block, block_states);
		if west_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) {
			if cardinal_direction == CardinalDirection::East {
				if stair_block_ids.contains(&south_block) {
					let south_block_state = data::blocks::get_block_state_from_block_state_id(south_block, block_states);
					if south_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::OuterRight;
					}
				} else {
					shape = StairShape::InnerRight;
				}
			} else if cardinal_direction != CardinalDirection::South
				|| (west_block_state.properties.contains(&Property::StairShape(StairShape::OuterRight))
					|| west_block_state.properties.contains(&Property::StairShape(StairShape::InnerLeft)))
			{
				shape = StairShape::OuterLeft;
			}
		} else if west_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) {
			if cardinal_direction == CardinalDirection::West {
				if stair_block_ids.contains(&south_block) {
					let south_block_state = data::blocks::get_block_state_from_block_state_id(south_block, block_states);
					if south_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) {
						shape = StairShape::Straight;
					} else {
						shape = StairShape::InnerRight;
					}
				} else {
					shape = StairShape::OuterRight;
				}
			} else if cardinal_direction != CardinalDirection::North
				|| (west_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft))
					|| west_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)))
			{
				shape = StairShape::InnerLeft;
			}
		}
	}

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::StairFacing(facing.clone()))
					&& x.properties.contains(&Property::StairHalf(stair_half.clone()))
					&& x.properties.contains(&Property::StairShape(shape.clone()))
					&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

	let block = data::blocks::get_block_state_from_block_state_id(block_state_id, block_states);

	let face = if block.properties.contains(&Property::StairHalf(StairHalf::Top)) { 0 } else { 1 };

	let cardinal_direction = if block.properties.contains(&Property::StairFacing(StairFacing::North)) {
		CardinalDirection::North
	} else if block.properties.contains(&Property::StairFacing(StairFacing::East)) {
		CardinalDirection::East
	} else if block.properties.contains(&Property::StairFacing(StairFacing::South)) {
		CardinalDirection::South
	} else {
		CardinalDirection::West
	};

	let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id, block_states);

	let res = get_block_state_id(face, cardinal_direction, 0.0, dimension, position, &block_name, block_states);

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
		fn on_top_of_other_block_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
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
		fn on_top_of_other_block_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
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
		fn on_bottom_of_other_block_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Top))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
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
		fn on_bottom_of_other_block_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Top))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
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
		fn on_lower_side_of_other_block_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				3,
				CardinalDirection::North,
				0.49,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
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
		fn curved_stair_inner_right_north_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_left_north_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_right_east_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_left_east_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_right_south_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_left_south_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_right_west_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_inner_left_west_bottom() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_surrounded_on_all_sides_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_surrounded_on_all_sides_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_surrounded_on_all_sides_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_surrounded_on_all_sides_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_west_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_east_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_north_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_south_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_east_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_west_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_south_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn straight_stair_north_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_right_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_left_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_right_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_left_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_right_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_left_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_right_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_outer_left_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn different_stair_types_connect() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:birch_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn different_stair_types_connect_upside_down() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:birch_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Top))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				0,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Top))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn different_stair_types_dont_connect_different_halves() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:birch_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Top))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio2_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio2_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio2_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio2_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio3_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio3_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio3_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio3_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio4_straight_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio4_straight_east() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let res = get_block_state_id(
				1,
				CardinalDirection::East,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio4_straight_south() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 9,
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
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::South,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::South))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio4_straight_west() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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

			let res = get_block_state_id(
				1,
				CardinalDirection::West,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::West))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
		fn curved_stair_prio_outer_right_north() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
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
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::InnerLeft))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
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
					x.properties.contains(&Property::StairFacing(StairFacing::East))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::Straight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 11,
						y: 80,
						z: 1,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let res = get_block_state_id(
				1,
				CardinalDirection::North,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_stairs",
				&block_states,
			);

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::StairFacing(StairFacing::North))
						&& x.properties.contains(&Property::StairHalf(StairHalf::Bottom))
						&& x.properties.contains(&Property::StairShape(StairShape::OuterRight))
						&& x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))
				})
				.unwrap()
				.id;
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
