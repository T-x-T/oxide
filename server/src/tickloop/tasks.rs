use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();

	for task in game.task_queue.iter() {
		match task.clone() {
			Task::PlayerUseNetherPortal(uuid, new_dimension_name) => {
				let player = players.iter_mut().find(|x| x.uuid == uuid).unwrap();
				let dimension = world.dimensions.get_mut(&new_dimension_name).unwrap();

				let current_position = player.get_position();
				let new_position = if new_dimension_name == "minecraft:overworld" {
					BlockPosition::from(EntityPosition {
						x: current_position.x * 8.0,
						y: current_position.y,
						z: current_position.z * 8.0,
						..current_position
					})
				} else if new_dimension_name == "minecraft:the_nether" {
					BlockPosition::from(EntityPosition {
						x: current_position.x / 8.0,
						y: current_position.y,
						z: current_position.z / 8.0,
						..current_position
					})
				} else {
					current_position.into()
				};

				let x_range: Vec<i32> = (0..32).zip((-32..0).rev()).flat_map(|(a, b)| [a, b]).collect();
				let z_range: Vec<i32> = (0..32).zip((-32..0).rev()).flat_map(|(a, b)| [a, b]).collect();

				let nether_portal_block_ids: Vec<u16> =
					data::blocks::get_block_from_name("minecraft:nether_portal", &game.block_state_data).states.iter().map(|x| x.id).collect();

				let mut portal_location: Option<BlockPosition> = None;
				for y in 0..256 {
					for x in &x_range {
						for z in &z_range {
							let position_to_check = BlockPosition {
								x: *x + new_position.x,
								y,
								z: *z + new_position.z,
							};

							let block = dimension.get_block(position_to_check);

							if block.is_ok_and(|x| nether_portal_block_ids.contains(&x)) {
								portal_location = Some(position_to_check);
							}
						}
					}
				}

				if let Some(portal_location) = portal_location {
					player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, portal_location);
				} else {
					let obsidian_block_id =
						data::blocks::get_block_from_name("minecraft:obsidian", &game.block_state_data).states.first().unwrap().id;
					let nether_portal_block_id = *nether_portal_block_ids.first().unwrap();
					let blocks_to_create = [
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y + 1,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y + 1,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y + 2,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y + 2,
								z: new_position.z,
							},
							nether_portal_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y - 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x - 1,
								y: new_position.y - 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y - 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 2,
								y: new_position.y - 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y - 1,
								z: new_position.z + 1,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y - 1,
								z: new_position.z - 1,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y - 1,
								z: new_position.z - 1,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y - 1,
								z: new_position.z + 1,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x - 1,
								y: new_position.y,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 2,
								y: new_position.y,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x - 1,
								y: new_position.y + 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 2,
								y: new_position.y + 1,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x - 1,
								y: new_position.y + 2,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 2,
								y: new_position.y + 2,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x - 1,
								y: new_position.y + 3,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 2,
								y: new_position.y + 3,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x,
								y: new_position.y + 3,
								z: new_position.z,
							},
							obsidian_block_id,
						),
						(
							BlockPosition {
								x: new_position.x + 1,
								y: new_position.y + 3,
								z: new_position.z,
							},
							obsidian_block_id,
						),
					];

					for block_to_create in blocks_to_create {
						dimension.overwrite_block(block_to_create.0, block_to_create.1).unwrap();
						game.packet_sender.send_packet_to_everyone_in_dimension(
							players_clone,
							&new_dimension_name,
							lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
							lib::packets::clientbound::play::BlockUpdate {
								location: block_to_create.0,
								block_id: block_to_create.1 as i32,
							},
						);
					}

					player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, new_position);
				}
			}
			Task::PlayerUseEndPortal(uuid, new_dimension_name) => {
				let player = players.iter_mut().find(|x| x.uuid == uuid).unwrap();
				let default_spawn_location = world.default_spawn_location;
				let dimension = world.dimensions.get_mut(&new_dimension_name).unwrap();

				if new_dimension_name == "minecraft:the_end" {
					let new_position = BlockPosition {
						x: 100,
						y: 49,
						z: 0,
					};

					let block_underneth = dimension
						.get_block(BlockPosition {
							x: 100,
							y: 48,
							z: 0,
						})
						.unwrap_or_default();

					let obsidian_block_id =
						data::blocks::get_block_from_name("minecraft:obsidian", &game.block_state_data).states.first().unwrap().id;

					if block_underneth != obsidian_block_id {
						for x in 98..=102 {
							for z in -2..=2 {
								let position = BlockPosition {
									x,
									y: 48,
									z,
								};
								dimension.overwrite_block(position, obsidian_block_id).unwrap();
								game.packet_sender.send_packet_to_everyone_in_dimension(
									players_clone,
									&new_dimension_name,
									lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
									lib::packets::clientbound::play::BlockUpdate {
										location: position,
										block_id: obsidian_block_id as i32,
									},
								);
							}
						}
						for x in 98..=102 {
							for y in 49..=51 {
								for z in -2..=2 {
									let position = BlockPosition {
										x,
										y,
										z,
									};
									dimension.overwrite_block(position, 0).unwrap();
									game.packet_sender.send_packet_to_everyone_in_dimension(
										players_clone,
										&new_dimension_name,
										lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
										lib::packets::clientbound::play::BlockUpdate {
											location: position,
											block_id: 0,
										},
									);
								}
							}
						}
					}

					player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, new_position);
				} else {
					player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, default_spawn_location);
				};
			}
		}
	}

	game.task_queue.clear();
}
