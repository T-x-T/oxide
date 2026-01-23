use std::sync::Arc;

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

fn execute(command: String, stream: Option<&mut TcpStream>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("this command doesnt work from the console");
		return Ok(());
	};

	let gamemode = command.replace("gamemode ", "");

	let parsed_gamemode = match gamemode.as_str() {
		"survival" => lib::player::Gamemode::Survival,
		"creative" => lib::player::Gamemode::Creative,
		"adventure" => lib::player::Gamemode::Adventure,
		"spectator" => lib::player::Gamemode::Spectator,
		_ => {
			game.send_packet(
				&stream.peer_addr()?,
				lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
				lib::packets::clientbound::play::SystemChatMessage {
					content: NbtTag::Root(vec![
						NbtTag::String("type".to_string(), "text".to_string()),
						NbtTag::String("text".to_string(), format!("invalid gamemode {gamemode}")),
					]),
					overlay: false,
				}
				.try_into()?,
			);
			return Ok(());
		}
	};

	let game_clone = game.clone();
	let mut players = game_clone.players.try_lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap();

	player.set_gamemode(parsed_gamemode, game)?;

	return Ok(());
}
