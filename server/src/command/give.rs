use std::sync::Arc;

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

	let item_name = command.replace("give ", "");
	let Some(item_id) = data::items::get_item_id_by_name(&item_name) else {
		game.packet_sender.send_packet_to_player(
			&stream.peer_addr()?,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), format!("Unkown item: {item_name}",)),
				]),
				overlay: false,
			},
		);

		return Ok(());
	};

	let slot = Slot {
		id: item_id,
		count: 1,
		components_to_add: Vec::new(),
		components_to_remove: Vec::new(),
	};

	game.world.lock().unwrap().dimensions.get_mut(player.get_dimension()).unwrap().summon_item(
		position,
		slot,
		Some(player.uuid),
		&players,
		&game.packet_sender,
		&game.entity_id_manager,
	);

	return Ok(());
}
