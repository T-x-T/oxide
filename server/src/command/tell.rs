use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "tell".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "to player".to_string(),
			properties: ParserProperty::Entity(3),
			next_arguments: vec![CommandArgument {
				name: "message".to_string(),
				properties: ParserProperty::String(2),
				next_arguments: Vec::new(),
				optional: false,
			}],
			optional: false,
		}],
	});
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let players = game.players.lock().unwrap();

	let Some(target_player) = players.iter().find(|x| x.display_name == command.split(" ").nth(1).unwrap_or_default()) else {
		let Some(socket_addr) = socket_addr else {
			println!("Couldn't find that player :(");
			return Ok(());
		};

		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "Couldn't find that player :(".to_string()),
				]),
				overlay: false,
			},
		);

		return Ok(());
	};

	let sending_player_name = if socket_addr.is_some() {
		players.iter().find(|x| x.peer_socket_address == socket_addr.unwrap()).unwrap().display_name.clone()
	} else {
		"console".to_string()
	};

	game.packet_sender.send_packet_to_player(
		&target_player.peer_socket_address,
		lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
		lib::packets::clientbound::play::SystemChatMessage {
			content: NbtTag::Root(vec![
				NbtTag::String("type".to_string(), "text".to_string()),
				NbtTag::String(
					"text".to_string(),
					format!("<{}> whispered: {}", sending_player_name, command.split(" ").skip(2).collect::<Vec<&str>>().join(" ")),
				),
			]),
			overlay: false,
		},
	);

	return Ok(());
}
