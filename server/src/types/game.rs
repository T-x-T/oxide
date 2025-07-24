use super::*;

#[derive(Debug)]
pub struct Game {
  pub players: Vec<Player>,
  pub world: World,
  pub last_created_entity_id: i32,
  pub chat_message_index: i32,
  pub commands: Vec<Command>,
  pub last_save_all_timestamp: std::time::Instant,
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
