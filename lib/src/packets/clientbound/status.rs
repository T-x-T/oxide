use super::*;


#[derive(Debug, Clone)]
pub struct StatusResponse {
	pub status: String,
}

impl Packet for StatusResponse {
	const PACKET_ID: u8 = 0x00;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Status
	}
}

impl TryFrom<StatusResponse> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: StatusResponse) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::string(&value.status));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for StatusResponse {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(StatusResponse {
			status: crate::deserialize::string(&mut value)?,
		});
	}
}

#[derive(Debug, Clone)]
pub struct PingResponse {
	pub timestamp: i64,
}

impl Packet for PingResponse {
	const PACKET_ID: u8 = 0x01;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Status
	}
}

impl TryFrom<PingResponse> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PingResponse) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::long(value.timestamp));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for PingResponse {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(PingResponse {
			timestamp: crate::deserialize::long(&mut value)?,
		});
	}
}
