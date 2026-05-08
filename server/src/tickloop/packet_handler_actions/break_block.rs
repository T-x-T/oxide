use super::*;

pub fn process(peer_addr: SocketAddr, status: u8, location: BlockPosition, sequence_id: i32, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();

	let player_clone = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().clone();

	let dimension = world.dimensions.get_mut(player_clone.get_dimension()).unwrap();

	let old_block_id = dimension.get_block(location).unwrap();

	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	if player_clone.get_gamemode() == Gamemode::Survival {
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
			player.apply_exhaustion_from_mining_block();
		} else if status == 5 {
			player.stop_eating();
		}

		let items_to_drop = lib::loot_table::get_block_drops(
			&game.loot_tables,
			old_block_id,
			player_clone.get_held_item(true).unwrap_or(&Slot::default()),
			&game.block_state_data,
		);

		for item_to_drop in items_to_drop {
			if item_to_drop.id != 0 {
				dimension.summon_item(location.into(), item_to_drop, None, &players, game.clone());
			}
		}
	}


	let res = dimension.overwrite_block(location, 0).unwrap();
	if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
		let block_entity =
			dimension.get_chunk_from_position(location).unwrap().block_entities.iter().find(|x| x.get_position() == location).unwrap();
		let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
		block_entity.remove_self(&mut players, dimension, game.clone());
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
		let res = lib::block::update(block_to_update, dimension, &game.block_state_data).unwrap();
		res.handle(dimension, block_to_update, &mut players, game.clone());
	}
}
