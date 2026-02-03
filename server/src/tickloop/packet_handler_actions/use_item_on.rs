use super::*;

#[allow(clippy::too_many_arguments)]
pub fn process(
	peer_addr: SocketAddr,
	location: BlockPosition,
	face: u8,
	cursor_position_x: f32,
	cursor_position_y: f32,
	cursor_position_z: f32,
	sequence_id: i32,
	game: Arc<Game>,
	players_clone: &[Player],
) {
	let mut players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();

	let mut new_block_location = location;
	match face {
		0 => new_block_location.y -= 1,
		1 => new_block_location.y += 1,
		2 => new_block_location.z -= 1,
		3 => new_block_location.z += 1,
		4 => new_block_location.x -= 1,
		_ => new_block_location.x += 1,
	}

	let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
	let player_get_looking_cardinal_direction = player.get_looking_cardinal_direction().clone();
	let gamemode = player.get_gamemode();

	let dimension = world.dimensions.get("minecraft:overworld").unwrap();
	let block_id_at_location = dimension.get_block(location).unwrap_or_default();
	let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location);

	let blocks_to_place: Vec<(u16, BlockPosition)> = if block_type_at_location.has_right_click_behavior() && !player.is_sneaking() {
		//Don't place block, because player right clicked something that does something when right clicked
		let block_interaction_result = lib::block::interact_with_block_at(location, block_id_at_location, face, &game.block_state_data);
		block_interaction_result
			.handle(dimension, location, player, players_clone, block_id_at_location, game.clone())
			.inspect_err(|x| {
				println!("lib::block::interact_with_block_at({location:?}, {block_id_at_location}, {face}) call resulted in error {x:?}")
			})
			.unwrap_or_default()
	} else {
		//Let's go - we can place a block
		let used_item_id = player
			.get_held_item(true)
			.unwrap_or(&Slot {
				item_count: 0,
				item_id: 0,
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			})
			.item_id;
		let used_item_name = data::items::get_item_name_by_id(used_item_id);
		let pitch = player.get_pitch();

		if used_item_name.ends_with("spawn_egg") {
			let dimension = world.dimensions.get_mut("minecraft:overworld").unwrap();
			lib::create_and_spawn_entity_from_egg(
				used_item_name,
				game.entity_id_manager.get_new(),
				new_block_location,
				dimension,
				players_clone,
				game.clone(),
			);
		}

		let block_state_ids = lib::block::get_block_state_id(
			face,
			player_get_looking_cardinal_direction,
			pitch,
			world.dimensions.get_mut("minecraft:overworld").unwrap(),
			new_block_location,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			&game.block_state_data,
		);

		//first part of the if tries to check if player just right clicked an item that doesnt place a block
		if !block_state_ids.is_empty() && block_state_ids[0].0 != 0 && gamemode == Gamemode::Survival {
			let Some(hand_slot) = player.get_held_item(true) else {
				return;
			};

			let hand_slot = hand_slot.clone();
			let new_hand_slot = if hand_slot.item_count == 1 {
				None
			} else {
				Some(Slot {
					item_count: hand_slot.item_count - 1,
					..hand_slot
				})
			};

			player.set_selected_inventory_slot(new_hand_slot, players_clone, game.clone());
		}

		block_state_ids
	};

	let mut blocks_to_update: Vec<BlockPosition> = Vec::new();
	for block_to_place in &blocks_to_place {
		match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_place.1, block_to_place.0) {
			Ok(res) => {
				let block = data::blocks::get_block_from_block_state_id(block_to_place.0, &game.block_state_data);
				//Logic to open sign editor when player placed a new sign, maybe move somewhere else or something idk
				if block.block_type == data::blocks::Type::WallSign
					|| block.block_type == data::blocks::Type::StandingSign
					|| block.block_type == data::blocks::Type::WallHangingSign
					|| block.block_type == data::blocks::Type::CeilingHangingSign
				{
					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::OpenSignEditor::PACKET_ID,
						lib::packets::clientbound::play::OpenSignEditor {
							location: block_to_place.1,
							is_front_text: true,
						}
						.try_into()
						.unwrap(),
					);
				}
				#[allow(clippy::collapsible_if)]
				if res.is_some() && res.unwrap() == BlockOverwriteOutcome::DestroyBlockentity {
					if let Some(block_entity) = world
						.dimensions
						.get("minecraft:overworld")
						.unwrap()
						.get_chunk_from_position(location)
						.unwrap()
						.block_entities
						.iter()
						.find(|x| x.get_position() == location)
					{
						let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
						block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
					};
				}

				blocks_to_update.append(&mut vec![
					BlockPosition {
						x: block_to_place.1.x + 1,
						..block_to_place.1
					},
					BlockPosition {
						x: block_to_place.1.x - 1,
						..block_to_place.1
					},
					BlockPosition {
						y: block_to_place.1.y + 1,
						..block_to_place.1
					},
					BlockPosition {
						y: block_to_place.1.y - 1,
						..block_to_place.1
					},
					BlockPosition {
						z: block_to_place.1.z + 1,
						..block_to_place.1
					},
					BlockPosition {
						z: block_to_place.1.z - 1,
						..block_to_place.1
					},
				]);
			}
			Err(err) => {
				println!("couldn't place block because {err}");
				continue;
			}
		};
	}

	blocks_to_update.sort();
	blocks_to_update.dedup();

	let mut updated_blocks: Vec<(u16, BlockPosition)> = Vec::new();
	for block_to_update in blocks_to_update {
		let res = lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
		if let Some(new_block) = res {
			match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block) {
				Ok(_) => {
					updated_blocks.push((new_block, block_to_update));
				}
				Err(err) => {
					println!("couldn't place block because {err}");
					continue;
				}
			}
		};
	}

	let all_changed_blocks: Vec<(u16, BlockPosition)> = vec![blocks_to_place, updated_blocks].into_iter().flatten().collect();

	for player in players.iter() {
		for block in &all_changed_blocks {
			game.send_packet(
				&player.peer_socket_address,
				lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
				lib::packets::clientbound::play::BlockUpdate {
					location: block.1,
					block_id: block.0 as i32,
				}
				.try_into()
				.unwrap(),
			);
		}
	}

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
		lib::packets::clientbound::play::AcknowledgeBlockChange {
			sequence_id,
		}
		.try_into()
		.unwrap(),
	);
}
