use std::{error::Error, fmt::{Display, Formatter, Result}};

pub mod serialize;
pub mod deserialize;
pub mod nbt;
pub mod utils;
pub mod packets;

#[derive(Debug, Clone)]
pub enum CustomError {
  ParseVarIntTooBig,
  DeserializeInvalidBoolean(u8),
  InputEmpty,
  InvalidNbtTag(u8),
  InvalidNextHandshakeState(u8),
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return match self {
      CustomError::ParseVarIntTooBig => write!(f, "Tried to parse varint that was too large"),
      CustomError::DeserializeInvalidBoolean(x) => write!(f, "Tried to deserialize a boolean, but the input was broken af: {x}"),
      CustomError::InputEmpty => write!(f, "Input was empty, whoops"),
      CustomError::InvalidNbtTag(x) => write!(f, "Got a f*cked up value as a nbt tag: {x}"),
      CustomError::InvalidNextHandshakeState(x) => write!(f, "Got a next handshake state that doesn't exist (yet): {x}"),
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