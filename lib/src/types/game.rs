use std::{collections::HashMap, net::SocketAddr, sync::{atomic::AtomicI32, Arc, Mutex}};

use super::*;

pub struct Game {
  pub players: Vec<Player>,
  pub world: World,
  pub last_created_entity_id: AtomicI32,
  pub commands: Arc<Mutex<Vec<Command>>>,
  pub last_save_all_timestamp: std::time::Instant,
  pub block_state_data: Arc<std::collections::HashMap<String, data::blocks::Block>>,
  pub connections: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
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
