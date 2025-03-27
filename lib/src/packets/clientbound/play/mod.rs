use super::*;

//
// MARK: 0x2c FinishConfiguration
//

#[derive(Debug, Clone)]
pub struct Login {
	pub entity_id: i32,
	pub is_hardcore: bool,
	pub dimension_names: Vec<String>,
	pub max_players: i32,
	pub view_distance: i32,
	pub simulation_distance: i32,
	pub reduced_debug_info: bool,
	pub enable_respawn_screen: bool,
	pub do_limited_crafting: bool,
	pub dimension_type: i32,
	pub dimension_name: String,
	pub hashed_seed: i64,
	pub game_mode: u8,
	pub previous_game_mode: i8,
	pub is_debug: bool,
	pub is_flat: bool,
	pub has_death_location: bool,
	pub death_dimension_name: Option<String>,
	pub death_location: Option<u64>,
	pub portal_cooldown: i32,
	pub sea_level: i32,
	pub enforces_secure_chat: bool,
}


impl TryFrom<Login> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Login) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.entity_id));
		output.append(&mut crate::serialize::bool(&value.is_hardcore));
		output.append(&mut crate::serialize::varint(value.dimension_names.len() as i32));
		for dimension_name in &value.dimension_names {
			output.append(&mut crate::serialize::string(dimension_name));
		}
		output.append(&mut crate::serialize::varint(value.max_players));
		output.append(&mut crate::serialize::varint(value.view_distance));
		output.append(&mut crate::serialize::varint(value.simulation_distance));
		output.append(&mut crate::serialize::bool(&value.reduced_debug_info));
		output.append(&mut crate::serialize::bool(&value.enable_respawn_screen));
		output.append(&mut crate::serialize::bool(&value.do_limited_crafting));
		output.append(&mut crate::serialize::varint(value.dimension_type));
		output.append(&mut crate::serialize::string(&value.dimension_name));
		output.append(&mut crate::serialize::long(value.hashed_seed));
		output.push(value.game_mode);
		output.push(value.previous_game_mode as u8);
		output.append(&mut crate::serialize::bool(&value.is_debug));
		output.append(&mut crate::serialize::bool(&value.is_flat));
		output.append(&mut crate::serialize::bool(&value.has_death_location));
		if value.has_death_location {
			output.append(&mut crate::serialize::string(&value.death_dimension_name.unwrap()));
			output.append(&mut crate::serialize::long(value.death_location.unwrap() as i64)); //probably fucked
		}
		output.append(&mut crate::serialize::varint(value.portal_cooldown));
		output.append(&mut crate::serialize::varint(value.sea_level));
		output.append(&mut crate::serialize::bool(&value.enforces_secure_chat));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for Login {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let entity_id = crate::deserialize::int(&mut value)?;
		println!("entity_id: {entity_id}");
		let is_hardcore = crate::deserialize::boolean(&mut value)?;

		let dimension_names_len = crate::deserialize::varint(&mut value)?;
		let mut dimension_names: Vec<String> = Vec::new();
		for _ in 0..dimension_names_len {
			dimension_names.push(crate::deserialize::string(&mut value)?);
		}
		
		let max_players = crate::deserialize::varint(&mut value)?;
		let view_distance = crate::deserialize::varint(&mut value)?;
		let simulation_distance = crate::deserialize::varint(&mut value)?;
		let reduced_debug_info = crate::deserialize::boolean(&mut value)?;
		let enable_respawn_screen = crate::deserialize::boolean(&mut value)?;
		let do_limited_crafting = crate::deserialize::boolean(&mut value)?;
		let dimension_type = crate::deserialize::varint(&mut value)?;
		let dimension_name = crate::deserialize::string(&mut value)?;
		let hashed_seed = crate::deserialize::long(&mut value)?;
		let game_mode = value.remove(0);
		let previous_game_mode = value.remove(0) as i8;
		let is_debug = crate::deserialize::boolean(&mut value)?;
		let is_flat = crate::deserialize::boolean(&mut value)?;
		let has_death_location = crate::deserialize::boolean(&mut value)?;
		
		let death_dimension_name: Option<String> = if has_death_location {
			Some(crate::deserialize::string(&mut value)?)
		} else {
			None
		};

		let death_location: Option<u64> = if has_death_location {
			Some(crate::deserialize::long(&mut value)? as u64) //Probably fucked
		} else {
			None
		};
		
		let portal_cooldown = crate::deserialize::varint(&mut value)?;
		let sea_level = crate::deserialize::varint(&mut value)?;
		let enforces_secure_chat = crate::deserialize::boolean(&mut value)?;
		

		return Ok(Self { 
			entity_id,
			is_hardcore,
			dimension_names,
			max_players,
			view_distance,
			simulation_distance,
			reduced_debug_info,
			enable_respawn_screen,
			do_limited_crafting,
			dimension_type,
			dimension_name,
			hashed_seed,
			game_mode,
			previous_game_mode,
			is_debug,
			is_flat,
			has_death_location,
			death_dimension_name,
			death_location,
			portal_cooldown,
			sea_level,
			enforces_secure_chat,
		});
	}
}

