use std::{collections::HashSet, fs::{self, File, OpenOptions}, io::{prelude::*, SeekFrom}, path::PathBuf, str::FromStr};
use data::blocks::Block;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::NbtTag;

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

  fn save_to_disk(&self, chunks: &[Chunk]) {
 		println!("start saving world with {} chunks to disk", chunks.len());
  	let mut regions: HashMap<(i32, i32), Vec<&Chunk>> = HashMap::new();
   	for chunk in chunks {
    	let region = chunk_to_region(chunk.x, chunk.z);
     	regions.entry(region).or_default().push(chunk);
    }
    println!("there are {} regions to save", regions.len());
    for region in regions {
    	let now = std::time::Instant::now();
   		save_region_to_disk(region.0, region.1.as_slice(), self.path.clone());
     	println!("saved region {:?} in {:.2?}", region.0, now.elapsed());
    }
  }
}

fn save_region_to_disk(region: (i32, i32), chunks: &[&Chunk], path: PathBuf) {
  let all_blocks = data::blocks::get_blocks();
	let mut locations_table = [(0u32, 0u8);1024];
	let mut timestamps_table = [0u32;1024];
	const EMPTY_CHUNK_DATA: Option<Vec<u8>> = None;
	let mut chunk_data: [Option<Vec<u8>>; 1024] = [EMPTY_CHUNK_DATA;1024];

	let mut region_file_path = path;
 	region_file_path.push(PathBuf::from_str("region").unwrap());
 	region_file_path.push(PathBuf::from_str(format!("r.{}.{}.mca", region.0, region.1).as_str()).unwrap());

 	if fs::exists(&region_file_path).unwrap() {
 		let mut region_file = File::open(&region_file_path).unwrap();

		let mut read_file_bytes: Vec<u8> = Vec::new();
   	region_file.read_to_end(&mut read_file_bytes).unwrap();

    for i in 0..1024 {
   		locations_table[i] = (u32::from_be_bytes([0, read_file_bytes[i*4], read_file_bytes[(i*4)+1], read_file_bytes[(i*4)+2]]), read_file_bytes[(i*4)+3]);
    	timestamps_table[i] = u32::from_be_bytes([read_file_bytes[(i*4) + 1024], read_file_bytes[(i*4) + 1025], read_file_bytes[(i*4) + 1026], read_file_bytes[(i*4) + 1027]]);
    }

    locations_table.iter()
      .enumerate()
      .filter(|(_i, (offset, length))| *offset != 0 && *length != 0)
     	.for_each(|(i, (offset, length))| {
     		chunk_data[i] = Some(read_file_bytes[(*offset as usize * 4096)..(*offset as usize * 4096 + *length as usize * 4096)].to_vec())
      });
  }

  for chunk in chunks {
  	let chunk_nbt = NbtTag::TagCompound(None, vec![
 			NbtTag::String(Some("Status".to_string()), "minecraft:full".to_string()),
   		NbtTag::Int(Some("xPos".to_string()), chunk.x),
   		NbtTag::Int(Some("yPos".to_string()), -4),
   		NbtTag::Int(Some("zPos".to_string()), chunk.z),
   		NbtTag::Int(Some("Dataversion".to_string()), 4325),
     	NbtTag::List(Some("sections".to_string()), chunk.sections.iter().enumerate().map(|(i, section)| {
      	let biome_palette: Vec<u8> = section.biomes.iter().copied().collect::<HashSet<u8>>().into_iter().collect();
				let biomes_bits_per_entry = match biome_palette.len() {
				  0..=2 => 1,
					3..=4 => 2,
					5..=8 => 3,
					9..=16 => 4,
				  17..=32 => 5,
				  _ => 6,
				};

      	let block_palette: Vec<u16> = if section.blocks.is_empty() {
          vec![0]
        } else {
          section.blocks.iter().copied().collect::<HashSet<u16>>().into_iter().collect()
        };
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

				let mut biome_data = vec![
				  NbtTag::List(Some("palette".to_string()), biome_palette.iter().map(|biome| {
           		NbtTag::String(None, data::biomes::get_biome_ids().into_iter().find(|(_, biome_id)| *biome_id == *biome).unwrap().0)
         	}).collect())
				];

				if biome_palette.len() > 1 {
		      biome_data.push(
  					NbtTag::LongArray(Some("data".to_string()), section.biomes.iter().map(|biome| {
  						biome_palette.iter().enumerate().find(|x| *x.1 == *biome).unwrap().0 as u8
  					}).collect::<Vec<u8>>().chunks(64/biomes_bits_per_entry).map(|byte_arr| {
     					let mut long = 0u64;
     					for byte in byte_arr {
    						long >>= biomes_bits_per_entry;
    						long += (*byte as u64) << (64-biomes_bits_per_entry);
     					}
     					if 64/biomes_bits_per_entry > byte_arr.len() {
     					  long >>= (64/biomes_bits_per_entry - byte_arr.len()) * biomes_bits_per_entry;
     					}
     					long >>= 64 % biomes_bits_per_entry;
     					return long as i64;
						}).collect())
				  )
				}

				let mut block_data = vec![
				  NbtTag::List(Some("palette".to_string()), block_palette.iter().map(|blockstate_id| {
            let block = all_blocks.iter().find(|x| x.1.states.iter().any(|x| x.id == *blockstate_id)).unwrap();
            let mut children = vec![
              NbtTag::String(Some("Name".to_string()), block.0.clone()),
            ];

            if block.1.states.len() > 1 {
              children.push(
                NbtTag::TagCompound(Some("Properties".to_string()), data::blocks::get_raw_properties_from_block_state_id(&all_blocks, *blockstate_id).into_iter().map(|x| {
                  NbtTag::String(Some(x.0), x.1)
                }).collect())
              );
            }

            NbtTag::TagCompound(None, children)
         	}).collect())
				];

				if block_palette.len() > 1 {
				  block_data.push(
						NbtTag::LongArray(Some("data".to_string()), section.blocks.iter().map(|block| {
							block_palette.iter().enumerate().find(|x| *x.1 == *block).unwrap().0 as u8
						}).collect::<Vec<u8>>().chunks(64/blocks_bits_per_entry).map(|byte_arr| {
							let mut long = 0u64;
							for byte in byte_arr {
								long >>= blocks_bits_per_entry;
								long += (*byte as u64) << (64-blocks_bits_per_entry);
							}
							if 64/blocks_bits_per_entry > byte_arr.len() {
							  long >>= (64/blocks_bits_per_entry - byte_arr.len()) * blocks_bits_per_entry;
							}
							long >>= 64 % blocks_bits_per_entry;
							return long as i64;
						}).collect())
					);
				}

				let mut nbt_arr = vec![
				  NbtTag::Byte(Some("Y".to_string()), (i as i8 - 4) as u8),
          NbtTag::TagCompound(Some("biomes".to_string()), biome_data),
          NbtTag::TagCompound(Some("block_states".to_string()), block_data),
				];

				if !section.block_lights.is_empty() {
		      nbt_arr.push(NbtTag::ByteArray(Some("BlockLight".to_string()), section.block_lights.clone()));
				}
				if !section.sky_lights.is_empty() {
		      nbt_arr.push(NbtTag::ByteArray(Some("SkyLight".to_string()), section.sky_lights.clone()));
				}

      	return NbtTag::TagCompound(None, nbt_arr);
      }).collect()),
   	]);

    let mut uncompressed_chunk = crate::serialize::nbt_disk(chunk_nbt);
		let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
		encoder.write_all(uncompressed_chunk.as_mut_slice()).unwrap();
		let mut compressed_chunk = encoder.finish().unwrap();
		let length = (compressed_chunk.len() + 1) as i32;

		let mut chunk_bytes: Vec<u8> = length.to_be_bytes().to_vec();
		chunk_bytes.push(2);
		chunk_bytes.append(&mut compressed_chunk);

		let rounded_chunk_len = ((chunk_bytes.len()-1) / 4096) + 1;
		let mut padding: Vec<u8> = (0..(rounded_chunk_len*4096)-chunk_bytes.len()).map(|_| 0).collect();
		chunk_bytes.append(&mut padding);

		assert!(rounded_chunk_len <= 255);
		assert_eq!(chunk_bytes.len() % 4096, 0);

		let chunk_index = (chunk.x & 31) + (chunk.z & 31) * 32;
		locations_table[chunk_index as usize].1 = rounded_chunk_len as u8;
		chunk_data[chunk_index as usize] = Some(chunk_bytes);
  }

  let mut first_chunk = true;
  let mut last_chunk_offset = 0;
  let mut last_chunk_len = 0;

  #[allow(clippy::needless_range_loop)] //tried to implement this but it broke so idk man
  for i in 0..locations_table.len() {
    if locations_table[i].1 == 0 {
      locations_table[i] = (0, 0);
      continue;
    }

    if first_chunk {
      locations_table[i].0 = 2;
      first_chunk = false;
    } else {
      locations_table[i].0 = last_chunk_offset + last_chunk_len as u32;
    }

    last_chunk_offset = locations_table[i].0;
    last_chunk_len = locations_table[i].1;
  }

 	let mut file = OpenOptions::new()
   	.read(true)
   	.write(true)
    .truncate(true)
    .create(true)
    .open(region_file_path)
    .unwrap();

  let mut output: Vec<u8> = Vec::new();
  for location in locations_table {
    output.push(location.0.to_be_bytes()[1]);
    output.push(location.0.to_be_bytes()[2]);
    output.push(location.0.to_be_bytes()[3]);
    output.push(location.1);
  }
  for timestamp in timestamps_table {
    output.append(&mut timestamp.to_be_bytes().to_vec());
  }
  chunk_data.into_iter().flatten().for_each(|mut x| output.append(&mut x));

  file.write_all(&output).unwrap();
  file.flush().unwrap();
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
