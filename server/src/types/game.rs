use super::*;

#[derive(Debug, Clone)]
pub struct Game {
  pub players: Vec<Player>,
  pub world: World,
  pub last_created_entity_id: i32,
  pub chat_message_index: i32,
}
