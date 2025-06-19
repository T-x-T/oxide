pub mod vanilla;

use super::*;

pub trait WorldLoader {
	fn load_chunk(&self, x: i32, z: i32) -> super::Chunk;
	fn is_initialized(&self) -> bool;
}
