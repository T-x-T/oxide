use super::*;

#[derive(Debug, Clone)]
pub struct Handshake {
	pub protocol_version: i32,
	pub server_address: String,
	pub sever_port: u16,
	pub next_state: HandshakeNextStates,
}

impl Packet for Handshake {
	const PACKET_ID: u8 = 0x00;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Handshaking }
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
			_ => return Err(Box::new(crate::CustomError::InvalidNextHandshakeState(value as u8))),
		}
	}
}

impl From<HandshakeNextStates> for crate::ConnectionState {
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

impl TryFrom<Handshake> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Handshake) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();
		output.append(&mut crate::serialize::varint(value.protocol_version));
		output.append(&mut crate::serialize::string(&value.server_address));
		output.append(&mut crate::serialize::unsigned_short(value.sever_port));
		output.append(&mut crate::serialize::varint(value.next_state as i32));

		return Ok(output);
	}
}
