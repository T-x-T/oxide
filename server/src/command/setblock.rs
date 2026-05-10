use std::sync::Arc;

use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "setblock".to_string(),
		execute,
		arguments: vec![CommandArgument {
			name: "block".to_string(),
			properties: ParserProperty::ResourceKey("minecraft:block".to_string()),
			next_arguments: vec![CommandArgument {
				name: "coordinates".to_string(),
				properties: ParserProperty::BlockPos,
				next_arguments: Vec::new(),
				optional: false,
			}],
			optional: false,
		}],
	});
}

fn execute(command: String, stream: Option<&mut TcpStream>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let Some(stream) = stream else {
		println!("This command currently only works in game");
		return Ok(());
	};

	let mut world = game.world.lock().unwrap();
	let players = game.players.lock().unwrap();
	let player = players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap();

	let command_parts: Vec<&str> = command.split(" ").collect();

	if command_parts.len() < 5 {
		game.packet_sender.send_packet_to_player(
			&stream.peer_addr()?,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "please use all the arguments".to_string()),
				]),
				overlay: false,
			},
		);
		return Ok(());
	}

	let block_name = command_parts[1];
	let (Ok(x), Ok(y), Ok(z)) = (command_parts[2].parse::<i32>(), command_parts[3].parse::<i16>(), command_parts[4].parse::<i32>()) else {
		game.packet_sender.send_packet_to_player(
			&stream.peer_addr()?,
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
	};

	if x > 30_000_000 || z > 30_000_000 || x < -30_000_000 || z < -30_000_000 {
		game.packet_sender.send_packet_to_player(
			&stream.peer_addr()?,
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

	let block_position = BlockPosition {
		x,
		y,
		z,
	};
	let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();

	let blocks_to_update = lib::block::get_block_state_id(
		0,
		CardinalDirection::North,
		0.0,
		dimension,
		block_position,
		block_name,
		0.0,
		0.0,
		0.0,
		&game.block_state_data,
	);

	for (block_state_id, position) in blocks_to_update {
		if dimension.overwrite_block(position, block_state_id).is_err() {
			game.packet_sender.send_packet_to_player(
				&stream.peer_addr()?,
				lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
				lib::packets::clientbound::play::SystemChatMessage {
					content: NbtTag::Root(vec![
						NbtTag::String("type".to_string(), "text".to_string()),
						NbtTag::String("text".to_string(), "something went wrong".to_string()),
					]),
					overlay: false,
				},
			);
		};

		game.packet_sender.send_packet_to_everyone_in_dimension(
			&players,
			player.get_dimension(),
			lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
			lib::packets::clientbound::play::BlockUpdate {
				location: position,
				block_id: block_state_id as i32,
			},
		);
	}


	return Ok(());
}
