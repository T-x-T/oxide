use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "hi".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(_command: String, stream: Option<&mut TcpStream>, _game: &mut Game, _connection_streams: &mut HashMap<SocketAddr, TcpStream>, _connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("Hi back :)");
		return Ok(());
	};
	lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
	  content: NbtTag::Root(vec![
			NbtTag::String("type".to_string(), "text".to_string()),
			NbtTag::String("text".to_string(), "Hi back :)".to_string()),
		]),
	  overlay: true,
 	}.try_into()?)?;

	return Ok(());
}
