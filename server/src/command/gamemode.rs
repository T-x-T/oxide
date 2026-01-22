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
		"survival" => 0,
		"creative" => 1,
		"adventure" => 2,
		"spectator" => 3,
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

	game.send_packet(
		&stream.peer_addr()?,
		lib::packets::clientbound::play::GameEvent::PACKET_ID,
		lib::packets::clientbound::play::GameEvent {
			event: 3,
			value: parsed_gamemode as f32,
		}
		.try_into()?,
	);

	return Ok(());
}
