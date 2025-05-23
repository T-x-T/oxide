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

//
// MARK: 0x27 player action
//

#[derive(Debug, Clone)]
pub struct PlayerAction {
  pub status: i32,
  pub location: crate::Position,
  pub face: u8,
  pub sequence: i32,
}

impl Packet for PlayerAction {
  fn get_id() -> u8 { 0x27 }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<PlayerAction> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerAction) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.status));
		result.append(&mut crate::serialize::position(&value.location));
		result.push(value.face);
		result.append(&mut crate::serialize::varint(value.sequence));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for PlayerAction {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			status: crate::deserialize::varint(&mut value)?,
			location: crate::deserialize::position(&mut value)?,
			face: value.remove(0),
			sequence: crate::deserialize::varint(&mut value)?,
		})
	}
}

//
// MARK: 0x33 set hand item
//

#[derive(Debug, Clone)]
pub struct SetHandItem {
  pub slot: i16,
}

impl Packet for SetHandItem {
  fn get_id() -> u8 { 0x33 }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetHandItem> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetHandItem) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::short(value.slot));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetHandItem {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::short(&mut value)?,
		})
	}
}

//
// MARK: 0x36 set creative mode slot
//

#[derive(Debug, Clone)]
pub struct SetCreativeModeSlot {
  pub slot: i16,
  pub item: Slot,
}

impl Packet for SetCreativeModeSlot {
  fn get_id() -> u8 { 0x36 }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetCreativeModeSlot> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetCreativeModeSlot) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::short(value.slot));
		result.append(&mut crate::serialize::slot(&value.item));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetCreativeModeSlot {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::short(&mut value)?,
			item: crate::deserialize::slot(&mut value)?,
		})
	}
}

//
// MARK: 0x3e use item on
//

#[derive(Debug, Clone)]
pub struct UseItemOn {
  pub hand: i32,
  pub location: crate::Position,
  pub face: u8,
  pub cursor_position_x: f32,
  pub cursor_position_y: f32,
  pub cursor_position_z: f32,
  pub inside_block: bool,
  pub world_border_hit: bool,
  pub sequence: i32,
}

impl Packet for UseItemOn {
  fn get_id() -> u8 { 0x3e }
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<UseItemOn> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UseItemOn) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.hand));
		result.append(&mut crate::serialize::position(&value.location));
		result.push(value.face);
		result.append(&mut crate::serialize::float(value.cursor_position_x));
		result.append(&mut crate::serialize::float(value.cursor_position_y));
		result.append(&mut crate::serialize::float(value.cursor_position_z));
		result.append(&mut crate::serialize::boolean(value.inside_block));
		result.append(&mut crate::serialize::boolean(value.world_border_hit));
		result.append(&mut crate::serialize::varint(value.sequence));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for UseItemOn {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			hand: crate::deserialize::varint(&mut value)?,
			location: crate::deserialize::position(&mut value)?,
			face: value.remove(0),
			cursor_position_x: crate::deserialize::float(&mut value)?,
			cursor_position_y: crate::deserialize::float(&mut value)?,
			cursor_position_z: crate::deserialize::float(&mut value)?,
			inside_block: crate::deserialize::boolean(&mut value)?,
			world_border_hit: crate::deserialize::boolean(&mut value)?,
			sequence: crate::deserialize::varint(&mut value)?,
		})
	}
}
