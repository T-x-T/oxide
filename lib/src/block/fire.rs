use super::*;

pub fn get_block_state_id(
	dimension: &Dimension,
	position: BlockPosition,
	_used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let obsidian_block_state_id = data::blocks::get_block_from_name("minecraft:obsidian", block_states).states.first().unwrap().id;

	let block_state_id_below = dimension
		.get_block(BlockPosition {
			y: position.y - 1,
			..position
		})
		.unwrap_or_default();

	let mut valid = true;
	if obsidian_block_state_id == block_state_id_below {
		let portal_blocks = get_portal_block_positions(dimension, position, block_states);
		if !portal_blocks.is_empty() {
			for position_to_check in &portal_blocks {
				if !dimension.get_block(*position_to_check).is_ok_and(|x| x == 0) {
					valid = false;
					break;
				}
			}
			let first_x = portal_blocks.first().unwrap().x;
			let last_x = portal_blocks.last().unwrap().x;

			let nether_portal_block_state_id = if first_x != last_x {
				data::blocks::get_block_from_name("minecraft:nether_portal", block_states).states.first().unwrap().id
			} else {
				data::blocks::get_block_from_name("minecraft:nether_portal", block_states).states.last().unwrap().id
			};
			for portal_block in &portal_blocks {
				output.push((nether_portal_block_state_id, *portal_block));
			}
		}
	}

	if !valid {
		let fire_block = data::blocks::get_block_from_name("minecraft:fire", block_states);
		let fire_block_state_id = fire_block.states[fire_block.default_state].id;
		return vec![(fire_block_state_id, position)];
	}

	return output;
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	XPos,
	XNeg,
	YPos,
	YNeg,
	ZPos,
	ZNeg,
}

fn get_portal_block_positions(
	dimension: &Dimension,
	starting_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> Vec<BlockPosition> {
	let mut portal_blocks: Vec<BlockPosition> = Vec::new();
	let obsidian_block_state_id = data::blocks::get_block_from_name("minecraft:obsidian", block_states).states.first().unwrap().id;

	let y_pos_blocks = find_air_blocks_in_direction(dimension, Direction::YPos, starting_position);
	let y_neg_blocks = find_air_blocks_in_direction(dimension, Direction::YNeg, starting_position);

	if ((y_pos_blocks + y_neg_blocks) >= 2) && ((y_pos_blocks + y_neg_blocks) <= 23) {
		let x_pos_blocks = find_air_blocks_in_direction(dimension, Direction::XPos, starting_position);
		let x_neg_blocks = find_air_blocks_in_direction(dimension, Direction::XNeg, starting_position);

		if ((x_pos_blocks + x_neg_blocks) >= 1) && ((x_pos_blocks + x_neg_blocks) <= 23) {
			for x in -x_neg_blocks..=x_pos_blocks {
				let block_to_check = BlockPosition {
					x: starting_position.x + x as i32,
					y: starting_position.y + y_pos_blocks as i16 + 1,
					z: starting_position.z,
				};
				if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
					return Vec::new();
				}
				let block_to_check = BlockPosition {
					x: starting_position.x + x as i32,
					y: starting_position.y - y_neg_blocks as i16 - 1,
					z: starting_position.z,
				};
				if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
					return Vec::new();
				}

				for y in -y_neg_blocks..=y_pos_blocks {
					let block_to_check = BlockPosition {
						x: starting_position.x + x_pos_blocks as i32 + 1,
						y: starting_position.y + y as i16,
						z: starting_position.z,
					};
					if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
						return Vec::new();
					}
					let block_to_check = BlockPosition {
						x: starting_position.x - x_neg_blocks as i32 - 1,
						y: starting_position.y + y as i16,
						z: starting_position.z,
					};
					if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
						return Vec::new();
					}

					portal_blocks.push(BlockPosition {
						x: starting_position.x + x as i32,
						y: starting_position.y + y as i16,
						z: starting_position.z,
					});
				}
			}
		} else {
			let z_pos_blocks = find_air_blocks_in_direction(dimension, Direction::ZPos, starting_position);
			let z_neg_blocks = find_air_blocks_in_direction(dimension, Direction::ZNeg, starting_position);

			if ((z_pos_blocks + z_neg_blocks) >= 1) && ((z_pos_blocks + z_neg_blocks) <= 23) {
				for z in -z_neg_blocks..=z_pos_blocks {
					let block_to_check = BlockPosition {
						x: starting_position.x,
						y: starting_position.y + y_pos_blocks as i16 + 1,
						z: starting_position.z + z as i32,
					};
					if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
						return Vec::new();
					}
					let block_to_check = BlockPosition {
						x: starting_position.x,
						y: starting_position.y - y_neg_blocks as i16 - 1,
						z: starting_position.z + z as i32,
					};
					if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
						return Vec::new();
					}

					for y in -y_neg_blocks..=y_pos_blocks {
						let block_to_check = BlockPosition {
							x: starting_position.x,
							y: starting_position.y + y as i16,
							z: starting_position.z + z_pos_blocks as i32 + 1,
						};
						if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
							return Vec::new();
						}
						let block_to_check = BlockPosition {
							x: starting_position.x,
							y: starting_position.y + y as i16,
							z: starting_position.z - z_neg_blocks as i32 - 1,
						};
						if dimension.get_block(block_to_check).unwrap_or_default() != obsidian_block_state_id {
							return Vec::new();
						}

						portal_blocks.push(BlockPosition {
							x: starting_position.x,
							y: starting_position.y + y as i16,
							z: starting_position.z + z as i32,
						});
					}
				}
			}
		}
	}

	return portal_blocks;
}

fn find_air_blocks_in_direction(dimension: &Dimension, direction: Direction, starting_position: BlockPosition) -> isize {
	let mut num_of_blocks = 0;

	for i in 1..=23 {
		let position_to_check = match direction {
			Direction::XPos => BlockPosition {
				x: starting_position.x + i,
				..starting_position
			},
			Direction::XNeg => BlockPosition {
				x: starting_position.x - i,
				..starting_position
			},
			Direction::YPos => BlockPosition {
				y: starting_position.y + i as i16,
				..starting_position
			},
			Direction::YNeg => BlockPosition {
				y: starting_position.y - i as i16,
				..starting_position
			},
			Direction::ZPos => BlockPosition {
				z: starting_position.z + i,
				..starting_position
			},
			Direction::ZNeg => BlockPosition {
				z: starting_position.z - i,
				..starting_position
			},
		};

		if dimension.get_block(position_to_check).is_ok_and(|x| x == 0) {
			num_of_blocks += 1;
		} else {
			return num_of_blocks;
		}
	}

	return num_of_blocks;
}
