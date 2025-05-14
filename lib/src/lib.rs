use std::{error::Error, fmt::{Display, Formatter, Result}};

pub use types::*;

pub mod serialize;
pub mod deserialize;
pub mod nbt;
pub mod utils;
pub mod packets;
pub mod blockstates;
pub mod types;

#[derive(Debug, Clone)]
pub enum CustomError {
  ParseVarIntTooBig,
  DeserializeInvalidBoolean(u8),
  InputEmpty,
  InvalidNbtTag(u8),
  InvalidNextHandshakeState(u8),
  ChunkNotFound(Position),
  PositionOutOfBounds(Position),
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return match self {
      CustomError::ParseVarIntTooBig => write!(f, "Tried to parse varint that was too large"),
      CustomError::DeserializeInvalidBoolean(x) => write!(f, "Tried to deserialize a boolean, but the input was broken af: {x}"),
      CustomError::InputEmpty => write!(f, "Input was empty, whoops"),
      CustomError::InvalidNbtTag(x) => write!(f, "Got a f*cked up value as a nbt tag: {x}"),
      CustomError::InvalidNextHandshakeState(x) => write!(f, "Got a next handshake state that doesn't exist (yet): {x}"),
      CustomError::ChunkNotFound(x) => write!(f, "couldn't find chunk at position {x:?}"),
      CustomError::PositionOutOfBounds(x) => write!(f, "tried to something at position {x:?}, but that is not within the bounds of possible locations"),
    }
  }
}

impl Error for CustomError {

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ConnectionState {
  #[default]
  Handshaking,
  Transfer,
  Status,
  Login,
  Configuration,
  Play,
}

#[derive(Debug, Clone)]
pub struct Packet {
  pub id: u8,
  pub length: u32,
  pub data: Vec<u8>,
  pub raw_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Datapack {
	pub namespace: String,
	pub id: String,
	pub version: String,
}
