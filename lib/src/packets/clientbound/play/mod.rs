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

//
// MARK: 0x01 Spawn entity
//

#[derive(Debug, Clone)]
pub struct SpawnEntity {
	pub entity_id: i32,
	pub entity_uuid: u128,
	pub entity_type: i32, //real name in the protocol is just type; enum?
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub pitch: u8,
	pub yaw: u8,
	pub head_yaw: u8,
	pub data: i32,
	pub velocity_x: i16,
	pub velocity_y: i16,
	pub velocity_z: i16,
}

impl TryFrom<SpawnEntity> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SpawnEntity) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::uuid(&value.entity_uuid));
		output.append(&mut crate::serialize::varint(value.entity_type));
		output.append(&mut crate::serialize::double(value.x));
		output.append(&mut crate::serialize::double(value.y));
		output.append(&mut crate::serialize::double(value.z));
		output.push(value.pitch);
		output.push(value.yaw);
		output.push(value.head_yaw);
		output.append(&mut crate::serialize::varint(value.data));
		output.append(&mut crate::serialize::short(value.velocity_x));
		output.append(&mut crate::serialize::short(value.velocity_y));
		output.append(&mut crate::serialize::short(value.velocity_z));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SpawnEntity {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self { 
			entity_id: crate::deserialize::varint(&mut value)?,
			entity_uuid: crate::deserialize::uuid(&mut value)?,
			entity_type: crate::deserialize::varint(&mut value)?,
		  x: crate::deserialize::double(&mut value)?,
		  y: crate::deserialize::double(&mut value)?,
		  z: crate::deserialize::double(&mut value)?,
			pitch: value.remove(0),
			yaw: value.remove(0),
			head_yaw: value.remove(0),
			data: crate::deserialize::varint(&mut value)?,
			velocity_x: crate::deserialize::short(&mut value)?,
			velocity_y: crate::deserialize::short(&mut value)?,
			velocity_z: crate::deserialize::short(&mut value)?,
		});
	}
}

//
// MARK: 0x2F Update entity position
//

#[derive(Debug, Clone)]
pub struct UpdateEntityPosition {
	pub entity_id: i32,
	pub delta_x: i16,
	pub delta_y: i16,
	pub delta_z: i16,
	pub on_ground: bool,
}

