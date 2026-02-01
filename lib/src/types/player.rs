use super::*;
use crate::entity::CommonEntity;
use crate::packets::clientbound::play::{EntityMetadata, EntityMetadataValue, PlayerAction};
use crate::packets::*;
use data::blocks::Block;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Gamemode {
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

impl TryFrom<u8> for Gamemode {
	type Error = Box<dyn Error>;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		return match value {
			0 => Ok(Gamemode::Survival),
			1 => Ok(Gamemode::Creative),
			2 => Ok(Gamemode::Adventure),
			3 => Ok(Gamemode::Spectator),
			x => Err(Box::new(crate::CustomError::InvalidInput(format!("I dont know what a gamemode of {x} is supposed to be")))),
		};
	}
}

//TODO: use new EntityPosition struct here too
#[derive(Debug)]
pub struct Player {
	x: f64,
	y: f64,
	z: f64,
	yaw: f32,
	pitch: f32,
	pub velocity: EntityPosition,
	pub display_name: String,
	pub uuid: u128,
	pub peer_socket_address: SocketAddr,
	pub connection_stream: TcpStream,
	pub entity_id: i32,
	pub waiting_for_confirm_teleportation: bool,
	pub current_teleport_id: i32,
	inventory: Vec<Option<Slot>>,
	selected_slot: u8,
	pub opened_inventory_at: Option<BlockPosition>,
	pub cursor_item: Option<Slot>,
	is_sneaking: bool,
	pub chat_message_index: i32,
	gamemode: Gamemode,
	mining_for_ticks: u16,
	is_mining: bool,
	health: f32,
	pub is_dead: bool,
}

//Manual implementation because TcpStream doesn't implement Clone, instead just call unwrap here on its try_clone() function
impl Clone for Player {
	fn clone(&self) -> Self {
		Self {
			x: self.x,
			y: self.y,
			z: self.z,
			yaw: self.yaw,
			pitch: self.pitch,
			velocity: EntityPosition::default(),
			display_name: self.display_name.clone(),
			uuid: self.uuid,
			peer_socket_address: self.peer_socket_address,
			connection_stream: self.connection_stream.try_clone().unwrap(),
			entity_id: self.entity_id,
			waiting_for_confirm_teleportation: self.waiting_for_confirm_teleportation,
			current_teleport_id: self.current_teleport_id,
			inventory: self.inventory.clone(),
			selected_slot: self.selected_slot,
			opened_inventory_at: self.opened_inventory_at,
			cursor_item: self.cursor_item.clone(),
			is_sneaking: self.is_sneaking,
			chat_message_index: self.chat_message_index,
			gamemode: self.gamemode,
			mining_for_ticks: self.mining_for_ticks,
			is_mining: self.is_mining,
			health: self.health,
			is_dead: self.is_dead,
		}
	}
}

impl CommonEntityTrait for Player {
	fn new(_data: CommonEntity, _extra_nbt: NbtListTag) -> Self {
		todo!()
	}

	fn get_common_entity_data(&self) -> &CommonEntity {
		todo!()
	}

	fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity {
		todo!();
	}

	fn set_common_entity_data(&mut self, _common_entity_data: CommonEntity) {
		todo!();
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:player");
	}

	fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata> {
		return vec![
			crate::packets::clientbound::play::EntityMetadata {
				index: 9,
				value: crate::packets::clientbound::play::EntityMetadataValue::Float(20.0),
			},
			crate::packets::clientbound::play::EntityMetadata {
				index: 16,
				value: crate::packets::clientbound::play::EntityMetadataValue::Byte(127),
			},
		];
	}

	fn get_yaw_u8(&self) -> u8 {
		return if self.yaw < 0.0 { (((self.yaw / 90.0) * 64.0) + 256.0) as u8 } else { ((self.yaw / 90.0) * 64.0) as u8 };
	}

	fn get_pitch_u8(&self) -> u8 {
		return if self.pitch < 0.0 { (((self.pitch / 90.0) * 64.0) + 256.0) as u8 } else { ((self.pitch / 90.0) * 64.0) as u8 };
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		todo!()
	}

	fn is_on_ground(&self, dimension: &Dimension) -> bool {
		return self.is_on_ground_at(
			dimension,
			EntityPosition {
				x: self.x,
				y: self.y,
				z: self.z,
				yaw: self.yaw,
				pitch: self.pitch,
			},
		);
	}

