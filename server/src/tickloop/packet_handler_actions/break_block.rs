use super::*;

pub fn process(peer_addr: SocketAddr, status: u8, location: BlockPosition, sequence_id: i32, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();

	let player_clone = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().clone();

	let old_block_id = world.dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();

	if player_clone.get_gamemode() == Gamemode::Survival {
		let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
		let block_hardness = lib::block::get_hardness(old_block_id, &game.block_state_data);
		if status == 0 && block_hardness != 0.0 {
			player.start_mining();
			game.send_packet(
				&peer_addr,
				lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
				lib::packets::clientbound::play::AcknowledgeBlockChange {
					sequence_id,
				}
				.try_into()
				.unwrap(),
			);
			return;
		} else if status == 1 {
			player.finish_mining();
			game.send_packet(
				&peer_addr,
				lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
				lib::packets::clientbound::play::AcknowledgeBlockChange {
					sequence_id,
				}
				.try_into()
				.unwrap(),
			);
			return;
		} else if status == 2 || (status == 0 && block_hardness == 0.0) {
			player.finish_mining();
		}

		let all_items = data::items::get_items();
		let hand_item = all_items
			.get(data::items::get_item_name_by_id(player_clone.get_held_item(true).unwrap_or(&Slot::default()).item_id))
			.unwrap_or(all_items.get("minecraft:air").unwrap());

		let item_to_drop = lib::block::get_item_drop(old_block_id, hand_item.id, &game.block_state_data);

		if item_to_drop.id != "minecraft:air" {
			let new_entity = lib::entity::ItemEntity {
				common: lib::entity::CommonEntity {
					position: EntityPosition {
						x: location.x as f64 + 0.5,
						y: location.y as f64,
						z: location.z as f64 + 0.5,
						yaw: 0.0,
						pitch: 0.0,
					},
					velocity: EntityPosition::default(),
					uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
					entity_id: game.entity_id_manager.get_new(),
					..Default::default()
				},
				age: 0,
				health: 5,
				item: item_to_drop,
				owner: player_clone.uuid,
				pickup_delay: 0,
				thrower: player_clone.uuid,
			};

			let packet = new_entity.to_spawn_entity_packet();

			let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
				entity_id: new_entity.get_common_entity_data().entity_id,
				metadata: new_entity.get_metadata(),
			};

			world.dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Entity::Item(new_entity));

			players.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
					packet.clone().try_into().unwrap(),
				);
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					metadata_packet.clone().try_into().unwrap(),
				);
			});
		}
	}

	let res = world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(location, 0).unwrap();
	if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
		let block_entity = world
			.dimensions
			.get("minecraft:overworld")
			.unwrap()
			.get_chunk_from_position(location)
			.unwrap()
			.block_entities
			.iter()
			.find(|x| x.get_position() == location)
			.unwrap();
		let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
		block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
	}

	players
		.iter()
		.inspect(|x| {
			game.send_packet(
				&x.peer_socket_address,
				lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
				lib::packets::clientbound::play::BlockUpdate {
					location,
					block_id: 0,
				}
				.try_into()
				.unwrap(),
			);
		})
		.filter(|x| x.peer_socket_address != peer_addr)
		.for_each(|x| {
			game.send_packet(
				&x.peer_socket_address,
				lib::packets::clientbound::play::WorldEvent::PACKET_ID,
				lib::packets::clientbound::play::WorldEvent {
					event: 2001,
					location,
					data: old_block_id as i32,
				}
				.try_into()
				.unwrap(),
			);
		});

	game.send_packet(
		&peer_addr,
		lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
		lib::packets::clientbound::play::AcknowledgeBlockChange {
			sequence_id,
		}
		.try_into()
		.unwrap(),
	);

	let blocks_to_update = [
		BlockPosition {
			x: location.x + 1,
			..location
		},
		BlockPosition {
			x: location.x - 1,
			..location
		},
		BlockPosition {
			y: location.y + 1,
			..location
		},
		BlockPosition {
			y: location.y - 1,
			..location
		},
		BlockPosition {
			z: location.z + 1,
			..location
		},
		BlockPosition {
			z: location.z - 1,
			..location
		},
	];

	for block_to_update in blocks_to_update {
		let res = lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
		if let Some(new_block) = res {
			match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block) {
				Ok(_) => {
					for player in players.iter() {
						game.send_packet(
							&player.peer_socket_address,
							lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
							lib::packets::clientbound::play::BlockUpdate {
								location: block_to_update,
								block_id: new_block as i32,
							}
							.try_into()
							.unwrap(),
						);
					}
				}
				Err(err) => {
					println!("couldn't place block because {err}");
					continue;
				}
			}
		};
	}
}
