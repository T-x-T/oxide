use std::{error::Error, fmt::{Display, Formatter, Result}};

pub mod serialize;
pub mod deserialize;
pub mod nbt;
pub mod utils;
pub mod packets;

#[derive(Debug, Clone)]
pub enum CustomError {
  ParseVarIntTooBig,
  ParseInvalidValue,
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return match self {
      CustomError::ParseVarIntTooBig => write!(f, "Tried to parse varint that was too large"),
      CustomError::ParseInvalidValue => write!(f, "Tried to parse invalid value"),
    }
  }
}

impl Error for CustomError {

}

#[derive(Debug, Clone)]
pub enum ConnectionStates {
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
}

#[derive(Debug, Clone)]
pub struct Datapack {
	pub namespace: String,
	pub id: String,
	pub version: String,
}