//
// MARK: 0x28 Chunk Data and Update Light
//

#[derive(Debug, Clone)]
pub struct ChunkDataAndUpdateLight {
	pub chunk_x: i32,
	pub chunk_z: i32,
	pub heightmaps: NbtTag,
	pub data: Vec<ChunkSection>,
	pub block_entities: Vec<BlockEntity>,
	pub sky_light_mask: Vec<i64>,
	pub block_light_mask: Vec<i64>,
	pub empty_sky_light_mask: Vec<i64>,
	pub empty_block_light_mask: Vec<i64>,
	pub sky_light_arrays: Vec<LightArray>,
	pub block_light_arrays: Vec<LightArray>,
}

#[derive(Debug, Clone)]
pub struct LightArray {
	pub len: i32,
	pub array: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BlockEntity {
	pub packed_xz: u8,
	pub y: i16,
	pub block_entity_type: i32,
	pub data: Option<NbtTag>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
	pub block_count: i16,
	pub block_states: PalettedContainer,
	pub biomes: PalettedContainer,
}

#[derive(Debug, Clone)]
pub enum PalettedContainer {
	SingleValued(SingleValued),
	Indirect(Indirect),
	Direct(Direct),
}

#[derive(Debug, Clone)]
pub struct SingleValued {
	pub bits_per_entry: u8,
	pub value: i32,
	pub data_array: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct Indirect {
	pub bits_per_entry: u8,
	pub palette: Vec<i32>,
	pub data_array: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct Direct {
	pub bits_per_entry: u8,
	pub data_array: Vec<i64>,
}

impl TryFrom<PalettedContainer> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PalettedContainer) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();
		
		match value {
			PalettedContainer::SingleValued(single_valued) => {
				output.push(single_valued.bits_per_entry);
				output.append(&mut crate::serialize::varint(single_valued.value));
				output.append(&mut crate::serialize::varint(single_valued.data_array.len() as i32));
				for data in single_valued.data_array {
					output.append(&mut crate::serialize::long(data));
				}
			},
			PalettedContainer::Indirect(indirect) => {
				output.push(indirect.bits_per_entry);
				output.append(&mut crate::serialize::varint(indirect.palette.len() as i32));
				for palette in indirect.palette {
					output.append(&mut crate::serialize::varint(palette));
				}
				output.append(&mut crate::serialize::varint(indirect.data_array.len() as i32));
				for data in indirect.data_array {
					output.append(&mut crate::serialize::long(data));
				}			
			},
			PalettedContainer::Direct(direct) => {
				output.push(direct.bits_per_entry);
				output.append(&mut crate::serialize::varint(direct.data_array.len() as i32));
				for data in direct.data_array {
					output.append(&mut crate::serialize::long(data));
				}
			},
		};

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PalettedContainer {
	type Error = Box<dyn Error>;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		return value.try_into();
	}
}

impl TryFrom<&mut Vec<u8>> for PalettedContainer {
	type Error = Box<dyn Error>;

