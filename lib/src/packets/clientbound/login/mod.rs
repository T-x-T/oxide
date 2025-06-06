use super::*;

#[derive(Debug, Clone)]
pub struct LoginSuccess {
	pub uuid: u128,
	pub username: String,
	//TODO: missing properties array: https://minecraft.wiki/w/Java_Edition_protocol#Login_Success
}

impl Packet for LoginSuccess {
	const PACKET_ID: u8 = 0x02;
  fn get_target() -> PacketTarget { PacketTarget::Client }
  fn get_state() -> ConnectionState { ConnectionState::Login }
}

impl TryFrom<LoginSuccess> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: LoginSuccess) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::uuid(&value.uuid));
		result.append(&mut crate::serialize::string(&value.username));
		result.push(0x00); //dont send properties array

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for LoginSuccess {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let uuid = crate::deserialize::uuid(&mut value)?;
		return Ok(LoginSuccess {
			uuid,
			username: crate::deserialize::string(&mut value)?,
		})
	}
}
