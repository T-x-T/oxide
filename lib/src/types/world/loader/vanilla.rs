use std::{fs::{self, File}, io::{prelude::*, SeekFrom}, path::PathBuf, str::FromStr};
use data::blocks::Block;
use flate2::read::ZlibDecoder;

use super::*;

#[derive(Debug)]
pub struct Loader {
	pub path: PathBuf,
	pub block_states: HashMap<String, Block>,
}

impl super::InnerWorldLoader for Loader{}

impl super::WorldLoader for Loader {
  fn load_chunk(&self, x: i32, z: i32) -> Chunk {
   	let region = chunk_to_region(x, z);

  	let mut region_file_path = self.path.clone();
  	region_file_path.push(PathBuf::from_str("region").unwrap());
  	region_file_path.push(PathBuf::from_str(format!("r.{}.{}.mca", region.0, region.1).as_str()).unwrap());

   	if !fs::exists(&region_file_path).unwrap() {
   		return Chunk::new(x, z);
    }

  	let mut region_file = File::open(region_file_path).unwrap();

    let chunk_pos_in_header = 4 * ((x & 31) + (z & 31) * 32);
    region_file.seek(SeekFrom::Start(chunk_pos_in_header as u64)).unwrap();
    let mut chunk_location_bytes = vec![0; 3];
    region_file.read_exact(&mut chunk_location_bytes).unwrap();
    let chunk_offset = i32::from_be_bytes([0, chunk_location_bytes[0], chunk_location_bytes[1], chunk_location_bytes[2]]) * 4096;

    region_file.seek(SeekFrom::Start(chunk_pos_in_header as u64 + 3)).unwrap();
    let mut chunk_length_padded_bytes = vec![0];
    region_file.read_exact(&mut chunk_length_padded_bytes).unwrap();
    let chunk_length_padded = chunk_location_bytes[0] as i32 * 4096;

    if chunk_offset == 0 && chunk_length_padded == 0 {
    	return Chunk::new(x, z);
    }

    region_file.seek(SeekFrom::Start(chunk_offset as u64)).unwrap();
    let mut actual_chunk_length_bytes = vec![0; 4];
    region_file.read_exact(&mut actual_chunk_length_bytes).unwrap();
    let actual_chunk_length = i32::from_be_bytes([actual_chunk_length_bytes[0], actual_chunk_length_bytes[1], actual_chunk_length_bytes[2], actual_chunk_length_bytes[3]]);

    region_file.seek(SeekFrom::Start(chunk_offset as u64 + 4)).unwrap();
    let mut compression_scheme_bytes = vec![0];
    region_file.read_exact(&mut compression_scheme_bytes).unwrap();
    let compression_scheme = compression_scheme_bytes[0];

    region_file.seek(SeekFrom::Start(chunk_offset as u64 + 5)).unwrap();
    let mut compressed_data = vec![0; (actual_chunk_length - 1) as usize];
    region_file.read_exact(&mut compressed_data).unwrap();
    let mut uncompressed_data: Vec<u8> = Vec::new();
    if compression_scheme == 2 {
	    let mut decoder: ZlibDecoder<&[u8]> = ZlibDecoder::new(compressed_data.as_slice());
	    decoder.read_to_end(&mut uncompressed_data).unwrap();
    } else {
   		panic!("unknown chunk compression scheme {compression_scheme}");
    }

    let chunk_nbt = crate::deserialize::nbt_disk(&mut uncompressed_data).unwrap();

    if chunk_nbt.get_child("Status").unwrap().as_string() != "minecraft:full" {
    	return Chunk::new(x, z);
    }

    let mut sections: Vec<super::ChunkSection> = Vec::new();
    for x in chunk_nbt.get_child("sections").unwrap().as_list() {
    	let block_light_nbt = x.get_child("BlockLight");
     	let mut block_lights: Vec<u8> = Vec::new();
     	if let Some(block_light_nbt) = block_light_nbt {
      	block_lights = block_light_nbt.as_byte_array();
      }
    	let sky_light_nbt = x.get_child("SkyLight");
     	let mut sky_lights: Vec<u8> = Vec::new();
     	if let Some(sky_light_nbt) = sky_light_nbt {
      	sky_lights = sky_light_nbt.as_byte_array();
      }

	    let biome_palette = x.get_child("biomes").unwrap().get_child("palette").unwrap().as_list();

			let mut biomes: Vec<u8> = Vec::new();
			if biome_palette.len() == 1 {
				biomes = vec![*data::biomes::get_biome_ids().get(biome_palette[0].as_string()).unwrap(); 64];
			} else {
				let biomes_bits_per_entry = match biome_palette.len() {
				  0..=2 => 1,
					3..=4 => 2,
					5..=8 => 3,
					9..=16 => 4,
				  17..=32 => 5,
				  _ => 6,
				};

				let long_array = x.get_child("biomes").unwrap().get_child("data").unwrap().as_long_array();
				let entries_per_long = 64 / biomes_bits_per_entry;
				for value in long_array {
				 	for i in 0..entries_per_long {
				 	if biomes.len() == 64 {
				 			break;
				   	}
				    let entry = (value as u64) << (64 - (biomes_bits_per_entry * (i+1))) >> (64 - biomes_bits_per_entry);
				    let biome_id = *data::biomes::get_biome_ids().get(biome_palette[entry as usize].as_string()).unwrap();
				    biomes.push(biome_id);
				  }
				}
			}

	    let block_palette = x.get_child("block_states").unwrap().get_child("palette").unwrap().as_list();

			let mut blocks: Vec<u16> = Vec::new();
    	if block_palette.len() == 1 {
     		let block_name = block_palette[0].get_child("Name").unwrap().as_string();
       	if block_name == "minecraft:air" {
       		blocks = vec![];
        } else {
      		blocks = vec![self.block_states.get(block_palette[0].get_child("Name").unwrap().as_string()).unwrap().states.iter().find(|x| x.default).unwrap().id; 4096];
        }
      } else {
	      let blocks_bits_per_entry = match block_palette.len() {
				  0..=16 => 4,
				  17..=32 => 5,
				  33..=64 => 6,
				  65..=128 => 7,
				  129..=256 => 8,
				  257..=512 => 9,
				  513..=1024 => 10,
				  1025..=2048 => 11,
				  _ => 12,
				};

				let long_array = x.get_child("block_states").unwrap().get_child("data").unwrap().as_long_array();
				let entries_per_long = 64 / blocks_bits_per_entry;
				for value in long_array {
				 	for i in 0..entries_per_long {
				 	if blocks.len() == 4096 {
				 			break;
				   	}
				    let entry = (value as u64) << (64 - (blocks_bits_per_entry * (i+1))) >> (64 - blocks_bits_per_entry);
				    let block_state_id = data::blocks::get_block_state_id_from_raw(&self.block_states, block_palette[entry as usize].get_child("Name").unwrap().as_string(), block_palette[entry as usize].get_child("Properties").unwrap_or(&crate::NbtTag::TagCompound(None, vec![])).get_children().iter().map(|x| (x.get_description().clone().unwrap(), x.as_string().to_string())).collect());
				    blocks.push(block_state_id);
				  }
				}
      }

	   	sections.push(ChunkSection { blocks, biomes, sky_lights, block_lights });
    }

	 	return Chunk {
	    x: chunk_nbt.get_child("xPos").unwrap().as_int(),
	    z: chunk_nbt.get_child("zPos").unwrap().as_int(),
	    sections,
		};
  }

  fn is_initialized(&self) -> bool {
  	let mut level_dat_path = self.path.clone();
  	level_dat_path.push(PathBuf::from_str("level.dat").unwrap());
  	return std::fs::exists(level_dat_path).unwrap();
  }
}


fn chunk_to_region(x: i32, z: i32) -> (i32, i32) {
	return ((x as f32 / 32.0).floor() as i32, (z as f32 / 32.0).floor() as i32)
}

#[cfg(test)]
mod test {
	use super::*;
	mod chunk_to_region {
		use super::*;

		#[test]
		fn works_positive() {
			assert_eq!(chunk_to_region(6, 5), (0, 0));
		}

		#[test]
		fn works_positive_large() {
			assert_eq!(chunk_to_region(12345, 321), (385, 10));
		}

		#[test]
		fn works_zero() {
			assert_eq!(chunk_to_region(0, 0), (0, 0));
		}

		#[test]
		fn works_negative() {
			assert_eq!(chunk_to_region(-10, -20), (-1, -1));
		}

		#[test]
		fn works_negative_large() {
			assert_eq!(chunk_to_region(-1000, -2000), (-32, -63));
		}
	}
}
