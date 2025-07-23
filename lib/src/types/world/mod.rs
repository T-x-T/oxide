pub mod loader;

use std::{collections::HashMap, error::Error, fmt::Debug};

use crate::{loader::WorldLoader, types::position::Position, SPAWN_CHUNK_RADIUS};

#[derive(Debug)]
pub struct World {
  pub dimensions: HashMap<String, Dimension>,
  pub loader: Box<dyn WorldLoader>,
}

#[derive(Debug, Clone)]
pub struct Dimension {
  pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone)]
pub struct Chunk {
  pub x: i32,
  pub z: i32,
  pub sections: Vec<ChunkSection>,
  pub inhabited_time: i64,
  pub last_update: i64,
  pub is_light_on: bool,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
  pub blocks: Vec<u16>,
  pub biomes: Vec<u8>,
  pub sky_lights: Vec<u8>,
  pub block_lights: Vec<u8>,
}


impl World {
  #[allow(clippy::new_without_default)]
  pub fn new(loader: impl WorldLoader + 'static) -> Self {
   	let mut dimensions: HashMap<String, Dimension> = HashMap::new();
  	if loader.is_initialized() {
   		let now = std::time::Instant::now();
 			println!("loading existing world");
   		dimensions.insert("minecraft:overworld".to_string(), Dimension::new_from_loader(&loader));
    	println!("finished loading existing world in {:.2?}", now.elapsed());
   	} else {
	    println!("create new world");
	    dimensions.insert("minecraft:overworld".to_string(), Dimension::new());
	    println!("creation of new world finished");
    }
    return Self { dimensions, loader: Box::new(loader) };
  }

  pub fn save_to_disk(&self) {
  	self.dimensions.iter().for_each(|x| x.1.save_to_disk(&*self.loader));
  }
}

impl Dimension {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let mut chunks: Vec<Chunk> = Vec::new();

    for x in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
      for z in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
        chunks.push(Chunk::new(x as i32, z as i32));
      }
    }

    return Self {
      chunks,
    };
  }

  pub fn new_from_loader(loader: &impl loader::WorldLoader) -> Self {
  	let mut chunks: Vec<Chunk> = Vec::new();

   	for x in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
    	for z in -SPAWN_CHUNK_RADIUS..=SPAWN_CHUNK_RADIUS {
     		chunks.push(loader.load_chunk(x as i32, z as i32));
      }
    }

    return Self {
   		chunks,
    }
	}

  pub fn get_chunk_from_position_mut(&mut self, position: Position) -> Option<&mut Chunk> {
    let chunk_coordinates = position.convert_to_coordinates_of_chunk();

    return self.chunks.iter_mut().find(|chunk| chunk.x == chunk_coordinates.x && chunk.z == chunk_coordinates.z);
  }

  pub fn get_chunk_from_position(&self, position: Position) -> Option<&Chunk> {
    let chunk_coordinates = position.convert_to_coordinates_of_chunk();

    return self.chunks.iter().find(|chunk| chunk.x == chunk_coordinates.x && chunk.z == chunk_coordinates.z);
  }

  pub fn get_chunk_from_chunk_position_mut(&mut self, chunk_coordinates: Position) -> Option<&mut Chunk> {
    return self.chunks.iter_mut().find(|chunk| chunk.x == chunk_coordinates.x && chunk.z == chunk_coordinates.z);
  }

  pub fn get_chunk_from_chunk_position(&self, chunk_coordinates: Position) -> Option<&Chunk> {
    return self.chunks.iter().find(|chunk| chunk.x == chunk_coordinates.x && chunk.z == chunk_coordinates.z);
  }

  pub fn overwrite_block(&mut self, position: Position, block_state_id: u16) -> Result<(), Box<dyn Error>> {
    let chunk = self.get_chunk_from_position_mut(position);
    if chunk.is_none() {
      return Err(Box::new(crate::CustomError::ChunkNotFound(position)));
    }
    if position.y < -64 || position.y > 319 {
      return Err(Box::new(crate::CustomError::PositionOutOfBounds(position)));
    }

    chunk.unwrap().set_block(position.convert_to_position_in_chunk(), block_state_id);

    return Ok(());
  }

  pub fn get_block(&self, position: Position) -> Result<u16, Box<dyn Error>> {
    let chunk = self.get_chunk_from_position(position);
    if chunk.is_none() {
      return Err(Box::new(crate::CustomError::ChunkNotFound(position)));
    }
    if position.y < -64 || position.y > 319 {
      return Err(Box::new(crate::CustomError::PositionOutOfBounds(position)));
    }

    return Ok(chunk.unwrap().get_block(position.convert_to_position_in_chunk()));
  }

  pub fn save_to_disk(&self, loader: &(impl WorldLoader + ?Sized)) {
 		loader.save_to_disk(&self.chunks);
  }
}

impl Chunk {
  pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
    let filled_chunk_sections = vec![ChunkSection {
      blocks: vec![1; 4096],
      biomes: vec![40; 64],
      sky_lights: vec![0xFF;2048],
      block_lights: vec![],
    }; 1];
    let empty_chunk_sections = vec![ChunkSection {
      blocks: vec![0; 4096],
      biomes: vec![40; 64],
      sky_lights: vec![0xFF;2048],
      block_lights: vec![],
    }; 23];
    let mut all_chunk_sections = filled_chunk_sections.clone();
    all_chunk_sections.append(&mut empty_chunk_sections.clone());

    return Self {
      x: chunk_x,
      z: chunk_z,
      sections: all_chunk_sections,
      last_update: 0,
      inhabited_time: 0,
      is_light_on: true,
    };
  }

  pub fn set_block(&mut self, position_in_chunk: Position, block_state_id: u16) {
    let section_id = (position_in_chunk.y + 64) / 16;
    let block_id = position_in_chunk.x + (position_in_chunk.z * 16) + (((position_in_chunk.y as i32 + 64) - (section_id as i32 * 16)) * 256);
    self.sections[section_id as usize].blocks[block_id as usize] = block_state_id;
  }

  pub fn get_block(&self, position_in_chunk: Position) -> u16 {
    let section_id = (position_in_chunk.y + 64) / 16;
    let block_id = position_in_chunk.x + (position_in_chunk.z * 16) + (((position_in_chunk.y as i32 + 64) - (section_id as i32 * 16)) * 256);
    return self.sections[section_id as usize].blocks[block_id as usize];
  }
}

impl ChunkSection {
  pub fn get_non_air_block_count(&self) -> u16 {
    return self.blocks.iter().filter(|x| **x != 0).count() as u16;
  }
}
