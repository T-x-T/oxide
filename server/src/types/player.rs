use lib::packets::Packet;

use super::*;
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct Player {
	//TODO: consider making position fields private, so we don't accidentally forget to call right methods for updating them
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub display_name: String,
  pub uuid: u128,
  pub peer_socket_address: SocketAddr,
  pub connection_stream: TcpStream,
  pub entity_id: i32,
  pub waiting_for_confirm_teleportation: bool,
  pub current_teleport_id: i32,
  pub inventory: Vec<Slot>,
  pub selected_slot: u8,
}

impl Player {
  pub fn new(position: Position, display_name: String, uuid: u128, peer_socket_address: SocketAddr, game: &mut Game, connection_stream: TcpStream) -> Self {
    let player = Self {
      x: position.x as f64,
      y: position.y as f64,
      z: position.z as f64,
      yaw: 0.0,
      pitch: 0.0,
      display_name,
      uuid,
      peer_socket_address,
      connection_stream,
      entity_id: game.last_created_entity_id + 1,
      waiting_for_confirm_teleportation: false,
      current_teleport_id: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() / (game.last_created_entity_id + 1 + 12345) as u64) as i32, //TODO: use random number instead
      inventory: vec![Slot { item_count: 0, item_id: None, components_to_add: Vec::new(), components_to_remove: Vec::new() }; 46],
      selected_slot: 0,
    };

    game.last_created_entity_id += 1;

    return player;
  }

  pub fn get_held_item(&self, main_hand: bool) -> &Slot {
    if main_hand {
      return self.inventory.get(36 + self.selected_slot as usize).unwrap();
    } else {
      return self.inventory.get(45).unwrap();
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

  //TODO: chunk loading only works when moving one chunk at a time and falls apart when teleporting. Keep track of chunks sent to player
  pub fn new_position(&mut self, x: f64, y: f64, z: f64, world: &mut World) {
  	let old_x = self.x;
   	let old_z = self.z;

  	self.x = x;
   	self.y = y;
    self.z = z;

    let old_chunk_position = Position {x: old_x as i32, y: 0, z: old_z as i32}.convert_to_coordinates_of_chunk();
    let new_chunk_position = Position {x: self.x as i32, y: 0, z: self.z as i32}.convert_to_coordinates_of_chunk();

    if old_chunk_position != new_chunk_position {
    	lib::utils::send_packet(&self.connection_stream, lib::packets::clientbound::play::SetCenterChunk::PACKET_ID, lib::packets::clientbound::play::SetCenterChunk {
	   		chunk_x: new_chunk_position.x,
	     	chunk_z: new_chunk_position.z,
     	}.try_into().unwrap()).unwrap();

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



      let dimension = &mut world.dimensions.get_mut("minecraft:overworld").unwrap();

      for chunk_coords in chunk_coords_to_send {
      	let chunk = dimension.get_chunk_from_chunk_position(Position { x: chunk_coords.0, y: 0, z: chunk_coords.1 });

	      let all_chunk_sections = if let Some(chunk) = chunk {
					&chunk.sections
				} else {
					let new_chunk = (*world.loader).load_chunk(chunk_coords.0, chunk_coords.1);
					dimension.chunks.push(new_chunk);
					&dimension.get_chunk_from_chunk_position(Position { x: chunk_coords.0, y: 0, z: chunk_coords.1 }).unwrap().sections
				};

	      let all_processed_chunk_sections = all_chunk_sections.iter().map(|section| {
	        lib::packets::clientbound::play::ChunkSection {
	          block_count: section.get_non_air_block_count(),
	          block_states: lib::packets::clientbound::play::BlockStatesPalettedContainer::Direct(lib::packets::clientbound::play::Direct {
	            bits_per_entry: 15,
	            data_array: if section.blocks.is_empty() { vec![0;4096] } else { section.blocks.iter().map(|x| *x as i32).collect() },
	          }),
	          biomes: lib::packets::clientbound::play::BiomesPalettedContainer::Direct(lib::packets::clientbound::play::Direct {
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

	      lib::utils::send_packet(&self.connection_stream, lib::packets::clientbound::play::ChunkDataAndUpdateLight::PACKET_ID, lib::packets::clientbound::play::ChunkDataAndUpdateLight {
	        chunk_x: chunk_coords.0,
	        chunk_z: chunk_coords.1,
	        heightmaps: vec![],
	        data: all_processed_chunk_sections,
	        block_entities: vec![],
	        sky_light_mask: vec![sky_light_mask],
	        block_light_mask: vec![block_light_mask],
	        empty_sky_light_mask: vec![!sky_light_mask],
	        empty_block_light_mask: vec![!block_light_mask],
	        sky_light_arrays,
	        block_light_arrays,
	      }.try_into().unwrap()).unwrap();
      }
    }
  }

  pub fn new_position_and_rotation(&mut self, x: f64, y: f64, z: f64, yaw: f32, pitch: f32, world: &mut World) {
    self.yaw = yaw;
    self.pitch = pitch;
 		self.new_position(x, y, z, world);
  }

  pub fn new_rotation(&mut self, yaw: f32, pitch: f32) {
    self.yaw = yaw;
    self.pitch = pitch;
  }
}