impl TryFrom<UpdateEntityPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateEntityPosition) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::short(value.delta_x));
		output.append(&mut crate::serialize::short(value.delta_y));
		output.append(&mut crate::serialize::short(value.delta_z));
		output.append(&mut crate::serialize::bool(&value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for UpdateEntityPosition {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self { 
			entity_id: crate::deserialize::varint(&mut value)?,
			delta_x: crate::deserialize::short(&mut value)?,
			delta_y: crate::deserialize::short(&mut value)?,
			delta_z: crate::deserialize::short(&mut value)?,
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x30 Update entity position and rotation
//

#[derive(Debug, Clone)]
pub struct UpdateEntityPositionAndRotation {
	pub entity_id: i32,
	pub delta_x: i16,
	pub delta_y: i16,
	pub delta_z: i16,
	pub yaw: u8,
	pub pitch: u8,
	pub on_ground: bool,
}

impl TryFrom<UpdateEntityPositionAndRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateEntityPositionAndRotation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::short(value.delta_x));
		output.append(&mut crate::serialize::short(value.delta_y));
		output.append(&mut crate::serialize::short(value.delta_z));
		output.push(value.yaw);
		output.push(value.pitch);
		output.append(&mut crate::serialize::bool(&value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for UpdateEntityPositionAndRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self { 
			entity_id: crate::deserialize::varint(&mut value)?,
			delta_x: crate::deserialize::short(&mut value)?,
			delta_y: crate::deserialize::short(&mut value)?,
			delta_z: crate::deserialize::short(&mut value)?,
			yaw: value.remove(0),
			pitch: value.remove(0),
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x40 PlayerInfoUpdate
//

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdate {
	pub actions: u8,
	pub players: Vec<(u128, Vec<PlayerAction>)>,
}

//TODO: proper types
#[derive(Debug, Clone)]
pub enum PlayerAction {
	AddPlayer(String, Vec<(String, String, Option<String>)>), //Name, Property<Name, Value, Signature>
	InitializeChat(Option<(u128, i64, Vec<u8>, Vec<u8>)>), //Chat session ID, Public key expiry time, encoded public key, public key signature
	UpdateGameMode(i32), //Gamemode
	UpdateListed(bool), //Listed
	UpdateLatency(i32), //Ping
	UpdateDisplayName(Option<NbtTag>), //Display Name
	UpdateListPriority(i32), //Priority
	UpdateHat(bool), //Visible
}

impl TryFrom<PlayerInfoUpdate> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerInfoUpdate) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.push(value.actions);
		output.append(&mut crate::serialize::varint(value.players.len() as i32));
		for player in value.players {
			output.append(&mut crate::serialize::uuid(&player.0));

			for action in player.1 {
				match action {
						PlayerAction::AddPlayer(name, properties) => {
							output.append(&mut crate::serialize::string(&name));
							output.append(&mut crate::serialize::varint(properties.len() as i32));
							for property in properties {
								output.append(&mut crate::serialize::string(&property.0));
								output.append(&mut crate::serialize::string(&property.1));
								output.append(&mut crate::serialize::bool(&property.2.is_some()));
								if property.2.is_some() {
									output.append(&mut crate::serialize::string(&property.2.unwrap()));
								}
							}
						},
						PlayerAction::InitializeChat(data) => {
							if data.is_some() {
								output.push(1);
								output.append(&mut crate::serialize::uuid(&data.clone().unwrap().0));
								output.append(&mut crate::serialize::long(data.clone().unwrap().1));
								output.append(&mut data.clone().unwrap().2.clone());
								output.append(&mut data.unwrap().3.clone());
							} else {
								output.push(0);
							}
						},
						PlayerAction::UpdateGameMode(game_mode) => output.append(&mut crate::serialize::varint(game_mode)),
						PlayerAction::UpdateListed(listed) => output.append(&mut crate::serialize::bool(&listed)),
						PlayerAction::UpdateLatency(ping) => output.append(&mut crate::serialize::varint(ping)),
						PlayerAction::UpdateDisplayName(display_name) => {
							output.append(&mut crate::serialize::bool(&display_name.is_some()));
							if display_name.is_some() {
								output.append(&mut crate::serialize::nbt(display_name.unwrap()));
							}
						},
						PlayerAction::UpdateListPriority(priority) => output.append(&mut crate::serialize::varint(priority)),
						PlayerAction::UpdateHat(visible) => output.append(&mut crate::serialize::bool(&visible)),
				}
			}
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PlayerInfoUpdate {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let actions = value.remove(0);
		let player_length = crate::deserialize::varint(&mut value)?;
		let mut players: Vec<(u128, Vec<PlayerAction>)> = Vec::new();
		for _ in 0..player_length {
			let uuid = crate::deserialize::uuid(&mut value)?;

			let mut player_actions: Vec<PlayerAction> = Vec::new();
			if actions & 0x01 != 0 {
				let name = crate::deserialize::string(&mut value)?;
				let mut properties: Vec<(String, String, Option<String>)> = Vec::new();
				let properties_len = crate::deserialize::varint(&mut value)?;
				for _ in 0..properties_len {
					let name = crate::deserialize::string(&mut value)?;
					let value_prop = crate::deserialize::string(&mut value)?;
					let signature: Option<String> = if crate::deserialize::boolean(&mut value)? {
						Some(crate::deserialize::string(&mut value)?)
					} else {
						None
					};
					properties.push((name, value_prop, signature));
				}
				player_actions.push(PlayerAction::AddPlayer(name, properties));
			}

			if actions & 0x02 != 0 {
				if crate::deserialize::boolean(&mut value)? {
					let chat_session_id = crate::deserialize::uuid(&mut value)?;
					let public_key_expiry_time = crate::deserialize::long(&mut value)?;
					let encoded_public_key_length = crate::deserialize::varint(&mut value)?;
					let mut encoded_public_key: Vec<u8> = Vec::new();
					for _ in 0..encoded_public_key_length {
						encoded_public_key.push(value.remove(0));
					}
					let public_key_signature_length = crate::deserialize::varint(&mut value)?;
					let mut public_key_signature: Vec<u8> = Vec::new();
					for _ in 0..public_key_signature_length {
						public_key_signature.push(value.remove(0));
					}
	
					player_actions.push(PlayerAction::InitializeChat(Some((chat_session_id, public_key_expiry_time, encoded_public_key, public_key_signature))));
				} else {
					player_actions.push(PlayerAction::InitializeChat(None));
				}
			}

			if actions & 0x04 != 0 {
				player_actions.push(PlayerAction::UpdateGameMode(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x08 != 0 {
				player_actions.push(PlayerAction::UpdateListed(crate::deserialize::boolean(&mut value)?));
			}

			if actions & 0x10 != 0 {
				player_actions.push(PlayerAction::UpdateLatency(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x20 != 0 {
				let display_name = if crate::deserialize::boolean(&mut value)? {
					Some(crate::deserialize::nbt(&mut value)?)
				} else {
					None
				};
				player_actions.push(PlayerAction::UpdateDisplayName(display_name));
			}

			if actions & 0x40 != 0 {
				player_actions.push(PlayerAction::UpdateListPriority(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x80 != 0 {
				player_actions.push(PlayerAction::UpdateHat(crate::deserialize::boolean(&mut value)?));
			}

			players.push((uuid, player_actions));
		}

		return Ok(Self {
			actions,
			players,
		});
	}
}

//
// MARK: 0x5d Set Entity Metadata
//

#[derive(Debug, Clone)]
pub struct SetEntityMetadata {
	pub entity_id: i32,
	pub metadata: Vec<EntityMetadata>,
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
	pub index: u8,
	pub value: EntityMetadataValue,
}

#[derive(Debug, Clone)]
pub enum EntityMetadataValue {
	Byte(u8),
	Varint(i32),
	Varlong(i64),
	Float(f32),
	String(String),
	TextComponent(NbtTag),
	OptionalTextComponent(Option<NbtTag>), //absence of value indicated by a 0x00, if present append 0x01 byte
	Slot(i32), //always do 0 for now, until properly implemented elsewhere
	Boolean(bool),
	Rotations(f32, f32, f32),
	Position(i64),
	OptionalPosition(Option<i64>), //absence of value indicated by a 0x00, if present append 0x01 byte
	Direction(i32),
	OptionalUuid(bool, u128),
	BlockState(i32),
	OptionalBlockState(i32),
	Nbt(NbtTag),
	Particle(i32), //Missing a type that varies, whatever the fuck that means
	Particles(i32, Vec<i32>), //Missing a type that varies, whatever the fuck that means
	VillagerData(i32, i32, i32),
	OptionalVarint(i32),
	Pose(i32),
	CatVariant(i32),
	WolfVariant(i32),
	FrogVariant(i32),
	OptionalGlobalPosition(bool, bool, String, bool, i64),
	PaintingVariant(i32),
	SnifferState(i32),
	ArmadilloState(i32),
	Vector3(f32, f32, f32),
	Quaternion(f32, f32, f32, f32),
}

impl Into<i32> for EntityMetadataValue {
	fn into(self) -> i32 {
		match self {
			EntityMetadataValue::Byte(_) => 0,
			EntityMetadataValue::Varint(_) => 1,
			EntityMetadataValue::Varlong(_) => 2,
			EntityMetadataValue::Float(_) => 3,
			EntityMetadataValue::String(_) => 4,
			EntityMetadataValue::TextComponent(_) => 5,
			EntityMetadataValue::OptionalTextComponent(_) => 6,
			EntityMetadataValue::Slot(_) => 7,
			EntityMetadataValue::Boolean(_) => 8,
			EntityMetadataValue::Rotations(_, _, _) => 9,
			EntityMetadataValue::Position(_) => 10,
			EntityMetadataValue::OptionalPosition(_) => 11,
			EntityMetadataValue::Direction(_) => 12,
			EntityMetadataValue::OptionalUuid(_, _) => 13,
			EntityMetadataValue::BlockState(_) => 14,
			EntityMetadataValue::OptionalBlockState(_) => 15,
			EntityMetadataValue::Nbt(_) => 16,
			EntityMetadataValue::Particle(_) => 17,
			EntityMetadataValue::Particles(_, _) => 18,
			EntityMetadataValue::VillagerData(_, _, _) => 19,
			EntityMetadataValue::OptionalVarint(_) => 20,
			EntityMetadataValue::Pose(_) => 21,
			EntityMetadataValue::CatVariant(_) => 22,
			EntityMetadataValue::WolfVariant(_) => 23,
			EntityMetadataValue::FrogVariant(_) => 24,
			EntityMetadataValue::OptionalGlobalPosition(_, _, _, _, _) => 25,
			EntityMetadataValue::PaintingVariant(_) => 26,
			EntityMetadataValue::SnifferState(_) => 27,
			EntityMetadataValue::ArmadilloState(_) => 28,
			EntityMetadataValue::Vector3(_, _, _) => 29,
			EntityMetadataValue::Quaternion(_, _, _, _) => 30,
		}
	}
}

impl TryFrom<SetEntityMetadata> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetEntityMetadata) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));

		for metadata in value.metadata {
			output.push(metadata.index);
			output.append(&mut crate::serialize::varint(metadata.value.clone().into()));

			match metadata.value {
				EntityMetadataValue::Byte(a) => output.push(a),
				EntityMetadataValue::Varint(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Varlong(_) => todo!(),
				EntityMetadataValue::Float(a) => output.append(&mut crate::serialize::float(a)),
				EntityMetadataValue::String(a) => output.append(&mut crate::serialize::string(&a)),
				EntityMetadataValue::TextComponent(a) => output.append(&mut crate::serialize::nbt(a)),
				EntityMetadataValue::OptionalTextComponent(a) => {
					if a.is_some() {
						output.push(0x01);
						output.append(&mut crate::serialize::nbt(a.unwrap()));
					} else {
						output.push(0x00);
					}
				},
				EntityMetadataValue::Slot(_) => output.append(&mut vec![0; 6]),
				EntityMetadataValue::Boolean(a) => output.append(&mut crate::serialize::bool(&a)),
				EntityMetadataValue::Rotations(a, b, c) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
				},
				EntityMetadataValue::Position(a) => output.append(&mut crate::serialize::long(a)),
				EntityMetadataValue::OptionalPosition(a) => {
					if a.is_some() {
						output.push(0x01);
						output.append(&mut crate::serialize::long(a.unwrap()));
					} else {
						output.push(0x00);
					}
				},
				EntityMetadataValue::Direction(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::OptionalUuid(a, b) => {
					output.append(&mut crate::serialize::bool(&a));
					if a {
						output.append(&mut crate::serialize::uuid(&b));
					}
				},
				EntityMetadataValue::BlockState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::OptionalBlockState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Nbt(a) => output.append(&mut crate::serialize::nbt(a)),
				EntityMetadataValue::Particle(_) => todo!(),
				EntityMetadataValue::Particles(_, _) => todo!(),
				EntityMetadataValue::VillagerData(_, _, _) => todo!(),
				EntityMetadataValue::OptionalVarint(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Pose(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::CatVariant(_) => todo!(),
				EntityMetadataValue::WolfVariant(_) => todo!(),
				EntityMetadataValue::FrogVariant(_) => todo!(),
				EntityMetadataValue::OptionalGlobalPosition(a, b, c, d, e) => {
					output.append(&mut crate::serialize::bool(&a));
					output.append(&mut crate::serialize::bool(&b));
					if b {
						output.append(&mut crate::serialize::string(&c));
					}

					output.append(&mut crate::serialize::bool(&d));
					if d {
						output.append(&mut crate::serialize::long(e));
					}
				},
				EntityMetadataValue::PaintingVariant(_) => todo!(),
				EntityMetadataValue::SnifferState(_) => todo!(),
				EntityMetadataValue::ArmadilloState(_) => todo!(),
				EntityMetadataValue::Vector3(a, b, c) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
				},
				EntityMetadataValue::Quaternion(a, b, c, d) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
					output.append(&mut crate::serialize::float(d));
				},
			}
		}

		output.push(255);

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetEntityMetadata {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let entity_id = crate::deserialize::varint(&mut value)?;
		let mut metadata: Vec<EntityMetadata> = Vec::new();

		loop {
			let index = value.remove(0);
			if index == 0xff {
				break;
			}

			let type_id = crate::deserialize::varint(&mut value)?;
			let metadata_value = match type_id {
				0 => EntityMetadataValue::Byte(value.remove(0)),
				1 => EntityMetadataValue::Varint(crate::deserialize::varint(&mut value)?),
				2 => todo!(),
				3 => EntityMetadataValue::Float(crate::deserialize::float(&mut value)?),
				4 => EntityMetadataValue::String(crate::deserialize::string(&mut value)?),
				5 => EntityMetadataValue::TextComponent(crate::deserialize::nbt(&mut value)?),
				6 => {
					let nbt_present = crate::deserialize::boolean(&mut value)?;
					let nbt = if nbt_present {
						Some(crate::deserialize::nbt(&mut value)?)
					} else {
						None
					};
					EntityMetadataValue::OptionalTextComponent(nbt)
				},
				7 => break,
				8 => EntityMetadataValue::Boolean(crate::deserialize::boolean(&mut value)?),
				9 => EntityMetadataValue::Rotations(crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?),
				10 => EntityMetadataValue::Position(crate::deserialize::long(&mut value)?),
				11 => {
					let position_present = crate::deserialize::boolean(&mut value)?;
					let position = if position_present {
						Some(crate::deserialize::long(&mut value)?)
					} else {
						None
					};
					EntityMetadataValue::OptionalPosition(position)
				},
				12 => EntityMetadataValue::Direction(crate::deserialize::varint(&mut value)?),
				13 => todo!(),
				14 => EntityMetadataValue::BlockState(crate::deserialize::varint(&mut value)?),
				15 => EntityMetadataValue::OptionalBlockState(crate::deserialize::varint(&mut value)?),
				16 => EntityMetadataValue::Nbt(crate::deserialize::nbt(&mut value)?),
				17 => todo!(),
				18 => todo!(),
				19 => EntityMetadataValue::VillagerData(crate::deserialize::varint(&mut value)?, crate::deserialize::varint(&mut value)?, crate::deserialize::varint(&mut value)?),
				20 => EntityMetadataValue::OptionalVarint(crate::deserialize::varint(&mut value)?),
				21 => EntityMetadataValue::Pose(crate::deserialize::varint(&mut value)?),
				22 => EntityMetadataValue::CatVariant(crate::deserialize::varint(&mut value)?),
				23 => EntityMetadataValue::WolfVariant(crate::deserialize::varint(&mut value)?),
				24 => EntityMetadataValue::FrogVariant(crate::deserialize::varint(&mut value)?),
				25 => todo!(),
				26 => EntityMetadataValue::PaintingVariant(crate::deserialize::varint(&mut value)?),
				27 => EntityMetadataValue::SnifferState(crate::deserialize::varint(&mut value)?),
				28 => EntityMetadataValue::ArmadilloState(crate::deserialize::varint(&mut value)?),
				29 => EntityMetadataValue::Vector3(crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?),
				30 => EntityMetadataValue::Quaternion(crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?, crate::deserialize::float(&mut value)?),
				id => panic!("type_id {id} is not a recognized entity type"),
			};

			metadata.push(EntityMetadata { index, value: metadata_value });
		}

		return Ok(Self { 
			entity_id,
			metadata,
		});
	}
}