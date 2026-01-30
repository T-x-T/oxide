use super::*;

pub fn process(peer_addr: SocketAddr, window_id: i32, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let world = game.world.lock().unwrap();

	if window_id != 0 {
		if let Some(position) = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().opened_inventory_at {
			let number_of_players_with_container_opened = players.iter().filter(|x| x.opened_inventory_at.is_some_and(|x| x == position)).count();

			if number_of_players_with_container_opened == 1 {
				//1, because we havent called close_inventory() on current player yet
				players.iter().for_each(|x| {
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

		players.iter_mut().filter(|x| x.peer_socket_address == peer_addr).for_each(|x| x.close_inventory(game.clone()).unwrap());
	}
}