	fn tick(&mut self, dimension: &Dimension, players: &[Player], game: Arc<Game>) -> EntityTickOutcome {
		let own_position = self.get_position();
		let entities_to_remove: Vec<i32> = dimension
			.entities
			.iter()
			.filter(|x| x.get_common_entity_data().position.distance_to(own_position) < crate::ITEM_PICKUP_DISTANCE)
			.filter_map(|x| {
				if let Entity::Item(item) = x {
					if self.pickup_item(item.item.clone(), item.get_common_entity_data().entity_id, players, game.clone()) {
						Some(item.get_common_entity_data().entity_id)
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect();

		if entities_to_remove.is_empty() {
			return EntityTickOutcome::None;
		} else {
			return EntityTickOutcome::RemoveOthers(entities_to_remove);
		}
	}
}

impl Player {
	pub fn new(
		display_name: String,
		uuid: u128,
		peer_socket_address: SocketAddr,
		game: Arc<Game>,
		connection_stream: TcpStream,
		default_spawn_location: BlockPosition,
	) -> Self {
		let Ok(mut file) = File::open(Player::get_playerdata_path(uuid)) else {
			let entity_id = game.entity_id_manager.get_new();
			let player = Self {
				x: default_spawn_location.x as f64,
				y: default_spawn_location.y as f64,
				z: default_spawn_location.z as f64,
				yaw: 0.0,
				pitch: 0.0,
				velocity: EntityPosition::default(),
				display_name,
				uuid,
				peer_socket_address,
				connection_stream,
				entity_id,
				waiting_for_confirm_teleportation: false,
				current_teleport_id: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
					/ (entity_id + 1 + 12345) as u64) as i32, //should probably use random number instead
				inventory: vec![None; 46],
				selected_slot: 0,
				opened_inventory_at: None,
				cursor_item: None,
				is_sneaking: false,
				chat_message_index: 0,
				gamemode: game.default_gamemode,
				mining_for_ticks: 0,
				is_mining: false,
				health: 20.0,
				is_dead: false,
			};

			return player;
		};

		let mut compressed_file_content: Vec<u8> = Vec::new();
		file.read_to_end(&mut compressed_file_content).unwrap();

		let mut file_content: Vec<u8> = Vec::new();
		let mut decoder: GzDecoder<&[u8]> = GzDecoder::new(compressed_file_content.as_slice());
		decoder.read_to_end(&mut file_content).unwrap();

		let player_data = crate::deserialize::nbt_disk(&mut file_content).unwrap();

		let mut inventory = vec![None; 46];
		player_data.get_child("Inventory").unwrap().as_list().iter().for_each(|x| {
			let slot_index = x.get_child("Slot").unwrap().as_byte() as usize;
			inventory[slot_index] = Some(Slot {
				item_count: x.get_child("count").unwrap().as_int(),
				item_id: data::items::get_items().get(x.get_child("id").unwrap().as_string()).unwrap().id,
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			});
		});

		if let Some(equipment) = player_data.get_child("equipment") {
			if let Some(head) = equipment.get_child("head") {
				inventory[5] = Some(Slot {
					item_count: head.get_child("count").unwrap().as_int(),
					item_id: data::items::get_items().get(head.get_child("id").unwrap().as_string()).unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})
			}
			if let Some(chest) = equipment.get_child("chest") {
				inventory[6] = Some(Slot {
					item_count: chest.get_child("count").unwrap().as_int(),
					item_id: data::items::get_items().get(chest.get_child("id").unwrap().as_string()).unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})
			}
			if let Some(legs) = equipment.get_child("legs") {
				inventory[7] = Some(Slot {
					item_count: legs.get_child("count").unwrap().as_int(),
					item_id: data::items::get_items().get(legs.get_child("id").unwrap().as_string()).unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})
			}
			if let Some(feet) = equipment.get_child("feet") {
				inventory[8] = Some(Slot {
					item_count: feet.get_child("count").unwrap().as_int(),
					item_id: data::items::get_items().get(feet.get_child("id").unwrap().as_string()).unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})
			}
			if let Some(offhand) = equipment.get_child("offhand") {
				inventory[45] = Some(Slot {
					item_count: offhand.get_child("count").unwrap().as_int(),
					item_id: data::items::get_items().get(offhand.get_child("id").unwrap().as_string()).unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})
			}
		}

		let mut parsed_gamemode = game.default_gamemode;
		if let Some(gamemode) = player_data.get_child("playerGameType") {
			match gamemode.as_int() {
				0 => parsed_gamemode = Gamemode::Survival,
				1 => parsed_gamemode = Gamemode::Creative,
				2 => parsed_gamemode = Gamemode::Adventure,
				3 => parsed_gamemode = Gamemode::Spectator,
				_ => (),
			};
		};

		let entity_id = game.entity_id_manager.get_new();
		let player = Self {
			x: player_data.get_child("Pos").unwrap().as_list()[0].as_double(),
			y: player_data.get_child("Pos").unwrap().as_list()[1].as_double(),
			z: player_data.get_child("Pos").unwrap().as_list()[2].as_double(),
			yaw: player_data.get_child("Rotation").unwrap().as_list()[0].as_float(),
			pitch: player_data.get_child("Rotation").unwrap().as_list()[1].as_float(),
			velocity: EntityPosition::default(),
			display_name,
			uuid,
			peer_socket_address,
			connection_stream,
			entity_id,
			waiting_for_confirm_teleportation: false,
			current_teleport_id: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
				/ (entity_id + 1 + 12345) as u64) as i32, //should probably use random number instead
			inventory,
			selected_slot: player_data.get_child("SelectedItemSlot").unwrap().as_int() as u8,
			opened_inventory_at: None,
			cursor_item: None,
			is_sneaking: false,
			chat_message_index: 0,
			gamemode: parsed_gamemode,
			mining_for_ticks: 0,
			is_mining: false,
			health: 20.0,
			is_dead: false,
		};

		return player;
	}

	pub fn save_to_disk(&self) {
		if !fs::exists(Player::get_playerdata_path(self.uuid).parent().unwrap()).unwrap() {
			fs::create_dir_all(Player::get_playerdata_path(self.uuid).parent().unwrap()).unwrap();
		}

		let mut file =
			OpenOptions::new().read(true).write(true).truncate(true).create(true).open(Player::get_playerdata_path(self.uuid)).unwrap();

		let empty_slot = Slot {
			item_count: 0,
			item_id: 0,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let player_data = NbtTag::Root(vec![
			NbtTag::List("Pos".to_string(), vec![NbtListTag::Double(self.x), NbtListTag::Double(self.y), NbtListTag::Double(self.z)]),
			NbtTag::List("Rotation".to_string(), vec![NbtListTag::Float(self.yaw), NbtListTag::Float(self.pitch)]),
			NbtTag::Int("SelectedItemSlot".to_string(), self.selected_slot as i32),
			NbtTag::Int("playerGameType".to_string(), self.gamemode as u8 as i32),
			NbtTag::List(
				"Inventory".to_string(),
				self
					.inventory
					.iter()
					.enumerate()
					.filter(|x| x.0 >= 9)
					.filter(|x| x.1.is_some())
					.map(|x| {
						NbtListTag::TagCompound(vec![
							NbtTag::Byte("Slot".to_string(), x.0 as u8),
							NbtTag::Int("count".to_string(), x.1.as_ref().unwrap().item_count),
							NbtTag::String("id".to_string(), data::items::get_item_name_by_id(x.1.as_ref().unwrap().item_id).to_string()),
						])
					})
					.collect(),
			),
			NbtTag::TagCompound(
				"equipment".to_string(),
				vec![
					NbtTag::TagCompound(
						"head".to_string(),
						vec![
							NbtTag::Int("count".to_string(), self.inventory[5].as_ref().unwrap_or(&empty_slot).item_count),
							NbtTag::String(
								"id".to_string(),
								data::items::get_item_name_by_id(self.inventory[5].as_ref().unwrap_or(&empty_slot).item_id).to_string(),
							),
						],
					),
					NbtTag::TagCompound(
						"chest".to_string(),
						vec![
							NbtTag::Int("count".to_string(), self.inventory[6].as_ref().unwrap_or(&empty_slot).item_count),
							NbtTag::String(
								"id".to_string(),
								data::items::get_item_name_by_id(self.inventory[6].as_ref().unwrap_or(&empty_slot).item_id).to_string(),
							),
						],
					),
					NbtTag::TagCompound(
						"legs".to_string(),
						vec![
							NbtTag::Int("count".to_string(), self.inventory[7].as_ref().unwrap_or(&empty_slot).item_count),
							NbtTag::String(
								"id".to_string(),
								data::items::get_item_name_by_id(self.inventory[7].as_ref().unwrap_or(&empty_slot).item_id).to_string(),
							),
						],
					),
					NbtTag::TagCompound(
						"feet".to_string(),
						vec![
							NbtTag::Int("count".to_string(), self.inventory[8].as_ref().unwrap_or(&empty_slot).item_count),
							NbtTag::String(
								"id".to_string(),
								data::items::get_item_name_by_id(self.inventory[8].as_ref().unwrap_or(&empty_slot).item_id).to_string(),
							),
						],
					),
					NbtTag::TagCompound(
						"offhand".to_string(),
						vec![
							NbtTag::Int("count".to_string(), self.inventory[45].as_ref().unwrap_or(&empty_slot).item_count),
							NbtTag::String(
								"id".to_string(),
								data::items::get_item_name_by_id(self.inventory[45].as_ref().unwrap_or(&empty_slot).item_id).to_string(),
							),
						],
					),
				],
			),
		]);

		let mut uncompressed_data = crate::serialize::nbt_disk(player_data);
		let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
		encoder.write_all(uncompressed_data.as_mut_slice()).unwrap();
		let new_file_content = encoder.finish().unwrap();
		file.write_all(&new_file_content).unwrap();
		file.flush().unwrap();
	}

	pub fn get_held_item(&self, main_hand: bool) -> Option<&Slot> {
		if main_hand {
			return self.inventory.get(36 + self.selected_slot as usize).unwrap().as_ref();
		} else {
			return self.inventory.get(45).unwrap().as_ref();
		}
	}

	pub fn get_looking_cardinal_direction(&self) -> CardinalDirection {
		let yaw = self.yaw;
		let cardinal_direction: CardinalDirection;
		if yaw >= 0.0 {
			if (135.0..225.0).contains(&yaw) {
				cardinal_direction = CardinalDirection::North;
			} else if (225.0..315.0).contains(&yaw) {
				cardinal_direction = CardinalDirection::East;
			} else if !(45.0..315.0).contains(&yaw) {
				cardinal_direction = CardinalDirection::South;
			} else {
				cardinal_direction = CardinalDirection::West;
			}
		} else if yaw <= -135.0 && yaw > -225.0 {
			cardinal_direction = CardinalDirection::North;
		} else if (-135.0..-45.0).contains(&yaw) {
			cardinal_direction = CardinalDirection::East;
		} else if yaw <= -315.0 || yaw > -45.0 {
			cardinal_direction = CardinalDirection::South;
		} else {
			cardinal_direction = CardinalDirection::West;
		}
		return cardinal_direction;
	}

	pub fn new_position(
		&mut self,
		x: f64,
		y: f64,
		z: f64,
		world: &mut World,
		entity_id_manger: &EntityIdManager,
		block_states: &HashMap<String, Block>,
		game: Arc<Game>,
	) -> Result<EntityPosition, Box<dyn Error>> {
		let old_x = self.x;
		let old_z = self.z;

		self.x = x;
		self.y = y;
		self.z = z;

		let old_chunk_position = BlockPosition {
			x: old_x as i32,
			y: 0,
			z: old_z as i32,
		}
		.convert_to_coordinates_of_chunk();
		let new_chunk_position = BlockPosition {
			x: self.x as i32,
			y: 0,
			z: self.z as i32,
		}
		.convert_to_coordinates_of_chunk();

		if old_chunk_position != new_chunk_position {
			game.send_packet(
				&self.peer_socket_address,
				crate::packets::clientbound::play::SetCenterChunk::PACKET_ID,
				crate::packets::clientbound::play::SetCenterChunk {
					chunk_x: new_chunk_position.x,
					chunk_z: new_chunk_position.z,
				}
				.try_into()?,
			);

			let old_chunk_coords: Vec<(i32, i32)> = (old_chunk_position.x - crate::VIEW_DISTANCE as i32
				..=old_chunk_position.x + crate::VIEW_DISTANCE as i32)
				.flat_map(|x| {
					(old_chunk_position.z - crate::VIEW_DISTANCE as i32..=old_chunk_position.z + crate::VIEW_DISTANCE as i32)
						.map(|z| (x, z))
						.collect::<Vec<(i32, i32)>>()
				})
				.collect();

			let new_chunk_coords: Vec<(i32, i32)> = (new_chunk_position.x - crate::VIEW_DISTANCE as i32
				..=new_chunk_position.x + crate::VIEW_DISTANCE as i32)
				.flat_map(|x| {
					(new_chunk_position.z - crate::VIEW_DISTANCE as i32..=new_chunk_position.z + crate::VIEW_DISTANCE as i32)
						.map(|z| (x, z))
						.collect::<Vec<(i32, i32)>>()
				})
				.collect();

			let chunks_missing: Vec<(i32, i32)> = new_chunk_coords.into_iter().filter(|x| !old_chunk_coords.contains(x)).collect();

			for chunk_coords in chunks_missing {
				self.send_chunk(world, chunk_coords.0, chunk_coords.1, entity_id_manger, block_states, game.clone())?;
			}
		}

		return Ok(self.get_position());
	}

	pub fn new_position_and_rotation(
		&mut self,
		new_position: EntityPosition,
		world: &mut World,
		entity_id_manger: &EntityIdManager,
		block_states: &HashMap<String, Block>,
		game: Arc<Game>,
	) -> Result<EntityPosition, Box<dyn Error>> {
		self.yaw = new_position.yaw;
		self.pitch = new_position.pitch;
		self.new_position(new_position.x, new_position.y, new_position.z, world, entity_id_manger, block_states, game)?;

		return Ok(self.get_position());
	}

	pub fn new_rotation(&mut self, yaw: f32, pitch: f32) -> EntityPosition {
		self.yaw = yaw;
		self.pitch = pitch;

		return self.get_position();
	}

	pub fn get_pitch(&self) -> f32 {
		return self.pitch;
	}

	pub fn send_chunk(
		&mut self,
		world: &mut World,
		chunk_x: i32,
		chunk_z: i32,
		entity_id_manger: &EntityIdManager,
		block_states: &HashMap<String, Block>,
		game: Arc<Game>,
	) -> Result<(), Box<dyn Error>> {
		let dimension = &mut world.dimensions.get_mut("minecraft:overworld").unwrap();
		let chunk = dimension.get_chunk_from_chunk_position(BlockPosition {
			x: chunk_x,
			y: 0,
			z: chunk_z,
		});
		let chunk = if let Some(chunk) = chunk {
			chunk
		} else {
			let new_chunk = (*world.loader).load_chunk(chunk_x, chunk_z, block_states);
			dimension.chunks.insert((new_chunk.x, new_chunk.z), new_chunk);

			let mut new_entities = (*world.loader).load_entities_in_chunk(chunk_x, chunk_z, entity_id_manger);
			dimension.entities.append(&mut new_entities);

			dimension
				.get_chunk_from_chunk_position(BlockPosition {
					x: chunk_x,
					y: 0,
					z: chunk_z,
				})
				.unwrap()
		};
		let all_chunk_sections = &chunk.sections;

		let all_processed_chunk_sections = all_chunk_sections
			.iter()
			.map(|section| {
				let different_blocks = section.blocks.iter().copied().collect::<BTreeSet<u16>>();

				crate::packets::clientbound::play::ChunkSection {
					block_count: section.get_non_air_block_count(),
					block_states: if section.blocks.is_empty() {
						crate::packets::clientbound::play::BlockStatesPalettedContainer::SingleValued(crate::packets::clientbound::play::SingleValued {
							bits_per_entry: 0,
							value: 0,
						})
					} else if different_blocks.len() <= 256 {
						let bits_per_entry = match different_blocks.len() {
							0..=16 => 4,
							17..=32 => 5,
							33..=64 => 6,
							65..=128 => 7,
							_ => 8,
						};

						let palette: Vec<i32> = different_blocks.iter().map(|x| *x as i32).collect();
						let data_array: Vec<i32> = section.blocks.iter().map(|x| palette.binary_search(&(*x as i32)).unwrap() as i32).collect();

						crate::packets::clientbound::play::BlockStatesPalettedContainer::Indirect(crate::packets::clientbound::play::Indirect {
							bits_per_entry,
							data_array,
							palette,
						})
					} else {
						crate::packets::clientbound::play::BlockStatesPalettedContainer::Direct(crate::packets::clientbound::play::Direct {
							bits_per_entry: 15,
							data_array: section.blocks.iter().map(|x| *x as i32).collect(),
						})
					},
					biomes: crate::packets::clientbound::play::BiomesPalettedContainer::Direct(crate::packets::clientbound::play::Direct {
						bits_per_entry: 7,
						data_array: section.biomes.iter().map(|x| *x as i32).collect(),
					}),
				}
			})
			.collect();

		let mut sky_light_mask = 0u64;
		let mut block_light_mask = 0u64;
		let mut sky_light_arrays: Vec<Vec<u8>> = Vec::new();
		let mut block_light_arrays: Vec<Vec<u8>> = Vec::new();
		for section in all_chunk_sections.iter().rev() {
			if section.sky_lights.is_empty() {
				sky_light_mask += 0;
			} else {
				sky_light_mask += 1;
				sky_light_arrays.push(section.sky_lights.clone());
			}
			sky_light_mask <<= 1;
			if section.block_lights.is_empty() {
				block_light_mask += 0;
			} else {
				block_light_mask += 1;
				block_light_arrays.push(section.block_lights.clone());
			}
			block_light_mask <<= 1;
		}

		let block_entity_types = data::blockentity::get_block_entity_types();
		let block_entities: Vec<crate::packets::clientbound::play::BlockEntity> = chunk
			.block_entities
			.iter()
			.map(|x| crate::packets::clientbound::play::BlockEntity {
				packed_xz: (x.get_position().convert_to_position_in_chunk().x as u8 & 0x0f) << 4
					| x.get_position().convert_to_position_in_chunk().z as u8 & 0x0f,
				y: x.get_position().y,
				block_entity_type: *block_entity_types.get(Into::<&str>::into(x.get_id().as_str())).unwrap() as i32,
				data: Some(NbtTag::Root(x.clone().into())),
			})
			.collect();

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::ChunkDataAndUpdateLight::PACKET_ID,
			crate::packets::clientbound::play::ChunkDataAndUpdateLight {
				chunk_x,
				chunk_z,
				heightmaps: vec![],
				data: all_processed_chunk_sections,
				block_entities,
				sky_light_mask: vec![sky_light_mask],
				block_light_mask: vec![block_light_mask],
				empty_sky_light_mask: vec![!sky_light_mask],
				empty_block_light_mask: vec![!block_light_mask],
				sky_light_arrays,
				block_light_arrays,
			}
			.try_into()?,
		);

		return Ok(());
	}

	fn get_playerdata_path(uuid: u128) -> PathBuf {
		let mut path = PathBuf::new();
		path.push(Path::new("./world/playerdata/"));
		path.push(Path::new(&crate::utils::u128_to_uuid_with_dashes(uuid)));
		path.set_extension("dat");
		return path;
	}

	pub fn get_selected_slot(&self) -> u8 {
		return self.selected_slot;
	}

	pub fn set_selected_slot(&mut self, slot: u8, players: &[Player], game: Arc<Game>) {
		self.selected_slot = slot;

		players.iter().filter(|x| x.uuid != self.uuid).for_each(|x| {
			game.send_packet(
				&x.peer_socket_address,
				crate::packets::clientbound::play::SetEquipment::PACKET_ID,
				crate::packets::clientbound::play::SetEquipment {
					entity_id: self.entity_id,
					equipment: vec![(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone())],
				}
				.try_into()
				.unwrap(),
			);
		});
	}

	pub fn get_inventory(&self) -> &Vec<Option<Slot>> {
		return &self.inventory;
	}

	pub fn set_selected_inventory_slot(&mut self, item: Option<Slot>, players: &[Player], game: Arc<Game>) {
		self.set_inventory_slot(self.get_selected_slot() + 36, item, players, game);
	}

	pub fn set_inventory_slot(&mut self, slot: u8, item: Option<Slot>, players: &[Player], game: Arc<Game>) {
		self.inventory[slot as usize] = item.clone();

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SetPlayerInventorySlot::PACKET_ID,
			crate::packets::clientbound::play::SetPlayerInventorySlot {
				slot_data: self.get_inventory()[(self.get_selected_slot() + 36) as usize].clone(),
				slot: (self.get_selected_slot()) as i32,
			}
			.try_into()
			.unwrap(),
		);

		if (slot <= 4 || (9..=44).contains(&slot)) && self.get_selected_slot() + 36 != slot {
			return;
		}

		players.iter().filter(|x| x.uuid != self.uuid).for_each(|x| {
			let mut equipment: Vec<(u8, Option<Slot>)> = vec![(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone())];

			if [5, 6, 7, 8, 45].contains(&slot) {
				equipment.push(match slot {
					5 => (5, item.clone()),
					6 => (4, item.clone()),
					7 => (3, item.clone()),
					8 => (2, item.clone()),
					45 => (1, item.clone()),
					_ => panic!(""),
				});
			};

			game.send_packet(
				&x.peer_socket_address,
				crate::packets::clientbound::play::SetEquipment::PACKET_ID,
				crate::packets::clientbound::play::SetEquipment {
					entity_id: self.entity_id,
					equipment,
				}
				.try_into()
				.unwrap(),
			);
		});
	}
	pub fn set_inventory_and_inform_client(&mut self, items: Vec<Option<Slot>>, players: &[Player], game: Arc<Game>) {
		self.set_inventory_and_dont_inform_client(items);

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SetContainerContent::PACKET_ID,
			crate::packets::clientbound::play::SetContainerContent {
				window_id: 0,
				state_id: 1,
				slot_data: self.inventory.clone(),
				carried_item: None,
			}
			.try_into()
			.unwrap(),
		);

		players.iter().filter(|x| x.uuid != self.uuid).for_each(|x| {
			let equipment: Vec<(u8, Option<Slot>)> = vec![
				(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone()),
				(1, self.inventory[45].clone()),
				(2, self.inventory[8].clone()),
				(3, self.inventory[7].clone()),
				(4, self.inventory[6].clone()),
				(5, self.inventory[5].clone()),
			];

			game.send_packet(
				&x.peer_socket_address,
				crate::packets::clientbound::play::SetEquipment::PACKET_ID,
				crate::packets::clientbound::play::SetEquipment {
					entity_id: self.entity_id,
					equipment,
				}
				.try_into()
				.unwrap(),
			);
		});
	}

