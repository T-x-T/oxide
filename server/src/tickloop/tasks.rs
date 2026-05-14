use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();

	for task in game.task_queue.iter() {
		match task.clone() {
			Task::PlayerChangeDimension(uuid, new_dimension_name) => {
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
					player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, new_position);
				}
			}
		}
	}

	game.task_queue.clear();
}
