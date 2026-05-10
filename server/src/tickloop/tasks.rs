use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();

	for task in game.task_queue.iter() {
		match task.clone() {
			Task::PlayerChangeDimension(uuid, new_dimension_name, new_position) => {
				let player = players.iter_mut().find(|x| x.uuid == uuid).unwrap();
				let dimension = world.dimensions.get_mut(&new_dimension_name).unwrap();

				player.change_dimension(&new_dimension_name, players_clone, dimension, &game.packet_sender, new_position);
			}
		}
	}

	game.task_queue.clear();
}
