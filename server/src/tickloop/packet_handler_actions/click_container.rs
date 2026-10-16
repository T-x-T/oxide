use std::collections::HashMap;

use lib::packets::serverbound::play::ClickContainer;

use super::*;

pub fn process(peer_addr: SocketAddr, parsed_packet: ClickContainer, game: Arc<Game>, players_clone: &[Player]) {
	let mut players = game.players.lock().unwrap();
	let dimensions = &mut game.world.lock().unwrap().dimensions;
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	let player_uuid = player.get_common_entity_data().uuid;

	// println!("{:?}", parsed_packet.changed_slots);

	if parsed_packet.state_id != player.get_container_click_state_id() {
		resolve_state_id_mismatch(parsed_packet, game.clone(), player, dimensions);
		return;
	}

	let Some(position) = player.opened_inventory_at else {
		handle_player_inventory(parsed_packet, game.clone(), players_clone, &mut players, player_uuid, peer_addr);
		return;
	};

	let player_currently_interacts_with_crafting_table = dimensions
		.get(player.get_dimension())
		.unwrap()
		.get_block(position)
		.is_ok_and(|x| game.block_state_data.get("minecraft:crafting_table").unwrap().states.first().unwrap().id == x);

	let streams_with_container_opened = players_clone
		.iter()
		.filter(|x| x.opened_inventory_at.is_some_and(|x| x == position))
		.map(|x| x.peer_socket_address)
		.collect::<Vec<SocketAddr>>();

	let block_entity =
		dimensions.get_mut(player.get_dimension()).unwrap().get_chunk_from_position_mut(position).unwrap().try_get_block_entity_mut(position);

	if let Some(block_entity) = block_entity {
		handle_blockentity(parsed_packet, game.clone(), block_entity, players_clone, &mut players, player_uuid, streams_with_container_opened);
	} else if player_currently_interacts_with_crafting_table {
		handle_crafting_table(parsed_packet, game.clone(), players_clone, &mut players, player_uuid, peer_addr);
	}
}

fn handle_player_inventory(
	parsed_packet: ClickContainer,
	game: Arc<Game>,
	players_clone: &[Player],
	players: &mut [Player],
	player_uuid: u128,
	peer_addr: SocketAddr,
) {
	let gamemode = players_clone.iter().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().get_gamemode();
	let inventory = players_clone.iter().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().get_inventory();
	if gamemode != Gamemode::Creative && parsed_packet.window_id == 0 {
		let mut inventory: Vec<Slot> = inventory.clone().into_iter().map(|x| x.unwrap_or_default()).collect();

		if parsed_packet.slot != 0 {
			lib::containerclick::handle(&parsed_packet, &mut inventory, player_uuid, Vec::new(), players, players_clone, &game.packet_sender);
		}

		let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
		let inventory_to_set: Vec<Option<Slot>> = inventory.into_iter().map(|x| if x.count == 0 { None } else { Some(x) }).collect();
		player.set_inventory_and_dont_inform_client(inventory_to_set.clone());
		player.send_armor_points(&game.packet_sender);

		let crafting_slots = player.get_inventory()[1..=4].to_vec();
		if [1, 2, 3, 4].contains(&parsed_packet.slot) {
			// Player tries to craft in their own inventory
			let mut inventory_to_set = inventory_to_set;
			if let Some(recipe) = game.recipe_manager.get_crafting_recipe_2x2(crafting_slots.as_array().unwrap()) {
				inventory_to_set[0] = Some(Slot {
					count: recipe.get_result_count(),
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				});
			} else {
				inventory_to_set[0] = None;
			}
			player.set_inventory_and_inform_client(inventory_to_set, players_clone, &game.packet_sender);
		} else if parsed_packet.slot == 0
			&& let Some(recipe) = game.recipe_manager.get_crafting_recipe_2x2(crafting_slots.as_array().unwrap())
		{
			// Player takes from the result slot
			if player.cursor_item.is_none() {
				player.cursor_item = Some(Slot {
					count: recipe.get_result_count(),
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				});
			} else if player.cursor_item.as_ref().unwrap().id == data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap() {
				player.cursor_item = Some(Slot {
					count: player.cursor_item.as_ref().unwrap().count + recipe.get_result_count(), //TODO: can overflow
					id: player.cursor_item.as_ref().unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				});
			}

			let mut inventory_after_crafting: Vec<Option<Slot>> = player
				.get_inventory()
				.iter()
				.enumerate()
				.map(|(i, x)| {
					if ![1, 2, 3, 4].contains(&i) {
						return x.clone();
					} else if let Some(mut slot) = x.clone() {
						if slot.count > 1 {
							slot.count -= 1;
							return Some(slot.clone());
						} else {
							return None;
						}
					}
					x.clone()
				})
				.collect();


			let crafting_slots = inventory_after_crafting[1..=4].to_vec();
			if let Some(recipe) = game.recipe_manager.get_crafting_recipe_2x2(crafting_slots.as_array().unwrap()) {
				inventory_after_crafting[0] = Some(Slot {
					count: recipe.get_result_count(),
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				});
			} else {
				inventory_after_crafting[0] = None;
			}
			player.set_inventory_and_inform_client(inventory_after_crafting, players_clone, &game.packet_sender);

			player.send_container_slot_data(
				&game.packet_sender,
				0,
				Some(Slot {
					count: recipe.get_result_count(),
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			);
		}
		let inventory = player.get_inventory().clone();
		resend_inventory_on_changed_slot_mismatch(parsed_packet, &inventory, player, game);
	} else {
		println!("player doesn't seem to have a container opened at the moment");
	}
}

fn handle_blockentity(
	parsed_packet: ClickContainer,
	game: Arc<Game>,
	block_entity: &mut BlockEntity,
	players_clone: &[Player],
	players: &mut [Player],
	player_uuid: u128,
	streams_with_container_opened: Vec<SocketAddr>,
) {
	match block_entity {
		BlockEntity::Barrel(barrel) => {
			let items = barrel.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 27);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::BlastFurnace(blast_furnace) => {
			let items = blast_furnace.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 3);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
			block_entity.set_needs_ticking(true);
		}
		BlockEntity::BrewingStand(brewing_stand) => {
			let items = brewing_stand.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 5);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Chest(chest) => {
			let items = chest.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 27);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Crafter(crafter) => {
			let items = crafter.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 9);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Dispenser(dispenser) => {
			let items = dispenser.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 9);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Dropper(dropper) => {
			let items = dropper.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 9);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Furnace(furnace) => {
			let items = furnace.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 3);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
			block_entity.set_needs_ticking(true);
		}
		BlockEntity::Hopper(hopper) => {
			let items = hopper.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 5);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::ShulkerBox(shulker_box) => {
			let items = shulker_box.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 27);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		BlockEntity::Smoker(smoker) => {
			let items = smoker.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 3);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
			block_entity.set_needs_ticking(true);
		}
		BlockEntity::TrappedChest(trapped_chest) => {
			let items = trapped_chest.get_contained_items_mut();
			assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
			assert!(items.len() == 27);
			lib::containerclick::handle(
				&parsed_packet,
				items,
				player_uuid,
				streams_with_container_opened,
				players,
				players_clone,
				&game.packet_sender,
			);
			let items: Vec<Option<Slot>> = items.iter().map(|x| if x.count == 0 { None } else { Some(x.clone()) }).collect();
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			resend_inventory_on_changed_slot_mismatch(parsed_packet, items.as_slice(), player, game);
		}
		x => {
			println!("can't handle click_container packet for entity {x:?}");
			return;
		}
	}
}

