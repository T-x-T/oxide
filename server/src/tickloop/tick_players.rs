use super::*;

pub fn process(game: Arc<Game>, players: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let dimension = world.dimensions.get_mut("minecraft:overworld").unwrap();
	let entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = game
		.players
		.lock()
		.unwrap()
		.iter_mut()
		.flat_map(|player| {
			let outcomes = player.tick(dimension, players, &game.packet_sender, &game.entity_id_manager, &game.block_state_data);
			let mut output: Vec<(i32, EntityTickOutcome)> = Vec::new();
			for outcome in outcomes {
				output.push((player.entity_id, outcome));
			}
			output
		})
		.collect();

	process_entity_tick_outcome::process(entity_tick_outcomes, game.clone(), players, dimension);
}
