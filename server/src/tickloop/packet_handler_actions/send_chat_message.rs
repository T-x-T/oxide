use super::*;

pub fn process(peer_addr: SocketAddr, message: String, timestamp: i64, salt: i64, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();

	let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

	println!("<{}>: {}", player.display_name, message);

	let mut packet_to_send = lib::packets::clientbound::play::PlayerChatMessage {
		global_index: -1,
		sender: player.uuid,
		index: 0,
		message_signature_bytes: Vec::new(),
		message: message.clone(),
		timestamp,
		salt,
		signature_array: Vec::new(),
		unsigned_content: None,
		filter_type: 0,
		filter_type_bits: Vec::new(),
		chat_type: 1,
		sender_name: NbtTag::Root(vec![
			NbtTag::TagCompound(
				"click_event".to_string(),
				vec![
					NbtTag::String("action".to_string(), "suggest_command".to_string()),
					NbtTag::String("command".to_string(), format!("/tell {}", player.display_name).to_string()),
				],
			),
			NbtTag::String("insertion".to_string(), player.display_name.clone()),
			NbtTag::String("text".to_string(), player.display_name.clone()),
		]),
		target_name: None,
	};

	for player in players.iter_mut() {
		packet_to_send.global_index = player.chat_message_index;
		player.chat_message_index += 1;
		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::PlayerChatMessage::PACKET_ID,
			packet_to_send.clone().try_into().unwrap(),
		);
	}
}
