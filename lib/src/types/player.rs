use super::*;
use crate::{ConnectionState, packets::*};
use std::{collections::HashMap, error::Error, fs::{File, OpenOptions}, io::prelude::*, path::{Path, PathBuf}};
use std::net::{SocketAddr, TcpStream};
use std::fs;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Debug)]
pub struct Player {
  x: f64,
  y: f64,
  z: f64,
  yaw: f32,
  pitch: f32,
  pub display_name: String,
  pub uuid: u128,
  pub peer_socket_address: SocketAddr,
  pub connection_stream: TcpStream,
  pub entity_id: i32,
  pub waiting_for_confirm_teleportation: bool,
  pub current_teleport_id: i32,
  inventory: Vec<Option<Slot>>,
  selected_slot: u8,
  pub opened_container_at: Option<Position>,
  pub cursor_item: Option<Slot>,
}

//Manual implementation because TcpStream doesn't implement Clone, instead just call unwrap here on its try_clone() function
impl Clone for Player {
  fn clone(&self) -> Self {
    Self { x: self.x, y: self.y, z: self.z, yaw: self.yaw, pitch: self.pitch, display_name: self.display_name.clone(), uuid: self.uuid, peer_socket_address: self.peer_socket_address, connection_stream: self.connection_stream.try_clone().unwrap(), entity_id: self.entity_id, waiting_for_confirm_teleportation: self.waiting_for_confirm_teleportation, current_teleport_id: self.current_teleport_id, inventory: self.inventory.clone(), selected_slot: self.selected_slot, opened_container_at: self.opened_container_at, cursor_item: self.cursor_item.clone() }
  }
}

impl Player {
  pub fn new(display_name: String, uuid: u128, peer_socket_address: SocketAddr, game: &mut Game, connection_stream: TcpStream) -> Self {
    let Ok(mut file) = File::open(Player::get_playerdata_path(uuid)) else {
	  	let player = Self {
	      x: game.world.default_spawn_location.x as f64,
	      y: game.world.default_spawn_location.y as f64,
	      z: game.world.default_spawn_location.z as f64,
	      yaw: 0.0,
	      pitch: 0.0,
	      display_name,
	      uuid,
	      peer_socket_address,
	      connection_stream,
	      entity_id: game.last_created_entity_id + 1,
	      waiting_for_confirm_teleportation: false,
	      current_teleport_id: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() / (game.last_created_entity_id + 1 + 12345) as u64) as i32, //should probably use random number instead
	      inventory: vec![None; 46],
	      selected_slot: 0,
				opened_container_at: None,
				cursor_item: None,
	    };

	    game.last_created_entity_id += 1;

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
        components_to_remove: Vec::new()
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

  	let player = Self {
      x: player_data.get_child("Pos").unwrap().as_list()[0].as_double(),
      y: player_data.get_child("Pos").unwrap().as_list()[1].as_double(),
      z: player_data.get_child("Pos").unwrap().as_list()[2].as_double(),
      yaw: player_data.get_child("Rotation").unwrap().as_list()[0].as_float(),
      pitch: player_data.get_child("Rotation").unwrap().as_list()[1].as_float(),
      display_name,
      uuid,
      peer_socket_address,
      connection_stream,
      entity_id: game.last_created_entity_id + 1,
      waiting_for_confirm_teleportation: false,
      current_teleport_id: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() / (game.last_created_entity_id + 1 + 12345) as u64) as i32, //should probably use random number instead
      inventory,
      selected_slot: player_data.get_child("SelectedItemSlot").unwrap().as_int() as u8,
      opened_container_at: None,
      cursor_item: None
    };

    game.last_created_entity_id += 1;

