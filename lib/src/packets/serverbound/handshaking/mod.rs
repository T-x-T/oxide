use super::*;

#[derive(Debug, Clone)]
pub struct Handshake {
	pub protocol_version: i32,
	pub server_address: String,
	pub sever_port: u16,
	pub next_state: HandshakeNextStates,
}

#[derive(Debug, Clone)]
pub enum HandshakeNextStates {
	Status = 1,
	Login = 2,
	Transfer = 3,
}

impl TryFrom<i32> for HandshakeNextStates {
	type Error = Box<dyn Error>;
	
	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(HandshakeNextStates::Status),
			2 => Ok(HandshakeNextStates::Login),
			3 => Ok(HandshakeNextStates::Transfer),
			_ => return Err(Box::new(crate::CustomError::ParseInvalidValue)),
		}
	}
}

impl From<HandshakeNextStates> for crate::ConnectionStates {
	fn from(value: HandshakeNextStates) -> Self {
		match value {
			HandshakeNextStates::Status => return Self::Status,
			HandshakeNextStates::Login => return Self::Login,
			HandshakeNextStates::Transfer => return Self::Transfer,
		}
	}
}

impl TryFrom<Vec<u8>> for Handshake {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Handshake {
			protocol_version: crate::deserialize::varint(&mut value)?,
			server_address: crate::deserialize::string(&mut value)?,
			sever_port: crate::deserialize::unsigned_short(&mut value)?,
			next_state: crate::deserialize::varint(&mut value)?.try_into()?,
		})
	}
}