use std::{collections::HashMap, net::SocketAddr, sync::{atomic::AtomicI32, Mutex}};

use super::*;

pub struct Game {
  pub players: Mutex<Vec<Player>>,
  pub world: Mutex<World>,
  pub entity_id_manager: EntityIdManager,
  pub commands: Mutex<Vec<Command>>,
  pub last_save_all_timestamp: Mutex<std::time::Instant>,
  pub last_player_keepalive_timestamp: Mutex<std::time::Instant>,
  pub block_state_data: std::collections::HashMap<String, data::blocks::Block>,
  pub connections: Mutex<HashMap<SocketAddr, Connection>>,
}

impl Game {
  pub fn save_all(&self) {
    self.world.lock().unwrap().save_to_disk(&self.block_state_data);
    for player in self.players.lock().unwrap().iter() {
      player.save_to_disk();
    }
    *self.last_save_all_timestamp.lock().unwrap() = std::time::Instant::now();
  }
}

#[derive(Debug, Default)]
pub struct EntityIdManager(AtomicI32);

impl EntityIdManager {
  pub fn get_new(&self) -> i32 {
    self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    return self.0.load(std::sync::atomic::Ordering::SeqCst);
  }
}
