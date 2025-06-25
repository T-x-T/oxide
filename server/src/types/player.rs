use super::*;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Player {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub display_name: String,
  pub uuid: u128,
  pub peer_socket_address: SocketAddr,
  pub entity_id: i32,
  pub waiting_for_confirm_teleportation: bool,
  pub current_teleport_id: i32,
  pub inventory: Vec<Slot>,
  pub selected_slot: u8,
}

impl Player {
  pub fn new(position: Position, display_name: String, uuid: u128, peer_socket_address: SocketAddr, game: &mut Game) -> Self {
    let player = Self {
      x: position.x as f64,
      y: position.y as f64,
      z: position.z as f64,
      yaw: 0.0,
      pitch: 0.0,
      display_name,
      uuid,
      peer_socket_address,
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

  pub fn new_position(&mut self, x: f64, y: f64, z: f64) {
  	let old_x = self.x;
   	let old_z = self.z;

  	self.x = x;
   	self.y = y;
    self.z = z;

    let old_chunk_position = Position {x: old_x as i32, y: 0, z: old_z as i32}.convert_to_coordinates_of_chunk();
    let new_chunk_position = Position {x: self.x as i32, y: 0, z: self.z as i32}.convert_to_coordinates_of_chunk();

    if old_chunk_position != new_chunk_position {

    }
  }

  pub fn new_position_and_rotation(&mut self, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) {
    self.yaw = yaw;
    self.pitch = pitch;
 		self.new_position(x, y, z);
  }

  pub fn new_rotation(&mut self, yaw: f32, pitch: f32) {
    self.yaw = yaw;
    self.pitch = pitch;
  }
}
