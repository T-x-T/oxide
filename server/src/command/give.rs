use lib::entity::{CommonEntity, ItemEntity};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "give".to_string(),
		execute,
		arguments: vec![
      CommandArgument {
        name: "entity type".to_string(),
        properties: ParserProperty::ResourceKey("minecraft:item".to_string()),
        next_arguments: Vec::new(),
        optional: false
      }
		],
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, game: &mut Game) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("This command currently only works in game");
		return Ok(());
	};

	let player = game.players.iter()
	  .find(|x| x.peer_socket_address == stream.peer_addr().unwrap())
    .unwrap();

	let position = player.get_position();

	let new_entity = ItemEntity {
    common: CommonEntity {
      position,
  		velocity: EntityPosition::default(),
      uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
      entity_id: game.last_created_entity_id.load(std::sync::atomic::Ordering::SeqCst),
      ..Default::default()
    },
    age: 0,
    health: 5,
    item: Item { id: command.replace("give ", ""), count: 1, components: Vec::new() },
    owner: player.uuid,
    pickup_delay: 0,
    thrower: player.uuid,
	};

	game.last_created_entity_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

	let spawn_packet = lib::packets::clientbound::play::SpawnEntity {
    entity_id: new_entity.get_common_entity_data().entity_id,
    entity_uuid: new_entity.get_common_entity_data().uuid,
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

	let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
    entity_id: new_entity.get_common_entity_data().entity_id,
    metadata: new_entity.get_metadata(),
	};

	game.world.dimensions
	  .get_mut("minecraft:overworld").unwrap()
    .add_entity(Box::new(new_entity));

	game.players.iter().for_each(|x| {
      lib::utils::send_packet(&x.connection_stream, lib::packets::clientbound::play::SpawnEntity::PACKET_ID, spawn_packet.clone().try_into().unwrap()).unwrap();
      lib::utils::send_packet(&x.connection_stream, lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID, metadata_packet.clone().try_into().unwrap()).unwrap();
    });

	return Ok(());
}
