use super::*;

pub fn process(peer_addr: SocketAddr, command_string: String, game: Arc<Game>) {
	let players = game.players.lock().unwrap();
	let player = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap();
	println!("<{}> invoked: {}", player.display_name, command_string);

	let commands = game.commands.lock().unwrap().clone();

	let Some(command) = commands.iter().find(|x| x.name == command_string.split(" ").next().unwrap_or_default()) else {
		game.send_packet(
			&peer_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "command not found".to_string()),
				]),
				overlay: false,
			}
			.try_into()
			.unwrap(),
		);
		return;
	};

	let mut stream = player.connection_stream.try_clone().unwrap();
	drop(players);
	(command.execute)(command_string, Some(&mut stream), game.clone()).unwrap();
}
