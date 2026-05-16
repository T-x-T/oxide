use lib::blocks::Type;

use super::*;

pub fn process(
	peer_addr: SocketAddr,
	parsed_packet: lib::packets::serverbound::play::UseItemOn,
	game: Arc<Game>,
	players_clone: &[Player],
) {
	let mut players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();

	let mut new_block_location = parsed_packet.location;
	match parsed_packet.face {
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

	let dimension = world.dimensions.get(player.get_dimension()).unwrap();
	let block_id_at_location = dimension.get_block(parsed_packet.location).unwrap_or_default();
	let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location);

	let blocks_to_place: Vec<(u16, BlockPosition)> = if (block_type_at_location.has_right_click_behavior()
		|| data::tags::get_item()
			.get("hoes")
			.unwrap()
			.contains(&data::items::get_item_name_by_id(player.get_selected_inventory_slot().clone().unwrap_or_default().id).unwrap()))
		&& !player.is_sneaking()
	{
		//Don't place block, because player right clicked something that does something when right clicked
		let block_interaction_result = lib::block::interact_with_block_at(
			parsed_packet.location,
			block_id_at_location,
			parsed_packet.face,
			&game.block_state_data,
			player,
			players_clone,
			&game.packet_sender,
			dimension,
		);
		block_interaction_result
			.handle(dimension, parsed_packet.location, player, players_clone, block_id_at_location, &game.packet_sender)
			.inspect_err(|x| {
				println!(
					"lib::block::interact_with_block_at({:?}, {block_id_at_location}, {}) call resulted in error {x:?}",
					parsed_packet.location, parsed_packet.face
				)
			})
			.unwrap_or_default()
	} else if player.get_held_item(true).is_some_and(|x| x.count > 0 && x.id == data::items::get_item_id_by_name("minecraft:bucket").unwrap())
	{
		let blocks_state_id = dimension.get_block(new_block_location).unwrap();
		let block_name = data::blocks::get_block_name_from_block_state_id(blocks_state_id, &game.block_state_data);
		if block_name == "minecraft:water" {
			let held_item = player.get_held_item(true).unwrap();
			if held_item.count > 1 {
				let slot = Slot {
					count: held_item.count - 1,
					..held_item.clone()
				};
				player.set_selected_inventory_slot(Some(slot), players_clone, &game.packet_sender);
			} else {
				player.set_selected_inventory_slot(None, players_clone, &game.packet_sender);
			}
			let water_bucket_slot = Slot {
				count: 1,
				id: data::items::get_item_id_by_name("minecraft:water_bucket").unwrap(),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			};
			player.add_item_to_inventory(water_bucket_slot, players_clone, &game.packet_sender);
			vec![(0, new_block_location)]
		} else {
			vec![]
		}
	} else if player
		.get_held_item(true)
		.is_some_and(|x| x.count > 0 && x.id == data::items::get_item_id_by_name("minecraft:water_bucket").unwrap())
	{
		let bucket_slot = Slot {
			count: 1,
			id: data::items::get_item_id_by_name("minecraft:bucket").unwrap(),
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		player.set_selected_inventory_slot(Some(bucket_slot), players_clone, &game.packet_sender);

		let water_block = data::blocks::get_block_from_name("minecraft:water", &game.block_state_data);
		let full_water_block_id = water_block.states[water_block.default_state].id;

		vec![(full_water_block_id, new_block_location)]
	} else {
		//Let's go - we can place a block
		let used_item_id = player
			.get_held_item(true)
			.unwrap_or(&Slot {
				count: 0,
				id: 0,
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			})
			.id;
		let mut used_item_name = data::items::get_item_name_by_id(used_item_id).unwrap();

		if block_type_at_location == Type::Farm {
			used_item_name = match used_item_name {
				"minecraft:wheat_seeds" => "minecraft:wheat",
				"minecraft:carrot" => "minecraft:carrots",
				"minecraft:potato" => "minecraft:potatoes",
				"minecraft:beetroot_seeds" => "minecraft:beetroots",
				_ => used_item_name,
			};
		}

		if used_item_name == "minecraft:flint_and_steel" {
			used_item_name = "minecraft:fire";
		}

		let pitch = player.get_pitch();

		if used_item_name.ends_with("spawn_egg") {
			let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();
			lib::create_and_spawn_entity_from_egg(
				used_item_name,
				game.entity_id_manager.get_new(),
				new_block_location,
				dimension,
				players_clone,
				&game.packet_sender,
			);
		}

		let block_state_ids = lib::block::get_block_state_id(
			parsed_packet.face,
			player_get_looking_cardinal_direction,
			pitch,
			world.dimensions.get_mut(player.get_dimension()).unwrap(),
			new_block_location,
			used_item_name,
			parsed_packet.cursor_position_x,
			parsed_packet.cursor_position_y,
			parsed_packet.cursor_position_z,
			&game.block_state_data,
		);

		//first part of the if tries to check if player just right clicked an item that doesnt place a block
		if !block_state_ids.is_empty() && block_state_ids[0].0 != 0 && gamemode == Gamemode::Survival {
			let Some(hand_slot) = player.get_held_item(true) else {
				return;
			};

			let hand_slot = hand_slot.clone();
			let new_hand_slot = if hand_slot.count == 1 {
				None
			} else {
				Some(Slot {
					count: hand_slot.count - 1,
					..hand_slot
				})
			};

			player.set_selected_inventory_slot(new_hand_slot, players_clone, &game.packet_sender);
		}

		block_state_ids
	};

	let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();
	for block_to_place in &blocks_to_place {
		match dimension.overwrite_block(block_to_place.1, block_to_place.0) {
			Ok(res) => {
				let block = data::blocks::get_block_from_block_state_id(block_to_place.0, &game.block_state_data);
				//Logic to open sign editor when player placed a new sign, maybe move somewhere else or something idk
				if block.block_type == basic_types::blocks::Type::WallSign
					|| block.block_type == basic_types::blocks::Type::StandingSign
					|| block.block_type == basic_types::blocks::Type::WallHangingSign
					|| block.block_type == basic_types::blocks::Type::CeilingHangingSign
				{
					game.packet_sender.send_packet_to_player(
						&peer_addr,
						lib::packets::clientbound::play::OpenSignEditor::PACKET_ID,
						lib::packets::clientbound::play::OpenSignEditor {
							location: block_to_place.1,
							is_front_text: true,
						},
					);
				}
				#[allow(clippy::collapsible_if)]
				if res.is_some() && res.unwrap() == BlockOverwriteOutcome::DestroyBlockentity {
					if let Some(block_entity) = dimension
						.get_chunk_from_position(parsed_packet.location)
						.unwrap()
						.block_entities
						.iter()
						.find(|x| x.get_position() == parsed_packet.location)
					{
						let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
						block_entity.remove_self(&mut players, dimension, &game.packet_sender, &game.entity_id_manager);
					};
				}
			}
			Err(err) => {
				println!("couldn't place block because {err}");
				continue;
			}
		};
	}


	for player in players.iter() {
		for block in &blocks_to_place {
			game.packet_sender.send_packet_to_player(
				&player.peer_socket_address,
				lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
				lib::packets::clientbound::play::BlockUpdate {
					location: block.1,
					block_id: block.0 as i32,
				},
			);
		}
	}
	game.packet_sender.send_packet_to_player(
		&peer_addr,
		lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
		lib::packets::clientbound::play::AcknowledgeBlockChange {
			sequence_id: parsed_packet.sequence,
		},
	);

	for block_to_place in blocks_to_place {
		lib::block::update_all_recursively(
			dimension,
			block_to_place.1,
			&mut players,
			&game.packet_sender,
			&game.entity_id_manager,
			&game.block_state_data,
			&game.loot_tables,
		);
	}
}
