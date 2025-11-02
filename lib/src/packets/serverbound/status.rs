use super::*;

#[derive(Debug, Clone)]
pub struct StatusRequest {

}

impl Packet for StatusRequest {
	const PACKET_ID: u8 = 0x00;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Status }
}

impl TryFrom<StatusRequest> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(_value: StatusRequest) -> Result<Self, Box<dyn Error>> {
		return Ok(Vec::new());
	}
}

impl TryFrom<Vec<u8>> for StatusRequest {
	type Error = Box<dyn Error>;

	fn try_from(_value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(StatusRequest {

		});
	}
}


#[derive(Debug, Clone)]
pub struct PingRequest {
  pub timestamp: i64,
}

impl Packet for PingRequest {
	const PACKET_ID: u8 = 0x01;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Status }
}

impl TryFrom<PingRequest> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PingRequest) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::long(value.timestamp));

	  return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PingRequest {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(PingRequest {
      timestamp: crate::deserialize::long(&mut value)?,
		});
	}
}
