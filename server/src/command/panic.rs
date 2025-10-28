use std::{collections::HashMap, net::SocketAddr, process};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "panic".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(_command: String, stream: Option<&mut TcpStream>, _game: &mut Game, _connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<(), Box<dyn Error>> {
	if let Some(stream) = stream {
		lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::Root(vec![
				NbtTag::String("type".to_string(), "text".to_string()),
				NbtTag::String("text".to_string(), "this command is only intended to be used from the console".to_string()),
			]),
		  overlay: false,
	 	}.try_into()?)?;

		return Ok(());
	}


	eprintln!("panic command was used");
	process::exit(1000);
}
