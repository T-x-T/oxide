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

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = socket_addr else {
		println!("this command can only be used in game");
		return Ok(());
	};

	let mut players = game.players.lock().unwrap();
	let players_clone = players.clone();
	let mut world = game.world.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == stream).unwrap();

	let default_spawn_location = world.default_spawn_location;
	let dimension = world.dimensions.get_mut(command.replace("dimension ", "").as_str()).unwrap();

	player.change_dimension(
		command.replace("dimension ", "").as_str(),
		&players_clone,
		dimension,
		&game.packet_sender,
		default_spawn_location,
	);

	return Ok(());
}
