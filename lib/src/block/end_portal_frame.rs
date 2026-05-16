use super::*;

pub fn interact(
	position: BlockPosition,
	block_id_at_location: u16,
	_face: u8,
	player: &mut Player,
	players_clone: &[Player],
	packet_sender: &PacketSender,
	block_states: &HashMap<String, Block>,
	dimension: &Dimension,
) -> BlockInteractionResult {
	if !player.get_held_item(true).is_some_and(|x| x.count > 0 && x.id == data::items::get_item_id_by_name("minecraft:ender_eye").unwrap()) {
		return BlockInteractionResult::Nothing;
	}

	let states = data::blocks::get_block_state_from_block_state_id(block_id_at_location, block_states);
	if states.properties.contains(&blocks::Property::EndPortalFrameEye(blocks::EndPortalFrameEye::True)) {
		return BlockInteractionResult::Nothing;
	} else {
		if player.get_gamemode() == Gamemode::Survival || player.get_gamemode() == Gamemode::Adventure {
			let held_item = player.get_held_item(true).unwrap();
			if held_item.count > 1 {
				let slot = Slot {
					count: held_item.count - 1,
					..held_item.clone()
				};
				player.set_selected_inventory_slot(Some(slot), players_clone, packet_sender);
			} else {
				player.set_selected_inventory_slot(None, players_clone, packet_sender);
			}
		}

		let mut blocks_to_overwrite: Vec<(u16, BlockPosition)> = Vec::new();

		let block = data::blocks::get_block_from_name("minecraft:end_portal_frame", block_states);
		let north_filled_portal_id = block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::North))
					&& x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True))
			})
			.unwrap()
			.id;
		let east_filled_portal_id = block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::East))
					&& x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True))
			})
			.unwrap()
			.id;
		let south_filled_portal_id = block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::South))
					&& x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True))
			})
			.unwrap()
			.id;
		let west_filled_portal_id = block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::West))
					&& x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True))
			})
			.unwrap()
			.id;
		let end_portal_id = data::blocks::get_block_from_name("minecraft:end_portal", block_states).states.first().unwrap().id;

		let facing = if states
			.properties
			.iter()
			.any(|x| matches!(x, crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::North)))
		{
			let mut offset: Option<i32> = None;
			if dimension
				.get_block(BlockPosition {
					x: position.x + 1,
					..position
				})
				.unwrap_or_default()
				== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 1,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
			{
				offset = Some(0);
				//middle one was filled last
			} else if dimension
				.get_block(BlockPosition {
					x: position.x + 1,
					..position
				})
				.unwrap_or_default()
				== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
			{
				offset = Some(1);
				//left one was filled last
			} else if dimension
				.get_block(BlockPosition {
					x: position.x - 1,
					..position
				})
				.unwrap_or_default()
				== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
			{
				offset = Some(-1);
				//right one was filled last
			}
			if let Some(offset) = offset
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z - 1,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z - 2,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z - 3,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z - 1,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z - 2,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z - 3,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 1 + offset,
						z: position.z - 4,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + offset,
						z: position.z - 4,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 1 + offset,
						z: position.z - 4,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
			{
				blocks_to_overwrite.append(&mut vec![
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z - 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z - 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z - 3,
							..position
						},
					),
				]);
			}
			crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::North)
		} else if states
			.properties
			.iter()
			.any(|x| matches!(x, crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::East)))
		{
			let mut offset: Option<i32> = None;
			if dimension
				.get_block(BlockPosition {
					z: position.z + 1,
					..position
				})
				.unwrap_or_default()
				== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 1,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
			{
				offset = Some(0);
				//middle one was filled last
			} else if dimension
				.get_block(BlockPosition {
					z: position.z + 1,
					..position
				})
				.unwrap_or_default()
				== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
			{
				offset = Some(1);
				//left one was filled last
			} else if dimension
				.get_block(BlockPosition {
					z: position.z - 1,
					..position
				})
				.unwrap_or_default()
				== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
			{
				offset = Some(-1);
				//right one was filled last
			}
			if let Some(offset) = offset
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x + 1,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x + 2,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x + 3,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x + 1,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x + 2,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x + 3,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 1 + offset,
						x: position.x + 4,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + offset,
						x: position.x + 4,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 1 + offset,
						x: position.x + 4,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
			{
				blocks_to_overwrite.append(&mut vec![
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x + 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x + 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x + 3,
							..position
						},
					),
				]);
			}
			crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::East)
		} else if states
			.properties
			.iter()
			.any(|x| matches!(x, crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::South)))
		{
			let mut offset: Option<i32> = None;
			if dimension
				.get_block(BlockPosition {
					x: position.x + 1,
					..position
				})
				.unwrap_or_default()
				== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 1,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
			{
				offset = Some(0);
				//middle one was filled last
			} else if dimension
				.get_block(BlockPosition {
					x: position.x + 1,
					..position
				})
				.unwrap_or_default()
				== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
			{
				offset = Some(1);
				//left one was filled last
			} else if dimension
				.get_block(BlockPosition {
					x: position.x - 1,
					..position
				})
				.unwrap_or_default()
				== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
			{
				offset = Some(-1);
				//right one was filled last
			}
			if let Some(offset) = offset
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z + 1,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z + 2,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 2 + offset,
						z: position.z + 3,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z + 1,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z + 2,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 2 + offset,
						z: position.z + 3,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x - 1 + offset,
						z: position.z + 4,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + offset,
						z: position.z + 4,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						x: position.x + 1 + offset,
						z: position.z + 4,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
			{
				blocks_to_overwrite.append(&mut vec![
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z + 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z + 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x - 1 + offset,
							z: position.z + 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + offset,
							z: position.z + 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							x: position.x + 1 + offset,
							z: position.z + 3,
							..position
						},
					),
				]);
			}
			crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::South)
		} else {
			let mut offset: Option<i32> = None;
			if dimension
				.get_block(BlockPosition {
					z: position.z + 1,
					..position
				})
				.unwrap_or_default()
				== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 1,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
			{
				offset = Some(0);
				//middle one was filled last
			} else if dimension
				.get_block(BlockPosition {
					z: position.z + 1,
					..position
				})
				.unwrap_or_default()
				== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
			{
				offset = Some(1);
				//left one was filled last
			} else if dimension
				.get_block(BlockPosition {
					z: position.z - 1,
					..position
				})
				.unwrap_or_default()
				== west_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2,
						..position
					})
					.unwrap_or_default()
					== west_filled_portal_id
			{
				offset = Some(-1);
				//right one was filled last
			}
			if let Some(offset) = offset
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x - 1,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x - 2,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 2 + offset,
						x: position.x - 3,
						..position
					})
					.unwrap_or_default()
					== north_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x - 1,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x - 2,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 2 + offset,
						x: position.x - 3,
						..position
					})
					.unwrap_or_default()
					== south_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z - 1 + offset,
						x: position.x - 4,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + offset,
						x: position.x - 4,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
				&& dimension
					.get_block(BlockPosition {
						z: position.z + 1 + offset,
						x: position.x - 4,
						..position
					})
					.unwrap_or_default()
					== east_filled_portal_id
			{
				blocks_to_overwrite.append(&mut vec![
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x - 1,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x - 2,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z - 1 + offset,
							x: position.x - 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + offset,
							x: position.x - 3,
							..position
						},
					),
					(
						end_portal_id,
						BlockPosition {
							z: position.z + 1 + offset,
							x: position.x - 3,
							..position
						},
					),
				]);
			}
			crate::blocks::Property::EndPortalFrameFacing(blocks::EndPortalFrameFacing::West)
		};
		let new_block_state_id = block.states.iter().find(|x| {
			x.properties.contains(&blocks::Property::EndPortalFrameEye(blocks::EndPortalFrameEye::True)) && x.properties.contains(&facing)
		});

		blocks_to_overwrite.push((new_block_state_id.unwrap().id, position));
		return BlockInteractionResult::OverwriteBlocks(blocks_to_overwrite);
	}
}

pub fn get_block_state_id(
	_face: u8,
	cardinal_direction: CardinalDirection,
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

	let facing = match cardinal_direction {
		CardinalDirection::North => EndPortalFrameFacing::South,
		CardinalDirection::East => EndPortalFrameFacing::West,
		CardinalDirection::South => EndPortalFrameFacing::North,
		CardinalDirection::West => EndPortalFrameFacing::East,
	};

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::EndPortalFrameFacing(facing.clone()))
					&& x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::False))
			})
			.unwrap()
			.id,
		position,
	));

	return output;
}
