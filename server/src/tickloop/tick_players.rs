use super::*;

pub fn process(game: Arc<Game>, players: &[Player]) {
	let entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = game
		.players
		.lock()
		.unwrap()
		.iter_mut()
		.map(|player| {
			(player.entity_id, player.tick(game.world.lock().unwrap().dimensions.get("minecraft:overworld").unwrap(), players, game.clone()))
		})
		.collect();

	process_entity_tick_outcome::process(
		entity_tick_outcomes,
		game.clone(),
		players,
		game.world.lock().unwrap().dimensions.get_mut("minecraft:overworld").unwrap(),
	);
}
