use std::error::Error;
use crate::{nbt::NbtTag, ConnectionState};

pub mod serverbound;
pub mod clientbound;

pub enum PacketTarget {
    Client,
    Server,
}

pub trait Packet {
    fn get_id() -> u8;
    fn get_target() -> PacketTarget;
    fn get_state() -> ConnectionState;
}

pub fn get_protocol_version() -> i32 {
    return 770;
}

pub fn get_version_string() -> String {
    return "1.21.5".to_string();
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub x: i32,
  pub y: i16,
  pub z: i32,
}
