use lib::packets::serverbound::play::ClickContainer;

use super::*;

pub fn process(peer_addr: SocketAddr, parsed_packet: ClickContainer, game: Arc<Game>) {
	println!("{parsed_packet:?}");
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

	let Some(position) = player.opened_inventory_at else {
		if player.get_gamemode() != Gamemode::Creative && parsed_packet.window_id == 0 {
			let mut inventory: Vec<Item> =
				player.get_inventory().clone().into_iter().map(|x| x.map(Item::from)).map(|x| x.unwrap_or_default()).collect();

			let player_uuid = player.uuid;
			//need to drop lock on players here, because lib::containerclick::handle also locks players
			drop(players);

			lib::containerclick::handle(parsed_packet, &mut inventory, player_uuid, game.clone(), Vec::new());

			let mut players = game.players.lock().unwrap();
			let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();


			let inventory_to_set: Vec<Option<Slot>> =
				inventory.into_iter().map(|x| if x.count == 0 { None } else { Some(Slot::from(x)) }).collect();
			player.set_inventory_and_dont_inform_client(inventory_to_set);
		} else {
			println!("player doesn't seem to have a container opened at the moment");
		}
		return;
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
