use crate::ConnectionState;

#[derive(Debug, Clone, Default)]
pub struct Connection {
  pub state: ConnectionState,
  pub player_name: Option<String>,
  pub player_uuid: Option<u128>,
}
