use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "gamemode".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "gamemode".to_string(),
			properties: ParserProperty::Gamemode,
			next_arguments: Vec::new(),
			optional: false,
		}],
	});
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(socket_addr) = socket_addr else {
		println!("this command doesnt work from the console");
		return Ok(());
	};

	let gamemode = command.replace("gamemode ", "");

	let parsed_gamemode = match gamemode.as_str() {
		"survival" => basic_types::Gamemode::Survival,
		"creative" => basic_types::Gamemode::Creative,
		"adventure" => basic_types::Gamemode::Adventure,
		"spectator" => basic_types::Gamemode::Spectator,
		_ => {
			game.packet_sender.send_packet_to_player(
				&socket_addr,
				lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
				lib::packets::clientbound::play::SystemChatMessage {
					content: NbtTag::Root(vec![
						NbtTag::String("type".to_string(), "text".to_string()),
						NbtTag::String("text".to_string(), format!("invalid gamemode {gamemode}")),
					]),
					overlay: false,
				},
			);
			return Ok(());
		}
	};

	let game_clone = game.clone();
	let players_clone = game.players.lock().unwrap().clone();
	let mut players = game_clone.players.try_lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == socket_addr).unwrap();

	player.set_gamemode(parsed_gamemode, players_clone.as_slice(), &game.packet_sender)?;

	return Ok(());
}
