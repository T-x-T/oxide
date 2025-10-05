use lib::{entity::creeper::Creeper, ConnectionState};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "summon".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>, connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("This command currently only works in game");
		return Ok(());
	};

	let position = game.players.iter()
	  .find(|x| x.peer_socket_address == stream.peer_addr().unwrap())
    .unwrap()
    .get_position();

	game.last_created_entity_id += 1;

	let new_entity = Creeper {
	  x: position.x as f64,
		y: position.y as f64,
		z: position.z as f64,
		pitch: 0.0,
		yaw: 0.0,
    uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
    entity_id: game.last_created_entity_id,
	};

	game.world.dimensions
	  .get_mut("minecraft:overworld").unwrap()
    .entities
    .push(Box::new(new_entity.clone()));

	let packet = lib::packets::clientbound::play::SpawnEntity {
    entity_id: new_entity.entity_id,
    entity_uuid: new_entity.uuid,
    entity_type: new_entity.get_type(),
    x: new_entity.x,
    y: new_entity.y,
    z: new_entity.z,
    pitch: new_entity.get_pitch_u8(),
    yaw: new_entity.get_yaw_u8(),
    head_yaw: 0,
    data: 0,
    velocity_x: 0,
    velocity_y: 0,
    velocity_z: 0,
	};

	connection_streams.iter()
    .filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
    .for_each(|x| lib::utils::send_packet(x.1, lib::packets::clientbound::play::SpawnEntity::PACKET_ID, packet.clone().try_into().unwrap()).unwrap());

	return Ok(());
}
