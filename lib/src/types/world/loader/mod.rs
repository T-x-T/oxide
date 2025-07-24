pub mod vanilla;

use super::*;

pub trait InnerWorldLoader {}

pub trait WorldLoader: InnerWorldLoader + std::fmt::Debug + Send {
	fn load_chunk(&self, x: i32, z: i32) -> super::Chunk;
	fn is_initialized(&self) -> bool;
	fn save_to_disk(&self, chunks: &[Chunk], default_spawn_location: Position);
	fn get_default_spawn_location(&self) -> Position;
}
