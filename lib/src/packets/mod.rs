use std::error::Error;
use crate::ConnectionState;

pub mod serverbound;
pub mod clientbound;

pub enum PacketTarget {
  Client,
  Server,
}

pub trait Packet {
	const PACKET_ID: u8;
  fn get_target() -> PacketTarget;
  fn get_state() -> ConnectionState;
}

pub fn get_protocol_version() -> i32 {
  return 773;
}

pub fn get_version_string() -> String {
  return "1.21.10".to_string();
}
