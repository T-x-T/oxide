use super::*;

pub fn process(
	peer_addr: SocketAddr,
	position: Option<(f64, f64, f64)>,
	rotation: Option<(f32, f32)>,
	game: Arc<Game>,
	player_clone: &[Player],
) {
	let mut world = game.world.lock().unwrap();


	let mut players = game.players.lock().unwrap();
	let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == peer_addr) else {
		return;
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

	for other_player in player_clone.iter() {
		if other_player.peer_socket_address != peer_addr {
			for packet in &packets {
				game.send_packet(&other_player.peer_socket_address, packet.0, packet.1.clone());
			}
		}
	}
}
