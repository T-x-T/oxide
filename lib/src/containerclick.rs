use std::net::SocketAddr;

use crate::types::*;

pub fn handle(
	parsed_packet: &crate::packets::serverbound::play::ClickContainer,
	chest_items: &mut [Slot],
	player_uuid: u128,
	peers_with_container_opened: Vec<SocketAddr>,
	players: &mut [Player],
	players_clone: &[Player],
	packet_sender: &PacketSender,
) {
	const PLAYER_INVENTORY_STARTING_INDEX: i16 = 9;
	let player_inventory_index = parsed_packet.slot - chest_items.len() as i16 + PLAYER_INVENTORY_STARTING_INDEX;

	let outside_clicked = parsed_packet.slot < 0;
	let chest_inventory_clicked = parsed_packet.slot < chest_items.len() as i16;
	let orig_inventory_item: Slot = if outside_clicked {
		Slot::default()
	} else if chest_inventory_clicked {
		chest_items[parsed_packet.slot as usize].clone()
	} else {
		players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().get_inventory()[player_inventory_index as usize]
			.clone()
	};

	//println!("orig item: {orig_inventory_item:?}");
	let orig_cursor_item: Slot = players.iter().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().cursor_item.clone();
	//println!("orig cursor: {orig_cursor_item:?}");

	if parsed_packet.mode == 0 {
		let (new_inventory_item, new_cursor_item) =
			handle_click(parsed_packet.button == 0, orig_inventory_item.clone(), orig_cursor_item.clone());

		//println!("new item: {new_inventory_item:?}; new cursor item: {new_cursor_item:?}");
		if new_inventory_item != orig_inventory_item {
			if outside_clicked {
				//nothing to do
			} else if chest_inventory_clicked {
				//Chest inventory got changed
				chest_items[parsed_packet.slot as usize] = new_inventory_item.clone();
				for peer in peers_with_container_opened {
					players.iter_mut().find(|x| x.peer_socket_address == peer).unwrap().send_container_slot_data(
						packet_sender,
						parsed_packet.slot,
						new_inventory_item.clone(),
					);
				}
			} else {
				//Player inventory got changed
				players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().set_inventory_slot(
					player_inventory_index as u8,
					new_inventory_item,
					players_clone,
					packet_sender,
				);
			}
		}

		//println!("new cursor: {new_cursor_item:?}");
		if new_cursor_item != orig_cursor_item {
			players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().cursor_item = new_cursor_item;
		}
	} else if parsed_packet.mode == 1 {
		if parsed_packet.window_id == 0 {
			println!("no shift clicking in your inventory!");
			let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
			player.send_container_content_data(packet_sender, 0, player.get_inventory().clone(), orig_cursor_item.clone());
		} else {
			let orig_chest_inventory: Vec<Slot> = chest_items.to_vec();
			let orig_player_inventory: Vec<Slot> =
				players.iter().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().get_inventory().clone();
			let (new_chest_inventory, new_player_inventory) =
				handle_shift_click(orig_chest_inventory.clone(), orig_player_inventory.clone(), parsed_packet.slot);

			if orig_chest_inventory != new_chest_inventory {
				let new_chest_items: Vec<Slot> = new_chest_inventory;
				assert_eq!(chest_items.len(), new_chest_items.len());
				chest_items.clone_from_slice(&new_chest_items);

				for peer in peers_with_container_opened {
					players.iter_mut().find(|x| x.peer_socket_address == peer).unwrap().send_container_content_data(
						packet_sender,
						1,
						chest_items.to_vec(),
						Slot::default(),
					);
				}
			}

			if orig_player_inventory != new_player_inventory {
				players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap().set_inventory_and_inform_client(
					new_player_inventory,
					players_clone,
					packet_sender,
				);
			}
		}
	} else {
		//this just resets the inventory of the container and player if we dont know how to handle the used mode
		let player = players.iter_mut().find(|x| x.get_common_entity_data().uuid == player_uuid).unwrap();
		player.send_container_content_data(packet_sender, 1, chest_items.to_vec(), orig_cursor_item.clone());
		player.send_container_content_data(packet_sender, 0, player.get_inventory().clone(), orig_cursor_item);
	}
}

