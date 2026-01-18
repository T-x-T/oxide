pub mod loader;

use data::blocks::Block;

use super::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use crate::SPAWN_CHUNK_RADIUS;
use crate::loader::WorldLoader;
use crate::types::position::BlockPosition;

pub struct World {
	pub dimensions: HashMap<String, Dimension>,
	pub loader: Box<dyn WorldLoader>,
	pub default_spawn_location: BlockPosition,
}

pub struct Dimension {
	pub chunks: HashMap<(i32, i32), Chunk>,
	pub entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Chunk {
	pub x: i32,
	pub z: i32,
	pub sections: Vec<ChunkSection>,
	pub inhabited_time: i64,
	pub last_update: i64,
	pub is_light_on: bool,
	pub modified: bool,
	pub block_entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
	pub blocks: Vec<u16>,
	pub biomes: Vec<u8>,
	pub sky_lights: Vec<u8>,
	pub block_lights: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockOverwriteOutcome {
	DestroyBlockentity,
}

impl World {
	#[allow(clippy::new_without_default)]
	pub fn new(loader: impl WorldLoader + 'static, entity_id_manager: &EntityIdManager, block_states: &HashMap<String, Block>) -> Self {
		let mut dimensions: HashMap<String, Dimension> = HashMap::new();
		let default_spawn_location: BlockPosition;
		if loader.is_initialized() {
			let now = std::time::Instant::now();
			println!("loading existing world");
			default_spawn_location = loader.get_default_spawn_location();
			dimensions.insert("minecraft:overworld".to_string(), Dimension::new_from_loader(&loader, entity_id_manager, block_states));
			println!("finished loading existing world in {:.2?}", now.elapsed());
		} else {
			println!("create new world");
			dimensions.insert("minecraft:overworld".to_string(), Dimension::new());
			default_spawn_location = BlockPosition {
				x: 0,
				y: -48,
				z: 0,
			};
			println!("creation of new world finished");
		}
		return Self {
			dimensions,
			loader: Box::new(loader),
			default_spawn_location,
		};
	}

	pub fn save_to_disk(&mut self, block_states: &HashMap<String, Block>) {
		self.dimensions.iter_mut().for_each(|x| x.1.save_to_disk(&*self.loader, self.default_spawn_location, block_states));
	}
}

impl Dimension {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

		for x in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
			for z in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
				chunks.insert((x as i32, z as i32), Chunk::new(x as i32, z as i32));
			}
		}

		return Self {
			chunks,
			entities: Vec::new(),
		};
	}

	pub fn new_from_loader(
		loader: &impl loader::WorldLoader,
		entity_id_manager: &EntityIdManager,
		block_states: &HashMap<String, Block>,
	) -> Self {
		let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new();
		let mut entities: Vec<Entity> = Vec::new();

		for x in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
			for z in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
				chunks.insert((x as i32, z as i32), loader.load_chunk(x as i32, z as i32, block_states));
				entities.append(&mut loader.load_entities_in_chunk(x as i32, z as i32, entity_id_manager));
			}
		}

		return Self {
			chunks,
			entities,
		};
	}

	pub fn get_chunk_from_position_mut(&mut self, position: BlockPosition) -> Option<&mut Chunk> {
		let chunk_coordinates = position.convert_to_coordinates_of_chunk();

		return self.chunks.get_mut(&(chunk_coordinates.x, chunk_coordinates.z));
	}

	pub fn get_chunk_from_position(&self, position: BlockPosition) -> Option<&Chunk> {
		let chunk_coordinates = position.convert_to_coordinates_of_chunk();

		return self.chunks.get(&(chunk_coordinates.x, chunk_coordinates.z));
	}

	pub fn get_chunk_from_chunk_position_mut(&mut self, chunk_coordinates: BlockPosition) -> Option<&mut Chunk> {
		return self.chunks.get_mut(&(chunk_coordinates.x, chunk_coordinates.z));
	}

	pub fn get_chunk_from_chunk_position(&self, chunk_coordinates: BlockPosition) -> Option<&Chunk> {
		return self.chunks.get(&(chunk_coordinates.x, chunk_coordinates.z));
	}

	pub fn overwrite_block(&mut self, position: BlockPosition, block_state_id: u16) -> Result<Option<BlockOverwriteOutcome>, Box<dyn Error>> {
		let chunk = self.get_chunk_from_position_mut(position);
		if chunk.is_none() {
			return Err(Box::new(crate::CustomError::ChunkNotFound(position)));
		}
		if position.y < -64 || position.y > 319 {
			return Err(Box::new(crate::CustomError::PositionOutOfBounds(position)));
		}

		return Ok(chunk.unwrap().set_block(position, block_state_id));
	}

	pub fn get_block(&self, position: BlockPosition) -> Result<u16, Box<dyn Error>> {
		let chunk = self.get_chunk_from_position(position);
		if chunk.is_none() {
			return Err(Box::new(crate::CustomError::ChunkNotFound(position)));
		}
		if position.y < -64 || position.y > 319 {
			return Err(Box::new(crate::CustomError::PositionOutOfBounds(position)));
		}

		let block_state_id = chunk.unwrap().get_block(position.convert_to_position_in_chunk());
		return Ok(block_state_id);
	}

	pub fn save_to_disk(
		&mut self,
		loader: &(impl WorldLoader + ?Sized),
		default_spawn_location: BlockPosition,
		block_states: &HashMap<String, Block>,
	) {
		{
			loader.save_to_disk(&self.chunks, default_spawn_location, self, block_states);
		}
		for chunk in &mut self.chunks {
			chunk.1.modified = false;
		}
	}

	#[allow(clippy::borrowed_box)]
	pub fn get_entities_in_chunk(&self, x: i32, z: i32) -> Vec<&Entity> {
		return self
			.entities
			.iter()
			.filter(|e| {
				let chunk_coords_of_entity = BlockPosition::from(e.get_common_entity_data().position).convert_to_coordinates_of_chunk();
				return chunk_coords_of_entity.x == x && chunk_coords_of_entity.z == z;
			})
			.collect();
	}

	pub fn add_entity(&mut self, entity: Entity) {
		self.get_chunk_from_position_mut(entity.get_common_entity_data().position.into()).unwrap().modified = true;
		self.entities.push(entity);
	}

	pub fn add_entities(&mut self, mut entities: Vec<Entity>) {
		for entity in &entities {
			self.get_chunk_from_position_mut(entity.get_common_entity_data().position.into()).unwrap().modified = true;
		}
		self.entities.append(&mut entities);
	}
}

