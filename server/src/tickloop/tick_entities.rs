use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	for dimension in &mut game.world.lock().unwrap().dimensions {
		let mut entities = std::mem::take(&mut dimension.1.entities);
		let mut entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = Vec::new();
		for entity in &mut entities {
			let outcome = entity.tick(dimension.1, players_clone, game.clone());
			//println!("ticked entity in {:.2?}", std::time::Instant::now() - now);
			if outcome != EntityTickOutcome::None {
				entity_tick_outcomes.push((entity.get_common_entity_data().entity_id, outcome));
			}
		}
		dimension.1.entities = entities;
		super::process_entity_tick_outcome::process(entity_tick_outcomes, game.clone(), players_clone, dimension.1);
	}
}
