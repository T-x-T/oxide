use std::sync::Arc;

use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "dimension".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "dimension".to_string(),
			properties: ParserProperty::Dimension,
			next_arguments: Vec::new(),
			optional: false,
		}],
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("this command can only be used in game");
		return Ok(());
	};

	let mut players = game.players.lock().unwrap();
	let players_clone = players.clone();
	let mut world = game.world.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap();

	player.change_dimension(
		command.replace("dimension ", "").as_str(),
		&players_clone,
		&mut world,
		&game.packet_sender,
		&game.entity_id_manager,
		&game.block_state_data,
	);

	return Ok(());
}
