use super::*;

pub fn process(game: Arc<Game>, players_clone: Vec<Player>) {
	if std::time::Instant::now() > *game.last_player_keepalive_timestamp.lock().unwrap() + std::time::Duration::from_secs(5) {
		*game.last_player_keepalive_timestamp.lock().unwrap() = std::time::Instant::now();

		let players = players_clone.clone();
		let game = game.clone();
		std::thread::spawn(move || {
			for player in &players {
				let useless_buf_no_one_crates_about = &mut [0; 1];
				if player.connection_stream.peek(useless_buf_no_one_crates_about).is_err() {
					crate::disconnect_player(&player.peer_socket_address, game.clone());
				}
				game.send_packet(
					&player.peer_socket_address,
					lib::packets::clientbound::play::ClientboundKeepAlive::PACKET_ID,
					lib::packets::clientbound::play::ClientboundKeepAlive {
						keep_alive_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
					}
					.try_into()
					.unwrap(),
				);
			}
		});
	}
}
