use super::*;
use std::process;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "panic".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(_command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	if let Some(socket_addr) = socket_addr {
		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "this command is only intended to be used from the console".to_string()),
				]),
				overlay: false,
			},
		);

		return Ok(());
	}


	eprintln!("panic command was used");
	process::exit(1000);
}