	pub fn set_inventory_and_dont_inform_client(&mut self, items: Vec<Option<Slot>>) {
		self.inventory = items.clone();
	}

	pub fn open_inventory(&mut self, inventory: data::inventory::Inventory, block_entity: &BlockEntity, game: Arc<Game>) {
		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::OpenScreen::PACKET_ID,
			crate::packets::clientbound::play::OpenScreen {
				window_id: 1,
				window_type: inventory as i32,
				window_title: NbtTag::Root(vec![NbtTag::String("text".to_string(), "".to_string())]),
			}
			.try_into()
			.unwrap(),
		);

		self.opened_inventory_at = Some(block_entity.get_position());
		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SetContainerContent::PACKET_ID,
			crate::packets::clientbound::play::SetContainerContent {
				window_id: 1,
				state_id: 1,
				slot_data: block_entity.get_contained_items_owned().into_iter().map(Into::into).collect(),
				carried_item: None,
			}
			.try_into()
			.unwrap(),
		);
	}

	pub fn close_inventory(&mut self, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
		self.opened_inventory_at = None;

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::CloseContainer::PACKET_ID,
			crate::packets::clientbound::play::CloseContainer {
				window_id: 1,
			}
			.try_into()?,
		);

		return Ok(());
	}

	pub fn is_sneaking(&self) -> bool {
		return self.is_sneaking;
	}

	pub fn set_sneaking(&mut self, is_sneaking: bool, players: &[Player], game: Arc<Game>) {
		//No need to do anything if is_sneaking didnt change
		if self.is_sneaking == is_sneaking {
			return;
		}

		self.is_sneaking = is_sneaking;

		for player in players {
			if player.uuid == self.uuid {
				continue;
			}

			game.send_packet(
				&player.peer_socket_address,
				crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
				crate::packets::clientbound::play::SetEntityMetadata {
					entity_id: self.entity_id,
					metadata: vec![EntityMetadata {
						index: 6,
						value: EntityMetadataValue::Pose(if self.is_sneaking { 5 } else { 0 }),
					}],
				}
				.try_into()
				.unwrap(),
			);
		}
	}

	pub fn get_position(&self) -> EntityPosition {
		return EntityPosition {
			x: self.x,
			y: self.y,
			z: self.z,
			yaw: self.yaw,
			pitch: self.pitch,
		};
	}

	pub fn set_gamemode(&mut self, gamemode: Gamemode, players: &[Player], game: Arc<Game>) -> Result<(), Box<dyn Error>> {
		self.gamemode = gamemode;

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::GameEvent::PACKET_ID,
			crate::packets::clientbound::play::GameEvent {
				event: 3,
				value: self.gamemode as u8 as f32,
			}
			.try_into()?,
		);

		players.iter().for_each(|player| {
			game.send_packet(
				&player.peer_socket_address,
				crate::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID,
				crate::packets::clientbound::play::PlayerInfoUpdate {
					actions: 0x04,
					players: vec![(self.uuid, vec![PlayerAction::UpdateGameMode(self.gamemode as u8 as i32)])],
				}
				.try_into()
				.unwrap(),
			);
		});

		return Ok(());
	}

	pub fn get_gamemode(&self) -> Gamemode {
		return self.gamemode;
	}

	pub fn start_mining(&mut self) {
		self.mining_for_ticks = 1;
		self.is_mining = true;
	}

	pub fn finish_mining(&mut self) {
		self.mining_for_ticks = 0;
		self.is_mining = false;
	}

	pub fn get_mining_for_ticks(&self) -> u16 {
		return self.mining_for_ticks;
	}

	pub fn get_is_mining(&self) -> bool {
		return self.is_mining;
	}

	//returns true when item pickup was succesfull and false when not
	pub fn pickup_item(&mut self, item: Item, item_entity_id: i32, players: &[Player], game: Arc<Game>) -> bool {
		if self.is_dead {
			return false;
		}

		let slot = Slot::from(item.clone());

		let slot_indecies = [
			36, 37, 38, 39, 40, 41, 42, 43, 44, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
			33, 34, 35,
		];

		let mut inventory = self.get_inventory().clone();
		let mut inventory_updated = false;

		//first check if we can stack up an existing slot
		for slot_index in slot_indecies {
			#[allow(clippy::collapsible_if)]
			if let Some(inventory_slot) = &mut inventory[slot_index] {
				if inventory_slot.item_id == slot.item_id
					&& inventory_slot.item_count
						< data::items::get_items().get(data::items::get_item_name_by_id(inventory_slot.item_id)).unwrap().max_stack_size as i32
				{
					inventory_slot.item_count += 1;
					inventory_updated = true;

					break;
				}
			}
		}

		//if the player doesnt have the item in its inventory, put it in the first free slot
		if !inventory_updated {
			for slot_index in slot_indecies {
				if inventory[slot_index].as_ref().is_some_and(|x| x.item_count == 0) || inventory[slot_index].is_none() {
					inventory[slot_index] = Some(slot.clone());
					inventory_updated = true;
					break;
				}
			}
		}

		if inventory_updated {
			self.set_inventory_and_inform_client(inventory, players, game.clone());

			let pickup_item_packet = crate::packets::clientbound::play::PickupItem {
				collected_entity_id: item_entity_id,
				collector_entity_id: self.entity_id,
				pickup_item_count: item.count as i32,
			};

			for player in players {
				game.send_packet(
					&player.peer_socket_address,
					crate::packets::clientbound::play::PickupItem::PACKET_ID,
					pickup_item_packet.clone().try_into().unwrap(),
				);
			}
		}

		return inventory_updated;
	}

	pub fn damage(&mut self, damage: f32, game: Arc<Game>, players: &[Player], dimension: &mut Dimension) {
		self.health -= damage;

		if self.health <= 0.0 {
			self.die(game.clone(), players, dimension);
		}

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SetHealth::PACKET_ID,
			crate::packets::clientbound::play::SetHealth {
				health: self.health,
				food: 20,
				food_saturation: 0.0,
			}
			.try_into()
			.unwrap(),
		);

		let hurt_animation_packet = crate::packets::clientbound::play::HurtAnimation {
			entity_id: self.entity_id,
			yaw: 0.0,
		};

		players.iter().for_each(|x| {
			game.send_packet(
				&x.peer_socket_address,
				crate::packets::clientbound::play::HurtAnimation::PACKET_ID,
				hurt_animation_packet.clone().try_into().unwrap(),
			);
		});
	}

	pub fn die(&mut self, game: Arc<Game>, players: &[Player], dimension: &mut Dimension) {
		self.is_dead = true;
		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::CombatDeath::PACKET_ID,
			crate::packets::clientbound::play::CombatDeath {
				player_id: self.entity_id,
				message: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "haha you dieded".to_string()),
				]),
			}
			.try_into()
			.unwrap(),
		);

		if self.gamemode == Gamemode::Survival || self.gamemode == Gamemode::Adventure {
			for slot in &self.inventory {
				if slot.as_ref().is_some_and(|x| x.item_count > 0) {
					let new_entity = crate::entity::ItemEntity {
						common: CommonEntity {
							position: self.get_position(),
							velocity: EntityPosition::default(),
							uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
							entity_id: game.entity_id_manager.get_new(),
							..Default::default()
						},
						age: 0,
						health: 5,
						item: Item::from(slot.clone().unwrap()),
						owner: self.uuid,
						pickup_delay: 0,
						thrower: self.uuid,
					};

					let spawn_packet = new_entity.to_spawn_entity_packet();

					let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
						entity_id: new_entity.get_common_entity_data().entity_id,
						metadata: new_entity.get_metadata(),
					};

					dimension.add_entity(Entity::Item(new_entity));

					players.iter().for_each(|x| {
						game.send_packet(
							&x.peer_socket_address,
							crate::packets::clientbound::play::SpawnEntity::PACKET_ID,
							spawn_packet.clone().try_into().unwrap(),
						);
						game.send_packet(
							&x.peer_socket_address,
							crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
							metadata_packet.clone().try_into().unwrap(),
						);
					});
				}
			}

			players.iter().filter(|x| x.uuid != self.uuid).for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					crate::packets::clientbound::play::RemoveEntities::PACKET_ID,
					crate::packets::clientbound::play::RemoveEntities {
						entity_ids: vec![self.entity_id],
					}
					.try_into()
					.unwrap(),
				);
			});

			self.inventory = vec![None; 46];
		}
	}

	pub fn respawn(&mut self, game: Arc<Game>, players: &[Player], world: &mut World) {
		self.health = 20.0;

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::Respawn::PACKET_ID,
			crate::packets::clientbound::play::Respawn {
				dimension_type: 0,
				dimension_name: "minecraft:overworld".to_string(),
				hashed_seed: 1,
				game_mode: self.get_gamemode(),
				previous_gamemode: self.get_gamemode() as i8,
				is_debug: false,
				is_flat: false,
				has_death_location: true,
				death_dimension_name: Some("minecraft:overworld".to_string()),
				death_location: Some(self.get_position().into()),
				portal_cooldown: 123,
				sea_level: 64,
				data_kept: 0x00,
			}
			.try_into()
			.unwrap(),
		);

		self
			.new_position(
				world.default_spawn_location.x as f64,
				world.default_spawn_location.y as f64,
				world.default_spawn_location.z as f64,
				world,
				&game.entity_id_manager,
				&game.block_state_data,
				game.clone(),
			)
			.unwrap();

		for other_stream in players.iter().map(|x| &x.connection_stream).collect::<Vec<&TcpStream>>() {
			if other_stream.peer_addr().unwrap() != self.peer_socket_address {
				game.send_packet(
					&other_stream.peer_addr().unwrap(),
					crate::packets::clientbound::play::SpawnEntity::PACKET_ID,
					crate::packets::clientbound::play::SpawnEntity {
						entity_id: self.entity_id,
						entity_uuid: self.uuid,
						entity_type: data::entities::get_id_from_name("minecraft:player"),
						x: self.get_position().x,
						y: self.get_position().y,
						z: self.get_position().z,
						pitch: self.get_pitch_u8(),
						yaw: self.get_yaw_u8(),
						head_yaw: self.get_yaw_u8(),
						data: 0,
						velocity_x: 0,
						velocity_y: 0,
						velocity_z: 0,
					}
					.try_into()
					.unwrap(),
				);
			}
		}

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID,
			crate::packets::clientbound::play::SynchronizePlayerPosition {
				teleport_id: self.current_teleport_id,
				x: self.get_position().x,
				y: self.get_position().y,
				z: self.get_position().z,
				velocity_x: 0.0,
				velocity_y: 0.0,
				velocity_z: 0.0,
				yaw: self.get_position().yaw,
				pitch: self.get_position().pitch,
				flags: 0,
			}
			.try_into()
			.unwrap(),
		);

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::GameEvent::PACKET_ID,
			crate::packets::clientbound::play::GameEvent {
				event: 13,
				value: 0.0,
			}
			.try_into()
			.unwrap(),
		);

		self.is_dead = false;

		game.send_packet(
			&self.peer_socket_address,
			crate::packets::clientbound::play::SetContainerContent::PACKET_ID,
			crate::packets::clientbound::play::SetContainerContent {
				window_id: 0,
				state_id: 1,
				slot_data: self.get_inventory().clone(),
				carried_item: None,
			}
			.try_into()
			.unwrap(),
		);

		let current_chunk_coords = BlockPosition::from(self.get_position()).convert_to_coordinates_of_chunk();

		for x in current_chunk_coords.x - crate::VIEW_DISTANCE as i32..=current_chunk_coords.x + crate::VIEW_DISTANCE as i32 {
			for z in current_chunk_coords.z - crate::VIEW_DISTANCE as i32..=current_chunk_coords.z + crate::VIEW_DISTANCE as i32 {
				self.send_chunk(world, x, z, &game.entity_id_manager, &game.block_state_data, game.clone()).unwrap();
			}
		}
	}
}
