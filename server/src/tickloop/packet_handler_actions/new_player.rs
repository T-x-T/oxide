use super::*;

pub fn process(peer_addr: SocketAddr, stream: TcpStream, game: Arc<Game>) {
	game.connections.entry(peer_addr).and_modify(|x| x.state = lib::ConnectionState::Play);

	let mut world = game.world.lock().unwrap();

	let connection_player = game.connections.get(&peer_addr).unwrap();

	let mut new_player = Player::new(
		connection_player.player_name.clone().unwrap_or_default(),
		connection_player.player_uuid.unwrap_or_default(),
		peer_addr,
		game.clone(),
		stream,
		world.default_spawn_location,
	);
	let mut players = game.players.lock().unwrap();

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::Login::PACKET_ID,
		lib::packets::clientbound::play::Login {
			entity_id: new_player.entity_id,
			is_hardcore: false,
			dimension_names: vec!["minecraft:overworld".to_string()],
			max_players: 9,
			view_distance: 32,
			simulation_distance: 32,
			reduced_debug_info: false,
			enable_respawn_screen: true,
			do_limited_crafting: false,
			dimension_type: 0,
			dimension_name: "minecraft:overworld".to_string(),
			hashed_seed: 1,
			game_mode: new_player.get_gamemode() as u8,
			previous_game_mode: -1,
			is_debug: false,
			is_flat: false,
			has_death_location: false,
			death_dimension_name: None,
			death_location: None,
			portal_cooldown: 123,
			sea_level: 64,
			enforces_secure_chat: false,
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID,
		lib::packets::clientbound::play::SynchronizePlayerPosition {
			teleport_id: new_player.current_teleport_id,
			x: new_player.get_position().x,
			y: new_player.get_position().y,
			z: new_player.get_position().z,
			velocity_x: 0.0,
			velocity_y: 0.0,
			velocity_z: 0.0,
			yaw: new_player.get_position().yaw,
			pitch: new_player.get_position().pitch,
			flags: 0,
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::GameEvent::PACKET_ID,
		lib::packets::clientbound::play::GameEvent {
			event: 13,
			value: 0.0,
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::EntityEvent::PACKET_ID,
		lib::packets::clientbound::play::EntityEvent {
			entity_id: new_player.entity_id,
			entity_status: 28, //set op permission level 4
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::SetHealth::PACKET_ID,
		lib::packets::clientbound::play::SetHealth {
			health: new_player.get_health(),
			food: 20,
			food_saturation: 0.0,
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::SetContainerContent::PACKET_ID,
		lib::packets::clientbound::play::SetContainerContent {
			window_id: 0,
			state_id: 1,
			slot_data: new_player.get_inventory().clone(),
			carried_item: None,
		}
		.try_into()
		.unwrap(),
	);

	let current_chunk_coords = BlockPosition::from(new_player.get_position()).convert_to_coordinates_of_chunk();

	let now = std::time::Instant::now();
	for x in current_chunk_coords.x - lib::VIEW_DISTANCE as i32..=current_chunk_coords.x + lib::VIEW_DISTANCE as i32 {
		for z in current_chunk_coords.z - lib::VIEW_DISTANCE as i32..=current_chunk_coords.z + lib::VIEW_DISTANCE as i32 {
			new_player.send_chunk(&mut world, x, z, &game.entity_id_manager, &game.block_state_data, game.clone()).unwrap();
		}
	}
	println!("send chunks: {:?}", std::time::Instant::now() - now);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::SetHeldItem::PACKET_ID,
		lib::packets::clientbound::play::SetHeldItem {
			slot: new_player.get_selected_slot(),
		}
		.try_into()
		.unwrap(),
	);

	let new_player_uuid = new_player.uuid;
	let new_player_entity_id = new_player.entity_id;
	let new_player_display_name = new_player.display_name.clone();
	let new_player_x = new_player.get_position().x;
	let new_player_y = new_player.get_position().y;
	let new_player_z = new_player.get_position().z;
	let new_player_inventory = new_player.get_inventory().clone();
	let new_player_selected_slot = new_player.get_selected_slot();
	let new_player_entity_metadata = new_player.get_metadata();
	let new_player_gamemode = new_player.get_gamemode();

	//send player list to newly connected player
	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID,
		lib::packets::clientbound::play::PlayerInfoUpdate {
			actions: 255,
			players: players
				.iter()
				.map(|y| {
					(
						y.uuid,
						vec![
							lib::packets::clientbound::play::PlayerAction::AddPlayer(y.display_name.clone(), vec![]),
							lib::packets::clientbound::play::PlayerAction::InitializeChat(None),
							lib::packets::clientbound::play::PlayerAction::UpdateGameMode(new_player_gamemode as u8 as i32),
							lib::packets::clientbound::play::PlayerAction::UpdateListed(true),
							lib::packets::clientbound::play::PlayerAction::UpdateLatency(0),
							lib::packets::clientbound::play::PlayerAction::UpdateDisplayName(None),
							lib::packets::clientbound::play::PlayerAction::UpdateListPriority(0),
							lib::packets::clientbound::play::PlayerAction::UpdateHat(true),
						],
					)
				})
				.collect(),
		}
		.try_into()
		.unwrap(),
	);

	players.push(new_player);

	//update player list for already connected players
	players.iter().for_each(|x| {
		game.send_packet(
			&x.peer_socket_address,
			lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID,
			lib::packets::clientbound::play::PlayerInfoUpdate {
				actions: 255,
				players: vec![(
					new_player_uuid,
					vec![
						lib::packets::clientbound::play::PlayerAction::AddPlayer(new_player_display_name.clone(), vec![]),
						lib::packets::clientbound::play::PlayerAction::InitializeChat(None),
						lib::packets::clientbound::play::PlayerAction::UpdateGameMode(new_player_gamemode as u8 as i32),
						lib::packets::clientbound::play::PlayerAction::UpdateListed(true),
						lib::packets::clientbound::play::PlayerAction::UpdateLatency(0),
						lib::packets::clientbound::play::PlayerAction::UpdateDisplayName(None),
						lib::packets::clientbound::play::PlayerAction::UpdateListPriority(0),
						lib::packets::clientbound::play::PlayerAction::UpdateHat(true),
					],
				)],
			}
			.try_into()
			.unwrap(),
		);
	});

	//Spawn other already connected player entities for newly joined player
	for player in players.iter() {
		if player.uuid == new_player_uuid {
			continue;
		}

		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
			lib::packets::clientbound::play::SpawnEntity {
				entity_id: player.entity_id,
				entity_uuid: player.uuid,
				entity_type: data::entities::get_id_from_name("minecraft:player"),
				x: player.get_position().x,
				y: player.get_position().y,
				z: player.get_position().z,
				pitch: player.get_pitch_u8(),
				yaw: player.get_yaw_u8(),
				head_yaw: player.get_yaw_u8(),
				data: 0,
				velocity_x: 0,
				velocity_y: 0,
				velocity_z: 0,
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
			lib::packets::clientbound::play::SetEntityMetadata {
				entity_id: player.entity_id,
				metadata: new_player_entity_metadata.clone(),
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SetEquipment::PACKET_ID,
			lib::packets::clientbound::play::SetEquipment {
				entity_id: player.entity_id,
				equipment: vec![
					(0, player.get_inventory()[(player.get_selected_slot() + 36) as usize].clone()),
					(1, player.get_inventory()[45].clone()),
					(2, player.get_inventory()[8].clone()),
					(3, player.get_inventory()[7].clone()),
					(4, player.get_inventory()[6].clone()),
					(5, player.get_inventory()[5].clone()),
				],
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID,
			lib::packets::clientbound::play::UpdateEntityRotation {
				entity_id: player.entity_id,
				on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
				yaw: player.get_yaw_u8(),
				pitch: player.get_pitch_u8(),
			}
			.try_into()
			.unwrap(),
		);
		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
			lib::packets::clientbound::play::SetHeadRotation {
				entity_id: player.entity_id,
				head_yaw: player.get_yaw_u8(),
			}
			.try_into()
			.unwrap(),
		);
	}

	//Spawn player entity for other players that are already connected
	for player in players.iter() {
		if player.peer_socket_address == peer_addr {
			continue;
		}

		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
			lib::packets::clientbound::play::SpawnEntity {
				entity_id: new_player_entity_id,
				entity_uuid: new_player_uuid,
				entity_type: data::entities::get_id_from_name("minecraft:player"),
				x: new_player_x,
				y: new_player_y,
				z: new_player_z,
				pitch: 0,
				yaw: 0,
				head_yaw: 0,
				data: 0,
				velocity_x: 0,
				velocity_y: 0,
				velocity_z: 0,
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
			lib::packets::clientbound::play::SetEntityMetadata {
				entity_id: new_player_entity_id,
				metadata: new_player_entity_metadata.clone(),
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::SetEquipment::PACKET_ID,
			lib::packets::clientbound::play::SetEquipment {
				entity_id: new_player_entity_id,
				equipment: vec![
					(0, new_player_inventory[(new_player_selected_slot + 36) as usize].clone()),
					(1, new_player_inventory[45].clone()),
					(2, new_player_inventory[8].clone()),
					(3, new_player_inventory[7].clone()),
					(4, new_player_inventory[6].clone()),
					(5, new_player_inventory[5].clone()),
				],
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID,
			lib::packets::clientbound::play::UpdateEntityRotation {
				entity_id: player.entity_id,
				on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
				yaw: player.get_yaw_u8(),
				pitch: player.get_pitch_u8(),
			}
			.try_into()
			.unwrap(),
		);
		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
			lib::packets::clientbound::play::SetHeadRotation {
				entity_id: player.entity_id,
				head_yaw: player.get_yaw_u8(),
			}
			.try_into()
			.unwrap(),
		);
	}


	for entity in &world.dimensions.get("minecraft:overworld").unwrap().entities {
		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
			entity.to_spawn_entity_packet().try_into().unwrap(),
		);

		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
			lib::packets::clientbound::play::SetEntityMetadata {
				entity_id: entity.get_common_entity_data().entity_id,
				metadata: entity.get_metadata(),
			}
			.try_into()
			.unwrap(),
		);
	}

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::Commands::PACKET_ID,
		lib::packets::clientbound::play::Commands {
			nodes: crate::command::get_command_packet_data(game.clone()),
			root_index: 0,
		}
		.try_into()
		.unwrap(),
	);

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::SetTabListHeaderAndFooter::PACKET_ID,
		lib::packets::clientbound::play::SetTabListHeaderAndFooter {
			header: NbtTag::Root(vec![NbtTag::String("text".to_string(), "".to_string())]),
			footer: NbtTag::Root(vec![
				NbtTag::String("type".to_string(), "text".to_string()),
				NbtTag::String("text".to_string(), " powered by Oxide ".to_string()),
				NbtTag::String("color".to_string(), "gray".to_string()),
				NbtTag::Byte("italic".to_string(), 1),
			]),
		}
		.try_into()
		.unwrap(),
	);
}