impl Chunk {
	pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
		let filled_chunk_sections = vec![
			ChunkSection {
				blocks: vec![1; 4096],
				biomes: vec![40; 64],
				sky_lights: vec![0xFF; 2048],
				block_lights: vec![]
			};
			1
		];
		let empty_chunk_sections = vec![
			ChunkSection {
				blocks: vec![0; 4096],
				biomes: vec![40; 64],
				sky_lights: vec![0xFF; 2048],
				block_lights: vec![]
			};
			23
		];
		let mut all_chunk_sections = filled_chunk_sections.clone();
		all_chunk_sections.append(&mut empty_chunk_sections.clone());

		return Self {
			x: chunk_x,
			z: chunk_z,
			sections: all_chunk_sections,
			last_update: 0,
			inhabited_time: 0,
			is_light_on: true,
			modified: true,
			block_entities: Vec::new(),
		};
	}

	fn set_block(&mut self, position_global: BlockPosition, block_state_id: u16) -> Option<BlockOverwriteOutcome> {
		self.modified = true;
		let position_in_chunk = position_global.convert_to_position_in_chunk();
		let section_id = (position_in_chunk.y + 64) / 16;
		let block_id =
			position_in_chunk.x + (position_in_chunk.z * 16) + (((position_in_chunk.y as i32 + 64) - (section_id as i32 * 16)) * 256);
		if self.sections[section_id as usize].blocks.is_empty() {
			self.sections[section_id as usize].blocks = [0; 4096].to_vec();
		}
		self.sections[section_id as usize].blocks[block_id as usize] = block_state_id;

		let destroy_blockentity =
			if self.try_get_block_entity(position_global).is_some() { Some(BlockOverwriteOutcome::DestroyBlockentity) } else { None };

		if let Some(blockentity) =
			crate::blockentity::BlockEntity::new_from_block(data::blocks::get_type_from_block_state_id(block_state_id), position_global)
		{
			self.block_entities.push(blockentity);
		}

		return destroy_blockentity;
	}

	pub fn get_block(&self, position_in_chunk: BlockPosition) -> u16 {
		if position_in_chunk.y < -64 {
			return 0;
		}

		let section_id = (position_in_chunk.y + 64) / 16;

		if section_id as usize >= self.sections.len() {
			return 0;
		}

		if self.sections[section_id as usize].blocks.is_empty() {
			return 0;
		}

		let block_id =
			position_in_chunk.x + (position_in_chunk.z * 16) + (((position_in_chunk.y as i32 + 64) - (section_id as i32 * 16)) * 256);
		return self.sections[section_id as usize].blocks[block_id as usize];
	}

	pub fn try_get_block_entity(&self, position_in_chunk: BlockPosition) -> Option<&BlockEntity> {
		return self.block_entities.iter().find(|x| x.get_position() == position_in_chunk);
	}

	pub fn try_get_block_entity_mut(&mut self, position_in_chunk: BlockPosition) -> Option<&mut BlockEntity> {
		self.modified = true; //cant know what caller will do with the &mut so better be safe
		return self.block_entities.iter_mut().find(|x| x.get_position() == position_in_chunk);
	}
}

impl ChunkSection {
	pub fn get_non_air_block_count(&self) -> u16 {
		return self.blocks.iter().filter(|x| **x != 0).count() as u16;
	}
}
