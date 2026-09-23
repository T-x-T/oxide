use super::*;
use lib::blocks::Type;
use rand::Rng;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut rng = rand::rng();

	if let Some(dimension) = world.dimensions.get_mut("minecraft:overworld") {
		let dimension_creature_mob_count = dimension.entities.iter().filter(|x| x.is_mob() && x.get_mob_type() == MobType::Creature).count();
		if dimension_creature_mob_count < lib::MOB_SPAWN_LIMIT_PER_DIMENSION_CREATURE as usize {
			let mut entities_to_summon: Vec<Entity> = Vec::new();

			for chunk in dimension.chunks.values() {
				let chunk_creature_mob_count =
					dimension.get_entities_in_chunk(chunk.x, chunk.z).iter().filter(|x| x.is_mob() && x.get_mob_type() == MobType::Creature).count();

				if chunk_creature_mob_count >= lib::MOB_SPAWN_LIMIT_PER_CHUNK_CREATURE as usize {
					continue;
				}

				for _ in 0..lib::MOB_SPAWN_ATTEMPTS_PER_CHUNK_PER_TICK {
					let chunk_coords_to_check = BlockPosition {
						x: rng.random_range(0..16),
						y: rng.random_range((dimension.lowest_block_y + 1)..(if dimension.lowest_block_y == -64 { 319 } else { 255 })),
						z: rng.random_range(0..16),
					};

					let block_to_check_for_grass = BlockPosition { y: chunk_coords_to_check.y - 1, ..chunk_coords_to_check };

					let block = dimension.get_block(block_to_check_for_grass).unwrap_or_default();
					let block_type = data::blocks::get_type_from_block_state_id(block);
					if block_type != Type::Grass {
						continue;
					}

					let distance_to_closest_player = players_clone
						.iter()
						.filter(|x| x.get_dimension() == dimension.name)
						.map(|x| x.get_position().distance_to(chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into()))
						.min_by(|a, b| a.total_cmp(b))
						.unwrap_or_default();

					if distance_to_closest_player < 24.0 || distance_to_closest_player > 128.0 {
						continue;
					}

					let mob_type_to_spawn = lib::CREATURE_OVERWORLD_SPAWNABLE_MOBS[rng.random_range(0..lib::CREATURE_OVERWORLD_SPAWNABLE_MOBS.len())];

					let Some(entity) = entity::new(
						mob_type_to_spawn,
						CommonEntity {
							position: chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
							velocity: EntityPosition::default(),
							uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
							entity_id: game.entity_id_manager.get_new(),
							..Default::default()
						},
						NbtListTag::TagCompound(Vec::new()),
					) else {
						println!("tried spawning invalid entity {mob_type_to_spawn}");
						continue;
					};

					if entity.collides_with_blocks_at(
						dimension,
						chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
						&game.block_state_data,
					) {
						continue;
					}

					if !entity.is_on_ground_at(
						dimension,
						chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
						&game.block_state_data,
					) {
						continue;
					}

					entities_to_summon.push(entity);
				}
			}

			for entity in entities_to_summon {
				let packet = entity.to_spawn_entity_packet();

				dimension.add_entity(entity);

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
					packet,
				);
			}
		}
	}

	for dimension in world.dimensions.values_mut() {
		let dimension_hostile_mob_count = dimension.entities.iter().filter(|x| x.is_mob() && x.get_mob_type() == MobType::Monster).count();
		if dimension_hostile_mob_count >= lib::MOB_SPAWN_LIMIT_PER_DIMENSION_HOSTILE as usize {
			continue;
		}

		let mut entities_to_summon: Vec<Entity> = Vec::new();

		for chunk in dimension.chunks.values() {
			let chunk_hostile_mob_count =
				dimension.get_entities_in_chunk(chunk.x, chunk.z).iter().filter(|x| x.is_mob() && x.get_mob_type() == MobType::Monster).count();

			if chunk_hostile_mob_count >= lib::MOB_SPAWN_LIMIT_PER_CHUNK_HOSTILE as usize {
				continue;
			}

			for _ in 0..lib::MOB_SPAWN_ATTEMPTS_PER_CHUNK_PER_TICK {
				let chunk_coords_to_check = BlockPosition {
					x: rng.random_range(0..16),
					y: rng.random_range(dimension.lowest_block_y..(if dimension.lowest_block_y == -64 { 319 } else { 255 })),
					z: rng.random_range(0..16),
				};

				if dimension.name.as_str() != "minecraft:the_end" {
					let light_level = chunk.get_light(chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z), dimension.lowest_block_y);
					if light_level > 7 {
						continue;
					}
				}

				let distance_to_closest_player = players_clone
					.iter()
					.filter(|x| x.get_dimension() == dimension.name)
					.map(|x| x.get_position().distance_to(chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into()))
					.min_by(|a, b| a.total_cmp(b))
					.unwrap_or_default();

				if distance_to_closest_player < 24.0 || distance_to_closest_player > 128.0 {
					continue;
				}

				let mob_type_to_spawn = match dimension.name.as_str() {
					"minecraft:overworld" => lib::HOSTILE_OVERWORLD_SPAWNABLE_MOBS[rng.random_range(0..lib::HOSTILE_OVERWORLD_SPAWNABLE_MOBS.len())],
					"minecraft:the_nether" => lib::HOSTILE_NETHER_SPAWNABLE_MOBS[rng.random_range(0..lib::HOSTILE_NETHER_SPAWNABLE_MOBS.len())],
					"minecraft:the_end" => lib::HOSTILE_END_SPAWNABLE_MOBS[rng.random_range(0..lib::HOSTILE_END_SPAWNABLE_MOBS.len())],
					_ => continue,
				};

				let Some(entity) = entity::new(
					mob_type_to_spawn,
					CommonEntity {
						position: chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
						velocity: EntityPosition::default(),
						uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
						entity_id: game.entity_id_manager.get_new(),
						..Default::default()
					},
					NbtListTag::TagCompound(Vec::new()),
				) else {
					println!("tried spawning invalid entity {mob_type_to_spawn}");
					continue;
				};

				if entity.collides_with_blocks_at(
					dimension,
					chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
					&game.block_state_data,
				) {
					continue;
				}

				if !entity.is_on_ground_at(
					dimension,
					chunk_coords_to_check.convert_to_position_global(chunk.x, chunk.z).into(),
					&game.block_state_data,
				) {
					continue;
				}

				entities_to_summon.push(entity);
			}
		}

		for entity in entities_to_summon {
			let packet = entity.to_spawn_entity_packet();

			dimension.add_entity(entity);

			game.packet_sender.send_packet_to_everyone_in_dimension(
				players_clone,
				&dimension.name,
				lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
				packet,
			);
		}
	}
}