	fn try_from(mut value: &mut Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let bits_per_entry = value.remove(0);

		match bits_per_entry {
			0 => {
				let value_entry = crate::deserialize::varint(&mut value)?;
				let data_array_length = crate::deserialize::varint(&mut value)?;
				let mut data_array: Vec<i64> = Vec::new();
				for _ in 0..data_array_length {
					data_array.push(crate::deserialize::long(&mut value)?);
				}
				let single_valued = SingleValued {
					bits_per_entry,
					value: value_entry,
					data_array,
				};

				return Ok(PalettedContainer::SingleValued(single_valued));
			},
			1..=14 => {
				let palette_length = crate::deserialize::varint(&mut value)?;
				let mut palette: Vec<i32> = Vec::new();
				for _ in 0..palette_length {
					palette.push(crate::deserialize::varint(&mut value)?);
				}

				let data_array_length = crate::deserialize::varint(&mut value)?;
				let mut data_array: Vec<i64> = Vec::new();
				for _ in 0..data_array_length {
					data_array.push(crate::deserialize::long(&mut value)?);
				}
				let indirect = Indirect {
					bits_per_entry,
					data_array,
					palette,
				};

				return Ok(PalettedContainer::Indirect(indirect));
			}
			_ => {
				let data_array_length = crate::deserialize::varint(&mut value)?;
				let mut data_array: Vec<i64> = Vec::new();
				for _ in 0..data_array_length {
					data_array.push(crate::deserialize::long(&mut value)?);
				}
				let direct = Direct {
					bits_per_entry,
					data_array,
				};

				return Ok(PalettedContainer::Direct(direct));
			}
		}
	}
}

impl TryFrom<ChunkSection> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChunkSection) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::short(value.block_count));
		output.append(&mut value.block_states.try_into()?);
		output.append(&mut value.biomes.try_into()?);

		return Ok(output);
	}
}

impl TryFrom<&mut Vec<u8>> for ChunkSection {
	type Error = Box<dyn Error>;
	
	fn try_from(mut value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		return Ok(Self {
			block_count: crate::deserialize::short(&mut value)?,
			block_states: value.try_into()?,
			biomes: value.try_into()?,
		});
	}
}

