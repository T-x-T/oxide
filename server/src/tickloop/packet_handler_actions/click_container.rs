use lib::packets::serverbound::play::ClickContainer;

use super::*;

pub fn process(peer_addr: SocketAddr, parsed_packet: ClickContainer, game: Arc<Game>, players_clone: &[Player]) {
	//println!("{parsed_packet:?}");
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
	let player_uuid = player.uuid;

	let Some(position) = player.opened_inventory_at else {
		if player.get_gamemode() != Gamemode::Creative && parsed_packet.window_id == 0 {
			let mut inventory: Vec<Slot> = player.get_inventory().clone().into_iter().map(|x| x.unwrap_or_default()).collect();

			if parsed_packet.slot != 0 {
				lib::containerclick::handle(
					parsed_packet.clone(),
					&mut inventory,
					player_uuid,
					game.clone(),
					Vec::new(),
					&mut players,
					players_clone,
				);
			}

			let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

			let inventory_to_set: Vec<Option<Slot>> = inventory.into_iter().map(|x| if x.count == 0 { None } else { Some(x) }).collect();
			player.set_inventory_and_dont_inform_client(inventory_to_set.clone());

			let crafting_slots = player.get_inventory()[1..=4].to_vec();
			if [1, 2, 3, 4].contains(&parsed_packet.slot) {
				// Player tries to craft in their own inventory
				if let Some(recipe) = game.recipe_manager.get_crafting_recipe_2x2(crafting_slots.as_array().unwrap()) {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
						lib::packets::clientbound::play::SetContainerSlot {
							window_id: 0,
							state_id: 1,
							slot: 0,
							slot_data: Some(Slot {
								count: recipe.get_result_count(),
								id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
								components_to_add: Vec::new(),
								components_to_remove: Vec::new(),
							}),
						}
						.try_into()
						.unwrap(),
					);
				} else {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
						lib::packets::clientbound::play::SetContainerSlot {
							window_id: 0,
							state_id: 1,
							slot: 0,
							slot_data: None,
						}
						.try_into()
						.unwrap(),
					);
				}
			} else if parsed_packet.slot == 0
				&& let Some(recipe) = game.recipe_manager.get_crafting_recipe_2x2(crafting_slots.as_array().unwrap())
			{
				// Player takes from the result slot
				if player.cursor_item.is_none() {
					player.cursor_item = Some(Slot {
						count: recipe.get_result_count(),
						id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
						components_to_add: Vec::new(),
						components_to_remove: Vec::new(),
					});
				} else if player.cursor_item.as_ref().unwrap().id == data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()) {
					player.cursor_item = Some(Slot {
						count: player.cursor_item.as_ref().unwrap().count + recipe.get_result_count(),
						id: player.cursor_item.as_ref().unwrap().id,
						components_to_add: Vec::new(),
						components_to_remove: Vec::new(),
					});
				}
				player.set_inventory_and_inform_client(
					player
						.get_inventory()
						.iter()
						.enumerate()
						.map(|(i, x)| {
							if ![1, 2, 3, 4].contains(&i) {
								return x.clone();
							}
							if let Some(mut slot) = x.clone() {
								slot.count -= 1;
								return Some(slot.clone());
							}
							x.clone()
						})
						.collect(),
					players_clone,
					game.clone(),
				);

				game.send_packet(
					&player.peer_socket_address,
					lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
					lib::packets::clientbound::play::SetContainerSlot {
						window_id: 0,
						state_id: 1,
						slot: 0,
						slot_data: Some(Slot {
							count: recipe.get_result_count(),
							id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
							components_to_add: Vec::new(),
							components_to_remove: Vec::new(),
						}),
					}
					.try_into()
					.unwrap(),
				);
			}
		} else {
			println!("player doesn't seem to have a container opened at the moment");
		}
		return;
	};

	let mut dimensions = std::mem::take(&mut game.world.lock().unwrap().dimensions);

	let player_currently_interacts_with_crafting_table = dimensions
		.get("minecraft:overworld")
		.unwrap()
		.get_block(position)
		.is_ok_and(|x| game.block_state_data.get("minecraft:crafting_table").unwrap().states.first().unwrap().id == x);

	let streams_with_container_opened = players_clone
		.iter()
		.filter(|x| x.opened_inventory_at.is_some_and(|x| x == position))
		.map(|x| x.connection_stream.try_clone().unwrap())
		.collect::<Vec<TcpStream>>();

	let block_entity =
		dimensions.get_mut("minecraft:overworld").unwrap().get_chunk_from_position_mut(position).unwrap().try_get_block_entity_mut(position);

	if let Some(mut block_entity) = block_entity {
		match &mut block_entity {
			BlockEntity::Barrel(barrel) => {
				let items = barrel.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 27);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::BlastFurnace(blast_furnace) => {
				let items = blast_furnace.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 3);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
				block_entity.set_needs_ticking(true);
			}
			BlockEntity::BrewingStand(brewing_stand) => {
				let items = brewing_stand.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 5);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Chest(chest) => {
				let items = chest.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 27);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Crafter(crafter) => {
				let items = crafter.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 9);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Dispenser(dispenser) => {
				let items = dispenser.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 9);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Dropper(dropper) => {
				let items = dropper.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 9);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Furnace(furnace) => {
				let items = furnace.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 3);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
				block_entity.set_needs_ticking(true);
			}
			BlockEntity::Hopper(hopper) => {
				let items = hopper.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 5);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::ShulkerBox(shulker_box) => {
				let items = shulker_box.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 27);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			BlockEntity::Smoker(smoker) => {
				let items = smoker.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 3);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
				block_entity.set_needs_ticking(true);
			}
			BlockEntity::TrappedChest(trapped_chest) => {
				let items = trapped_chest.get_contained_items_mut();
				assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
				assert!(items.len() == 27);
				lib::containerclick::handle(
					parsed_packet,
					items,
					player_uuid,
					game.clone(),
					streams_with_container_opened,
					&mut players,
					players_clone,
				);
			}
			x => println!("can't handle click_container packet for entity {x:?}"),
		}
	} else if player_currently_interacts_with_crafting_table {
		let mut items = player.crafting_table_slots.clone().to_vec();
		items.insert(0, Slot::default()); //need to append empty result slot

		let _ = player;
		//no need to handle when taking from result slot as that gets handled further down
		if parsed_packet.slot != 0 {
			lib::containerclick::handle(
				parsed_packet.clone(),
				&mut items,
				player_uuid,
				game.clone(),
				streams_with_container_opened,
				&mut players,
				players_clone,
			);
		}
		items.remove(0); //remove empty result slot again

		let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
		player.crafting_table_slots = items.as_array().unwrap().clone();

		//crafting in a crafting table
		let crafting_slots = player.crafting_table_slots.to_vec();
		if [1, 2, 3, 4, 5, 6, 7, 8, 9, 9].contains(&parsed_packet.slot) {
			// Player tries to craft in the crafting table
			if let Some(recipe) = game.recipe_manager.get_crafting_recipe_3x3(
				crafting_slots.into_iter().map(|x| if x.count == 0 { None } else { Some(x) }).collect::<Vec<Option<Slot>>>().as_array().unwrap(),
			) {
				game.send_packet(
					&player.peer_socket_address,
					lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
					lib::packets::clientbound::play::SetContainerSlot {
						window_id: 1,
						state_id: 1,
						slot: 0,
						slot_data: Some(Slot {
							count: recipe.get_result_count(),
							id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
							components_to_add: Vec::new(),
							components_to_remove: Vec::new(),
						}),
					}
					.try_into()
					.unwrap(),
				);
			} else {
				game.send_packet(
					&player.peer_socket_address,
					lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
					lib::packets::clientbound::play::SetContainerSlot {
						window_id: 1,
						state_id: 1,
						slot: 0,
						slot_data: None,
					}
					.try_into()
					.unwrap(),
				);
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
					id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				});
			} else if player.cursor_item.as_ref().unwrap().id == data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()) {
				player.cursor_item = Some(Slot {
					count: player.cursor_item.as_ref().unwrap().count + recipe.get_result_count(),
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

			game.send_packet(
				&player.peer_socket_address,
				lib::packets::clientbound::play::SetContainerSlot::PACKET_ID,
				lib::packets::clientbound::play::SetContainerSlot {
					window_id: 1,
					state_id: 1,
					slot: 0,
					slot_data: Some(Slot {
						count: recipe.get_result_count(),
						id: data::items::get_item_id_by_name(recipe.get_result_item_id().unwrap()),
						components_to_add: Vec::new(),
						components_to_remove: Vec::new(),
					}),
				}
				.try_into()
				.unwrap(),
			);
		}
	}

	game.world.lock().unwrap().dimensions = dimensions;
}
