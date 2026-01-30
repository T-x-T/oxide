use super::*;

mod break_block;
mod click_container;
mod close_container;
mod interact;
mod move_player;
mod new_player;
mod pick_item_from_block;
mod send_chat_message;
mod send_command;
mod update_sign;
mod use_item_on;

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
				if moved_players.contains(&peer_addr) {
					continue;
				}

				moved_players.push(peer_addr);

				move_player::process(peer_addr, position, rotation, game.clone(), players_clone);
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
				pick_item_from_block::process(peer_addr, location, game.clone(), players_clone);
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
			PacketHandlerAction::BreakBlock(peer_addr, status, location, _face, sequence_id) => {
				break_block::process(peer_addr, status, location, sequence_id, game.clone());
			}
			PacketHandlerAction::UseItemOn(
				peer_addr,
				_hand,
				location,
				face,
				cursor_pos_x,
				cursor_pos_y,
				cursor_pos_z,
				_inside_block,
				_world_border_hit,
				sequence_id,
			) => {
				use_item_on::process(peer_addr, location, face, cursor_pos_x, cursor_pos_y, cursor_pos_z, sequence_id, game.clone(), players_clone);
			}
			PacketHandlerAction::SendChatMessage(peer_addr, message, timestamp, salt) => {
				send_chat_message::process(peer_addr, message, timestamp, salt, game.clone());
			}
			PacketHandlerAction::SendCommand(peer_addr, command_string) => {
				send_command::process(peer_addr, command_string, game.clone());
			}
			PacketHandlerAction::ClickContainer(peer_addr, parsed_packet) => {
				click_container::process(peer_addr, parsed_packet, game.clone());
			}
			PacketHandlerAction::CloseContainer(peer_addr, window_id) => {
				close_container::process(peer_addr, window_id, game.clone());
			}
			PacketHandlerAction::UpdateSign(location, _is_front, text) => {
				update_sign::process(location, text, game.clone());
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
				interact::process(peer_addr, parsed_packet, game.clone());
			}
			PacketHandlerAction::NewPlayer(peer_addr, stream) => {
				new_player::process(peer_addr, stream, game.clone());
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
