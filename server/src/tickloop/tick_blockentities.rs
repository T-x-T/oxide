use super::*;

pub fn process(game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	for dimension in &mut game.world.lock().unwrap().dimensions {
		for chunk in &mut dimension.1.chunks.values_mut() {
			if chunk.keep_loaded_for_ticks <= 0 {
				continue;
			}
			for blockentity in &mut chunk.block_entities {
				if blockentity.get_needs_ticking() {
					blockentity.tick(&mut players, &dimension.1.name, &game.packet_sender, &game.recipe_manager);
				}
			}
		}
	}
}
