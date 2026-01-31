use lib::packets::serverbound::play::Interact;

use super::*;

pub fn process(peer_addr: SocketAddr, parsed_packet: Interact, game: Arc<Game>, players_clone: &[Player]) {
	let players = game.players.lock().unwrap();


	let world = game.world.lock().unwrap();

	if players.iter().find(|x| x.entity_id == parsed_packet.entity_id).is_some() {
		target_is_player(parsed_packet, game.clone(), players, players_clone, peer_addr);
	} else {
		target_is_entity(parsed_packet, game.clone(), world, players, peer_addr);
	}
}

fn target_is_player(
	parsed_packet: Interact,
	game: Arc<Game>,
	mut players: std::sync::MutexGuard<Vec<Player>>,
	players_clone: &[Player],
	peer_addr: SocketAddr,
) {
	let Some(target_player) = players.iter_mut().find(|x| x.entity_id == parsed_packet.entity_id) else {
		return;
	};
	let Some(source_player) = players_clone.iter().find(|x| x.peer_socket_address == peer_addr) else {
		return;
	};

	let held_item = source_player.get_held_item(true);

	if parsed_packet.interact_type == 1 {
		//attack
		let damage = if held_item.is_some() { 10.0 } else { 1.0 };
		target_player.damage(damage, game, players_clone);
	}
}

fn target_is_entity(
	parsed_packet: Interact,
	game: Arc<Game>,
	mut world: std::sync::MutexGuard<World>,
	mut players: std::sync::MutexGuard<Vec<Player>>,
	peer_addr: SocketAddr,
) {
	let player = players.iter().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
	let held_item = player.get_held_item(true);

	let Some(entity) = world
		.dimensions
		.get_mut("minecraft:overworld")
		.unwrap()
		.entities
		.iter_mut()
		.find(|x| x.get_common_entity_data().entity_id == parsed_packet.entity_id)
	else {
		return;
	};
	let entity_id = entity.get_common_entity_data().entity_id;


	if parsed_packet.interact_type == 1 {
		//Attack
		let damage = if held_item.is_some() { 10.0 } else { 1.0 };

		if entity.is_mob() {
			let mob_data = entity.get_mob_data_mut();

			if mob_data.hurt_time > 0 {
				return;
			}

			mob_data.health -= damage;
			mob_data.hurt_time = 10;
			mob_data.hurt_by_timestamp = mob_data.alive_for_ticks;

			let entity_metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
				entity_id,
				metadata: vec![lib::packets::clientbound::play::EntityMetadata {
					index: 9,
					value: lib::packets::clientbound::play::EntityMetadataValue::Float(mob_data.health),
				}],
			};

			let hurt_animation_packet = lib::packets::clientbound::play::HurtAnimation {
				entity_id,
				yaw: 0.0,
			};

			let entity_data = entity.get_common_entity_data_mut();
			entity_data.velocity.y += 0.05;

			let horizontal_velocity = 0.05;
			match player.get_looking_cardinal_direction() {
				CardinalDirection::North => entity_data.velocity.z -= horizontal_velocity,
				CardinalDirection::East => entity_data.velocity.x += horizontal_velocity,
				CardinalDirection::South => entity_data.velocity.z += horizontal_velocity,
				CardinalDirection::West => entity_data.velocity.x -= horizontal_velocity,
			};

			players.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					entity_metadata_packet.clone().try_into().unwrap(),
				);
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::HurtAnimation::PACKET_ID,
					hurt_animation_packet.clone().try_into().unwrap(),
				);
			});
		}
	} else if parsed_packet.interact_type == 0 {
		//interact
		if data::entities::get_name_from_id(entity.get_type()) == "minecraft:creeper"
			&& held_item.is_some()
			&& held_item.unwrap().item_id == data::items::get_items().get("minecraft:flint_and_steel").unwrap().id
		{
			//right clicked a creeper with flint and steel -> explode!
			entity.get_mob_data_mut().health = 0.0;

			let explosion_packet = lib::packets::clientbound::play::Explosion {
				x: entity.get_common_entity_data().position.x,
				y: entity.get_common_entity_data().position.y,
				z: entity.get_common_entity_data().position.z,
				radius: 2.0,
				block_count: 64,
				player_delta_velocity: None,
				particle_id: 23,
				particle_data: (),
				sound: 616,
			};

			let creeper_position = BlockPosition::from(entity.get_common_entity_data().position);
			for x in (creeper_position.x - 2)..creeper_position.x + 2 {
				for y in (creeper_position.y - 2)..creeper_position.y + 2 {
					for z in (creeper_position.z - 2)..creeper_position.z + 2 {
						let res = world
							.dimensions
							.get_mut("minecraft:overworld")
							.unwrap()
							.overwrite_block(
								BlockPosition {
									x,
									y,
									z,
								},
								0,
							)
							.unwrap();
						if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
							let block_entity = world
								.dimensions
								.get("minecraft:overworld")
								.unwrap()
								.get_chunk_from_position(BlockPosition {
									x,
									y,
									z,
								})
								.unwrap()
								.block_entities
								.iter()
								.find(|a| {
									a.get_position()
										== BlockPosition {
											x,
											y,
											z,
										}
								})
								.unwrap();
							let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
							block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
						}

						for player in players.iter() {
							game.send_packet(
								&player.peer_socket_address,
								lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
								lib::packets::clientbound::play::BlockUpdate {
									location: BlockPosition {
										x,
										y,
										z,
									},
									block_id: 0,
								}
								.try_into()
								.unwrap(),
							);
						}
					}
				}
			}

			players.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					lib::packets::clientbound::play::Explosion::PACKET_ID,
					explosion_packet.clone().try_into().unwrap(),
				);
			});
		}
	} else {
		//interact at
	}
}
