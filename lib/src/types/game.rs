use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;
use std::sync::atomic::AtomicI32;
use std::sync::mpsc::Sender;

use dashmap::{DashMap, DashSet};

use super::*;

type RawPacket = (u8, Vec<u8>);

pub struct Game {
	pub players: Mutex<Vec<Player>>,
	pub world: Mutex<World>,
	pub entity_id_manager: EntityIdManager,
	pub commands: Mutex<Vec<Command>>,
	pub last_save_all_timestamp: Mutex<std::time::Instant>,
	pub last_player_keepalive_timestamp: Mutex<std::time::Instant>,
	pub block_state_data: HashMap<String, basic_types::blocks::Block>,
	pub connections: DashMap<SocketAddr, Connection>,
	pub packet_handler_actions: Mutex<Vec<PacketHandlerAction>>,
	pub default_gamemode: Gamemode,
	pub loot_tables: HashMap<&'static str, HashMap<&'static str, loot_table::LootTable>>,
	pub recipe_manager: RecipeManager,
	pub packet_sender: PacketSender,
	pub task_queue: DashSet<Task>,
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
	UseItemOn(SocketAddr, crate::packets::serverbound::play::UseItemOn),
	SendChatMessage(SocketAddr, String, i64, i64), //Message, timestamp, salt
	SendCommand(SocketAddr, String),
	ClickContainer(SocketAddr, crate::packets::serverbound::play::ClickContainer),
	CloseContainer(SocketAddr, i32),
	UpdateSign(SocketAddr, BlockPosition, bool, [String; 4]),
	PlayerInput(SocketAddr, crate::packets::serverbound::play::PlayerInput),
	Interact(SocketAddr, crate::packets::serverbound::play::Interact),
	NewPlayer(SocketAddr, TcpStream),
	UpdateGamemode(SocketAddr, crate::types::Gamemode),
	Respawn(SocketAddr),
	UseItem(SocketAddr, crate::packets::serverbound::play::UseItem),
}

#[derive(Debug, Default)]
pub struct EntityIdManager(AtomicI32);

impl EntityIdManager {
	pub fn get_new(&self) -> i32 {
		return self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
	}
}

#[derive(Debug, Default)]
pub struct PacketSender {
	pub packet_send_queues: DashMap<SocketAddr, Sender<RawPacket>>,
}

impl PacketSender {
	pub fn send_packet_to_player<T>(&self, peer_addr: &SocketAddr, packet_id: u8, packet_data: T)
	where
		T: TryInto<Vec<u8>> + Clone,
		T::Error: std::fmt::Debug,
	{
		self.packet_send_queues.get(peer_addr).unwrap().send((packet_id, packet_data.try_into().unwrap())).unwrap();
	}

	pub fn send_packet_to_everyone<T>(&self, players: &[Player], packet_id: u8, packet_data: T)
	where
		T: TryInto<Vec<u8>> + Clone,
		T::Error: std::fmt::Debug,
	{
		for player in players {
			self.packet_send_queues.get(&player.peer_socket_address).unwrap().send((packet_id, packet_data.clone().try_into().unwrap())).unwrap();
		}
	}

	pub fn send_packet_to_everyone_in_dimension<T>(&self, players: &[Player], dimension_name: &str, packet_id: u8, packet_data: T)
	where
		T: TryInto<Vec<u8>> + Clone,
		T::Error: std::fmt::Debug,
	{
		players.iter().filter(|x| x.get_dimension() == dimension_name).for_each(|x| {
			self.packet_send_queues.get(&x.peer_socket_address).unwrap().send((packet_id, packet_data.clone().try_into().unwrap())).unwrap()
		});
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Task {
	PlayerChangeDimension(u128, String),
}
