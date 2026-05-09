pub mod vanilla;

use basic_types::blocks::Block;

use super::*;

pub trait InnerWorldLoader {}

pub trait WorldLoader: InnerWorldLoader + std::fmt::Debug + Send {
	fn load_chunk(&self, x: i32, z: i32, block_states: &HashMap<String, Block>, dimension_name: &str) -> super::Chunk;
	fn load_entities_in_chunk(&self, x: i32, z: i32, entity_id_manager: &EntityIdManager, dimension_name: &str) -> Vec<Entity>;
	fn is_initialized(&self) -> bool;
	fn save_to_disk(
		&self,
		chunks: &HashMap<(i32, i32), Chunk>,
		default_spawn_location: BlockPosition,
		dimension: &Dimension,
		block_states: &HashMap<String, Block>,
		dimension_name: &str,
	);
	fn get_default_spawn_location(&self) -> BlockPosition;
}
