use super::*;

pub fn process(peer_addr: SocketAddr, window_id: i32, game: Arc<Game>, players_clone: &[Player]) {
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	let mut world = game.world.lock().unwrap();


	if window_id == 0 {
		//Drop items in crafting field
		let mut inventory = player.get_inventory().clone();
		let mut inventory_updated = false;
		for i in 1..=4 {
			let slot = &mut inventory[i];
			if slot.as_ref().is_some_and(|x| x.count > 0) {
				inventory_updated = true;
				let item_entity = slot.take().unwrap().get_entity(player.get_position(), game.entity_id_manager.get_new());

				let spawn_packet = item_entity.to_spawn_entity_packet();

				let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
					entity_id: item_entity.get_common_entity_data().entity_id,
					metadata: item_entity.get_metadata(),
				};

				world.dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Entity::Item(item_entity));

				players_clone.iter().for_each(|x| {
					game.send_packet(
						&x.peer_socket_address,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						spawn_packet.clone().try_into().unwrap(),
					);
					game.send_packet(
						&x.peer_socket_address,
						lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
						metadata_packet.clone().try_into().unwrap(),
					);
				});
			}
		}

		if inventory_updated {
			player.set_inventory_and_inform_client(inventory, players_clone, game.clone());
		}
	} else {
		let player_position = player.get_position();
		player.crafting_table_slots.iter_mut().filter(|x| x.count > 0).for_each(|x| {
			let item_entity = x.get_entity(player_position, game.entity_id_manager.get_new());
			*x = Slot::default();

			let spawn_packet = item_entity.to_spawn_entity_packet();

			let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
				entity_id: item_entity.get_common_entity_data().entity_id,
				metadata: item_entity.get_metadata(),
			};

			world.dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Entity::Item(item_entity));

			players_clone.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
					spawn_packet.clone().try_into().unwrap(),
				);
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					metadata_packet.clone().try_into().unwrap(),
				);
			});
		});

		if let Some(position) = player.opened_inventory_at {
			let number_of_players_with_container_opened =
				players_clone.iter().filter(|x| x.opened_inventory_at.is_some_and(|x| x == position)).count();
			//Close chest animation logic, to close chest when no players are using it anymore
			if number_of_players_with_container_opened == 1 {
				//1, because we havent called close_inventory() on current player yet
				players_clone.iter().for_each(|x| {
					game.send_packet(
						&x.peer_socket_address,
						lib::packets::clientbound::play::BlockAction::PACKET_ID,
						lib::packets::clientbound::play::BlockAction {
							location: position,
							action_id: 1,
							action_parameter: 0,
							block_type: world.dimensions.get("minecraft:overworld").unwrap().get_block(position).unwrap() as i32,
						}
						.try_into()
						.unwrap(),
					);
				});
			}
		};

		player.close_inventory(game.clone()).unwrap();
	}
}
