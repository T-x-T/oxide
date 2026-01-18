use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
	for dimension in &mut game.world.lock().unwrap().dimensions {
		for chunk in &mut dimension.1.chunks.values_mut() {
			for blockentity in &mut chunk.block_entities {
				if blockentity.get_needs_ticking() {
					blockentity.tick(players_clone, game.clone());
				}
			}
		}
	}
}
