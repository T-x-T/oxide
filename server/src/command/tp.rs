use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "tp".to_string(),
		execute,
		arguments: vec![
			CommandArgument {
				name: "to player".to_string(),
				properties: ParserProperty::Entity(3),
				next_arguments: Vec::new(),
				optional: false,
			},
			CommandArgument {
				name: "to coordinates".to_string(),
				properties: ParserProperty::Vec3,
				next_arguments: Vec::new(),
				optional: false,
			},
		],
	});
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(socket_addr) = socket_addr else {
		println!("this command doesnt work from the console");
		return Ok(());
	};

	let mut players = game.players.lock().unwrap();

	let arg_string = command.replace("tp ", "");
	if arg_string.is_empty() {
		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "missing a destination to teleport to".to_string()),
				]),
				overlay: false,
			},
		);
	}

	let target_player = players.iter().find(|x| x.display_name == arg_string.split(" ").next().unwrap_or_default());

	let target_coordinates: EntityPosition = if target_player.as_ref().is_some() {
		target_player.unwrap().get_position()
	} else {
		let mut arg_iter = arg_string.split(" ");
		let x = arg_iter.next().unwrap_or_default();
		let x: f64 = str::parse(x).unwrap_or_default();
		let y = arg_iter.next().unwrap_or_default();
		let y: f64 = str::parse(y).unwrap_or_default();
		let z = arg_iter.next().unwrap_or_default();
		let z: f64 = str::parse(z).unwrap_or_default();

		if x > 30_000_000.0 || y > 30_000_000.0 || z > 30_000_000.0 || x < -30_000_000.0 || y < -30_000_000.0 || z < -30_000_000.0 {
			game.packet_sender.send_packet_to_player(
				&socket_addr,
				lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
				lib::packets::clientbound::play::SystemChatMessage {
					content: NbtTag::Root(vec![
						NbtTag::String("type".to_string(), "text".to_string()),
						NbtTag::String("text".to_string(), "coordinates must be between -30 million and +30 million".to_string()),
					]),
					overlay: false,
				},
			);

			return Ok(());
		}

		EntityPosition {
			x,
			y,
			z,
			yaw: 0.0,
			pitch: 0.0,
		}
	};

	let sending_player = players.iter_mut().find(|x| x.peer_socket_address == socket_addr).unwrap();

	let sending_player_entity_id = sending_player.entity_id;

	let mut world = game.world.lock().unwrap();
	let dimension = world.dimensions.get_mut(sending_player.get_dimension()).unwrap();

	sending_player.new_position(target_coordinates.x, target_coordinates.y, target_coordinates.z, dimension, &game.packet_sender)?;

	sending_player.current_teleport_id += 1;
	game.packet_sender.send_packet_to_player(
		&socket_addr,
		lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID,
		lib::packets::clientbound::play::SynchronizePlayerPosition {
			teleport_id: sending_player.current_teleport_id,
			x: target_coordinates.x,
			y: target_coordinates.y,
			z: target_coordinates.z,
			velocity_x: 0.0,
			velocity_y: 0.0,
			velocity_z: 0.0,
			yaw: target_coordinates.yaw,
			pitch: target_coordinates.pitch,
			flags: 0,
		},
	);

	for other_addr in players.iter().map(|x| &x.peer_socket_address).collect::<Vec<&SocketAddr>>() {
		if *other_addr != socket_addr {
			game.packet_sender.send_packet_to_player(
				other_addr,
				lib::packets::clientbound::play::TeleportEntity::PACKET_ID,
				lib::packets::clientbound::play::TeleportEntity {
					entity_id: sending_player_entity_id,
					x: target_coordinates.x,
					y: target_coordinates.y,
					z: target_coordinates.z,
					velocity_x: 0.0,
					velocity_y: 0.0,
					velocity_z: 0.0,
					yaw: target_coordinates.yaw,
					pitch: target_coordinates.pitch,
					on_ground: true,
				},
			);
		}
	}

	return Ok(());
}
