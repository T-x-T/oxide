use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use super::*;

pub struct Game {
  pub players: Vec<Player>,
  pub world: World,
  pub last_created_entity_id: i32,
  pub chat_message_index: i32,
  pub commands: Vec<Command>,
  pub last_save_all_timestamp: std::time::Instant,
  pub block_state_data: std::collections::HashMap<String, data::blocks::Block>,
  pub connections: Arc<Mutex<HashMap<SocketAddr, Connection>>>
}

impl Game {
  pub fn save_all(&mut self) {
    self.world.save_to_disk();
    for player in &self.players {
      player.save_to_disk();
    }
    self.last_save_all_timestamp = std::time::Instant::now();
  }
}
