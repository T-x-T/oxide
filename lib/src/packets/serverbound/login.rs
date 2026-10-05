use super::*;

#[derive(Debug, Clone)]
pub struct LoginStart {
	pub name: String,
	pub uuid: u128,
}

impl Packet for LoginStart {
	const PACKET_ID: u8 = 0x00;
	fn get_target() -> PacketTarget {
		PacketTarget::Server
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Login
	}
}

impl TryFrom<LoginStart> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: LoginStart) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::string(&value.name));
		result.append(&mut crate::serialize::uuid(&value.uuid));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for LoginStart {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(LoginStart { name: crate::deserialize::string(&mut value)?, uuid: crate::deserialize::uuid(&mut value)? });
	}
}

#[derive(Debug, Clone)]
pub struct LoginAcknowledged {}

impl Packet for LoginAcknowledged {
	const PACKET_ID: u8 = 0x03;
	fn get_target() -> PacketTarget {
		PacketTarget::Server
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Login
	}
}

impl TryFrom<LoginAcknowledged> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(_value: LoginAcknowledged) -> Result<Self, Box<dyn Error>> {
		let result: Vec<u8> = Vec::new();

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for LoginAcknowledged {
	type Error = Box<dyn Error>;

	fn try_from(_value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(LoginAcknowledged {});
	}
}
