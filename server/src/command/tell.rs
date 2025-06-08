use std::{collections::HashMap, net::SocketAddr};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
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

fn execute(command: String, stream: &mut TcpStream, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>, _connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	let Some(target_player) = game.players.iter().find(|x| x.display_name == command.split(" ").nth(1).unwrap_or_default()) else {
		lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::TagCompound(None, vec![
				NbtTag::String(Some("type".to_string()), "text".to_string()),
				NbtTag::String(Some("text".to_string()), "Couldn't find that player :(".to_string()),
			]),
		  overlay: false,
	 	}.try_into()?)?;

		return Ok(());
	};

	let sending_player = game.players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap();

	lib::utils::send_packet(connection_streams.get(&target_player.peer_socket_address).unwrap(), lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
		  content: NbtTag::TagCompound(None, vec![
			NbtTag::String(Some("type".to_string()), "text".to_string()),
			NbtTag::String(Some("text".to_string()), format!("<{}> whispered: {}", sending_player.display_name, command.split(" ").skip(2).collect::<Vec<&str>>().join(" "))),
		]),
	  overlay: false,
 	}.try_into()?)?;

	return Ok(());
}
