use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
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

fn execute(command: String, stream: Option<&mut TcpStream>, game: &mut Game) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("this command doesnt work from the console");
		return Ok(());
	};

	let arg_string = command.replace("tp ", "");
	if arg_string.is_empty() {
		lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::Root(vec![
				NbtTag::String("type".to_string(), "text".to_string()),
				NbtTag::String("text".to_string(), "missing a destination to teleport to".to_string()),
			]),
		  overlay: false,
	 	}.try_into()?)?;
	}

	let target_player = game.players
		.iter()
		.find(|x| x.display_name == arg_string.split(" ").next().unwrap_or_default());

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
  		lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
 			  content: NbtTag::Root(vec![
  				NbtTag::String("type".to_string(), "text".to_string()),
  				NbtTag::String("text".to_string(), "coordinates must be between -30 million and +30 million".to_string()),
  			]),
  		  overlay: false,
  	 	}.try_into()?)?;

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

	let sending_player = game.players
    .iter_mut()
    .find(|x| x.peer_socket_address == stream.peer_addr().unwrap())
    .unwrap();

	let sending_player_entity_id = sending_player.entity_id;

	sending_player.new_position(target_coordinates.x, target_coordinates.y, target_coordinates.z, &mut game.world, &game.last_created_entity_id)?;

	sending_player.current_teleport_id += 1;
	lib::utils::send_packet(stream, lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID, lib::packets::clientbound::play::SynchronizePlayerPosition {
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
	}.try_into()?)?;

  for other_stream in game.players.iter().map(|x| &x.connection_stream).collect::<Vec<&TcpStream>>() {
    if other_stream.peer_addr().unwrap() != stream.peer_addr().unwrap() {
      	lib::utils::send_packet(other_stream, lib::packets::clientbound::play::TeleportEntity::PACKET_ID, lib::packets::clientbound::play::TeleportEntity {
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
      }.try_into().unwrap())?;
    }
  }

	return Ok(());
}
