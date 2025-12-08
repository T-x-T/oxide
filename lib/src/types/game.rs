use std::{collections::HashMap, net::{SocketAddr, TcpStream}, sync::{Mutex, atomic::AtomicI32}};

use dashmap::DashMap;

use super::*;

type RawPacket = (u8, Vec<u8>);

pub struct Game {
  pub players: Mutex<Vec<Player>>,
  pub world: Mutex<World>,
  pub entity_id_manager: EntityIdManager,
  pub commands: Mutex<Vec<Command>>,
  pub last_save_all_timestamp: Mutex<std::time::Instant>,
  pub last_player_keepalive_timestamp: Mutex<std::time::Instant>,
  pub block_state_data: HashMap<String, data::blocks::Block>,
  pub connections: DashMap<SocketAddr, Connection>,
  pub packet_handler_actions: Mutex<Vec<PacketHandlerAction>>,
  pub packet_send_queues: DashMap<SocketAddr, Vec<RawPacket>>,
}

impl Game {
  pub fn save_all(&self) {
    self.world.lock().unwrap().save_to_disk(&self.block_state_data);
    for player in self.players.lock().unwrap().iter() {
      player.save_to_disk();
    }
    *self.last_save_all_timestamp.lock().unwrap() = std::time::Instant::now();
  }

  //TODO: maybe move this into something similar to EntityIdManager, so we dont have to pass a reference to entire Game struct _everywhere_
  pub fn send_packet(&self, peer_addr: &SocketAddr, packet_id: u8, packet_data: Vec<u8>) {
    self.packet_send_queues.entry(*peer_addr).or_default().push((packet_id, packet_data));
  }
}


#[derive(Debug)]
pub enum PacketHandlerAction {
	DisconnectPlayer(SocketAddr),
	MovePlayer(SocketAddr, Option<(f64, f64, f64)>, Option<(f32, f32)>), //(x,y,z), (yaw,pitch)
	ConfirmTeleportation(SocketAddr, i32),
	SetCreativeModeSlot(SocketAddr, u8, Option<Slot>),
	SetSelectedSlot(SocketAddr, u8),
	PickItemFromBlock(SocketAddr, BlockPosition, bool), //bool = include_data
	SwingArm(SocketAddr, u8),
	BreakBlock(SocketAddr, u8, BlockPosition, u8, i32), //status, location, face, sequence
	UseItemOn(SocketAddr, u8, BlockPosition, u8, f32, f32, f32, bool, bool, i32), //hand, location, face, cursor_position x,y,z, inside_block, world_border_hit, sequence
	SendChatMessage(SocketAddr, String, i64, i64), //Message, timestamp, salt
	SendCommand(SocketAddr, String),
	ClickContainer(SocketAddr, crate::packets::serverbound::play::ClickContainer),
	CloseContainer(SocketAddr, i32),
	UpdateSign(BlockPosition, bool, [String;4]),
	PlayerInput(SocketAddr, crate::packets::serverbound::play::PlayerInput),
	Interact(SocketAddr, crate::packets::serverbound::play::Interact),
	NewPlayer(SocketAddr, TcpStream),
}

#[derive(Debug, Default)]
pub struct EntityIdManager(AtomicI32);

impl EntityIdManager {
  pub fn get_new(&self) -> i32 {
    return self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
  }
}