fn handle_crafting_table(
	parsed_packet: ClickContainer,
	game: Arc<Game>,
	players_clone: &[Player],
	players: &mut [Player],
	player_uuid: u128,
	peer_addr: SocketAddr,
) {
	let mut crafting_slots =
		players_clone.iter().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().crafting_table_slots.clone().to_vec();
	crafting_slots.insert(0, Slot::default()); //need to append empty result slot

	//no need to handle when taking from result slot as that gets handled further down
	if parsed_packet.slot != 0 {
		lib::containerclick::handle(&parsed_packet, &mut crafting_slots, player_uuid, Vec::new(), players, players_clone, &game.packet_sender);
	}
	crafting_slots.remove(0); //remove empty result slot again

	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	player.crafting_table_slots = crafting_slots.as_array().unwrap().clone();

	//crafting in a crafting table
	let crafting_slots = player.crafting_table_slots.to_vec();
	if [1, 2, 3, 4, 5, 6, 7, 8, 9, 9].contains(&parsed_packet.slot) {
		// Player tries to craft in the crafting table
		if let Some(recipe) = game.recipe_manager.get_crafting_recipe_3x3(
			crafting_slots.into_iter().map(|x| if x.count == 0 { None } else { Some(x) }).collect::<Vec<Option<Slot>>>().as_array().unwrap(),
		) {
			player.send_container_slot_data(
				&game.packet_sender,
				0,
				Some(Slot {
					count: recipe.get_result_count(),
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			);
			let mut container_content: Vec<Option<Slot>> = vec![Some(Slot {
				count: recipe.get_result_count(),
				id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			})];
			container_content
				.append(&mut player.crafting_table_slots.iter().cloned().map(|x| if x.count != 0 { Some(x) } else { None }).collect());
			container_content.append(&mut player.get_inventory()[9..=44].to_vec());
			resend_inventory_on_changed_slot_mismatch(parsed_packet, &container_content, player, game);
		} else {
			player.send_container_slot_data(&game.packet_sender, 0, None);
			let mut container_content: Vec<Option<Slot>> = vec![None];
			container_content
				.append(&mut player.crafting_table_slots.iter().cloned().map(|x| if x.count != 0 { Some(x) } else { None }).collect());
			container_content.append(&mut player.get_inventory()[9..=44].to_vec());
			resend_inventory_on_changed_slot_mismatch(parsed_packet, &container_content, player, game);
		}
	} else if parsed_packet.slot == 0
		&& let Some(recipe) = game.recipe_manager.get_crafting_recipe_3x3(
			crafting_slots
				.clone()
				.into_iter()
				.map(|x| if x.count == 0 { None } else { Some(x) })
				.collect::<Vec<Option<Slot>>>()
				.as_array()
				.unwrap(),
		) {
		// Player takes from the result slot
		if player.cursor_item.as_ref().is_none_or(|x| x.count == 0) {
			player.cursor_item = Some(Slot {
				count: recipe.get_result_count(),
				id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			});
		} else if player.cursor_item.as_ref().unwrap().id == data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap() {
			player.cursor_item = Some(Slot {
				count: player.cursor_item.as_ref().unwrap().count + recipe.get_result_count(), //TODO: possible overflow??
				id: player.cursor_item.as_ref().unwrap().id,
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			});
		}
		player.crafting_table_slots = crafting_slots
			.clone()
			.iter()
			.enumerate()
			.map(|(i, x)| {
				let mut slot = x.clone();
				if ![1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&i) {
					return slot;
				}
				if slot.count > 0 {
					slot.count -= 1;
					return slot;
				}
				slot
			})
			.collect::<Vec<Slot>>()
			.as_array()
			.unwrap()
			.clone();

		let output_after_crafting = game
			.recipe_manager
			.get_crafting_recipe_3x3(
				player
					.crafting_table_slots
					.clone()
					.into_iter()
					.map(|x| if x.count == 0 { None } else { Some(x) })
					.collect::<Vec<Option<Slot>>>()
					.as_array()
					.unwrap(),
			)
			.map(|recipe| Slot {
				count: recipe.get_result_count(),
				id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			});

		let mut container_content: Vec<Option<Slot>> = vec![output_after_crafting];
		container_content.append(&mut player.crafting_table_slots.iter().cloned().map(|x| if x.count != 0 { Some(x) } else { None }).collect());
		container_content.append(&mut player.get_inventory()[9..=44].to_vec());

		player.send_container_content_data(&game.packet_sender, 1, container_content.clone(), player.cursor_item.clone());
		resend_inventory_on_changed_slot_mismatch(parsed_packet, &container_content, player, game);
	}
}

fn resolve_state_id_mismatch(parsed_packet: ClickContainer, game: Arc<Game>, player: &mut Player, dimensions: &HashMap<String, Dimension>) {
	if parsed_packet.window_id == 0 {
		player.send_container_content_data(&game.packet_sender, 0, player.get_inventory().clone(), player.cursor_item.clone());
		println!("resent player inventory upon state mismatch, returning");
	} else {
		if let Some(position) = player.opened_inventory_at {
			let dimension = dimensions.get(player.get_dimension()).unwrap();
			let player_currently_interacts_with_crafting_table = dimension
				.get_block(position)
				.is_ok_and(|x| game.block_state_data.get("minecraft:crafting_table").unwrap().states.first().unwrap().id == x);

			if let Some(block_entity) = dimension.get_chunk_from_position(position).unwrap().try_get_block_entity(position) {
				player.send_container_content_data(
					&game.packet_sender,
					1,
					block_entity.get_contained_items_owned().clone().into_iter().map(|x| if x.count == 0 { None } else { Some(x) }).collect(),
					player.cursor_item.clone(),
				);
				println!("resent blockentities inventory upon state mismatch, returning");
			} else if player_currently_interacts_with_crafting_table {
				let output = game
					.recipe_manager
					.get_crafting_recipe_3x3(
						player
							.crafting_table_slots
							.clone()
							.into_iter()
							.map(|x| if x.count == 0 { None } else { Some(x) })
							.collect::<Vec<Option<Slot>>>()
							.as_array()
							.unwrap(),
					)
					.map(|recipe| Slot {
						count: recipe.get_result_count(),
						id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()).unwrap(),
						components_to_add: Vec::new(),
						components_to_remove: Vec::new(),
					});
				let mut container_content: Vec<Option<Slot>> = vec![output];
				container_content
					.append(&mut player.crafting_table_slots.iter().cloned().map(|x| if x.count != 0 { Some(x) } else { None }).collect());

				container_content.append(&mut player.get_inventory()[9..=44].to_vec());
				println!("{parsed_packet:?}");
				player.send_container_content_data(&game.packet_sender, 1, container_content, player.cursor_item.clone());
				println!("resent crafting tables inventory upon state mismatch, returning");
			}
		}
	}
}

fn resend_inventory_on_changed_slot_mismatch(
	parsed_packet: ClickContainer,
	inventory: &[Option<Slot>],
	player: &mut Player,
	game: Arc<Game>,
) {
	for (i, predicted_slot) in parsed_packet.changed_slots {
		if let Some(actual_slot) = inventory.get(i as usize)
			&& predicted_slot != actual_slot.clone()
		{
			let cursor_item = player.cursor_item.clone();
			if parsed_packet.window_id == 1 {
				player.send_container_content_data(&game.packet_sender, 1, inventory.to_vec(), cursor_item);
			} else {
				player.send_container_content_data(&game.packet_sender, 0, player.get_inventory().clone(), cursor_item);
			}
			// println!("predicted: {predicted_slot:?}, actual: {actual_slot:?}");
		}
	}
}
