use super::*;

pub fn process(location: BlockPosition, text: [String; 4], game: Arc<Game>) {
	let players = game.players.lock().unwrap();
	let mut world = game.world.lock().unwrap();
	let chunk = world.dimensions.get_mut("minecraft:overworld").unwrap().get_chunk_from_position_mut(location).unwrap();

	chunk.modified = true;

	let blockentity = chunk.block_entities.iter_mut().find(|x| x.get_position() == location).unwrap();

	if let BlockEntity::Sign(sign) = blockentity {
		sign.front_text.as_tag_compound_mut().push(NbtTag::Byte("has_glowing_text".to_string(), 0));
		sign.front_text.as_tag_compound_mut().push(NbtTag::String("color".to_string(), "black".to_string()));
		sign.front_text.as_tag_compound_mut().push(NbtTag::List(
			"messages".to_string(),
			vec![
				NbtListTag::String(text[0].clone()),
				NbtListTag::String(text[1].clone()),
				NbtListTag::String(text[2].clone()),
				NbtListTag::String(text[3].clone()),
			],
		));
	}

	let packet_to_send = lib::packets::clientbound::play::BlockEntityData {
		location,
		block_entity_type: *data::blockentity::get_block_entity_types().get(Into::<&str>::into(blockentity.get_id().as_str())).unwrap() as i32,
		nbt_data: NbtTag::Root(blockentity.clone().into()),
	};

	for player in players.iter() {
		game.send_packet(
			&player.peer_socket_address,
			lib::packets::clientbound::play::BlockEntityData::PACKET_ID,
			packet_to_send.clone().try_into().unwrap(),
		);
	}
}