fn handle_click(left_click: bool, orig_inventory_item: Slot, orig_cursor_item: Slot) -> (Slot, Slot) {
	let mut new_inventory_item: Slot = Slot::default();
	let mut new_cursor_item: Slot = Slot::default();

	if !orig_inventory_item.is_empty() {
		//Slot has items
		if !orig_cursor_item.is_empty() {
			//Swap items or stack up?
			if orig_cursor_item.get_item_id() == orig_inventory_item.get_item_id() {
				//stack up
				let item_count_cursor = orig_cursor_item.get_count();
				let item_count_chest = orig_inventory_item.get_count();
				let max_stack_size = orig_inventory_item.get_max_stack_size();

				if left_click {
					//put all down
					let left_over_item_count = (item_count_chest + item_count_cursor).saturating_sub(max_stack_size);

					if left_over_item_count > 0 {
						new_inventory_item = Slot::new(orig_cursor_item.get_item_id(), max_stack_size);
					} else {
						new_inventory_item = Slot::new(orig_cursor_item.get_item_id(), item_count_chest + item_count_cursor);
					}
					new_cursor_item = Slot::new(orig_cursor_item.get_item_id(), left_over_item_count);
				} else {
					//put one down
					if orig_inventory_item.get_count() < max_stack_size {
						new_inventory_item = Slot::new(orig_inventory_item.get_item_id(), orig_inventory_item.get_count() + 1);
						new_cursor_item = Slot::new(orig_cursor_item.get_item_id(), orig_cursor_item.get_count() - 1);
					} else {
						new_inventory_item = orig_inventory_item;
						new_cursor_item = orig_cursor_item;
					}
				}
			} else {
				//swap
				new_cursor_item = orig_inventory_item;
				new_inventory_item = orig_cursor_item;
			}
		} else {
			//can just take item from slot
			if left_click || orig_inventory_item.get_count() < 2 {
				new_cursor_item = orig_inventory_item;
				new_inventory_item = Slot::default();
			} else {
				new_inventory_item = Slot::new(orig_inventory_item.get_item_id(), orig_inventory_item.get_count() / 2);
				new_cursor_item = Slot::new(orig_inventory_item.get_item_id(), (f64::from(orig_inventory_item.get_count()) / 2.0).ceil() as u8);
			}
		}
	} else {
		//Slot doesnt have items
		if !orig_cursor_item.is_empty() {
			if left_click {
				new_inventory_item = orig_cursor_item;
				new_cursor_item = Slot::default();
			} else {
				new_inventory_item = Slot::new(orig_cursor_item.get_item_id(), 1);
				new_cursor_item = Slot::new(orig_cursor_item.get_item_id(), orig_cursor_item.get_count() - 1);
			}
		}
	}

	return (new_inventory_item, new_cursor_item);
}

