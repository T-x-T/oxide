use lib::entity::CommonEntity;

use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "summon".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "entity type".to_string(),
			properties: ParserProperty::ResourceKey("minecraft:entity_type".to_string()),
			next_arguments: Vec::new(),
			optional: false,
		}],
	});
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(socket_addr) = socket_addr else {
		println!("This command currently only works in game");
		return Ok(());
	};

	let players = game.players.lock().unwrap();

	let position = players.iter().find(|x| x.peer_socket_address == socket_addr).unwrap().get_position();

	let new_entity = entity::new(
		command.replace("summon ", "").as_str(),
		CommonEntity {
			position,
			velocity: EntityPosition::default(),
			uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
			entity_id: game.entity_id_manager.get_new(),
			..Default::default()
		},
		NbtListTag::TagCompound(Vec::new()),
	);

	let Some(new_entity) = new_entity else {
		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), format!("cant summon unknown entity {}", command.replace("summon ", "").as_str())),
				]),
				overlay: false,
			},
		);
		println!("cant summon unknown entity {}", command.replace("summon ", "").as_str());
		return Ok(());
	};

	let packet = new_entity.to_spawn_entity_packet();

	let player = players.iter().find(|x| x.peer_socket_address == socket_addr).unwrap();
	game.world.lock().unwrap().dimensions.get_mut(player.get_dimension()).unwrap().add_entity(new_entity);

	game.packet_sender.send_packet_to_everyone_in_dimension(
		&players,
		player.get_dimension(),
		lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
		packet,
	);

	return Ok(());
}
