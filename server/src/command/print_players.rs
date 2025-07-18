use std::{collections::HashMap, net::SocketAddr};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "print_players".to_string(),
		execute,
		arguments: vec![],
	});
}

fn execute(_command: String, stream: Option<&mut TcpStream>, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>, connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	if stream.is_some() {
		lib::utils::send_packet(stream.unwrap(), lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::TagCompound(None, vec![
				NbtTag::String(Some("type".to_string()), "text".to_string()),
				NbtTag::String(Some("text".to_string()), "this command is only intended to be used from the console".to_string()),
			]),
		  overlay: false,
	 	}.try_into()?)?;

		return Ok(());
	}

	println!("game.players:\n{:?}", game.players);
	println!("connection_streams:\n{connection_streams:?}");
	println!("connections:\n{connections:?}");

	return Ok(());
}
