use super::*;

pub fn process(peer_addr: SocketAddr, status: u8, location: BlockPosition, sequence_id: i32, game: Arc<Game>, players_clone: &[Player]) {
	let mut players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();

	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	let player_clone = player.clone();

	let dimension = world.dimensions.get_mut(player_clone.get_dimension()).unwrap();

	let old_block_id = dimension.get_block(location).unwrap();

	if player_clone.get_gamemode() == Gamemode::Survival {
		let block_hardness = lib::block::get_hardness(old_block_id, &game.block_state_data);
		if status == 0 && block_hardness != 0.0 {
			player.start_mining();
			game.packet_sender.send_packet_to_player(
				&peer_addr,
				lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
				lib::packets::clientbound::play::AcknowledgeBlockChange { sequence_id },
			);
			return;
		} else if status == 1 {
			player.finish_mining();
			game.packet_sender.send_packet_to_player(
				&peer_addr,
				lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
				lib::packets::clientbound::play::AcknowledgeBlockChange { sequence_id },
			);
			return;
		} else if status == 2 || (status == 0 && block_hardness == 0.0) {
			player.finish_mining();
			player.apply_exhaustion_from_mining_block();
		} else if status == 5 {
			player.stop_eating();
		} else if status == 4 {
			let mut slot = player.get_selected_inventory_slot().clone();
			slot.subtract_count_mut(1).unwrap();

			dimension.summon_item(
				player.get_position(),
				Slot::new(slot.get_item_id(), 1),
				Some(player.get_common_entity_data().uuid),
				players_clone,
				&game.packet_sender,
				&game.entity_id_manager,
				20,
			);

			player.set_selected_inventory_slot(slot, players_clone, &game.packet_sender);
		} else if status == 3 {
			let slot = player.get_selected_inventory_slot().clone();

			dimension.summon_item(
				player.get_position(),
				slot,
				Some(player.get_common_entity_data().uuid),
				players_clone,
				&game.packet_sender,
				&game.entity_id_manager,
				20,
			);

			player.set_selected_inventory_slot(Slot::default(), players_clone, &game.packet_sender);
		}

		let items_to_drop = lib::loot_table::get_block_drops(
			&game.loot_tables,
			old_block_id,
			player_clone.get_held_item(true),
			&game.block_state_data,
			Some(&Entity::Player(player.clone())),
		);

		for item_to_drop in items_to_drop {
			if !item_to_drop.is_empty() {
				dimension.summon_item(location.into(), item_to_drop, None, &players, &game.packet_sender, &game.entity_id_manager, 0);
			}
		}
	}

	let res = dimension.overwrite_block(location, 0).unwrap();
	if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
		let block_entity =
			dimension.get_chunk_from_position(location).unwrap().block_entities.iter().find(|x| x.get_position() == location).unwrap();
		let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
		block_entity.remove_self(&mut players, dimension, &game.packet_sender, &game.entity_id_manager);
	}

	players
		.iter()
		.inspect(|x| {
			game.packet_sender.send_packet_to_player(
				&x.peer_socket_address,
				lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
				lib::packets::clientbound::play::BlockUpdate { location, block_id: 0 },
			);
		})
		.filter(|x| x.peer_socket_address != peer_addr)
		.for_each(|x| {
			game.packet_sender.send_packet_to_player(
				&x.peer_socket_address,
				lib::packets::clientbound::play::WorldEvent::PACKET_ID,
				lib::packets::clientbound::play::WorldEvent { event: 2001, location, data: old_block_id as i32 },
			);
		});

	game.packet_sender.send_packet_to_player(
		&peer_addr,
		lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
		lib::packets::clientbound::play::AcknowledgeBlockChange { sequence_id },
	);

	lib::block::update_all_recursively(
		dimension,
		location,
		&mut players,
		&game.packet_sender,
		&game.entity_id_manager,
		&game.block_state_data,
		&game.loot_tables,
	);
}
