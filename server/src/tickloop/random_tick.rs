use rand::Rng;

use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();
	let mut blocks_to_update: Vec<(BlockPosition, u16)> = Vec::new();

	for dimension in world.dimensions.values() {
		for chunk in dimension.chunks.values() {
			for chunk_section_index in 0..chunk.sections.len() {
				let mut rng = rand::rng();
				for _ in 0..=lib::RANDOM_TICK_SPEED {
					if chunk.sections[chunk_section_index].blocks.is_empty() {
						break;
					}
					let block_index_to_tick = rng.random_range(0..(16 * 16 * 16));
					let block_id_to_tick = chunk.sections[chunk_section_index].blocks[block_index_to_tick as usize];

					let x: i32 = (block_index_to_tick & 0b0000_0000_1111) + (chunk.x * 16);
					let y: i16 = (((block_index_to_tick & 0b1111_0000_0000) >> 8) as i16) + ((chunk_section_index as i16 - 4) * 16);
					let z: i32 = ((block_index_to_tick & 0b0000_1111_0000) >> 4) + (chunk.z * 16);

					let position = BlockPosition {
						x,
						y,
						z,
					};

					let new_block_id = lib::block::tick(block_id_to_tick, dimension, position, &game.block_state_data);

					if block_id_to_tick != new_block_id {
						blocks_to_update.push((position, new_block_id));
					}
				}
			}
		}
	}

	for (position, new_block_id) in blocks_to_update {
		let res = world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(position, new_block_id).unwrap();

		#[allow(clippy::collapsible_if)]
		if let Some(res) = res {
			if matches!(res, BlockOverwriteOutcome::DestroyBlockentity) {
				let block_entity = world
					.dimensions
					.get("minecraft:overworld")
					.unwrap()
					.get_chunk_from_position(position)
					.unwrap()
					.block_entities
					.iter()
					.find(|x| x.get_position() == position)
					.unwrap();
				let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
				block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
			}
		};

		for player in players_clone {
			game.send_packet(
				&player.peer_socket_address,
				lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
				lib::packets::clientbound::play::BlockUpdate {
					location: position,
					block_id: new_block_id as i32,
				}
				.try_into()
				.unwrap(),
			);
		}
	}
}
