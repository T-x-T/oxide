use lib::packets::serverbound::play::Interact;

use super::*;

pub fn process(peer_addr: SocketAddr, parsed_packet: Interact, game: Arc<Game>, players_clone: &[Player]) {
	let players = game.players.lock().unwrap();
	let world = game.world.lock().unwrap();

	if players.iter().find(|x| x.entity_id == parsed_packet.entity_id).is_some() {
		target_is_player(parsed_packet, game.clone(), players, players_clone, peer_addr);
	} else {
		target_is_entity(parsed_packet, game.clone(), world, players, peer_addr, players_clone);
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

// --------------
//  WARNING! Does std::mem::take on world.dimensions and dimension.entities!
//  This needs to be put back before function returns, otherwise chaos ensues
//  (Yes this is probably stupid, but its just a quick hack I swear)
// --------------

fn target_is_entity(
	parsed_packet: Interact,
	game: Arc<Game>,
	mut world: std::sync::MutexGuard<World>,
	mut players: std::sync::MutexGuard<Vec<Player>>,
	peer_addr: SocketAddr,
	players_clone: &[Player],
) {
	let mut dimensions = std::mem::take(&mut world.dimensions);
	let player = players_clone.iter().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
	let dimension = dimensions.get_mut(player.get_dimension()).unwrap();

	let mut entities = std::mem::take(&mut dimension.entities);

	let held_item = player.get_held_item(true);

	let Some(entity) = entities.iter_mut().find(|x| x.get_common_entity_data().entity_id == parsed_packet.entity_id) else {
		dimension.entities = entities;
		world.dimensions = dimensions;
		return;
	};
	let entity_id = entity.get_common_entity_data().entity_id;


	if parsed_packet.interact_type == 1 {
		//Attack
		let damage = if held_item.is_some() { 10.0 } else { 1.0 };

		if entity.is_mob() {
			let mob_data = entity.get_mob_data_mut();

			if mob_data.hurt_time > 0 {
				dimension.entities = entities;
				world.dimensions = dimensions;
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
		if let Some(held_item) = player.get_held_item(true)
			&& held_item.count > 0
		{
			let success = entity.feed(held_item, game.clone(), players_clone);
			if success {
				let mut held_item = held_item.clone();
				held_item.count -= 1;
				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
				if held_item.count == 0 {
					player.set_selected_inventory_slot(None, players_clone, game.clone());
				} else {
					player.set_selected_inventory_slot(Some(held_item), players_clone, game.clone());
				}
			}
		}

		let res = entity.interact(&held_item.cloned().unwrap_or_default(), game.clone(), dimension, players_clone, &mut players, player.uuid);

		match res {
			EntityInteractResult::DoNothing => (),
			EntityInteractResult::AddEntity(new_entity) => {
				dimension.entities = entities;
				world.dimensions = dimensions;

				let spawn_packet = new_entity.to_spawn_entity_packet();

				for player in players_clone {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						spawn_packet.clone().try_into().unwrap(),
					);
				}

				new_entity.resend_metadata_to_players(players_clone, game);

				world.dimensions.get_mut(player.get_dimension()).unwrap().add_entity(*new_entity);

				return;
			}
		};
	} else {
		//interact at
	}

	dimension.entities = entities;
	world.dimensions = dimensions;
}
