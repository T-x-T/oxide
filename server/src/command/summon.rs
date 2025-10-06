use lib::ConnectionState;

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "summon".to_string(),
		execute,
		arguments: vec![
      CommandArgument {
        name: "entity type".to_string(),
        properties: ParserProperty::ResourceKey("minecraft:entity_type".to_string()),
        next_arguments: Vec::new(),
        optional: false
      }
		],
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

	let new_entity = entity::new(
	  command.replace("summon ", "").as_str(),
		position,
		EntityPosition::default(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
		game.last_created_entity_id,
		NbtListTag::TagCompound(Vec::new()),
	);

	let Some(new_entity) = new_entity else {
	  lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
   	  content: NbtTag::Root(vec![
   			NbtTag::String("type".to_string(), "text".to_string()),
   			NbtTag::String("text".to_string(), format!("cant summon unknown entity {}", command.replace("summon ", "").as_str())),
      ]),
   	  overlay: false,
 	  }.try_into()?)?;
		println!("cant summon unknown entity {}", command.replace("summon ", "").as_str());
		return Ok(());
	};

	let packet = lib::packets::clientbound::play::SpawnEntity {
    entity_id: new_entity.get_id(),
    entity_uuid: new_entity.get_uuid(),
    entity_type: new_entity.get_type(),
    x: position.x,
    y: position.y,
    z: position.z,
    pitch: new_entity.get_pitch_u8(),
    yaw: new_entity.get_yaw_u8(),
    head_yaw: 0,
    data: 0,
    velocity_x: 0,
    velocity_y: 0,
    velocity_z: 0,
	};

	game.world.dimensions
	  .get_mut("minecraft:overworld").unwrap()
    .add_entity(new_entity);

	connection_streams.iter()
    .filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
    .for_each(|x| lib::utils::send_packet(x.1, lib::packets::clientbound::play::SpawnEntity::PACKET_ID, packet.clone().try_into().unwrap()).unwrap());

	return Ok(());
}
