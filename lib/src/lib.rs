#![allow(clippy::needless_return, clippy::too_many_arguments)]

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub use types::*;

pub mod block;
pub mod blockentities;
pub mod containerclick;
pub mod deserialize;
pub mod entity;
pub mod packets;
pub mod serialize;
pub mod types;
pub mod utils;

pub const SPAWN_CHUNK_RADIUS: i8 = 10;
pub const VIEW_DISTANCE: i8 = 10;
pub const ITEM_PICKUP_DISTANCE: f64 = 1.0;

#[derive(Debug, Clone)]
pub enum CustomError {
	ParseVarIntTooBig,
	DeserializeInvalidBoolean(u8),
	InputEmpty,
	InvalidNbtTag(u8),
	InvalidNextHandshakeState(u8),
	ChunkNotFound(BlockPosition),
	PositionOutOfBounds(BlockPosition),
	InvalidInput(String),
	TriedParsingUnknown(String),
	BlockEntityNotFoundAtLocation(BlockPosition),
	NotActuallyAnErrorButPleaseDisconnectMe,
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
			CustomError::PositionOutOfBounds(x) => {
				write!(f, "tried to something at position {x:?}, but that is not within the bounds of possible locations")
			}
			CustomError::InvalidInput(x) => write!(f, "whatever the input was, was not good. that was the non good input: {x}"),
			CustomError::TriedParsingUnknown(x) => {
				write!(f, "whatever the input was, we did not know what it was supposed to represent. This is the unknown input: {x}")
			}
			CustomError::BlockEntityNotFoundAtLocation(x) => {
				write!(f, "tried accessing a block_entity at {x:?}, but there was no block_entity at {x:?}")
			}
			CustomError::NotActuallyAnErrorButPleaseDisconnectMe => write!(
				f,
				"yeah this is just a normal disconnect, that we are somehow treating as an error, because it is cleaner or so i dont know :cry:"
			),
		};
	}
}

impl Error for CustomError {}

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
