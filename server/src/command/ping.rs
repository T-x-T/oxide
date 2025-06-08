use std::{collections::HashMap, net::SocketAddr};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "ping".to_string(),
		execute,
		arguments: vec![
			CommandArgument {
				name: "message".to_string(),
				properties: ParserProperty::String(1),
				next_arguments: Vec::new(),
				optional: true,
			},
			CommandArgument {
				name: "first_arg".to_string(),
				properties: ParserProperty::String(1),
				next_arguments: vec![
					CommandArgument {
						name: "second_arg".to_string(),
						properties: ParserProperty::Bool,
						next_arguments: vec![
							CommandArgument {
								name: "third_arg".to_string(),
								properties: ParserProperty::Gamemode,
								next_arguments: Vec::new(),
								optional: true,
							}
						],
						optional: false,
					}
				],
				optional: false,
			},
		],
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, _game: &mut Game, _connection_streams: &mut HashMap<SocketAddr, TcpStream>, _connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	let reply_msg = if command.as_str() == "ping" {
   	"pong".to_string()
  } else {
  		command.replace("ping ", "")
  };

	let Some(stream) = stream else {
		println!("{reply_msg}");
		return Ok(());
	};

 	lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
	  content: NbtTag::TagCompound(None, vec![
			NbtTag::String(Some("type".to_string()), "text".to_string()),
			NbtTag::String(Some("text".to_string()), reply_msg),
		]),
	  overlay: false,
 	}.try_into()?)?;

	return Ok(());
}