fn handle_shift_click(orig_chest_inventory: Vec<Slot>, orig_player_inventory: Vec<Slot>, clicked_slot: i16) -> (Vec<Slot>, Vec<Slot>) {
	const PLAYER_INVENTORY_STARTING_INDEX: i16 = 9;
	let player_inventory_index = (clicked_slot - orig_chest_inventory.len() as i16 + PLAYER_INVENTORY_STARTING_INDEX) as usize;
	let clicked_slot = clicked_slot as usize;

	let mut new_chest_inventory = orig_chest_inventory.clone();
	let mut new_player_inventory = orig_player_inventory.clone();

	if clicked_slot > orig_chest_inventory.len() {
		//player inventory clicked
		if orig_player_inventory[player_inventory_index].is_empty() {
			return (new_chest_inventory, new_player_inventory);
		}
		let max_stack_size = orig_player_inventory[player_inventory_index].get_max_stack_size();

		orig_chest_inventory
			.iter()
			.enumerate()
			.filter(|x| !x.1.is_empty() && x.1.get_item_id() == orig_player_inventory[player_inventory_index].get_item_id())
			.for_each(|(i, slot)| {
				if !new_player_inventory[player_inventory_index].is_empty() {
					let left_over_count = max_stack_size - slot.get_count();
					if left_over_count >= new_player_inventory[player_inventory_index].get_count() {
						new_chest_inventory[i] =
							Slot::new(orig_chest_inventory[i].get_item_id(), slot.get_count() + new_player_inventory[player_inventory_index].get_count());
						new_player_inventory[player_inventory_index] = Slot::default();
					} else {
						new_chest_inventory[i] = Slot::new(orig_chest_inventory[i].get_item_id(), max_stack_size);
						new_player_inventory[player_inventory_index] = Slot::new(
							new_player_inventory[player_inventory_index].get_item_id(),
							(slot.get_count() + new_player_inventory[player_inventory_index].get_count()) - max_stack_size,
						);
					}
				}
			});

		if new_player_inventory[player_inventory_index].is_empty() {
			return (new_chest_inventory, new_player_inventory);
		}

		let first_free_slot = orig_chest_inventory.iter().enumerate().find(|x| x.1.is_empty()).map(|x| x.0);
		if let Some(first_free_slot) = first_free_slot {
			new_chest_inventory[first_free_slot] = new_player_inventory[player_inventory_index].clone();
			new_player_inventory[player_inventory_index] = Slot::default();
		}
	} else {
		//chest inventory clicked
		if orig_chest_inventory[clicked_slot].is_empty() {
			return (new_chest_inventory, new_player_inventory);
		}

		let max_stack_size = orig_chest_inventory[clicked_slot].get_max_stack_size();

		orig_player_inventory
			.iter()
			.enumerate()
			.filter(|x| !x.1.is_empty() && x.1.get_item_id() == orig_chest_inventory[clicked_slot].get_item_id())
			.for_each(|(i, slot)| {
				if !new_chest_inventory[clicked_slot].is_empty() {
					let left_over_count = max_stack_size - slot.get_count();
					if left_over_count >= new_chest_inventory[clicked_slot].get_count() {
						new_player_inventory[i] = Slot::new(slot.get_item_id(), slot.get_count() + new_chest_inventory[clicked_slot].get_count());
						new_chest_inventory[clicked_slot] = Slot::default();
					} else {
						new_player_inventory[i] = Slot::new(new_player_inventory[i].get_item_id(), max_stack_size);
						new_chest_inventory[clicked_slot] = Slot::new(
							new_chest_inventory[clicked_slot].get_item_id(),
							(slot.get_count() + new_chest_inventory[clicked_slot].get_count()) - max_stack_size,
						);
					}
				}
			});

		if new_chest_inventory[clicked_slot].is_empty() {
			return (new_chest_inventory, new_player_inventory);
		}

		let slot_order: Vec<usize> = vec![(36..=44).rev().map(|x| x as usize).collect::<Vec<usize>>(), (9..=35).map(|x| x as usize).collect()]
			.into_iter()
			.flatten()
			.collect();
		let first_free_slot = slot_order.into_iter().find(|x| new_player_inventory[*x].is_empty());
		if let Some(first_free_slot) = first_free_slot {
			new_player_inventory[first_free_slot] = new_chest_inventory[clicked_slot].clone();
			new_chest_inventory[clicked_slot] = Slot::default();
		}
	}

	return (new_chest_inventory, new_player_inventory);
}

#[cfg(test)]
mod test {
	use super::*;

	mod handle_click {
		use super::*;

		#[test]
		fn pick_item_up() {
			let orig_inventory_item = Slot::new(1, 1);
			let orig_cursor_item = Slot::default();

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, orig_cursor_item);
			assert_eq!(new_cursor_item, orig_inventory_item);
		}

