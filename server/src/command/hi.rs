use std::{collections::HashMap, net::SocketAddr};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "hi".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(_command: String, stream: &mut TcpStream, _game: &mut Game, _connection_streams: &mut HashMap<SocketAddr, TcpStream>, _connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
  	lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::TagCompound(None, vec![
				NbtTag::String(Some("type".to_string()), "text".to_string()),
				NbtTag::String(Some("text".to_string()), "Hi back :)".to_string()),
			]),
		  overlay: true,
   	}.try_into()?)?;

	return Ok(());
}
