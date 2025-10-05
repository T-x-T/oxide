use std::{collections::HashMap, net::SocketAddr};

use lib::ConnectionState;

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

fn execute(command: String, stream: Option<&mut TcpStream>, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>, connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
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

	let target_coordinates: (f64, f64, f64, f32, f32) = if target_player.as_ref().is_some() {
		target_player.unwrap().get_position_and_rotation_float()
	} else {
		let mut arg_iter = arg_string.split(" ");
		let x = arg_iter.next().unwrap_or_default();
		let x: i32 = str::parse(x).unwrap_or_default(); //parsing needs proper bounds checking https://git.thetxt.io/thetxt/oxide/issues/19
		let y = arg_iter.next().unwrap_or_default();
		let y: i32 = str::parse(y).unwrap_or_default();
		let z = arg_iter.next().unwrap_or_default();
		let z: i32 = str::parse(z).unwrap_or_default();
		(x as f64, y as f64, z as f64, 0.0, 0.0)
	};

	let sending_player_index = game.players
    .iter()
    .enumerate()
    .find_map(|x| if x.1.peer_socket_address == stream.peer_addr().unwrap() { Some(x.0) } else { None })
    .unwrap();

	let sending_player = game.players.get_mut(sending_player_index).unwrap();
	game.last_created_entity_id += 1;
	sending_player.new_position(target_coordinates.0, target_coordinates.1, target_coordinates.2, &mut game.world, &mut game.last_created_entity_id)?;

	sending_player.current_teleport_id += 1;
	lib::utils::send_packet(stream, lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID, lib::packets::clientbound::play::SynchronizePlayerPosition {
    teleport_id: sending_player.current_teleport_id,
    x: target_coordinates.0,
    y: target_coordinates.1,
    z: target_coordinates.2,
    velocity_x: 0.0,
    velocity_y: 0.0,
    velocity_z: 0.0,
    yaw: target_coordinates.3,
    pitch: target_coordinates.4,
    flags: 0,
	}.try_into()?)?;

	let default_connection = Connection::default();
    for other_stream in connection_streams {
      if *other_stream.0 != stream.peer_addr().unwrap() && connections.get(other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
	      	lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::TeleportEntity::PACKET_ID, lib::packets::clientbound::play::TeleportEntity {
            entity_id: sending_player.entity_id,
            x: target_coordinates.0,
            y: target_coordinates.1,
            z: target_coordinates.2,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: target_coordinates.3,
            pitch: target_coordinates.4,
            on_ground: true,
        }.try_into().unwrap())?;
      }
    }

	return Ok(());
}
