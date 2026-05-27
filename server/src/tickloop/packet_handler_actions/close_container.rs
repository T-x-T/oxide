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
			if let Some(slot) = slot
				&& slot.count > 0
			{
				inventory_updated = true;
				let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();

				dimension.summon_item(player.get_position(), slot.clone(), None, players_clone, &game.packet_sender, &game.entity_id_manager);
			}
		}

		if inventory_updated {
			player.set_inventory_and_inform_client(inventory, players_clone, &game.packet_sender);
		}
	} else {
		let player_position = player.get_position();
		let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();
		player.crafting_table_slots.iter_mut().filter(|x| x.count > 0).for_each(|x| {
			dimension.summon_item(player_position, x.clone(), None, players_clone, &game.packet_sender, &game.entity_id_manager);
		});

		if let Some(position) = player.opened_inventory_at {
			let number_of_players_with_container_opened =
				players_clone.iter().filter(|x| x.opened_inventory_at.is_some_and(|x| x == position)).count();
			//Close chest animation logic, to close chest when no players are using it anymore
			if number_of_players_with_container_opened == 1 {
				//1, because we havent called close_inventory() on current player yet
				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::BlockAction::PACKET_ID,
					lib::packets::clientbound::play::BlockAction {
						location: position,
						action_id: 1,
						action_parameter: 0,
						block_type: dimension.get_block(position).unwrap() as i32,
					},
				);
			}
		};

		player.close_inventory(&game.packet_sender).unwrap();
	}
}
