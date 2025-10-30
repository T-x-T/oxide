pub mod vanilla;

use std::sync::atomic::AtomicI32;

use super::*;

pub trait InnerWorldLoader {}

pub trait WorldLoader: InnerWorldLoader + std::fmt::Debug + Send {
	fn load_chunk(&self, x: i32, z: i32) -> super::Chunk;
	fn load_entities_in_chunk(&self, x: i32, z: i32, last_created_entity_id: &AtomicI32) -> Vec<Box<dyn SaveableEntity + Send>>;
	fn is_initialized(&self) -> bool;
	fn save_to_disk(&self, chunks: &[Chunk], default_spawn_location: BlockPosition, dimension: &Dimension);
	fn get_default_spawn_location(&self) -> BlockPosition;
}
