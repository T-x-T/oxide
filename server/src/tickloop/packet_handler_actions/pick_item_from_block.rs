use super::*;

pub fn process(peer_addr: SocketAddr, location: BlockPosition, game: Arc<Game>, players_clone: &[Player]) {
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

	if player.get_gamemode() != Gamemode::Creative {
		return;
	}

	let picked_block = game.world.lock().unwrap().dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();
	let picked_block_name = game.block_state_data.iter().find(|x| x.1.states.iter().any(|x| x.id == picked_block)).unwrap().0.clone();
	let item_id = data::items::get_items()
		.get(picked_block_name.as_str())
		.unwrap_or(&data::items::Item {
			max_stack_size: 0,
			rarity: data::items::ItemRarity::Common,
			id: 0,
			repair_cost: 0,
			tool_rules: Vec::new(),
		})
		.id;

	let new_slot_data = Slot {
		item_count: 1,
		item_id,
		components_to_add: Vec::new(),
		components_to_remove: Vec::new(),
	};

	player.set_selected_inventory_slot(Some(new_slot_data), players_clone, game.clone());
}
