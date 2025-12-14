use std::sync::Arc;

use lib::entity::{CommonEntity, ItemEntity};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "give".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "entity type".to_string(),
			properties: ParserProperty::ResourceKey("minecraft:item".to_string()),
			next_arguments: Vec::new(),
			optional: false,
		}],
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("This command currently only works in game");
		return Ok(());
	};

	let players = game.players.lock().unwrap();

	let player = players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap();

	let position = player.get_position();

	let new_entity = ItemEntity {
		common: CommonEntity {
			position,
			velocity: EntityPosition::default(),
			uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
			entity_id: game.entity_id_manager.get_new(),
			..Default::default()
		},
		age: 0,
		health: 5,
		item: Item {
			id: command.replace("give ", ""),
			count: 1,
			components: Vec::new(),
		},
		owner: player.uuid,
		pickup_delay: 0,
		thrower: player.uuid,
	};

	let spawn_packet = new_entity.to_spawn_entity_packet();

	let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
		entity_id: new_entity.get_common_entity_data().entity_id,
		metadata: new_entity.get_metadata(),
	};

	game.world.lock().unwrap().dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Box::new(new_entity));

	players.iter().for_each(|x| {
		game.send_packet(
			&x.peer_socket_address,
			lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
			spawn_packet.clone().try_into().unwrap(),
		);
		game.send_packet(
			&x.peer_socket_address,
			lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
			metadata_packet.clone().try_into().unwrap(),
		);
	});

	return Ok(());
}