		#[test]
		fn pick_empty_item_up() {
			let orig_inventory_item = Slot::default();
			let orig_cursor_item = Slot::default();

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, orig_inventory_item);
			assert_eq!(new_cursor_item, orig_cursor_item);
		}

		#[test]
		fn put_item_down() {
			let orig_inventory_item = Slot::default();
			let orig_cursor_item = Slot::new(1, 1);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, orig_cursor_item);
			assert_eq!(new_cursor_item, orig_inventory_item);
		}

		#[test]
		fn put_item_down_on_full_stack() {
			let orig_inventory_item = Slot::new(1, 64);
			let orig_cursor_item = Slot::new(1, 1);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, orig_inventory_item);
			assert_eq!(new_cursor_item, orig_cursor_item);
		}

		#[test]
		fn swap_different_items() {
			let orig_inventory_item = Slot::new(2, 12);
			let orig_cursor_item = Slot::new(1, 1);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, orig_cursor_item);
			assert_eq!(new_cursor_item, orig_inventory_item);
		}

		#[test]
		fn stack_up_under_limit() {
			let orig_inventory_item = Slot::new(1, 12);
			let orig_cursor_item = Slot::new(1, 1);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 13));
			assert!(new_cursor_item.is_empty());
		}

		#[test]
		fn stack_up_to_limit() {
			let orig_inventory_item = Slot::new(1, 12);
			let orig_cursor_item = Slot::new(1, 52);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 64));
			assert!(new_cursor_item.is_empty());
		}

		#[test]
		fn stack_up_over_limit() {
			let orig_inventory_item = Slot::new(1, 22);
			let orig_cursor_item = Slot::new(1, 52);

			let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 64));
			assert_eq!(new_cursor_item, Slot::new(1, 10));
		}

		#[test]
		fn pick_half_stack_up_even() {
			let orig_inventory_item = Slot::new(1, 10);
			let orig_cursor_item = Slot::default();

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 5));
			assert_eq!(new_cursor_item, Slot::new(1, 5));
		}

		#[test]
		fn pick_half_stack_up_odd() {
			let orig_inventory_item = Slot::new(1, 11);
			let orig_cursor_item = Slot::default();

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 5));
			assert_eq!(new_cursor_item, Slot::new(1, 6));
		}

		#[test]
		fn pick_half_stack_up_one() {
			let orig_inventory_item = Slot::new(1, 1);
			let orig_cursor_item = Slot::default();

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::default());
			assert_eq!(new_cursor_item, Slot::new(1, 1));
		}

		#[test]
		fn put_one_item_down() {
			let orig_inventory_item = Slot::default();
			let orig_cursor_item = Slot::new(1, 10);

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 1));
			assert_eq!(new_cursor_item, Slot::new(1, 9));
		}

		#[test]
		fn put_one_item_down_with_same_already_there() {
			let orig_inventory_item = Slot::new(1, 10);
			let orig_cursor_item = Slot::new(1, 10);

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 11));
			assert_eq!(new_cursor_item, Slot::new(1, 9));
		}

		#[test]
		fn put_one_item_down_with_stack_full() {
			let orig_inventory_item = Slot::new(1, 64);
			let orig_cursor_item = Slot::new(1, 10);

			let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

			assert_eq!(new_inventory_item, Slot::new(1, 64));
			assert_eq!(new_cursor_item, Slot::new(1, 10));
		}
	}

	mod handle_shift_click {
		use super::*;

		#[test]
		fn shift_click_on_empty_player_slot() {
			let orig_chest_inventory: Vec<Slot> = vec![Slot::default(); 27];
			let orig_player_inventory: Vec<Slot> = vec![Slot::default(); 46];
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);
			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_on_empty_chest_slot() {
			let orig_chest_inventory: Vec<Slot> = vec![Slot::default(); 27];
			let orig_player_inventory: Vec<Slot> = vec![Slot::default(); 46];
			let clicked_slot = 1;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);
			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_empty_chest() {
			let orig_chest_inventory: Vec<Slot> = vec![Slot::default(); 27];
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26],].into_iter().flatten().collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items() {
			let orig_chest_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 12)], vec![Slot::default(); 2], vec![Slot::new(2, 12)], vec![Slot::default(); 23]]
					.into_iter()
					.flatten()
					.collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(3, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 12)], vec![Slot::new(3, 12)], vec![Slot::default()], vec![Slot::new(2, 12)], vec![Slot::default(); 23],]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items_stack_up() {
			let orig_chest_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 12)], vec![Slot::default(); 2], vec![Slot::new(2, 12)], vec![Slot::default(); 23]]
					.into_iter()
					.flatten()
					.collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 24)], vec![Slot::default(); 2], vec![Slot::new(2, 12)], vec![Slot::default(); 23],]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items_stack_up_first_stack() {
			let orig_chest_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 12)], vec![Slot::default(); 2], vec![Slot::new(1, 12)], vec![Slot::default(); 23]]
					.into_iter()
					.flatten()
					.collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 24)], vec![Slot::default(); 2], vec![Slot::new(1, 12)], vec![Slot::default(); 23],]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items_stack_up_multiple() {
			let orig_chest_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 60)], vec![Slot::default(); 2], vec![Slot::new(1, 12)], vec![Slot::default(); 23]]
					.into_iter()
					.flatten()
					.collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 64)], vec![Slot::default(); 2], vec![Slot::new(1, 20)], vec![Slot::default(); 23],]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items_stack_up_lots() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 63); 20], vec![Slot::default(); 7]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 64); 12], vec![Slot::new(1, 63); 8], vec![Slot::default(); 7],].into_iter().flatten().collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_with_items_stack_up_lots_overflow() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 63); 20], vec![Slot::default(); 7]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(1, 24)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(
				new_chest_inventory,
				vec![vec![Slot::new(1, 64); 20], vec![Slot::new(1, 4)], vec![Slot::default(); 6],].into_iter().flatten().collect::<Vec<Slot>>()
			);

			assert_eq!(new_player_inventory, vec![Slot::default(); 46]);
		}

		#[test]
		fn shift_click_to_chest_full() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12); 27]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 36], vec![Slot::new(3, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect();
			let clicked_slot = 54;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![vec![Slot::new(1, 12); 27],].into_iter().flatten().collect::<Vec<Slot>>());

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::default(); 36], vec![Slot::new(3, 12)], vec![Slot::default(); 9]].into_iter().flatten().collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_empty_inventory() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();

			let orig_player_inventory: Vec<Slot> = vec![Slot::default(); 46];
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::default(); 44], vec![Slot::new(1, 12)], vec![Slot::default(); 1],].into_iter().flatten().collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_items() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();

			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 43], vec![Slot::new(2, 12)], vec![Slot::new(3, 12)], vec![Slot::default(); 1]]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::default(); 42], vec![Slot::new(1, 12)], vec![Slot::new(2, 12)], vec![Slot::new(3, 12)], vec![Slot::default(); 1],]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_full_hotbar() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();

			let orig_player_inventory: Vec<Slot> = vec![
				vec![Slot::default(); 9],
				vec![Slot::new(2, 12)],
				vec![Slot::default(); 26],
				vec![Slot::new(3, 12); 9],
				vec![Slot::default(); 1],
			]
			.into_iter()
			.flatten()
			.collect::<Vec<Slot>>();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![
					vec![Slot::default(); 9],
					vec![Slot::new(2, 12)],
					vec![Slot::new(1, 12)],
					vec![Slot::default(); 25],
					vec![Slot::new(3, 12); 9],
					vec![Slot::default(); 1],
				]
				.into_iter()
				.flatten()
				.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_full() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();

			let orig_player_inventory: Vec<Slot> = vec![vec![Slot::new(2, 12); 46]].into_iter().flatten().collect::<Vec<Slot>>();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect::<Vec<Slot>>());

			assert_eq!(new_player_inventory, vec![vec![Slot::new(2, 12); 46],].into_iter().flatten().collect::<Vec<Slot>>());
		}

		#[test]
		fn shift_click_to_inventory_with_items_stack_up() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::new(2, 12)], vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::new(2, 12)], vec![Slot::default(); 36], vec![Slot::new(1, 24)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_items_stack_up_first_stack() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 12)], vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::new(1, 24)], vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_items_stack_up_multiple() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 12)], vec![Slot::default(); 26]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 60)], vec![Slot::default(); 36], vec![Slot::new(1, 12)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![vec![Slot::new(1, 64)], vec![Slot::default(); 36], vec![Slot::new(1, 20)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_items_stack_up_lots() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 10)], vec![Slot::default(); 26]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::new(1, 63); 20], vec![Slot::default(); 16], vec![Slot::new(1, 12)], vec![Slot::default(); 9]]
					.into_iter()
					.flatten()
					.collect();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![
					vec![Slot::new(1, 64); 10],
					vec![Slot::new(1, 63); 10],
					vec![Slot::default(); 16],
					vec![Slot::new(1, 12)],
					vec![Slot::default(); 9]
				]
				.into_iter()
				.flatten()
				.collect::<Vec<Slot>>()
			);
		}

		#[test]
		fn shift_click_to_inventory_with_items_stack_up_lots_with_overflow() {
			let orig_chest_inventory: Vec<Slot> = vec![vec![Slot::new(1, 24)], vec![Slot::default(); 26]].into_iter().flatten().collect();
			let orig_player_inventory: Vec<Slot> =
				vec![vec![Slot::default(); 9], vec![Slot::new(1, 63); 20], vec![Slot::default(); 17]].into_iter().flatten().collect();
			let clicked_slot = 0;

			let (new_chest_inventory, new_player_inventory) =
				handle_shift_click(orig_chest_inventory, orig_player_inventory.clone(), clicked_slot);

			assert_eq!(new_chest_inventory, vec![Slot::default(); 27]);

			assert_eq!(
				new_player_inventory,
				vec![
					vec![Slot::default(); 9],
					vec![Slot::new(1, 64); 20],
					vec![Slot::default(); 15],
					vec![Slot::new(1, 4)],
					vec![Slot::default(); 1],
				]
				.into_iter()
				.flatten()
				.collect::<Vec<Slot>>()
			);
		}
	}
}
