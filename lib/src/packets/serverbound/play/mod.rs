use super::*;

//
// MARK: 0x00 confirm teleportation
//

#[derive(Debug, Clone)]
pub struct ConfirmTeleportation {
	pub teleport_id: i32,
}

impl Packet for ConfirmTeleportation {
  fn get_id() -> u8 { 0x00 }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<ConfirmTeleportation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ConfirmTeleportation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.teleport_id));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for ConfirmTeleportation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(ConfirmTeleportation {
			teleport_id: crate::deserialize::varint(&mut value)?,
		})
	}
}

//
// MARK: 0x1c set player position
//

#[derive(Debug, Clone)]
pub struct SetPlayerPosition {
	pub x: f64,
	pub feet_y: f64,
	pub z: f64,
	pub flags: u8,
}

impl Packet for SetPlayerPosition {
  fn get_id() -> u8 { 0x1c }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerPosition) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::double(value.x));
		result.append(&mut crate::serialize::double(value.feet_y));
		result.append(&mut crate::serialize::double(value.z));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerPosition {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(SetPlayerPosition {
			x: crate::deserialize::double(&mut value)?,
			feet_y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			flags: value.remove(0),
		})
	}
}

//
// MARK: 0x1d set player position and rotation
//

#[derive(Debug, Clone)]
pub struct SetPlayerPositionAndRotation {
	pub x: f64,
	pub feet_y: f64,
	pub z: f64,
	pub yaw: f32,
	pub pitch: f32,
	pub flags: u8,
}

impl Packet for SetPlayerPositionAndRotation {
  fn get_id() -> u8 { 0x1d }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerPositionAndRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerPositionAndRotation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::double(value.x));
		result.append(&mut crate::serialize::double(value.feet_y));
		result.append(&mut crate::serialize::double(value.z));
		result.append(&mut crate::serialize::float(value.yaw));
		result.append(&mut crate::serialize::float(value.pitch));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerPositionAndRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			x: crate::deserialize::double(&mut value)?,
			feet_y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			flags: value.remove(0),
		})
	}
}

//
// MARK: 0x1e set player rotation
//

#[derive(Debug, Clone)]
pub struct SetPlayerRotation {
	pub yaw: f32,
	pub pitch: f32,
	pub flags: u8,
}

impl Packet for SetPlayerRotation {
  fn get_id() -> u8 { 0x1e }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerRotation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::float(value.yaw));
		result.append(&mut crate::serialize::float(value.pitch));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			flags: value.remove(0),
		})
	}
}