impl TryFrom<ChunkDataAndUpdateLight> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChunkDataAndUpdateLight) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();
		
		output.append(&mut crate::serialize::int(value.chunk_x));
		output.append(&mut crate::serialize::int(value.chunk_z));
		output.append(&mut crate::serialize::nbt(value.heightmaps));
		let mut chunk_section_data: Vec<u8> = Vec::new();
		for chunk_section in value.data {
			chunk_section_data.append(&mut chunk_section.try_into()?);
		}
		output.append(&mut crate::serialize::varint(chunk_section_data.len() as i32));
		output.append(&mut chunk_section_data);
		output.append(&mut crate::serialize::varint(value.block_entities.len() as i32));
		for x in value.block_entities {
			output.push(x.packed_xz);
			output.append(&mut crate::serialize::short(x.y));
			output.append(&mut crate::serialize::varint(x.block_entity_type));
			if x.data.is_some() {
				output.append(&mut crate::serialize::nbt(x.data.unwrap()));
			} else {
				output.push(0x00);
			}
		}
		output.append(&mut crate::serialize::varint(value.sky_light_mask.len() as i32));
		for x in value.sky_light_mask {
			output.append(&mut crate::serialize::long(x));
		}
		output.append(&mut crate::serialize::varint(value.block_light_mask.len() as i32));
		for x in value.block_light_mask {
			output.append(&mut crate::serialize::long(x));
		}
		output.append(&mut crate::serialize::varint(value.empty_sky_light_mask.len() as i32));
		for x in value.empty_sky_light_mask {
			output.append(&mut crate::serialize::long(x));
		}
		output.append(&mut crate::serialize::varint(value.empty_block_light_mask.len() as i32));
		for x in value.empty_block_light_mask {
			output.append(&mut crate::serialize::long(x));
		}
		output.append(&mut crate::serialize::varint(value.sky_light_arrays.len() as i32));
		for x in value.sky_light_arrays {
			output.append(&mut crate::serialize::varint(x.array.len() as i32));
			output.append(&mut x.array.clone());
		}
		output.append(&mut crate::serialize::varint(value.block_light_arrays.len() as i32));
		for x in value.block_light_arrays {
			output.append(&mut crate::serialize::varint(x.array.len() as i32));
			output.append(&mut x.array.clone());
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for ChunkDataAndUpdateLight {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let chunk_x = crate::deserialize::int(&mut value)?;
		let chunk_z = crate::deserialize::int(&mut value)?;
		let heightmaps = crate::deserialize::nbt(&mut value)?;
		let _size = crate::deserialize::varint(&mut value)?;
		let mut data: Vec<ChunkSection> = Vec::new();
		let chunk_sections = 24;
		for _ in 0..chunk_sections {
			data.push((&mut value).try_into()?);
		}
		
		let block_entities_len = crate::deserialize::varint(&mut value)?;
		let mut block_entities: Vec<BlockEntity> = Vec::new();
		for _ in 0..block_entities_len {
			let packed_xz = value.remove(0);
			let y = crate::deserialize::short(&mut value)?;
			let block_entity_type = crate::deserialize::varint(&mut value)?;
			let data = if *value.first().unwrap() == 0 {
				None
			} else {
				Some(crate::deserialize::nbt(&mut value)?)
			};
			block_entities.push(BlockEntity {
				packed_xz,
				y,
				block_entity_type,
				data,
			});
		}
		let sky_light_mask_len = crate::deserialize::varint(&mut value)?;
		let mut sky_light_mask: Vec<i64> = Vec::new();
		for _ in 0..sky_light_mask_len {
			sky_light_mask.push(crate::deserialize::long(&mut value)?);
		}
		let block_light_mask_len = crate::deserialize::varint(&mut value)?;
		let mut block_light_mask: Vec<i64> = Vec::new();
		for _ in 0..block_light_mask_len {
			block_light_mask.push(crate::deserialize::long(&mut value)?);
		}
		let empty_sky_light_mask_len = crate::deserialize::varint(&mut value)?;
		let mut empty_sky_light_mask: Vec<i64> = Vec::new();
		for _ in 0..empty_sky_light_mask_len {
			empty_sky_light_mask.push(crate::deserialize::long(&mut value)?);
		}
		let empty_block_light_mask_len = crate::deserialize::varint(&mut value)?;
		let mut empty_block_light_mask: Vec<i64> = Vec::new();
		for _ in 0..empty_block_light_mask_len {
			empty_block_light_mask.push(crate::deserialize::long(&mut value)?);
		}
		let sky_light_arrays_len = crate::deserialize::varint(&mut value)?;
		let mut sky_light_arrays: Vec<LightArray> = Vec::new();
		for _ in 0..sky_light_arrays_len {
			let len = crate::deserialize::varint(&mut value)?;
			let mut array: Vec<u8> = Vec::new();
			for _ in 0..len {
				array.push(value.remove(0));
			}
			sky_light_arrays.push(LightArray {
				len,
				array,
			});
		}
		let block_light_arrays_len = crate::deserialize::varint(&mut value)?;
		let mut block_light_arrays: Vec<LightArray> = Vec::new();
		for _ in 0..block_light_arrays_len {
			let len = crate::deserialize::varint(&mut value)?;
			let mut array: Vec<u8> = Vec::new();
			for _ in 0..len {
				array.push(value.remove(0));
			}
			block_light_arrays.push(LightArray {
				len,
				array,
			});
		}

		return Ok(Self { 
			chunk_x,
			chunk_z,
			heightmaps,
			data,
			block_entities,
			sky_light_mask,
			block_light_mask,
			empty_sky_light_mask,
			empty_block_light_mask,
			sky_light_arrays,
			block_light_arrays,  
		});
	}
}

//
// MARK: 0x23 Game event
//

#[derive(Debug, Clone)]
pub struct GameEvent {
	pub event: u8,
	pub value: f32,
}

impl TryFrom<GameEvent> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: GameEvent) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.push(value.event);
		output.append(&mut crate::serialize::float(value.value));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for GameEvent {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self { 
			event: value.remove(0),
			value: crate::deserialize::float(&mut value)?,
		});
	}
}