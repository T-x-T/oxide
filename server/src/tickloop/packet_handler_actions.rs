use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	//prevents handling of two movements packets from one player during tick
	let mut moved_players: Vec<SocketAddr> = Vec::new();

	let mut packet_handler_actions: Vec<PacketHandlerAction> = Vec::new();
	packet_handler_actions.append(&mut game.packet_handler_actions.lock().unwrap());

	for packet_handler_action in packet_handler_actions.into_iter().rev() {
		match packet_handler_action {
			PacketHandlerAction::DisconnectPlayer(peer_addr) => {
				println!("handler told us to disconnect");
				crate::disconnect_player(&peer_addr, game.clone());
			}
			PacketHandlerAction::MovePlayer(peer_addr, position, rotation) => {
				let mut world = game.world.lock().unwrap();

				if moved_players.contains(&peer_addr) {
					continue;
				}

				moved_players.push(peer_addr);

				let mut players = game.players.lock().unwrap();
				let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == peer_addr) else {
					continue;
				};

				let player_entity_id = player.entity_id;
				let old_position = player.get_position();

				let position_updated = position.is_some();
				let rotation_updated = rotation.is_some();

				let packets: Vec<(u8, Vec<u8>)> = if position_updated && rotation_updated {
					let new_position = player
						.new_position_and_rotation(
							EntityPosition {
								x: position.unwrap().0,
								y: position.unwrap().1,
								z: position.unwrap().2,
								yaw: rotation.unwrap().0,
								pitch: rotation.unwrap().1,
							},
							&mut world,
							&game.entity_id_manager,
							&game.block_state_data,
							game.clone(),
						)
						.unwrap();
					vec![
						(
							lib::packets::clientbound::play::UpdateEntityPositionAndRotation::PACKET_ID,
							lib::packets::clientbound::play::UpdateEntityPositionAndRotation {
								entity_id: player_entity_id,
								delta_x: ((new_position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
								delta_y: ((new_position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
								delta_z: ((new_position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
								yaw: player.get_yaw_u8(),
								pitch: player.get_pitch_u8(),
								on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
							}
							.try_into()
							.unwrap(),
						),
						(
							lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
							lib::packets::clientbound::play::SetHeadRotation {
								entity_id: player_entity_id,
								head_yaw: player.get_yaw_u8(),
							}
							.try_into()
							.unwrap(),
						),
					]
				} else if position_updated {
					let new_position = player
						.new_position(
							position.unwrap().0,
							position.unwrap().1,
							position.unwrap().2,
							&mut world,
							&game.entity_id_manager,
							&game.block_state_data,
							game.clone(),
						)
						.unwrap();
					vec![(
						lib::packets::clientbound::play::UpdateEntityPosition::PACKET_ID,
						lib::packets::clientbound::play::UpdateEntityPosition {
							entity_id: player_entity_id,
							delta_x: ((new_position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
							delta_y: ((new_position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
							delta_z: ((new_position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
							on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
						}
						.try_into()
						.unwrap(),
					)]
				} else if rotation_updated {
					player.new_rotation(rotation.unwrap().0, rotation.unwrap().1);
					vec![
						(
							lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID,
							lib::packets::clientbound::play::UpdateEntityRotation {
								entity_id: player_entity_id,
								yaw: player.get_yaw_u8(),
								pitch: player.get_pitch_u8(),
								on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
							}
							.try_into()
							.unwrap(),
						),
						(
							lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
							lib::packets::clientbound::play::SetHeadRotation {
								entity_id: player_entity_id,
								head_yaw: player.get_yaw_u8(),
							}
							.try_into()
							.unwrap(),
						),
					]
				} else {
					Vec::new()
				};

				for other_player in players_clone.iter() {
					if other_player.peer_socket_address != peer_addr {
						for packet in &packets {
							game.send_packet(&other_player.peer_socket_address, packet.0, packet.1.clone());
						}
					}
				}
			}
			PacketHandlerAction::ConfirmTeleportation(peer_addr, teleport_id) => {
				let mut players = game.players.lock().unwrap();
				let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == peer_addr) else {
					continue;
				};

				if player.current_teleport_id == teleport_id {
					player.waiting_for_confirm_teleportation = false;
				}
			}
			PacketHandlerAction::SetCreativeModeSlot(peer_addr, slot, item) => {
				let mut players = game.players.lock().unwrap();
				let Some(player) = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr) else {
					continue;
				};

				player.set_inventory_slot(slot, item, players_clone, game.clone());
			}
			PacketHandlerAction::SetSelectedSlot(peer_addr, slot) => {
				let mut players = game.players.lock().unwrap();
				let Some(player) = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr) else {
					continue;
				};

				player.set_selected_slot(slot, players_clone, game.clone());
			}
			PacketHandlerAction::PickItemFromBlock(peer_addr, location, _include_data) => {
				let picked_block = game.world.lock().unwrap().dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();
				let picked_block_name = game.block_state_data.iter().find(|x| x.1.states.iter().any(|x| x.id == picked_block)).unwrap().0.clone();
				let item_id = data::items::get_items()
					.get(picked_block_name.as_str())
					.unwrap_or(&data::items::Item {
						max_stack_size: 0,
						rarity: data::items::ItemRarity::Common,
						id: 0,
						repair_cost: 0,
					})
					.id;

				let mut players = game.players.lock().unwrap();
				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

				let new_slot_data = Slot {
					item_count: 1,
					item_id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				};

				player.set_selected_inventory_slot(Some(new_slot_data), players_clone, game.clone());
			}
			PacketHandlerAction::SwingArm(peer_addr, hand) => {
				let entity_id = players_clone.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().entity_id;

				for other_player in players_clone.iter() {
					if other_player.peer_socket_address != peer_addr {
						game.send_packet(
							&other_player.peer_socket_address,
							lib::packets::clientbound::play::EntityAnimation::PACKET_ID,
							lib::packets::clientbound::play::EntityAnimation {
								entity_id,
								animation: if hand == 0 { 0 } else { 3 },
							}
							.try_into()
							.unwrap(),
						);
					}
				}
			}
			PacketHandlerAction::BreakBlock(peer_addr, _status, location, _face, sequence_id) => {
				let mut players = game.players.lock().unwrap();
				let mut world = game.world.lock().unwrap();

				let player_clone = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().clone();

				let old_block_id = world.dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();
				let old_block = data::blocks::get_block_from_block_state_id(old_block_id, &game.block_state_data);

				//TODO: move to a update function or similar
				if old_block.block_type == data::blocks::Type::Door {
					let block_state = data::blocks::get_block_state_from_block_state_id(old_block_id, &game.block_state_data);
					let location: Option<BlockPosition> =
						if block_state.properties.iter().any(|x| x == &data::blocks::Property::DoorHalf(data::blocks::DoorHalf::Upper)) {
							Some(BlockPosition {
								y: location.y - 1,
								..location
							})
						} else if block_state.properties.iter().any(|x| x == &data::blocks::Property::DoorHalf(data::blocks::DoorHalf::Lower)) {
							Some(BlockPosition {
								y: location.y + 1,
								..location
							})
						} else {
							None
						};

					if let Some(location) = location {
						world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(location, 0).unwrap();

						players
							.iter()
							.inspect(|x| {
								game.send_packet(
									&x.peer_socket_address,
									lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
									lib::packets::clientbound::play::BlockUpdate {
										location,
										block_id: 0,
									}
									.try_into()
									.unwrap(),
								);
							})
							.filter(|x| x.peer_socket_address != peer_addr)
							.for_each(|x| {
								game.send_packet(
									&x.peer_socket_address,
									lib::packets::clientbound::play::WorldEvent::PACKET_ID,
									lib::packets::clientbound::play::WorldEvent {
										event: 2001,
										location,
										data: old_block_id as i32,
									}
									.try_into()
									.unwrap(),
								);
							});
					}
				}


				let res = world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(location, 0).unwrap();
				if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
					let block_entity = world
						.dimensions
						.get("minecraft:overworld")
						.unwrap()
						.get_chunk_from_position(location)
						.unwrap()
						.block_entities
						.iter()
						.find(|x| x.get_position() == location)
						.unwrap();
					let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
					block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
				}

				if player_clone.gamemode == Gamemode::Survival {
					let new_entity = lib::entity::ItemEntity {
						common: lib::entity::CommonEntity {
							position: EntityPosition {
								x: location.x as f64 + 0.5,
								y: location.y as f64,
								z: location.z as f64 + 0.5,
								yaw: 0.0,
								pitch: 0.0,
							},
							velocity: EntityPosition::default(),
							uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
							entity_id: game.entity_id_manager.get_new(),
							..Default::default()
						},
						age: 0,
						health: 5,
						item: Item {
							id: old_block.block_name.to_string(),
							count: 1,
							components: Vec::new(),
						},
						owner: player_clone.uuid,
						pickup_delay: 0,
						thrower: player_clone.uuid,
					};

					let packet = new_entity.to_spawn_entity_packet();

					let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
						entity_id: new_entity.get_common_entity_data().entity_id,
						metadata: new_entity.get_metadata(),
					};

					world.dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Entity::Item(new_entity));

					players.iter().for_each(|x| {
						game.send_packet(
							&x.peer_socket_address,
							lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
							packet.clone().try_into().unwrap(),
						);
						game.send_packet(
							&x.peer_socket_address,
							lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
							metadata_packet.clone().try_into().unwrap(),
						);
					});
				}

				players
					.iter()
					.inspect(|x| {
						game.send_packet(
							&x.peer_socket_address,
							lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
							lib::packets::clientbound::play::BlockUpdate {
								location,
								block_id: 0,
							}
							.try_into()
							.unwrap(),
						);
					})
					.filter(|x| x.peer_socket_address != peer_addr)
					.for_each(|x| {
						game.send_packet(
							&x.peer_socket_address,
							lib::packets::clientbound::play::WorldEvent::PACKET_ID,
							lib::packets::clientbound::play::WorldEvent {
								event: 2001,
								location,
								data: old_block_id as i32,
							}
							.try_into()
							.unwrap(),
						);
					});

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
					lib::packets::clientbound::play::AcknowledgeBlockChange {
						sequence_id,
					}
					.try_into()
					.unwrap(),
				);

				let blocks_to_update = [
					BlockPosition {
						x: location.x + 1,
						..location
					},
					BlockPosition {
						x: location.x - 1,
						..location
					},
					BlockPosition {
						y: location.y + 1,
						..location
					},
					BlockPosition {
						y: location.y - 1,
						..location
					},
					BlockPosition {
						z: location.z + 1,
						..location
					},
					BlockPosition {
						z: location.z - 1,
						..location
					},
				];

				for block_to_update in blocks_to_update {
					let res =
						lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
					if let Some(new_block) = res {
						match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block) {
							Ok(_) => {
								for player in players.iter() {
									game.send_packet(
										&player.peer_socket_address,
										lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
										lib::packets::clientbound::play::BlockUpdate {
											location: block_to_update,
											block_id: new_block as i32,
										}
										.try_into()
										.unwrap(),
									);
								}
							}
							Err(err) => {
								println!("couldn't place block because {err}");
								continue;
							}
						}
					};
				}
			}
			PacketHandlerAction::UseItemOn(
				peer_addr,
				_hand,
				location,
				face,
				cursor_position_x,
				cursor_position_y,
				cursor_position_z,
				_inside_block,
				_world_border_hit,
				sequence_id,
			) => {
				let mut players = game.players.lock().unwrap();
				let mut world = game.world.lock().unwrap();

				let mut new_block_location = location;
				match face {
					0 => new_block_location.y -= 1,
					1 => new_block_location.y += 1,
					2 => new_block_location.z -= 1,
					3 => new_block_location.z += 1,
					4 => new_block_location.x -= 1,
					_ => new_block_location.x += 1,
				}

				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
				let player_get_looking_cardinal_direction = player.get_looking_cardinal_direction().clone();

				let dimension = world.dimensions.get("minecraft:overworld").unwrap();
				let block_id_at_location = dimension.get_block(location).unwrap_or_default();
				let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location);

				let blocks_to_place: Vec<(u16, BlockPosition)> = if block_type_at_location.has_right_click_behavior() && !player.is_sneaking() {
					//Don't place block, because player right clicked something that does something when right clicked
					let block_interaction_result = lib::block::interact_with_block_at(location, block_id_at_location, face, &game.block_state_data);
					block_interaction_result.handle(dimension, location, player, players_clone, block_id_at_location, game.clone()).unwrap()
				} else {
					//Let's go - we can place a block
					let used_item_id = player
						.get_held_item(true)
						.unwrap_or(&Slot {
							item_count: 0,
							item_id: 0,
							components_to_add: Vec::new(),
							components_to_remove: Vec::new(),
						})
						.item_id;
					let used_item_name = data::items::get_item_name_by_id(used_item_id);
					let pitch = player.get_pitch();

					if used_item_name.ends_with("spawn_egg") {
						let dimension = world.dimensions.get_mut("minecraft:overworld").unwrap();
						lib::create_and_spawn_entity_from_egg(
							used_item_name,
							game.entity_id_manager.get_new(),
							new_block_location,
							dimension,
							&players,
							game.clone(),
						);
					}

					lib::block::get_block_state_id(
						face,
						player_get_looking_cardinal_direction,
						pitch,
						world.dimensions.get_mut("minecraft:overworld").unwrap(),
						new_block_location,
						used_item_name,
						cursor_position_x,
						cursor_position_y,
						cursor_position_z,
						&game.block_state_data,
					)
				};

				let mut blocks_to_update: Vec<BlockPosition> = Vec::new();
				for block_to_place in &blocks_to_place {
					match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_place.1, block_to_place.0) {
						Ok(res) => {
							let block = data::blocks::get_block_from_block_state_id(block_to_place.0, &game.block_state_data);
							//Logic to open sign editor when player placed a new sign, maybe move somewhere else or something idk
							if block.block_type == data::blocks::Type::WallSign
								|| block.block_type == data::blocks::Type::StandingSign
								|| block.block_type == data::blocks::Type::WallHangingSign
								|| block.block_type == data::blocks::Type::CeilingHangingSign
							{
								game.send_packet(
									&peer_addr,
									lib::packets::clientbound::play::OpenSignEditor::PACKET_ID,
									lib::packets::clientbound::play::OpenSignEditor {
										location: block_to_place.1,
										is_front_text: true,
									}
									.try_into()
									.unwrap(),
								);
							}
							#[allow(clippy::collapsible_if)]
							if res.is_some() && res.unwrap() == BlockOverwriteOutcome::DestroyBlockentity {
								if let Some(block_entity) = world
									.dimensions
									.get("minecraft:overworld")
									.unwrap()
									.get_chunk_from_position(location)
									.unwrap()
									.block_entities
									.iter()
									.find(|x| x.get_position() == location)
								{
									let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
									block_entity.remove_self(&game.entity_id_manager, &mut players, &mut world, game.clone());
								};
							}

							blocks_to_update.append(&mut vec![
								BlockPosition {
									x: block_to_place.1.x + 1,
									..block_to_place.1
								},
								BlockPosition {
									x: block_to_place.1.x - 1,
									..block_to_place.1
								},
								BlockPosition {
									y: block_to_place.1.y + 1,
									..block_to_place.1
								},
								BlockPosition {
									y: block_to_place.1.y - 1,
									..block_to_place.1
								},
								BlockPosition {
									z: block_to_place.1.z + 1,
									..block_to_place.1
								},
								BlockPosition {
									z: block_to_place.1.z - 1,
									..block_to_place.1
								},
							]);
						}
						Err(err) => {
							println!("couldn't place block because {err}");
							continue;
						}
					};
				}

				blocks_to_update.sort();
				blocks_to_update.dedup();

				let mut updated_blocks: Vec<(u16, BlockPosition)> = Vec::new();
				for block_to_update in blocks_to_update {
					let res =
						lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
					if let Some(new_block) = res {
						match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block) {
							Ok(_) => {
								updated_blocks.push((new_block, block_to_update));
							}
							Err(err) => {
								println!("couldn't place block because {err}");
								continue;
							}
						}
					};
				}

				let all_changed_blocks: Vec<(u16, BlockPosition)> = vec![blocks_to_place, updated_blocks].into_iter().flatten().collect();

				for player in players.iter() {
					for block in &all_changed_blocks {
						game.send_packet(
							&player.peer_socket_address,
							lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
							lib::packets::clientbound::play::BlockUpdate {
								location: block.1,
								block_id: block.0 as i32,
							}
							.try_into()
							.unwrap(),
						);
					}
				}

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID,
					lib::packets::clientbound::play::AcknowledgeBlockChange {
						sequence_id,
					}
					.try_into()
					.unwrap(),
				);
			}
			PacketHandlerAction::SendChatMessage(peer_addr, message, timestamp, salt) => {
				let mut players = game.players.lock().unwrap();

				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

				println!("<{}>: {}", player.display_name, message);

				let mut packet_to_send = lib::packets::clientbound::play::PlayerChatMessage {
					global_index: -1,
					sender: player.uuid,
					index: 0,
					message_signature_bytes: Vec::new(),
					message: message.clone(),
					timestamp,
					salt,
					signature_array: Vec::new(),
					unsigned_content: None,
					filter_type: 0,
					filter_type_bits: Vec::new(),
					chat_type: 1,
					sender_name: NbtTag::Root(vec![
						NbtTag::TagCompound(
							"click_event".to_string(),
							vec![
								NbtTag::String("action".to_string(), "suggest_command".to_string()),
								NbtTag::String("command".to_string(), format!("/tell {}", player.display_name).to_string()),
							],
						),
						NbtTag::String("insertion".to_string(), player.display_name.clone()),
						NbtTag::String("text".to_string(), player.display_name.clone()),
					]),
					target_name: None,
				};

				for player in players.iter_mut() {
					packet_to_send.global_index = player.chat_message_index;
					player.chat_message_index += 1;
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::PlayerChatMessage::PACKET_ID,
						packet_to_send.clone().try_into().unwrap(),
					);
				}
			}
			PacketHandlerAction::SendCommand(peer_addr, command_string) => {
				let players = game.players.lock().unwrap();
				let player = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap();
				println!("<{}> invoked: {}", player.display_name, command_string);

				let commands = game.commands.lock().unwrap().clone();

				let Some(command) = commands.iter().find(|x| x.name == command_string.split(" ").next().unwrap_or_default()) else {
					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
						lib::packets::clientbound::play::SystemChatMessage {
							content: NbtTag::Root(vec![
								NbtTag::String("type".to_string(), "text".to_string()),
								NbtTag::String("text".to_string(), "command not found".to_string()),
							]),
							overlay: false,
						}
						.try_into()
						.unwrap(),
					);
					continue;
				};

				let mut stream = player.connection_stream.try_clone().unwrap();
				drop(players);
				(command.execute)(command_string, Some(&mut stream), game.clone()).unwrap();
			}
			PacketHandlerAction::ClickContainer(peer_addr, parsed_packet) => {
				let mut players = game.players.lock().unwrap();
				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

				let Some(position) = player.opened_inventory_at else {
					println!("player doesn't seems to have a container opened at the moment");
					continue;
				};

				let streams_with_container_opened = players
					.iter()
					.filter(|x| x.opened_inventory_at.is_some_and(|x| x == position))
					.map(|x| x.connection_stream.try_clone().unwrap())
					.collect::<Vec<TcpStream>>();

				let mut dimensions = std::mem::take(&mut game.world.lock().unwrap().dimensions);
				let mut block_entity = dimensions
					.get_mut("minecraft:overworld")
					.unwrap()
					.get_chunk_from_position_mut(position)
					.unwrap()
					.try_get_block_entity_mut(position)
					.unwrap();

				let player_uuid = players.iter().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap().uuid;
				drop(players);

				match &mut block_entity {
					BlockEntity::Barrel(barrel) => {
						let items = barrel.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 27);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::BlastFurnace(blast_furnace) => {
						let items = blast_furnace.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 3);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::BrewingStand(brewing_stand) => {
						let items = brewing_stand.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 5);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Chest(chest) => {
						let items = chest.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 27);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Crafter(crafter) => {
						let items = crafter.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 9);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Dispenser(dispenser) => {
						let items = dispenser.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 9);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Dropper(dropper) => {
						let items = dropper.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 9);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Furnace(furnace) => {
						let items = furnace.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 3);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
						block_entity.set_needs_ticking(true);
					}
					BlockEntity::Hopper(hopper) => {
						let items = hopper.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 5);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::ShulkerBox(shulker_box) => {
						let items = shulker_box.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 27);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::Smoker(smoker) => {
						let items = smoker.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 3);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					BlockEntity::TrappedChest(trapped_chest) => {
						let items = trapped_chest.get_contained_items_mut();
						assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
						assert!(items.len() == 27);
						lib::containerclick::handle(parsed_packet, items, player_uuid, game.clone(), streams_with_container_opened);
					}
					x => println!("can't handle click_container packet for entity {x:?}"),
				}

				game.world.lock().unwrap().dimensions = dimensions;
			}
			PacketHandlerAction::CloseContainer(peer_addr, window_id) => {
				let mut players = game.players.lock().unwrap();
				let world = game.world.lock().unwrap();

				if window_id != 0 {
					if let Some(position) = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().opened_inventory_at {
						let number_of_players_with_container_opened =
							players.iter().filter(|x| x.opened_inventory_at.is_some_and(|x| x == position)).count();

						if number_of_players_with_container_opened == 1 {
							//1, because we havent called close_inventory() on current player yet
							players.iter().for_each(|x| {
								game.send_packet(
									&x.peer_socket_address,
									lib::packets::clientbound::play::BlockAction::PACKET_ID,
									lib::packets::clientbound::play::BlockAction {
										location: position,
										action_id: 1,
										action_parameter: 0,
										block_type: world.dimensions.get("minecraft:overworld").unwrap().get_block(position).unwrap() as i32,
									}
									.try_into()
									.unwrap(),
								);
							});
						}
					};

					players.iter_mut().filter(|x| x.peer_socket_address == peer_addr).for_each(|x| x.close_inventory(game.clone()).unwrap());
				}
			}
			PacketHandlerAction::UpdateSign(location, _is_front, text) => {
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
					block_entity_type: *data::blockentity::get_block_entity_types().get(Into::<&str>::into(blockentity.get_id().as_str())).unwrap()
						as i32,
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
			PacketHandlerAction::PlayerInput(peer_addr, parsed_packet) => {
				let players_clone = game.players.lock().unwrap().clone();
				let mut players = game.players.lock().unwrap();
				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

				if parsed_packet.sneak {
					player.set_sneaking(true, &players_clone, game.clone());
				} else {
					player.set_sneaking(false, &players_clone, game.clone());
				}
			}
			PacketHandlerAction::Interact(peer_addr, parsed_packet) => {
				let mut players = game.players.lock().unwrap();
				let player = players.iter().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
				let held_item = player.get_held_item(true);

				let mut world = game.world.lock().unwrap();
				let Some(entity) = world
					.dimensions
					.get_mut("minecraft:overworld")
					.unwrap()
					.entities
					.iter_mut()
					.find(|x| x.get_common_entity_data().entity_id == parsed_packet.entity_id)
				else {
					continue;
				};
				let entity_id = entity.get_common_entity_data().entity_id;


				if parsed_packet.interact_type == 1 {
					//Attack
					let damage = if held_item.is_some() { 10.0 } else { 1.0 };

					if entity.is_mob() {
						let mob_data = entity.get_mob_data_mut();

						if mob_data.hurt_time > 0 {
							continue;
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
			PacketHandlerAction::NewPlayer(peer_addr, stream) => {
				game.connections.entry(peer_addr).and_modify(|x| x.state = lib::ConnectionState::Play);

				let mut world = game.world.lock().unwrap();

				let connection_player = game.connections.get(&peer_addr).unwrap();

				let mut new_player = Player::new(
					connection_player.player_name.clone().unwrap_or_default(),
					connection_player.player_uuid.unwrap_or_default(),
					peer_addr,
					game.clone(),
					stream,
					world.default_spawn_location,
				);
				let mut players = game.players.lock().unwrap();

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::Login::PACKET_ID,
					lib::packets::clientbound::play::Login {
						entity_id: new_player.entity_id,
						is_hardcore: false,
						dimension_names: vec!["minecraft:overworld".to_string()],
						max_players: 9,
						view_distance: 32,
						simulation_distance: 32,
						reduced_debug_info: false,
						enable_respawn_screen: true,
						do_limited_crafting: false,
						dimension_type: 0,
						dimension_name: "minecraft:overworld".to_string(),
						hashed_seed: 1,
						game_mode: new_player.gamemode as u8,
						previous_game_mode: -1,
						is_debug: false,
						is_flat: false,
						has_death_location: false,
						death_dimension_name: None,
						death_location: None,
						portal_cooldown: 123,
						sea_level: 64,
						enforces_secure_chat: false,
					}
					.try_into()
					.unwrap(),
				);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID,
					lib::packets::clientbound::play::SynchronizePlayerPosition {
						teleport_id: new_player.current_teleport_id,
						x: new_player.get_position().x,
						y: new_player.get_position().y,
						z: new_player.get_position().z,
						velocity_x: 0.0,
						velocity_y: 0.0,
						velocity_z: 0.0,
						yaw: new_player.get_position().yaw,
						pitch: new_player.get_position().pitch,
						flags: 0,
					}
					.try_into()
					.unwrap(),
				);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::GameEvent::PACKET_ID,
					lib::packets::clientbound::play::GameEvent {
						event: 13,
						value: 0.0,
					}
					.try_into()
					.unwrap(),
				);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::EntityEvent::PACKET_ID,
					lib::packets::clientbound::play::EntityEvent {
						entity_id: new_player.entity_id,
						entity_status: 28, //set op permission level 4
					}
					.try_into()
					.unwrap(),
				);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::SetContainerContent::PACKET_ID,
					lib::packets::clientbound::play::SetContainerContent {
						window_id: 0,
						state_id: 1,
						slot_data: new_player.get_inventory().clone(),
						carried_item: None,
					}
					.try_into()
					.unwrap(),
				);

				let current_chunk_coords = BlockPosition::from(new_player.get_position()).convert_to_coordinates_of_chunk();

				let now = std::time::Instant::now();
				for x in current_chunk_coords.x - lib::VIEW_DISTANCE as i32..=current_chunk_coords.x + lib::VIEW_DISTANCE as i32 {
					for z in current_chunk_coords.z - lib::VIEW_DISTANCE as i32..=current_chunk_coords.z + lib::VIEW_DISTANCE as i32 {
						new_player.send_chunk(&mut world, x, z, &game.entity_id_manager, &game.block_state_data, game.clone()).unwrap();
					}
				}
				println!("send chunks: {:?}", std::time::Instant::now() - now);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::SetHeldItem::PACKET_ID,
					lib::packets::clientbound::play::SetHeldItem {
						slot: new_player.get_selected_slot(),
					}
					.try_into()
					.unwrap(),
				);

				let new_player_uuid = new_player.uuid;
				let new_player_entity_id = new_player.entity_id;
				let new_player_display_name = new_player.display_name.clone();
				let new_player_x = new_player.get_position().x;
				let new_player_y = new_player.get_position().y;
				let new_player_z = new_player.get_position().z;
				let new_player_inventory = new_player.get_inventory().clone();
				let new_player_selected_slot = new_player.get_selected_slot();
				let new_player_entity_metadata = new_player.get_metadata();
				let new_player_gamemode = new_player.gamemode;

				//send player list to newly connected player
				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID,
					lib::packets::clientbound::play::PlayerInfoUpdate {
						actions: 255,
						players: players
							.iter()
							.map(|y| {
								(
									y.uuid,
									vec![
										lib::packets::clientbound::play::PlayerAction::AddPlayer(y.display_name.clone(), vec![]),
										lib::packets::clientbound::play::PlayerAction::InitializeChat(None),
										lib::packets::clientbound::play::PlayerAction::UpdateGameMode(new_player_gamemode as u8 as i32),
										lib::packets::clientbound::play::PlayerAction::UpdateListed(true),
										lib::packets::clientbound::play::PlayerAction::UpdateLatency(0),
										lib::packets::clientbound::play::PlayerAction::UpdateDisplayName(None),
										lib::packets::clientbound::play::PlayerAction::UpdateListPriority(0),
										lib::packets::clientbound::play::PlayerAction::UpdateHat(true),
									],
								)
							})
							.collect(),
					}
					.try_into()
					.unwrap(),
				);

				players.push(new_player);

				//update player list for already connected players
				players.iter().for_each(|x| {
					game.send_packet(
						&x.peer_socket_address,
						lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID,
						lib::packets::clientbound::play::PlayerInfoUpdate {
							actions: 255,
							players: vec![(
								new_player_uuid,
								vec![
									lib::packets::clientbound::play::PlayerAction::AddPlayer(new_player_display_name.clone(), vec![]),
									lib::packets::clientbound::play::PlayerAction::InitializeChat(None),
									lib::packets::clientbound::play::PlayerAction::UpdateGameMode(new_player_gamemode as u8 as i32),
									lib::packets::clientbound::play::PlayerAction::UpdateListed(true),
									lib::packets::clientbound::play::PlayerAction::UpdateLatency(0),
									lib::packets::clientbound::play::PlayerAction::UpdateDisplayName(None),
									lib::packets::clientbound::play::PlayerAction::UpdateListPriority(0),
									lib::packets::clientbound::play::PlayerAction::UpdateHat(true),
								],
							)],
						}
						.try_into()
						.unwrap(),
					);
				});

				//Spawn other already connected player entities for newly joined player
				for player in players.iter() {
					if player.uuid == new_player_uuid {
						continue;
					}

					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						lib::packets::clientbound::play::SpawnEntity {
							entity_id: player.entity_id,
							entity_uuid: player.uuid,
							entity_type: data::entities::get_id_from_name("minecraft:player"),
							x: player.get_position().x,
							y: player.get_position().y,
							z: player.get_position().z,
							pitch: player.get_pitch_u8(),
							yaw: player.get_yaw_u8(),
							head_yaw: player.get_yaw_u8(),
							data: 0,
							velocity_x: 0,
							velocity_y: 0,
							velocity_z: 0,
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
						lib::packets::clientbound::play::SetEntityMetadata {
							entity_id: player.entity_id,
							metadata: new_player_entity_metadata.clone(),
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SetEquipment::PACKET_ID,
						lib::packets::clientbound::play::SetEquipment {
							entity_id: player.entity_id,
							equipment: vec![
								(0, player.get_inventory()[(player.get_selected_slot() + 36) as usize].clone()),
								(1, player.get_inventory()[45].clone()),
								(2, player.get_inventory()[8].clone()),
								(3, player.get_inventory()[7].clone()),
								(4, player.get_inventory()[6].clone()),
								(5, player.get_inventory()[5].clone()),
							],
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID,
						lib::packets::clientbound::play::UpdateEntityRotation {
							entity_id: player.entity_id,
							on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
							yaw: player.get_yaw_u8(),
							pitch: player.get_pitch_u8(),
						}
						.try_into()
						.unwrap(),
					);
					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
						lib::packets::clientbound::play::SetHeadRotation {
							entity_id: player.entity_id,
							head_yaw: player.get_yaw_u8(),
						}
						.try_into()
						.unwrap(),
					);
				}

				//Spawn player entity for other players that are already connected
				for player in players.iter() {
					if player.peer_socket_address == peer_addr {
						continue;
					}

					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						lib::packets::clientbound::play::SpawnEntity {
							entity_id: new_player_entity_id,
							entity_uuid: new_player_uuid,
							entity_type: data::entities::get_id_from_name("minecraft:player"),
							x: new_player_x,
							y: new_player_y,
							z: new_player_z,
							pitch: 0,
							yaw: 0,
							head_yaw: 0,
							data: 0,
							velocity_x: 0,
							velocity_y: 0,
							velocity_z: 0,
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
						lib::packets::clientbound::play::SetEntityMetadata {
							entity_id: new_player_entity_id,
							metadata: new_player_entity_metadata.clone(),
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SetEquipment::PACKET_ID,
						lib::packets::clientbound::play::SetEquipment {
							entity_id: new_player_entity_id,
							equipment: vec![
								(0, new_player_inventory[(new_player_selected_slot + 36) as usize].clone()),
								(1, new_player_inventory[45].clone()),
								(2, new_player_inventory[8].clone()),
								(3, new_player_inventory[7].clone()),
								(4, new_player_inventory[6].clone()),
								(5, new_player_inventory[5].clone()),
							],
						}
						.try_into()
						.unwrap(),
					);

					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID,
						lib::packets::clientbound::play::UpdateEntityRotation {
							entity_id: player.entity_id,
							on_ground: player.is_on_ground(world.dimensions.get("minecraft:overworld").unwrap()),
							yaw: player.get_yaw_u8(),
							pitch: player.get_pitch_u8(),
						}
						.try_into()
						.unwrap(),
					);
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SetHeadRotation::PACKET_ID,
						lib::packets::clientbound::play::SetHeadRotation {
							entity_id: player.entity_id,
							head_yaw: player.get_yaw_u8(),
						}
						.try_into()
						.unwrap(),
					);
				}


				for entity in &world.dimensions.get("minecraft:overworld").unwrap().entities {
					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						entity.to_spawn_entity_packet().try_into().unwrap(),
					);

					game.send_packet(
						&peer_addr,
						lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
						lib::packets::clientbound::play::SetEntityMetadata {
							entity_id: entity.get_common_entity_data().entity_id,
							metadata: entity.get_metadata(),
						}
						.try_into()
						.unwrap(),
					);
				}

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::Commands::PACKET_ID,
					lib::packets::clientbound::play::Commands {
						nodes: crate::command::get_command_packet_data(game.clone()),
						root_index: 0,
					}
					.try_into()
					.unwrap(),
				);

				game.send_packet(
					&peer_addr,
					lib::packets::clientbound::play::SetTabListHeaderAndFooter::PACKET_ID,
					lib::packets::clientbound::play::SetTabListHeaderAndFooter {
						header: NbtTag::Root(vec![NbtTag::String("text".to_string(), "".to_string())]),
						footer: NbtTag::Root(vec![
							NbtTag::String("type".to_string(), "text".to_string()),
							NbtTag::String("text".to_string(), " powered by Oxide ".to_string()),
							NbtTag::String("color".to_string(), "gray".to_string()),
							NbtTag::Byte("italic".to_string(), 1),
						]),
					}
					.try_into()
					.unwrap(),
				);
			}
			PacketHandlerAction::UpdateGamemode(peer_addr, gamemode) => {
				let players_clone = game.players.lock().unwrap().clone();
				let mut players = game.players.lock().unwrap();
				let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
				player.set_gamemode(gamemode, &players_clone, game.clone()).unwrap();
			}
		}
	}
	*game.packet_handler_actions.lock().unwrap() = Vec::new();
}