    return player;
  }

  pub fn save_to_disk(&self) {
    if !fs::exists(Player::get_playerdata_path(self.uuid).parent().unwrap()).unwrap() {
      fs::create_dir_all(Player::get_playerdata_path(self.uuid).parent().unwrap()).unwrap();
    }

  	let mut file = OpenOptions::new()
    	.read(true)
     	.write(true)
	    .truncate(true)
	    .create(true)
	    .open(Player::get_playerdata_path(self.uuid))
	    .unwrap();

    let empty_slot = Slot { item_count: 0, item_id: 0, components_to_add: Vec::new(), components_to_remove: Vec::new() };
	  let player_data = NbtTag::Root(vec![
			NbtTag::List("Pos".to_string(), vec![
				NbtListTag::Double(self.x),
				NbtListTag::Double(self.y),
				NbtListTag::Double(self.z),
			]),
			NbtTag::List("Rotation".to_string(), vec![
				NbtListTag::Float(self.yaw),
				NbtListTag::Float(self.pitch),
			]),
			NbtTag::Int("SelectedItemSlot".to_string(), self.selected_slot as i32),
			NbtTag::List("Inventory".to_string(), self.inventory.iter().enumerate().filter(|x| x.0 >= 9).filter(|x| x.1.is_some()).map(|x| {
				NbtListTag::TagCompound(vec![
					NbtTag::Byte("Slot".to_string(), x.0 as u8),
					NbtTag::Int("count".to_string(), x.1.as_ref().unwrap().item_count),
					NbtTag::String("id".to_string(), data::items::get_item_name_by_id(x.1.as_ref().unwrap().item_id)),
				])
			}).collect()),
			NbtTag::TagCompound("equipment".to_string(), vec![
				NbtTag::TagCompound("head".to_string(), vec![
					NbtTag::Int("count".to_string(), self.inventory[5].as_ref().unwrap_or(&empty_slot).item_count),
					NbtTag::String("id".to_string(), data::items::get_item_name_by_id(self.inventory[5].as_ref().unwrap_or(&empty_slot).item_id)),
				]),
				NbtTag::TagCompound("chest".to_string(), vec![
  				NbtTag::Int("count".to_string(), self.inventory[6].as_ref().unwrap_or(&empty_slot).item_count),
  				NbtTag::String("id".to_string(), data::items::get_item_name_by_id(self.inventory[6].as_ref().unwrap_or(&empty_slot).item_id)),
				]),
				NbtTag::TagCompound("legs".to_string(), vec![
  				NbtTag::Int("count".to_string(), self.inventory[7].as_ref().unwrap_or(&empty_slot).item_count),
  				NbtTag::String("id".to_string(), data::items::get_item_name_by_id(self.inventory[7].as_ref().unwrap_or(&empty_slot).item_id)),
				]),
				NbtTag::TagCompound("feet".to_string(), vec![
  				NbtTag::Int("count".to_string(), self.inventory[8].as_ref().unwrap_or(&empty_slot).item_count),
  				NbtTag::String("id".to_string(), data::items::get_item_name_by_id(self.inventory[8].as_ref().unwrap_or(&empty_slot).item_id)),
				]),
				NbtTag::TagCompound("offhand".to_string(), vec![
  				NbtTag::Int("count".to_string(), self.inventory[45].as_ref().unwrap_or(&empty_slot).item_count),
  				NbtTag::String("id".to_string(), data::items::get_item_name_by_id(self.inventory[45].as_ref().unwrap_or(&empty_slot).item_id)),
				]),
			]),
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

  //chunk loading only works when moving one chunk at a time and falls apart when teleporting. Keep track of chunks sent to player https://git.thetxt.io/thetxt/oxide/issues/24
  pub fn new_position(&mut self, x: f64, y: f64, z: f64, world: &mut World) -> Result<(), Box<dyn Error>> {
  	let old_x = self.x;
   	let old_z = self.z;

  	self.x = x;
   	self.y = y;
    self.z = z;

    let old_chunk_position = Position {x: old_x as i32, y: 0, z: old_z as i32}.convert_to_coordinates_of_chunk();
    let new_chunk_position = Position {x: self.x as i32, y: 0, z: self.z as i32}.convert_to_coordinates_of_chunk();

    if old_chunk_position != new_chunk_position {
    	crate::utils::send_packet(&self.connection_stream, crate::packets::clientbound::play::SetCenterChunk::PACKET_ID, crate::packets::clientbound::play::SetCenterChunk {
	   		chunk_x: new_chunk_position.x,
	     	chunk_z: new_chunk_position.z,
     	}.try_into()?)?;

     	let temp_chunk_coords_to_send: (Vec<i32>, Vec<i32>) = if new_chunk_position.x > old_chunk_position.x {
      	let new_x = new_chunk_position.x + 10;
      	(vec![new_x;21], ((new_chunk_position.z - 10)..=(new_chunk_position.z + 10)).collect())
      } else if new_chunk_position.x < old_chunk_position.x {
      	let new_x = new_chunk_position.x - 10;
      	(vec![new_x;21], ((new_chunk_position.z - 10)..=(new_chunk_position.z + 10)).collect())
      } else if new_chunk_position.z > old_chunk_position.z {
	      let new_z = new_chunk_position.z + 10;
	     	(((new_chunk_position.x - 10)..=(new_chunk_position.x + 10)).collect(), vec![new_z;21])
      } else {
      	let new_z = new_chunk_position.z - 10;
	     	(((new_chunk_position.x - 10)..=(new_chunk_position.x + 10)).collect(), vec![new_z;21])
      };

      let chunk_coords_to_send: Vec<(i32, i32)> = temp_chunk_coords_to_send.0.iter().enumerate().map(|x| {
      	(temp_chunk_coords_to_send.0[x.0], temp_chunk_coords_to_send.1[x.0])
      }).collect();

      for chunk_coords in chunk_coords_to_send {
      	self.send_chunk(world, chunk_coords.0, chunk_coords.1)?;
      }
    }

    return Ok(());
  }

  pub fn new_position_and_rotation(&mut self, x: f64, y: f64, z: f64, yaw: f32, pitch: f32, world: &mut World) -> Result<(), Box<dyn Error>> {
    self.yaw = yaw;
    self.pitch = pitch;
 		self.new_position(x, y, z, world)?;

    return Ok(());
  }

  pub fn new_rotation(&mut self, yaw: f32, pitch: f32) {
    self.yaw = yaw;
    self.pitch = pitch;
  }

  pub fn send_chunk(&mut self, world: &mut World, chunk_x: i32, chunk_z: i32) -> Result<(), Box<dyn Error>> {
  	let dimension = &mut world.dimensions.get_mut("minecraft:overworld").unwrap();
	 	let chunk = dimension.get_chunk_from_chunk_position(Position { x: chunk_x, y: 0, z: chunk_z });
	  let chunk = if let Some(chunk) = chunk {
			chunk
		} else {
			let new_chunk = (*world.loader).load_chunk(chunk_x, chunk_z);
			dimension.chunks.push(new_chunk);
			dimension.get_chunk_from_chunk_position(Position { x: chunk_x, y: 0, z: chunk_z }).unwrap()
		};
		let all_chunk_sections = &chunk.sections;

	  let all_processed_chunk_sections = all_chunk_sections.iter().map(|section| {
	    crate::packets::clientbound::play::ChunkSection {
	      block_count: section.get_non_air_block_count(),
	      block_states: crate::packets::clientbound::play::BlockStatesPalettedContainer::Direct(crate::packets::clientbound::play::Direct {
	        bits_per_entry: 15,
	        data_array: if section.blocks.is_empty() { vec![0;4096] } else { section.blocks.iter().map(|x| *x as i32).collect() },
	      }),
	      biomes: crate::packets::clientbound::play::BiomesPalettedContainer::Direct(crate::packets::clientbound::play::Direct {
	        bits_per_entry: 7,
	        data_array: section.biomes.iter().map(|x| *x as i32).collect(),
	      }),
	    }
	  }).collect();

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
		let block_entities: Vec<crate::packets::clientbound::play::BlockEntity> = chunk.block_entities
		  .iter()
		  .map(|x| crate::packets::clientbound::play::BlockEntity {
        packed_xz: (x.position.convert_to_position_in_chunk().x as u8 & 0x0f) << 4 | x.position.convert_to_position_in_chunk().z as u8 & 0x0f,
        y: x.position.y,
        block_entity_type: *block_entity_types.get(Into::<&str>::into(x.id)).unwrap() as i32,
        data: Some(NbtTag::Root(x.data.clone().into())),
      })
		  .collect();

	  crate::utils::send_packet(&self.connection_stream, crate::packets::clientbound::play::ChunkDataAndUpdateLight::PACKET_ID, crate::packets::clientbound::play::ChunkDataAndUpdateLight {
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
	  }.try_into()?)?;

		return Ok(());
  }

  pub fn get_position_and_rotation_float(&self) -> (f64, f64, f64, f32, f32) {
  	return (self.x, self.y, self.z, self.yaw, self.pitch);
  }

  pub fn get_x(&self) -> f64 {
  	return self.x;
  }

  pub fn get_y(&self) -> f64 {
  	return self.y;
  }

  pub fn get_z(&self) -> f64 {
  	return self.z;
  }

  pub fn get_yaw(&self) -> f32 {
  	return self.yaw;
  }

  pub fn get_pitch(&self) -> f32 {
  	return self.pitch;
  }

  pub fn get_position(&self) -> Position {
	 	return Position {
		  x: self.get_x() as i32,
			y: self.get_y() as i16,
			z: self.get_z() as i32,
	  };
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

	pub fn set_selected_slot(&mut self, slot: u8, connections: &HashMap<SocketAddr, Connection>, connection_streams: &HashMap<SocketAddr, TcpStream>) {
		self.selected_slot = slot;

		connection_streams.iter()
	   	.filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
	   	.filter(|x| *x.0 != self.peer_socket_address)
	   	.for_each(|x| {
		   	crate::utils::send_packet(x.1, crate::packets::clientbound::play::SetEquipment::PACKET_ID, crate::packets::clientbound::play::SetEquipment {
		 			entity_id: self.entity_id,
		  		equipment: vec![
						(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone())
		     	]
		   	}.try_into().unwrap()).unwrap();
			}
		);
	}

	pub fn get_inventory(&self) -> &Vec<Option<Slot>> {
		return &self.inventory;
	}

	pub fn set_selected_inventory_slot(&mut self, item: Option<Slot>, connections: &HashMap<SocketAddr, Connection>, connection_streams: &HashMap<SocketAddr, TcpStream>) {
		self.set_inventory_slot(self.get_selected_slot() + 36, item, connections, connection_streams);
	}

	pub fn set_inventory_slot(&mut self, slot: u8, item: Option<Slot>, connections: &HashMap<SocketAddr, Connection>, connection_streams: &HashMap<SocketAddr, TcpStream>) {
		self.inventory[slot as usize] = item.clone();

  	crate::utils::send_packet(connection_streams.get(&self.peer_socket_address).unwrap(), crate::packets::clientbound::play::SetPlayerInventorySlot::PACKET_ID, crate::packets::clientbound::play::SetPlayerInventorySlot {
   		slot_data: self.get_inventory()[(self.get_selected_slot() + 36) as usize].clone(),
    	slot: (self.get_selected_slot()) as i32,
    }.try_into().unwrap()).unwrap();

		if (slot <= 4 || (9..=44).contains(&slot)) && self.get_selected_slot() + 36 != slot {
			return;
		}

		connection_streams.iter()
	   	.filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
	   	.filter(|x| *x.0 != self.peer_socket_address)
	   	.for_each(|x| {
				let mut equipment: Vec<(u8, Option<Slot>)> = vec![
					(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone())
				];

				if [5,6,7,8,45].contains(&slot) {
   				equipment.push(
						match slot {
		      		5 => (5, item.clone()),
		      		6 => (4, item.clone()),
		      		7 => (3, item.clone()),
		      		8 => (2, item.clone()),
		      		45 => (1, item.clone()),
		         	_ => panic!(""),
		       	}
       		);
				};

		   	crate::utils::send_packet(x.1, crate::packets::clientbound::play::SetEquipment::PACKET_ID, crate::packets::clientbound::play::SetEquipment {
		 			entity_id: self.entity_id,
		  		equipment,
		   	}.try_into().unwrap()).unwrap();
			}
		);
	}
	pub fn set_inventory(&mut self, items: Vec<Option<Slot>>, connections: &HashMap<SocketAddr, Connection>, connection_streams: &HashMap<SocketAddr, TcpStream>) {
		self.inventory = items.clone();

		crate::utils::send_packet(connection_streams.get(&self.peer_socket_address).unwrap(), crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
      window_id: 0,
      state_id: 1,
      slot_data: self.inventory.clone(),
      carried_item: None,
		}.try_into().unwrap()).unwrap();

		connection_streams.iter()
	   	.filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
	   	.filter(|x| *x.0 != self.peer_socket_address)
	   	.for_each(|x| {
				let equipment: Vec<(u8, Option<Slot>)> = vec![
					(0, self.inventory[(self.get_selected_slot() + 36) as usize].clone()),
					(1, self.inventory[45].clone()),
					(2, self.inventory[8].clone()),
					(3, self.inventory[7].clone()),
					(4, self.inventory[6].clone()),
					(5, self.inventory[5].clone()),
				];

		   	crate::utils::send_packet(x.1, crate::packets::clientbound::play::SetEquipment::PACKET_ID, crate::packets::clientbound::play::SetEquipment {
		 			entity_id: self.entity_id,
		  		equipment,
		   	}.try_into().unwrap()).unwrap();
			}
		);
	}

	pub fn open_inventory(&mut self, inventory: data::inventory::Inventory, block_entity: &BlockEntity) {
	  let _ = crate::utils::send_packet(&self.connection_stream, crate::packets::clientbound::play::OpenScreen::PACKET_ID, crate::packets::clientbound::play::OpenScreen {
      window_id: 1,
      window_type: inventory as i32,
      window_title: NbtTag::Root(vec![NbtTag::String("text".to_string(), "".to_string())]),
    }.try_into().unwrap());

	  self.opened_container_at = Some(block_entity.position);

    let _ = crate::utils::send_packet(&self.connection_stream, crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
      window_id: 1,
      state_id: 1,
      slot_data: match block_entity.clone().data {
        BlockEntityData::Chest(block_entity_data_items) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        BlockEntityData::Furnace(block_entity_data_items, _, _, _, _) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        BlockEntityData::BrewingStand(block_entity_data_items) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        BlockEntityData::Crafter(block_entity_data_items) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        BlockEntityData::Dispenser(block_entity_data_items) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        BlockEntityData::Hopper(block_entity_data_items) => {
          block_entity_data_items.into_iter().map(Into::into).collect()
        },
        _ => Vec::new(),
      },
      carried_item: None,
    }.try_into().unwrap());
	}

	pub fn close_inventory(&mut self) -> Result<(), Box<dyn Error>> {
	  self.opened_container_at = None;

		crate::utils::send_packet(&self.connection_stream, crate::packets::clientbound::play::CloseContainer::PACKET_ID, crate::packets::clientbound::play::CloseContainer {
      window_id: 1,
    }.try_into()?)?;

		return Ok(());
	}
}
